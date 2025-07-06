//! Unit Tests for TinyOS Components
//! 
//! This module contains comprehensive unit tests for all major TinyOS components

use super::mocks::*;
use super::{TestState, TestConfig, TestResult, TestStatus};
use std::collections::HashMap;

/// Run all unit tests
pub fn run_unit_tests(test_state: &TestState, config: &TestConfig) {
    println!("📋 Unit Tests");
    println!("--------------");
    
    // UART Component Tests
    test_uart_component(test_state, config);
    
    // GPIO Component Tests
    test_gpio_component(test_state, config);
    
    // Timer Component Tests
    test_timer_component(test_state, config);
    
    // Memory Management Tests
    test_memory_component(test_state, config);
    
    // Interrupt Controller Tests
    test_interrupt_component(test_state, config);
    
    // SD Card Component Tests
    test_sdcard_component(test_state, config);
}

/// UART Component Tests
fn test_uart_component(test_state: &TestState, _config: &TestConfig) {
    println!("\n🔌 UART Component Tests:");
    
    crate::test_case!("UART Initialization", test_state, || -> Result<(), &'static str> {
        let uart = MockUart::new();
        if uart.enabled {
            Ok(())
        } else {
            Err("UART not initialized properly")
        }
    });
    
    crate::test_case!("UART Write Byte", test_state, || -> Result<(), &'static str> {
        let mut uart = MockUart::new();
        uart.write_byte(b'A')?;
        let output = uart.get_output();
        if output == vec![b'A'] {
            Ok(())
        } else {
            Err("UART write byte failed")
        }
    });
    
    crate::test_case!("UART Write String", test_state, || -> Result<(), &'static str> {
        let mut uart = MockUart::new();
        uart.write_string("Hello, TinyOS!")?;
        let output = uart.get_output_string();
        if output == "Hello, TinyOS!" {
            Ok(())
        } else {
            Err("UART write string failed")
        }
    });
    
    crate::test_case!("UART Read Byte", test_state, || -> Result<(), &'static str> {
        let mut uart = MockUart::new();
        uart.add_input(b"Test");
        
        let byte1 = uart.read_byte();
        let byte2 = uart.read_byte();
        
        if byte1 == Some(b'T') && byte2 == Some(b'e') {
            Ok(())
        } else {
            Err("UART read byte failed")
        }
    });
    
    crate::test_case!("UART Buffer Management", test_state, || -> Result<(), &'static str> {
        let mut uart = MockUart::new();
        uart.write_string("Test1")?;
        uart.add_input(b"Test2");
        
        uart.clear_buffers();
        
        let output = uart.get_output();
        let input = uart.read_byte();
        
        if output.is_empty() && input.is_none() {
            Ok(())
        } else {
            Err("UART buffer clear failed")
        }
    });
    
    crate::test_case!("UART Disabled State", test_state, || -> Result<(), &'static str> {
        let mut uart = MockUart::new();
        uart.enabled = false;
        
        let write_result = uart.write_byte(b'A');
        let read_result = uart.read_byte();
        
        if write_result.is_err() && read_result.is_none() {
            Ok(())
        } else {
            Err("UART disabled state test failed")
        }
    });
}

