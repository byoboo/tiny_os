//! Performance Tests for TinyOS
//! 
//! This module contains performance benchmarks and stress tests to ensure
//! TinyOS meets performance requirements and handles load effectively.

use super::mocks::*;
use super::{TestState, TestConfig};
use std::time::{Duration, Instant};

/// Run all performance tests
pub fn run_performance_tests(test_state: &TestState, config: &TestConfig) {
    println!("⚡ Performance Tests");
    println!("--------------------");
    
    // Memory Performance Tests
    test_memory_performance(test_state, config);
    
    // UART Performance Tests
    test_uart_performance(test_state, config);
    
    // GPIO Performance Tests
    test_gpio_performance(test_state, config);
    
    // Interrupt Performance Tests
    test_interrupt_performance(test_state, config);
    
    // System-wide Performance Tests
    test_system_performance(test_state, config);
}

/// Memory Performance Tests
fn test_memory_performance(test_state: &TestState, _config: &TestConfig) {
    println!("\n🧠 Memory Performance:");
    
    crate::test_case!("Memory Allocation Speed", test_state, || -> Result<(), &'static str> {
        let mut memory = MockMemoryManager::new(0x100000, 1024 * 1024, 64);
        let start = Instant::now();
        
        // Allocate 1000 blocks
        let mut addresses = Vec::new();
        for _ in 0..1000 {
            if let Some(addr) = memory.allocate(64) {
                addresses.push(addr);
            }
        }
        
        let duration = start.elapsed();
        let allocations_per_ms = (1000.0 / duration.as_millis() as f64) as u32;
        
        // Should be able to allocate at least 100 blocks per millisecond
        if allocations_per_ms >= 100 && addresses.len() == 1000 {
            Ok(())
        } else {
            Err("Memory allocation too slow")
        }
    });
    
    crate::test_case!("System Load Test", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        let start = Instant::now();
        
        // Simulate heavy system load for performance testing
        for load_cycle in 0..100 {
            // Memory operations
            let mut temp_addrs = Vec::new();
            for _ in 0..5 {
                if let Some(addr) = system.memory.allocate(64) {
                    temp_addrs.push(addr);
                }
            }
            for addr in temp_addrs {
                system.memory.free(addr);
            }
            
            // GPIO operations
            system.gpio.toggle_pin(42).ok();
            
            // UART operations
            system.uart.write_string("test").ok();
            
            // Interrupt simulation
            system.interrupts.trigger_interrupt(64);
        }
        
        let duration = start.elapsed();
        
        // Should complete load test in reasonable time
        if duration.as_millis() < 50 {
            let health = system.run_system_health_check()?;
            if health.health_score() >= 80.0 {
                Ok(())
            } else {
                Err("System health degraded under load")
            }
        } else {
            Err("System load test too slow")
        }
    });
}

/// UART Performance Tests
fn test_uart_performance(test_state: &TestState, _config: &TestConfig) {
    println!("\n🔌 UART Performance:");
    
    crate::test_case!("UART Throughput Test", test_state, || -> Result<(), &'static str> {
        let mut uart = MockUart::new();
        let start = Instant::now();
        
        // Send 1KB of data
        let test_data = "A".repeat(1024);
        uart.write_string(&test_data)?;
        
        let duration = start.elapsed();
        let bytes_per_ms = (1024.0 / duration.as_millis() as f64) as u32;
        
        // Should achieve reasonable throughput
        if bytes_per_ms >= 100 {
            let output = uart.get_output();
            if output.len() == 1024 {
                Ok(())
            } else {
                Err("UART output length incorrect")
            }
        } else {
            Err("UART throughput too low")
        }
    });
}

/// GPIO Performance Tests
fn test_gpio_performance(test_state: &TestState, _config: &TestConfig) {
    println!("\n📌 GPIO Performance:");
    
    crate::test_case!("GPIO Pin Toggle Speed", test_state, || -> Result<(), &'static str> {
        let mut gpio = MockGpio::new();
        gpio.set_pin_mode(42, GpioMode::Output)?;
        
        let start = Instant::now();
        
        // Toggle pin 100 times
        for _ in 0..100 {
            gpio.toggle_pin(42)?;
        }
        
        let duration = start.elapsed();
        let toggles_per_ms = (100.0 / duration.as_millis() as f64) as u32;
        
        // Should achieve reasonable toggle speed
        if toggles_per_ms >= 50 {
            Ok(())
        } else {
            Err("GPIO toggle speed too slow")
        }
    });
}

/// Interrupt Performance Tests
fn test_interrupt_performance(test_state: &TestState, _config: &TestConfig) {
    println!("\n⚡ Interrupt Performance:");
    
    crate::test_case!("Interrupt Throughput", test_state, || -> Result<(), &'static str> {
        let mut controller = MockInterruptController::new();
        controller.enable_interrupt(64)?;
        
        let start = Instant::now();
        
        // Generate 100 interrupts
        for _ in 0..100 {
            controller.trigger_interrupt(64);
        }
        
        let duration = start.elapsed();
        let interrupts_per_ms = (100.0 / duration.as_millis() as f64) as u32;
        
        // Should handle reasonable interrupt rate
        if interrupts_per_ms >= 50 {
            let total = controller.get_total_interrupts();
            if total == 100 {
                Ok(())
            } else {
                Err("Interrupt count mismatch")
            }
        } else {
            Err("Interrupt throughput too low")
        }
    });
}

/// System-wide Performance Tests
fn test_system_performance(test_state: &TestState, _config: &TestConfig) {
    println!("\n🖥️ System Performance:");
    
    crate::test_case!("System Boot Performance", test_state, || -> Result<(), &'static str> {
        let start = Instant::now();
        
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        let boot_duration = start.elapsed();
        
        // Boot should complete in reasonable time
        if boot_duration.as_millis() < 10 {
            let health = system.run_system_health_check()?;
            if health.all_healthy() {
                Ok(())
            } else {
                Err("System not healthy after boot")
            }
        } else {
            Err("System boot too slow")
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_framework() {
        let test_state = TestState::new();
        let config = TestConfig::default();
        
        // Run a subset of performance tests
        test_memory_performance(&test_state, &config);
        
        let summary = test_state.get_summary();
        assert!(summary.total_tests > 0);
    }
}
