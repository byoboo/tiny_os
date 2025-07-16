//! Thermal Management Controller
//!
//! Thermal monitoring and control
//! Extracted from Week 4 implementation

use super::PerformanceError;

/// Thermal status
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThermalStatus {
    Normal,
    Warning,
    Critical,
    Emergency,
}

/// Thermal controller
pub struct ThermalController {
    current_temp_celsius: u8,
    max_temp_celsius: u8,
    throttle_temp_celsius: u8,
    status: ThermalStatus,
}

impl ThermalController {
    pub fn new() -> Self {
        Self {
            current_temp_celsius: 45,  // Default Pi temperature
            max_temp_celsius: 85,      // Pi thermal limit
            throttle_temp_celsius: 70, // Start throttling
            status: ThermalStatus::Normal,
        }
    }

    /// Initialize thermal management
    pub fn init(&mut self) -> Result<(), PerformanceError> {
        // Placeholder for thermal sensor initialization
        Ok(())
    }

    /// Read current temperature
    pub fn read_temperature(&mut self) -> Result<u8, PerformanceError> {
        // Placeholder for actual temperature reading
        // Would read from BCM2835 thermal sensor
        self.update_status();
        Ok(self.current_temp_celsius)
    }

    /// Update thermal status
    fn update_status(&mut self) {
        self.status = match self.current_temp_celsius {
            temp if temp < self.throttle_temp_celsius => ThermalStatus::Normal,
            temp if temp < self.max_temp_celsius => ThermalStatus::Warning,
            temp if temp < self.max_temp_celsius + 5 => ThermalStatus::Critical,
            _ => ThermalStatus::Emergency,
        };
    }

    /// Get thermal status
    pub fn get_status(&self) -> ThermalStatus {
        self.status
    }

    /// Get current temperature
    pub fn get_current_temperature(&self) -> u8 {
        self.current_temp_celsius
    }

    /// Check if throttling is needed
    pub fn needs_throttling(&self) -> bool {
        self.current_temp_celsius >= self.throttle_temp_celsius
    }

    /// Set throttle temperature
    pub fn set_throttle_temperature(&mut self, temp_celsius: u8) -> Result<(), PerformanceError> {
        if temp_celsius > self.max_temp_celsius {
            return Err(PerformanceError::InvalidConfiguration);
        }

        self.throttle_temp_celsius = temp_celsius;
        Ok(())
    }

    /// Get thermal metrics
    pub fn get_thermal_metrics(&self) -> ThermalMetrics {
        ThermalMetrics {
            current_temp_celsius: self.current_temp_celsius,
            max_temp_celsius: self.max_temp_celsius,
            throttle_temp_celsius: self.throttle_temp_celsius,
            status: self.status,
            throttling_active: self.needs_throttling(),
        }
    }
}

/// Thermal metrics
#[derive(Debug)]
pub struct ThermalMetrics {
    pub current_temp_celsius: u8,
    pub max_temp_celsius: u8,
    pub throttle_temp_celsius: u8,
    pub status: ThermalStatus,
    pub throttling_active: bool,
}
