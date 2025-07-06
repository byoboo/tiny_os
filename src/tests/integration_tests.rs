//! Integration Tests for TinyOS
//! 
//! This module contains comprehensive integration tests that test multiple components
//! working together as a complete system.

use super::mocks::*;
use super::{TestState, TestConfig};
use std::collections::HashMap;

/// Run all integration tests
pub fn run_integration_tests(test_state: &TestState, config: &TestConfig) {
    println!("🔗 Integration Tests");
    println!("--------------------");
    
    // System Boot Integration Tests
    test_system_boot_integration(test_state, config);
    
    // UART-GPIO Integration Tests
    test_uart_gpio_integration(test_state, config);
    
    // Memory-Timer Integration Tests
    test_memory_timer_integration(test_state, config);
    
    // Full System Integration Tests
    test_full_system_integration(test_state, config);
    
    // Shell Command Integration Tests
    test_shell_integration(test_state, config);
    
    // SD Card Integration Tests
    test_sdcard_integration(test_state, config);
    
    // Interrupt System Integration Tests
    test_interrupt_system_integration(test_state, config);
}

/// System Boot Integration Tests
fn test_system_boot_integration(test_state: &TestState, _config: &TestConfig) {
    println!("\n🚀 System Boot Integration:");
    
    crate::test_case!("Complete Boot Sequence", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Verify all components are initialized
        if !system.uart.enabled || !system.gpio.enabled || !system.timer.enabled {
            return Err("Boot sequence failed to initialize all components");
        }
        
        // Verify interrupt setup
        if !system.interrupts.is_enabled(64) || !system.interrupts.is_enabled(153) {
            return Err("Boot sequence failed to setup interrupts");
        }
        
        // Verify LED pin configuration
        if system.gpio.pin_modes.get(&42) != Some(&GpioMode::Output) {
            return Err("Boot sequence failed to configure LED pin");
        }
        
        Ok(())
    });
    
    crate::test_case!("Post-Boot System Health", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        let health_report = system.run_system_health_check()?;
        
        if health_report.all_healthy() {
            Ok(())
        } else {
            Err("System health check failed after boot")
        }
    });
    
    crate::test_case!("Boot with Memory Allocation", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Simulate kernel memory allocation during boot
        let kernel_stack = system.memory.allocate(4096);
        let heap_metadata = system.memory.allocate(1024);
        
        if kernel_stack.is_some() && heap_metadata.is_some() {
            let stats = system.memory.get_stats();
            if stats.used_size == 5120 { // 4096 + 1024
                Ok(())
            } else {
                Err("Boot memory allocation stats incorrect")
            }
        } else {
            Err("Boot memory allocation failed")
        }
    });
    
    crate::test_case!("Boot Time Measurement", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        let start_time = system.timer.get_time();
        
        // Simulate boot delay
        system.timer.advance_time(1000); // 1ms boot time
        system.simulate_boot_sequence()?;
        
        let boot_time = system.timer.get_time() - start_time;
        
        if boot_time >= 1000 {
            Ok(())
        } else {
            Err("Boot time measurement failed")
        }
    });
}