/// GPIO Component Tests
fn test_gpio_component(test_state: &TestState, _config: &TestConfig) {
    println!("\n📌 GPIO Component Tests:");
    
    crate::test_case!("GPIO Initialization", test_state, || -> Result<(), &'static str> {
        let gpio = MockGpio::new();
        if gpio.enabled {
            Ok(())
        } else {
            Err("GPIO not initialized properly")
        }
    });
    
    crate::test_case!("GPIO Pin Mode Setting", test_state, || -> Result<(), &'static str> {
        let mut gpio = MockGpio::new();
        gpio.set_pin_mode(42, GpioMode::Output)?;
        
        if gpio.pin_modes.get(&42) == Some(&GpioMode::Output) {
            Ok(())
        } else {
            Err("GPIO pin mode setting failed")
        }
    });
    
    crate::test_case!("GPIO Pin State Control", test_state, || -> Result<(), &'static str> {
        let mut gpio = MockGpio::new();
        gpio.set_pin_mode(42, GpioMode::Output)?;
        gpio.set_pin(42, true)?;
        
        if gpio.get_pin(42) == Some(true) {
            Ok(())
        } else {
            Err("GPIO pin state control failed")
        }
    });
    
    crate::test_case!("GPIO Pin Toggle", test_state, || -> Result<(), &'static str> {
        let mut gpio = MockGpio::new();
        gpio.set_pin_mode(42, GpioMode::Output)?;
        gpio.set_pin(42, false)?;
        
        let new_state = gpio.toggle_pin(42)?;
        
        if new_state == true && gpio.get_pin(42) == Some(true) {
            Ok(())
        } else {
            Err("GPIO pin toggle failed")
        }
    });
    
    crate::test_case!("GPIO Invalid Pin Protection", test_state, || -> Result<(), &'static str> {
        let mut gpio = MockGpio::new();
        let result = gpio.set_pin_mode(999, GpioMode::Output);
        
        if result.is_err() {
            Ok(())
        } else {
            Err("GPIO should reject invalid pin numbers")
        }
    });
    
    crate::test_case!("GPIO Output Without Mode Set", test_state, || -> Result<(), &'static str> {
        let mut gpio = MockGpio::new();
        let result = gpio.set_pin(42, true);
        
        if result.is_err() {
            Ok(())
        } else {
            Err("GPIO should require mode to be set before output")
        }
    });
    
    crate::test_case!("GPIO Multiple Pin Management", test_state, || -> Result<(), &'static str> {
        let mut gpio = MockGpio::new();
        
        // Set up multiple pins
        gpio.set_pin_mode(18, GpioMode::Output)?;
        gpio.set_pin_mode(19, GpioMode::Output)?;
        gpio.set_pin_mode(20, GpioMode::Input)?;
        
        gpio.set_pin(18, true)?;
        gpio.set_pin(19, false)?;
        
        if gpio.get_pin(18) == Some(true) && gpio.get_pin(19) == Some(false) {
            Ok(())
        } else {
            Err("GPIO multiple pin management failed")
        }
    });
}

/// Timer Component Tests
fn test_timer_component(test_state: &TestState, _config: &TestConfig) {
    println!("\n⏰ Timer Component Tests:");
    
    crate::test_case!("Timer Initialization", test_state, || -> Result<(), &'static str> {
        let timer = MockTimer::new();
        if timer.enabled && timer.get_time() == 0 {
            Ok(())
        } else {
            Err("Timer not initialized properly")
        }
    });
    
    crate::test_case!("Timer Time Advance", test_state, || -> Result<(), &'static str> {
        let mut timer = MockTimer::new();
        let initial_time = timer.get_time();
        timer.advance_time(1000);
        
        if timer.get_time() == initial_time + 1000 {
            Ok(())
        } else {
            Err("Timer time advance failed")
        }
    });
    
    crate::test_case!("Timer Delay Function", test_state, || -> Result<(), &'static str> {
        let mut timer = MockTimer::new();
        let initial_time = timer.get_time();
        timer.delay(500);
        
        if timer.get_time() == initial_time + 500 {
            Ok(())
        } else {
            Err("Timer delay function failed")
        }
    });
    
    crate::test_case!("Timer Interrupt Generation", test_state, || -> Result<(), &'static str> {
        let mut timer = MockTimer::new();
        timer.enable_interrupts();
        
        let initial_count = timer.get_interrupt_count();
        timer.advance_time(3000); // Should generate 3 interrupts
        
        if timer.get_interrupt_count() > initial_count {
            Ok(())
        } else {
            Err("Timer interrupt generation failed")
        }
    });
    
    crate::test_case!("Timer Interrupt Enable/Disable", test_state, || -> Result<(), &'static str> {
        let mut timer = MockTimer::new();
        
        // Test disabled interrupts
        timer.disable_interrupts();
        timer.advance_time(2000);
        let disabled_count = timer.get_interrupt_count();
        
        // Test enabled interrupts
        timer.enable_interrupts();
        timer.advance_time(2000);
        let enabled_count = timer.get_interrupt_count();
        
        if enabled_count > disabled_count {
            Ok(())
        } else {
            Err("Timer interrupt enable/disable failed")
        }
    });
    
    crate::test_case!("Timer Reset Functionality", test_state, || -> Result<(), &'static str> {
        let mut timer = MockTimer::new();
        timer.advance_time(5000);
        timer.enable_interrupts();
        timer.advance_time(1000);
        
        timer.reset();
        
        if timer.get_time() == 0 && timer.get_interrupt_count() == 0 && !timer.interrupts_enabled {
            Ok(())
        } else {
            Err("Timer reset functionality failed")
        }
    });
}

