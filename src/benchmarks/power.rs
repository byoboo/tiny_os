//! Power Measurement Interface for Raspberry Pi
//!
//! This module provides power monitoring capabilities for efficiency
//! measurement. Focuses on Pi 4/5 advanced power states while maintaining Pi 3B
//! compatibility.

use crate::benchmarks::timing;

/// Power measurement state
pub struct PowerMonitor {
    /// Current frequency setting
    frequency: u64,
    /// Power state tracking
    power_state: PowerState,
    /// Measurement calibrated
    calibrated: bool,
}

/// Raspberry Pi power states
#[derive(Debug, Clone, Copy)]
pub enum PowerState {
    /// Maximum performance
    HighPerformance,
    /// Balanced performance/power
    Balanced,
    /// Power saving mode
    PowerSave,
    /// Deep sleep (Pi 4/5 advanced)
    DeepSleep,
}

impl PowerMonitor {
    /// Create new power monitor
    pub fn new() -> Self {
        Self {
            frequency: 1200_000_000, // Default 1.2GHz (Pi 3B base)
            power_state: PowerState::Balanced,
            calibrated: false,
        }
    }

    /// Initialize power monitoring
    pub fn initialize(&mut self) {
        self.calibrated = true;

        // Set initial power state for optimal benchmarking
        self.set_power_state(PowerState::HighPerformance);
    }

    /// Set power state (Pi 4/5 optimized)
    pub fn set_power_state(&mut self, state: PowerState) {
        self.power_state = state;

        match state {
            PowerState::HighPerformance => {
                // Pi 4: Up to 1.5GHz, Pi 5: Up to 2.4GHz
                self.frequency = self.get_max_frequency();
            }
            PowerState::Balanced => {
                // Conservative frequency for balanced operation
                self.frequency = 1_000_000_000; // 1.0GHz
            }
            PowerState::PowerSave => {
                // Minimum frequency for power saving
                self.frequency = 600_000_000; // 600MHz
            }
            PowerState::DeepSleep => {
                // Pi 4/5 advanced power management
                self.frequency = 200_000_000; // 200MHz minimal
            }
        }
    }

    /// Get maximum frequency based on Pi model
    fn get_max_frequency(&self) -> u64 {
        // Pi 3B: 1.2GHz, Pi 4: 1.5GHz, Pi 5: 2.4GHz
        // For development, assume Pi 4/5 capabilities
        1_500_000_000 // 1.5GHz (Pi 4 target)
    }

    /// Measure power efficiency of operation
    pub fn measure_power_efficiency<F>(&self, operation: F) -> PowerMeasurement
    where
        F: FnOnce(),
    {
        let start_time = timing::get_cycles();

        // Execute operation
        operation();

        let end_time = timing::get_cycles();
        let cycles = end_time.saturating_sub(start_time);

        PowerMeasurement {
            cycles,
            frequency: self.frequency,
            power_state: self.power_state,
            estimated_energy: self.estimate_energy_consumption(cycles),
        }
    }

    /// Estimate energy consumption (micro-Joules)
    fn estimate_energy_consumption(&self, cycles: u64) -> u64 {
        // Simplified power model based on Pi specifications
        let base_power_mw = match self.power_state {
            PowerState::HighPerformance => 7500, // ~7.5W (Pi 4/5 max)
            PowerState::Balanced => 4000,        // ~4W typical
            PowerState::PowerSave => 2000,       // ~2W power save
            PowerState::DeepSleep => 500,        // ~0.5W deep sleep
        };

        // Convert cycles to time (microseconds)
        let time_us = (cycles * 1_000_000) / self.frequency;

        // Energy = Power × Time (microjoules)
        (base_power_mw * time_us) / 1000
    }

    /// Get current power state
    pub fn get_power_state(&self) -> PowerState {
        self.power_state
    }

    /// Get current frequency
    pub fn get_frequency(&self) -> u64 {
        self.frequency
    }

    /// Test power monitoring capabilities
    pub fn test_power_monitoring(&mut self) -> PowerTestResults {
        if !self.calibrated {
            self.initialize();
        }

        let mut results = PowerTestResults::new();

        // Test different power states
        for &state in &[
            PowerState::HighPerformance,
            PowerState::Balanced,
            PowerState::PowerSave,
        ] {
            self.set_power_state(state);

            let measurement = self.measure_power_efficiency(|| {
                // Standard test workload
                let mut total = 0u64;
                for i in 0..1000 {
                    total = total.wrapping_add(i);
                }
                core::hint::black_box(total);
            });

            results.add_measurement(state, measurement);
        }

        results
    }
}

/// Power measurement result
#[derive(Debug, Clone, Copy)]
pub struct PowerMeasurement {
    /// Cycles consumed
    pub cycles: u64,
    /// CPU frequency during measurement
    pub frequency: u64,
    /// Power state during measurement
    pub power_state: PowerState,
    /// Estimated energy consumption (μJ)
    pub estimated_energy: u64,
}

impl PowerMeasurement {
    /// Calculate performance per watt
    pub fn performance_per_watt(&self) -> u64 {
        if self.estimated_energy > 0 {
            // Operations per joule (scaled)
            1_000_000 / self.estimated_energy
        } else {
            0
        }
    }

    /// Get execution time in microseconds
    pub fn execution_time_us(&self) -> u64 {
        if self.frequency > 0 {
            (self.cycles * 1_000_000) / self.frequency
        } else {
            0
        }
    }
}

/// Power test results collection
pub struct PowerTestResults {
    measurements: [(PowerState, PowerMeasurement); 8],
    count: usize,
}

impl PowerTestResults {
    pub fn new() -> Self {
        Self {
            measurements: [(
                PowerState::Balanced,
                PowerMeasurement {
                    cycles: 0,
                    frequency: 0,
                    power_state: PowerState::Balanced,
                    estimated_energy: 0,
                },
            ); 8],
            count: 0,
        }
    }

    pub fn add_measurement(&mut self, state: PowerState, measurement: PowerMeasurement) {
        if self.count < 8 {
            self.measurements[self.count] = (state, measurement);
            self.count += 1;
        }
    }

    /// Get measurement for specific power state
    pub fn get_measurement(&self, state: PowerState) -> Option<&PowerMeasurement> {
        for i in 0..self.count {
            if self.measurements[i].0 as u8 == state as u8 {
                return Some(&self.measurements[i].1);
            }
        }
        None
    }

    /// Calculate efficiency improvement vs balanced mode
    pub fn efficiency_improvement(&self) -> Option<f32> {
        let balanced = self.get_measurement(PowerState::Balanced)?;
        let high_perf = self.get_measurement(PowerState::HighPerformance)?;

        let balanced_efficiency = balanced.performance_per_watt() as f32;
        let high_perf_efficiency = high_perf.performance_per_watt() as f32;

        if balanced_efficiency > 0.0 {
            Some((high_perf_efficiency - balanced_efficiency) / balanced_efficiency * 100.0)
        } else {
            None
        }
    }

    /// Get all measurements
    pub fn iter(&self) -> impl Iterator<Item = &(PowerState, PowerMeasurement)> {
        self.measurements[..self.count].iter()
    }
}

/// Test the power monitoring system
pub fn test_power_monitoring() -> PowerTestResults {
    let mut monitor = PowerMonitor::new();
    monitor.test_power_monitoring()
}

/// Quick power measurement for a given operation
pub fn measure_operation_power<F>(operation: F) -> PowerMeasurement
where
    F: FnOnce(),
{
    let mut monitor = PowerMonitor::new();
    monitor.initialize();
    monitor.measure_power_efficiency(operation)
}