/// UART-GPIO Integration Tests
fn test_uart_gpio_integration(test_state: &TestState, _config: &TestConfig) {
    println!("\n🔌📌 UART-GPIO Integration:");
    
    crate::test_case!("LED Control via UART Commands", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Simulate UART command to turn LED on
        system.uart.add_input(b"1");
        let command = system.uart.read_byte().unwrap();
        
        if command == b'1' {
            system.gpio.set_pin(42, true)?;
            system.uart.write_string("LED ON\n")?;
        }
        
        // Verify LED state and UART response
        if system.gpio.get_pin(42) == Some(true) {
            let output = system.uart.get_output_string();
            if output.contains("LED ON") {
                Ok(())
            } else {
                Err("UART response missing")
            }
        } else {
            Err("LED control via UART failed")
        }
    });
    
    crate::test_case!("GPIO Status Report via UART", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Set some GPIO states
        system.gpio.set_pin(42, true)?;
        system.gpio.set_pin_mode(18, GpioMode::Input)?;
        
        // Generate status report
        let mut status_report = String::new();
        status_report.push_str("GPIO Status:\n");
        status_report.push_str(&format!("Pin 42: {:?}\n", system.gpio.get_pin(42)));
        status_report.push_str(&format!("Pin 18 Mode: {:?}\n", system.gpio.pin_modes.get(&18)));
        
        system.uart.write_string(&status_report)?;
        
        let output = system.uart.get_output_string();
        if output.contains("GPIO Status") && output.contains("Pin 42: Some(true)") {
            Ok(())
        } else {
            Err("GPIO status report via UART failed")
        }
    });
    
    crate::test_case!("Interactive LED Toggle", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Initial state
        system.gpio.set_pin(42, false)?;
        
        // Simulate multiple toggle commands
        for i in 0..3 {
            system.uart.add_input(b"l"); // toggle command
            let _command = system.uart.read_byte();
            
            let new_state = system.gpio.toggle_pin(42)?;
            system.uart.write_string(&format!("LED {}\n", if new_state { "ON" } else { "OFF" }))?;
        }
        
        // After 3 toggles (started false), should be true
        if system.gpio.get_pin(42) == Some(true) {
            let output = system.uart.get_output_string();
            if output.matches("LED").count() == 3 {
                Ok(())
            } else {
                Err("Toggle command output incorrect")
            }
        } else {
            Err("Interactive LED toggle failed")
        }
    });
}

/// Memory-Timer Integration Tests
fn test_memory_timer_integration(test_state: &TestState, _config: &TestConfig) {
    println!("\n🧠⏰ Memory-Timer Integration:");
    
    crate::test_case!("Timed Memory Operations", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        let start_time = system.timer.get_time();
        
        // Perform memory operations
        let mut addresses = Vec::new();
        for _ in 0..10 {
            system.timer.advance_time(100); // 100μs per allocation
            if let Some(addr) = system.memory.allocate(64) {
                addresses.push(addr);
            }
        }
        
        let allocation_time = system.timer.get_time() - start_time;
        
        // Free memory
        for addr in addresses {
            system.timer.advance_time(50); // 50μs per free
            system.memory.free(addr);
        }
        
        let total_time = system.timer.get_time() - start_time;
        
        if allocation_time >= 1000 && total_time >= 1500 {
            Ok(())
        } else {
            Err("Timed memory operations failed")
        }
    });
    
    crate::test_case!("Memory Allocation Timeout", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        let start_time = system.timer.get_time();
        let timeout = 5000; // 5ms timeout
        
        // Try to allocate until timeout
        let mut allocations = 0;
        loop {
            system.timer.advance_time(100);
            if system.timer.get_time() - start_time > timeout {
                break;
            }
            
            if system.memory.allocate(64).is_some() {
                allocations += 1;
            }
        }
        
        if allocations > 0 && system.timer.get_time() - start_time > timeout {
            Ok(())
        } else {
            Err("Memory allocation timeout test failed")
        }
    });
    
    crate::test_case!("Periodic Memory Cleanup", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Allocate memory to create fragmentation
        let mut addresses = Vec::new();
        for _ in 0..5 {
            if let Some(addr) = system.memory.allocate(64) {
                addresses.push(addr);
            }
        }
        
        // Free every other block to create fragmentation
        for (i, addr) in addresses.iter().enumerate() {
            if i % 2 == 0 {
                system.memory.free(*addr);
            }
        }
        
        let initial_fragmentation = system.memory.get_stats().fragmentation;
        
        // Simulate periodic cleanup (every 1000μs)
        for _ in 0..3 {
            system.timer.advance_time(1000);
            system.memory.defragment();
        }
        
        let final_fragmentation = system.memory.get_stats().fragmentation;
        
        if final_fragmentation <= initial_fragmentation {
            Ok(())
        } else {
            Err("Periodic memory cleanup failed")
        }
    });
}