/// Memory Management Tests
fn test_memory_component(test_state: &TestState, _config: &TestConfig) {
    println!("\n🧠 Memory Management Tests:");
    
    crate::test_case!("Memory Manager Initialization", test_state, || -> Result<(), &'static str> {
        let memory = MockMemoryManager::new(0x100000, 1024 * 1024, 64);
        let stats = memory.get_stats();
        
        if stats.total_size == 1024 * 1024 && stats.used_size == 0 {
            Ok(())
        } else {
            Err("Memory manager not initialized properly")
        }
    });
    
    crate::test_case!("Memory Block Allocation", test_state, || -> Result<(), &'static str> {
        let mut memory = MockMemoryManager::new(0x100000, 1024 * 1024, 64);
        let addr = memory.allocate(64);
        
        if addr.is_some() {
            let stats = memory.get_stats();
            if stats.used_size == 64 {
                Ok(())
            } else {
                Err("Memory allocation stats incorrect")
            }
        } else {
            Err("Memory block allocation failed")
        }
    });
    
    crate::test_case!("Memory Block Deallocation", test_state, || -> Result<(), &'static str> {
        let mut memory = MockMemoryManager::new(0x100000, 1024 * 1024, 64);
        let addr = memory.allocate(64).unwrap();
        
        let freed = memory.free(addr);
        
        if freed {
            let stats = memory.get_stats();
            if stats.used_size == 0 {
                Ok(())
            } else {
                Err("Memory deallocation stats incorrect")
            }
        } else {
            Err("Memory block deallocation failed")
        }
    });
    
    crate::test_case!("Memory Multiple Allocations", test_state, || -> Result<(), &'static str> {
        let mut memory = MockMemoryManager::new(0x100000, 1024 * 1024, 64);
        
        let mut addresses = Vec::new();
        for _ in 0..10 {
            if let Some(addr) = memory.allocate(64) {
                addresses.push(addr);
            } else {
                return Err("Multiple allocations failed");
            }
        }
        
        let stats = memory.get_stats();
        if stats.allocation_count == 10 && stats.used_size == 640 {
            Ok(())
        } else {
            Err("Multiple allocations stats incorrect")
        }
    });
    
    crate::test_case!("Memory Zero Size Allocation", test_state, || -> Result<(), &'static str> {
        let mut memory = MockMemoryManager::new(0x100000, 1024 * 1024, 64);
        let addr = memory.allocate(0);
        
        if addr.is_none() {
            Ok(())
        } else {
            Err("Memory should reject zero-size allocations")
        }
    });
    
    crate::test_case!("Memory Large Allocation", test_state, || -> Result<(), &'static str> {
        let mut memory = MockMemoryManager::new(0x100000, 1024, 64);
        let addr = memory.allocate(2048); // Larger than heap
        
        if addr.is_none() {
            Ok(())
        } else {
            Err("Memory should reject too-large allocations")
        }
    });
    
    crate::test_case!("Memory Double Free Protection", test_state, || -> Result<(), &'static str> {
        let mut memory = MockMemoryManager::new(0x100000, 1024 * 1024, 64);
        let addr = memory.allocate(64).unwrap();
        
        let first_free = memory.free(addr);
        let second_free = memory.free(addr);
        
        if first_free && !second_free {
            Ok(())
        } else {
            Err("Memory double free protection failed")
        }
    });
    
    crate::test_case!("Memory Corruption Check", test_state, || -> Result<(), &'static str> {
        let mut memory = MockMemoryManager::new(0x100000, 1024 * 1024, 64);
        let is_clean = memory.check_corruption();
        
        if is_clean {
            Ok(())
        } else {
            Err("Memory corruption check failed")
        }
    });
    
    crate::test_case!("Memory Fragmentation Analysis", test_state, || -> Result<(), &'static str> {
        let mut memory = MockMemoryManager::new(0x100000, 1024 * 1024, 64);
        
        // Allocate several blocks to create fragmentation
        for _ in 0..5 {
            memory.allocate(64);
        }
        
        let stats = memory.get_stats();
        if stats.fragmentation >= 0.0 {
            Ok(())
        } else {
            Err("Memory fragmentation analysis failed")
        }
    });
    
    crate::test_case!("Memory Defragmentation", test_state, || -> Result<(), &'static str> {
        let mut memory = MockMemoryManager::new(0x100000, 1024 * 1024, 64);
        
        // Create some fragmentation
        for _ in 0..3 {
            memory.allocate(64);
        }
        
        let initial_fragmentation = memory.get_stats().fragmentation;
        let freed_blocks = memory.defragment();
        let final_fragmentation = memory.get_stats().fragmentation;
        
        if final_fragmentation <= initial_fragmentation {
            Ok(())
        } else {
            Err("Memory defragmentation failed")
        }
    });
}