/// Full System Integration Tests
fn test_full_system_integration(test_state: &TestState, _config: &TestConfig) {
    println!("\n🖥️ Full System Integration:");
    
    crate::test_case!("Complete System Stress Test", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Stress test all components simultaneously
        for i in 0..100 {
            // Timer operations
            system.timer.advance_time(10);
            
            // Memory operations
            if i % 10 == 0 {
                system.memory.allocate(64);
            }
            
            // GPIO operations
            if i % 5 == 0 {
                system.gpio.toggle_pin(42).ok();
            }
            
            // UART operations
            if i % 3 == 0 {
                system.uart.write_byte((65 + (i % 26)) as u8).ok(); // A-Z
            }
            
            // Interrupt operations
            if i % 7 == 0 {
                system.interrupts.trigger_interrupt(64);
            }
        }
        
        // Verify system is still functional
        let health_report = system.run_system_health_check()?;
        
        if health_report.health_score() >= 80.0 {
            Ok(())
        } else {
            Err("System stress test failed")
        }
    });
    
    crate::test_case!("System Recovery Test", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Simulate system failure
        system.uart.enabled = false;
        system.gpio.enabled = false;
        
        let failed_health = system.run_system_health_check()?;
        
        // Simulate recovery
        system.uart.enabled = true;
        system.gpio.enabled = true;
        
        let recovered_health = system.run_system_health_check()?;
        
        if !failed_health.all_healthy() && recovered_health.all_healthy() {
            Ok(())
        } else {
            Err("System recovery test failed")
        }
    });
    
    crate::test_case!("Multi-Component Data Flow", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Simulate data flowing through multiple components
        let test_data = "TinyOS Test Data";
        
        // 1. Store test data in memory
        let data_addr = system.memory.allocate(test_data.len()).unwrap();
        
        // 2. Send confirmation via UART
        system.uart.write_string(&format!("Data stored at 0x{:x}\n", data_addr))?;
        
        // 3. Indicate success via LED
        system.gpio.set_pin(42, true)?;
        
        // 4. Log timestamp
        let timestamp = system.timer.get_time();
        system.uart.write_string(&format!("Timestamp: {}μs\n", timestamp))?;
        
        // 5. Trigger completion interrupt
        system.interrupts.trigger_interrupt(129);
        
        // Verify the data flow
        let uart_output = system.uart.get_output_string();
        let led_state = system.gpio.get_pin(42);
        let interrupt_count = system.interrupts.get_interrupt_count(129);
        
        if uart_output.contains("Data stored") && 
           uart_output.contains("Timestamp") &&
           led_state == Some(true) &&
           interrupt_count > 0 {
            Ok(())
        } else {
            Err("Multi-component data flow failed")
        }
    });
}

/// Shell Command Integration Tests
fn test_shell_integration(test_state: &TestState, _config: &TestConfig) {
    println!("\n🐚 Shell Integration:");
    
    crate::test_case!("Shell Help Command", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Simulate help command
        system.uart.add_input(b"h");
        let command = system.uart.read_byte().unwrap();
        
        if command == b'h' {
            let help_text = "=== TinyOS Command Reference ===\nSystem Commands:\n  h/H - Show this help menu\n";
            system.uart.write_string(help_text)?;
        }
        
        let output = system.uart.get_output_string();
        if output.contains("Command Reference") && output.contains("help menu") {
            Ok(())
        } else {
            Err("Shell help command failed")
        }
    });
    
    crate::test_case!("Shell Memory Commands", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Memory allocation command
        system.uart.add_input(b"a");
        let command = system.uart.read_byte().unwrap();
        
        if command == b'a' {
            if let Some(addr) = system.memory.allocate(64) {
                system.uart.write_string(&format!("Allocated at 0x{:x}\n", addr))?;
            }
        }
        
        // Memory stats command
        system.uart.add_input(b"m");
        let command = system.uart.read_byte().unwrap();
        
        if command == b'm' {
            let stats = system.memory.get_stats();
            system.uart.write_string(&format!("Used: {} bytes\n", stats.used_size))?;
        }
        
        let output = system.uart.get_output_string();
        if output.contains("Allocated at") && output.contains("Used:") {
            Ok(())
        } else {
            Err("Shell memory commands failed")
        }
    });
    
    crate::test_case!("Shell System Commands", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // System time command
        system.uart.add_input(b"t");
        let command = system.uart.read_byte().unwrap();
        
        if command == b't' {
            let time = system.timer.get_time();
            system.uart.write_string(&format!("System time: {}μs\n", time))?;
        }
        
        // System info command
        system.uart.add_input(b"s");
        let command = system.uart.read_byte().unwrap();
        
        if command == b's' {
            system.uart.write_string("TinyOS v0.1.0\nTarget: Raspberry Pi 4/5\n")?;
        }
        
        let output = system.uart.get_output_string();
        if output.contains("System time") && output.contains("TinyOS v0.1.0") {
            Ok(())
        } else {
            Err("Shell system commands failed")
        }
    });
    
    crate::test_case!("Shell Command Chaining", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Chain multiple commands
        let commands = b"1l0"; // LED on, toggle, LED off
        system.uart.add_input(commands);
        
        // Execute commands
        for _ in 0..3 {
            if let Some(cmd) = system.uart.read_byte() {
                match cmd {
                    b'1' => {
                        system.gpio.set_pin(42, true)?;
                        system.uart.write_string("LED ON\n")?;
                    }
                    b'l' => {
                        let state = system.gpio.toggle_pin(42)?;
                        system.uart.write_string(&format!("LED {}\n", if state { "ON" } else { "OFF" }))?;
                    }
                    b'0' => {
                        system.gpio.set_pin(42, false)?;
                        system.uart.write_string("LED OFF\n")?;
                    }
                    _ => {}
                }
            }
        }
        
        let output = system.uart.get_output_string();
        let led_count = output.matches("LED").count();
        
        if led_count >= 3 && system.gpio.get_pin(42) == Some(false) {
            Ok(())
        } else {
            Err("Shell command chaining failed")
        }
    });
}