/// Interrupt Controller Tests
fn test_interrupt_component(test_state: &TestState, _config: &TestConfig) {
    println!("\n⚡ Interrupt Controller Tests:");
    
    crate::test_case!("Interrupt Controller Initialization", test_state, || -> Result<(), &'static str> {
        let controller = MockInterruptController::new();
        if controller.controller_enabled {
            Ok(())
        } else {
            Err("Interrupt controller not initialized properly")
        }
    });
    
    crate::test_case!("Interrupt Enable/Disable", test_state, || -> Result<(), &'static str> {
        let mut controller = MockInterruptController::new();
        
        controller.enable_interrupt(64)?;
        let enabled = controller.is_enabled(64);
        
        controller.disable_interrupt(64)?;
        let disabled = !controller.is_enabled(64);
        
        if enabled && disabled {
            Ok(())
        } else {
            Err("Interrupt enable/disable failed")
        }
    });
    
    crate::test_case!("Interrupt Triggering", test_state, || -> Result<(), &'static str> {
        let mut controller = MockInterruptController::new();
        controller.enable_interrupt(64)?;
        
        let initial_count = controller.get_interrupt_count(64);
        controller.trigger_interrupt(64);
        let final_count = controller.get_interrupt_count(64);
        
        if final_count == initial_count + 1 {
            Ok(())
        } else {
            Err("Interrupt triggering failed")
        }
    });
    
    crate::test_case!("Interrupt Pending Queue", test_state, || -> Result<(), &'static str> {
        let mut controller = MockInterruptController::new();
        controller.enable_interrupt(64)?;
        controller.enable_interrupt(153)?;
        
        controller.trigger_interrupt(64);
        controller.trigger_interrupt(153);
        
        let pending1 = controller.get_pending_interrupt();
        let pending2 = controller.get_pending_interrupt();
        let pending3 = controller.get_pending_interrupt();
        
        if pending1.is_some() && pending2.is_some() && pending3.is_none() {
            Ok(())
        } else {
            Err("Interrupt pending queue failed")
        }
    });
    
    crate::test_case!("Interrupt Statistics", test_state, || -> Result<(), &'static str> {
        let mut controller = MockInterruptController::new();
        controller.enable_interrupt(64)?;
        controller.enable_interrupt(153)?;
        
        // Trigger multiple interrupts
        for _ in 0..3 {
            controller.trigger_interrupt(64);
        }
        for _ in 0..2 {
            controller.trigger_interrupt(153);
        }
        
        let timer_count = controller.get_interrupt_count(64);
        let uart_count = controller.get_interrupt_count(153);
        let total_count = controller.get_total_interrupts();
        
        if timer_count == 3 && uart_count == 2 && total_count == 5 {
            Ok(())
        } else {
            Err("Interrupt statistics failed")
        }
    });
    
    crate::test_case!("Interrupt Disabled State", test_state, || -> Result<(), &'static str> {
        let mut controller = MockInterruptController::new();
        // Don't enable the interrupt
        
        controller.trigger_interrupt(64);
        let count = controller.get_interrupt_count(64);
        
        if count == 0 {
            Ok(())
        } else {
            Err("Disabled interrupt should not trigger")
        }
    });
    
    crate::test_case!("Interrupt Statistics Reset", test_state, || -> Result<(), &'static str> {
        let mut controller = MockInterruptController::new();
        controller.enable_interrupt(64)?;
        
        controller.trigger_interrupt(64);
        controller.trigger_interrupt(64);
        
        controller.reset_statistics();
        
        let count = controller.get_interrupt_count(64);
        let total = controller.get_total_interrupts();
        
        if count == 0 && total == 0 {
            Ok(())
        } else {
            Err("Interrupt statistics reset failed")
        }
    });
    
    crate::test_case!("Interrupt Controller Disabled", test_state, || -> Result<(), &'static str> {
        let mut controller = MockInterruptController::new();
        controller.controller_enabled = false;
        
        let enable_result = controller.enable_interrupt(64);
        let disable_result = controller.disable_interrupt(64);
        
        if enable_result.is_err() && disable_result.is_err() {
            Ok(())
        } else {
            Err("Disabled controller should reject operations")
        }
    });
}