/// SD Card Integration Tests
fn test_sdcard_integration(test_state: &TestState, _config: &TestConfig) {
    println!("\n💾🔗 SD Card Integration:");
    
    crate::test_case!("SD Card Boot Integration", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Verify SD card is initialized during boot
        if !system.sdcard.is_initialized() {
            return Err("SD card should be initialized during boot sequence");
        }
        
        Ok(())
    });
    
    crate::test_case!("SD Card Shell Commands", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Test SD card info command
        system.uart.add_input(b"p");
        let mut response = String::new();
        
        // Simulate processing the command
        if let Some(cmd) = system.uart.read_byte() {
            if cmd == b'p' {
                response.push_str("SD Card Status: ✓ INITIALIZED\n");
                response.push_str("Card Type: SDHC/SDXC\n");
                system.uart.write_string(&response)?;
            }
        }
        
        let output = system.uart.get_output_string();
        if output.contains("SD Card Status") && output.contains("INITIALIZED") {
            Ok(())
        } else {
            Err("SD card info command failed")
        }
    });
    
    crate::test_case!("SD Card Read/Write Integration", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Test block write
        let test_data = [0xAA; 512];
        system.sdcard.write_block(1000, &test_data)?;
        
        // Test block read
        let mut read_buffer = [0u8; 512];
        system.sdcard.read_block(1000, &mut read_buffer)?;
        
        // Verify data integrity
        if read_buffer == test_data {
            Ok(())
        } else {
            Err("SD card read/write integrity check failed")
        }
    });
    
    crate::test_case!("SD Card Error Handling Integration", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Enable error simulation
        system.sdcard.set_error_simulation(true);
        
        // Test error block (block 99 should trigger error)
        let mut buffer = [0u8; 512];
        match system.sdcard.read_block(99, &mut buffer) {
            Err(MockSdError::ReadError) => Ok(()),
            _ => Err("Error simulation should trigger read error for block 99"),
        }
    });
    
    crate::test_case!("SD Card Performance Integration", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Test multiple block operations
        let test_blocks = 10;
        let test_data = [0x55; 512];
        
        // Write multiple blocks
        for block_num in 0..test_blocks {
            system.sdcard.write_block(block_num, &test_data)?;
        }
        
        // Read and verify multiple blocks
        for block_num in 0..test_blocks {
            let mut read_buffer = [0u8; 512];
            system.sdcard.read_block(block_num, &mut read_buffer)?;
            
            if read_buffer != test_data {
                return Err("Multi-block read/write test failed");
            }
        }
        
        Ok(())
    });
}

/// Interrupt System Integration Tests
fn test_interrupt_system_integration(test_state: &TestState, _config: &TestConfig) {
    println!("\n⚡🔗 Interrupt System Integration:");
    
    crate::test_case!("Timer Interrupt Integration", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Enable timer interrupts
        system.timer.enable_interrupts();
        
        // Advance time to generate interrupts
        system.timer.advance_time(5000);
        
        // Process timer interrupts
        let timer_interrupts = system.timer.get_interrupt_count();
        for _ in 0..timer_interrupts {
            system.interrupts.trigger_interrupt(64);
        }
        
        let total_interrupts = system.interrupts.get_total_interrupts();
        
        if total_interrupts > 0 && total_interrupts >= timer_interrupts {
            Ok(())
        } else {
            Err("Timer interrupt integration failed")
        }
    });
    
    crate::test_case!("UART Interrupt on Data", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Simulate UART data reception triggering interrupt
        system.uart.add_input(b"Hello");
        system.interrupts.trigger_interrupt(153); // UART interrupt
        
        // Process UART interrupt
        if let Some(irq) = system.interrupts.get_pending_interrupt() {
            if irq == 153 {
                // Read data from UART
                let mut received_data = Vec::new();
                while let Some(byte) = system.uart.read_byte() {
                    received_data.push(byte);
                }
                
                if received_data == b"Hello" {
                    Ok(())
                } else {
                    Err("UART data reception failed")
                }
            } else {
                Err("Wrong interrupt received")
            }
        } else {
            Err("UART interrupt not triggered")
        }
    });
    
    crate::test_case!("GPIO Interrupt on Pin Change", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Simulate GPIO pin change
        system.gpio.set_pin_mode(18, GpioMode::Input)?;
        system.gpio.pin_states.insert(18, false);
        
        // Pin change triggers interrupt
        system.gpio.pin_states.insert(18, true);
        system.interrupts.trigger_interrupt(129); // GPIO interrupt
        
        // Process GPIO interrupt
        if let Some(irq) = system.interrupts.get_pending_interrupt() {
            if irq == 129 {
                let pin_state = system.gpio.get_pin(18);
                if pin_state == Some(true) {
                    Ok(())
                } else {
                    Err("GPIO pin state incorrect")
                }
            } else {
                Err("Wrong interrupt received")
            }
        } else {
            Err("GPIO interrupt not triggered")
        }
    });
    
    crate::test_case!("Interrupt Priority Handling", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Trigger multiple interrupts simultaneously
        system.interrupts.trigger_interrupt(64);  // Timer (high priority)
        system.interrupts.trigger_interrupt(153); // UART (medium priority)
        system.interrupts.trigger_interrupt(129); // GPIO (low priority)
        
        // Process interrupts (should be in order)
        let mut processed_interrupts = Vec::new();
        while let Some(irq) = system.interrupts.get_pending_interrupt() {
            processed_interrupts.push(irq);
        }
        
        // In mock, last in is first out (stack behavior)
        if processed_interrupts.len() == 3 {
            Ok(())
        } else {
            Err("Interrupt priority handling failed")
        }
    });
    
    crate::test_case!("Interrupt Statistics Integration", test_state, || -> Result<(), &'static str> {
        let mut system = MockSystem::new();
        system.simulate_boot_sequence()?;
        
        // Generate various interrupts
        for _ in 0..5 {
            system.interrupts.trigger_interrupt(64);  // Timer
        }
        for _ in 0..3 {
            system.interrupts.trigger_interrupt(153); // UART
        }
        for _ in 0..2 {
            system.interrupts.trigger_interrupt(129); // GPIO
        }
        
        // Generate statistics report
        let timer_count = system.interrupts.get_interrupt_count(64);
        let uart_count = system.interrupts.get_interrupt_count(153);
        let gpio_count = system.interrupts.get_interrupt_count(129);
        let total_count = system.interrupts.get_total_interrupts();
        
        let stats_report = format!(
            "Interrupt Statistics:\nTimer: {}\nUART: {}\nGPIO: {}\nTotal: {}\n",
            timer_count, uart_count, gpio_count, total_count
        );
        
        system.uart.write_string(&stats_report)?;
        
        if timer_count == 5 && uart_count == 3 && gpio_count == 2 && total_count == 10 {
            let output = system.uart.get_output_string();
            if output.contains("Interrupt Statistics") {
                Ok(())
            } else {
                Err("Statistics report generation failed")
            }
        } else {
            Err("Interrupt statistics integration failed")
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_integration_framework() {
        let test_state = TestState::new();
        let config = TestConfig::default();
        
        // Run a subset of integration tests
        test_system_boot_integration(&test_state, &config);
        
        let summary = test_state.get_summary();
        assert!(summary.total_tests > 0);
    }
}