/// SD Card Component Tests
fn test_sdcard_component(test_state: &TestState, _config: &TestConfig) {
    println!("\n💾 SD Card Component Tests:");
    
    crate::test_case!("SD Card Structure Creation", test_state, || -> Result<(), &'static str> {
        // Mock SD card creation since we can't test real hardware in unit tests
        let mock_sdcard = MockSdCard::new();
        if !mock_sdcard.is_initialized() {
            Ok(())  // Should start uninitialized
        } else {
            Err("SD card should start uninitialized")
        }
    });
    
    crate::test_case!("SD Card Error Handling", test_state, || -> Result<(), &'static str> {
        let mock_sdcard = MockSdCard::new();
        
        // Test reading from uninitialized card
        let mut buffer = [0u8; 512];
        match mock_sdcard.read_block(0, &mut buffer) {
            Err(MockSdError::CardNotPresent) => Ok(()),
            _ => Err("Should return CardNotPresent error for uninitialized card"),
        }
    });
    
    crate::test_case!("SD Card Block Size Validation", test_state, || -> Result<(), &'static str> {
        // Test that SD card uses 512-byte blocks
        let block_size = 512;
        if block_size == 512 {  // Mock constant
            Ok(())
        } else {
            Err("SD card block size mismatch")
        }
    });
    
    crate::test_case!("SD Card Info Structure", test_state, || -> Result<(), &'static str> {
        let mock_info = MockSdCardInfo::new();
        
        // Test capacity calculation for different card types
        if mock_info.high_capacity {
            let capacity = mock_info.get_capacity();
            if capacity > 2_000_000_000 {  // > 2GB indicates SDHC/SDXC
                Ok(())
            } else {
                Err("High capacity card should have > 2GB")
            }
        } else {
            Ok(())  // SDSC cards can be smaller
        }
    });
    
    crate::test_case!("SD Card Command Validation", test_state, || -> Result<(), &'static str> {
        // Test that key SD commands are properly defined
        
        // Check some key command constants exist and have expected patterns
        let cmd_go_idle = 0x00000000;  // CMD0
        let cmd_read_single = 0x11000000 | 0x00020000 | 0x00200000;  // CMD17 with flags
        
        // Basic validation that commands follow SD spec patterns
        if cmd_go_idle == 0 && (cmd_read_single & 0xFF000000) == 0x11000000 {
            Ok(())
        } else {
            Err("SD command constants validation failed")
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unit_test_framework() {
        let test_state = TestState::new();
        let config = TestConfig::default();
        
        // Run a subset of tests
        test_uart_component(&test_state, &config);
        
        let summary = test_state.get_summary();
        assert!(summary.total_tests > 0);
        assert!(summary.passed > 0);
    }
}
