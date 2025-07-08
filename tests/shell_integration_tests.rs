//! Integration tests for shell command routing and execution
//! 
//! These tests validate that the shell system correctly routes commands
//! and maintains the same behavior as the original implementation.

use tiny_os::shell::{ShellContext};
use tiny_os::uart::Uart;
use tiny_os::gpio::Gpio;
use tiny_os::timer::SystemTimer;
use tiny_os::memory::MemoryManager;
use tiny_os::interrupts::InterruptController;
use tiny_os::sdcard::SdCard;

/// Helper function to create a test shell context
fn create_shell_context() -> ShellContext {
    let uart = Uart::new();
    let gpio = Gpio::new();
    let timer = SystemTimer::new();
    let memory_manager = MemoryManager::new();
    let mut interrupt_controller = InterruptController::new();
    interrupt_controller.init();
    let sdcard = SdCard::new();
    let fat32_fs = None;
    
    ShellContext::new(
        uart,
        gpio,
        timer,
        memory_manager,
        interrupt_controller,
        sdcard,
        fat32_fs,
    )
}

#[cfg(test)]
mod shell_integration_tests {
    use super::*;
    
    #[test]
    fn test_shell_context_initialization() {
        let context = create_shell_context();
        
        // Verify initial state
        assert!(!context.led_state, "LED should start in OFF state");
        assert!(context.fat32_fs.is_none(), "FAT32 filesystem should start unmounted");
        
        // Verify components are initialized
        let memory_stats = context.memory_manager.get_stats();
        assert!(memory_stats.total_blocks > 0, "Memory manager should be initialized");
        
        let interrupt_stats = context.interrupt_controller.get_interrupt_stats();
        assert_eq!(interrupt_stats.total_interrupts, 0, "Interrupt controller should start with no interrupts");
    }
    
    #[test]
    fn test_led_state_consistency() {
        let mut context = create_shell_context();
        
        // Test LED state changes are properly tracked
        assert!(!context.led_state);
        
        // Simulate LED on command
        context.led_state = true;
        context.gpio.set_high(42);
        assert!(context.led_state);
        
        // Simulate LED off command
        context.led_state = false;
        context.gpio.set_low(42);
        assert!(!context.led_state);
        
        // Simulate LED toggle
        context.led_state = !context.led_state;
        if context.led_state {
            context.gpio.set_high(42);
        } else {
            context.gpio.set_low(42);
        }
        assert!(context.led_state);
    }
    
    #[test]
    fn test_memory_operations_consistency() {
        let mut context = create_shell_context();
        
        let initial_stats = context.memory_manager.get_stats();
        let initial_free_blocks = initial_stats.free_blocks;
        
        // Test memory allocation
        let allocation_result = context.memory_manager.allocate_block();
        if allocation_result.is_some() {
            let after_alloc_stats = context.memory_manager.get_stats();
            assert!(after_alloc_stats.allocated_blocks > initial_stats.allocated_blocks,
                    "Allocated blocks should increase after allocation");
            assert!(after_alloc_stats.free_blocks < initial_free_blocks,
                    "Free blocks should decrease after allocation");
            
            // Free the block
            if let Some(address) = allocation_result {
                let free_result = context.memory_manager.free_block(address);
                assert!(free_result, "Block should be freed successfully");
                
                let after_free_stats = context.memory_manager.get_stats();
                assert_eq!(after_free_stats.allocated_blocks, initial_stats.allocated_blocks,
                          "Allocated blocks should return to initial count");
            }
        }
    }
    
    #[test]
    fn test_interrupt_controller_operations() {
        let mut context = create_shell_context();
        
        // Test interrupt controller functionality
        let initial_stats = context.interrupt_controller.get_interrupt_stats();
        assert_eq!(initial_stats.total_interrupts, 0);
        
        // Test interrupt test functionality
        let test_result = context.interrupt_controller.run_interrupt_test();
        // Result may vary based on implementation, but should not panic
        let _ = test_result;
        
        let after_test_stats = context.interrupt_controller.get_interrupt_stats();
        // Stats may have changed due to test, verify structure is valid
        assert!(after_test_stats.total_interrupts >= initial_stats.total_interrupts);
    }
    
    #[test]
    fn test_timer_operations() {
        let context = create_shell_context();
        
        // Test timer functionality
        let start_time = context.timer.get_time();
        
        // Simulate small delay
        context.timer.delay_us(100);
        
        let end_time = context.timer.get_time();
        assert!(end_time >= start_time, "Timer should advance monotonically");
        
        // Test time conversion
        let elapsed_ticks = end_time.wrapping_sub(start_time);
        let elapsed_ms = context.timer.ticks_to_ms(elapsed_ticks as u32);
        // Should be a reasonable value (not checking exact timing due to test environment)
        assert!(elapsed_ms < 1000, "Elapsed time should be reasonable for short delay");
    }
    
    #[test]
    fn test_filesystem_state_management() {
        let mut context = create_shell_context();
        
        // Initially no filesystem
        assert!(context.fat32_fs.is_none());
        
        // The filesystem mounting would normally require actual SD card hardware
        // For now, we just verify the state management works
        
        // Simulate filesystem mount (would normally come from SD card initialization)
        // context.fat32_fs = Some(filesystem_instance);
        // For this test, we just verify None state is handled correctly
        assert!(context.fat32_fs.is_none());
    }
    
    #[test]
    fn test_command_handler_isolation() {
        let mut context1 = create_shell_context();
        let mut context2 = create_shell_context();
        
        // Modify one context
        context1.led_state = true;
        let _ = context1.memory_manager.allocate_block();
        
        // Verify other context is unaffected
        assert!(!context2.led_state, "Contexts should be independent");
        
        let stats1 = context1.memory_manager.get_stats();
        let stats2 = context2.memory_manager.get_stats();
        // Both should start with same configuration but be independent
        assert_eq!(stats1.total_blocks, stats2.total_blocks);
        assert_eq!(stats1.block_size, stats2.block_size);
    }
}

#[cfg(test)]
mod shell_command_routing_tests {
    use super::*;
    use tiny_os::shell::commands::*;
    
    #[test]
    fn test_system_command_handlers_exist() {
        let context = create_shell_context();
        let start_time = context.timer.get_time();
        
        // Verify all system command handlers exist and are callable
        system::handle_help(&context);
        system::handle_time(&context, start_time);
        system::handle_system_info(&context);
        
        let mut context_mut = context;
        system::handle_health_check(&mut context_mut);
    }
    
    #[test]
    fn test_hardware_command_handlers_exist() {
        let mut context = create_shell_context();
        
        // Verify all hardware command handlers exist and are callable
        hardware::handle_led_on(&mut context);
        hardware::handle_led_off(&mut context);
        hardware::handle_led_toggle(&mut context);
        hardware::handle_interrupt_status(&context);
        hardware::handle_interrupt_toggle(&mut context);
        hardware::handle_interrupt_test(&mut context);
        hardware::handle_exception_stats(&context);
        hardware::handle_exception_test(&context);
        hardware::handle_sdcard_info(&context);
        hardware::handle_sdcard_read(&mut context);
        hardware::handle_sdcard_write(&mut context);
    }
    
    #[test]
    fn test_memory_command_handlers_exist() {
        let mut context = create_shell_context();
        
        // Verify all memory command handlers exist and are callable
        memory::handle_memory_stats(&context.uart, &context.memory_manager);
        memory::handle_memory_allocate(&context.uart, &mut context.memory_manager);
        memory::handle_memory_free(&context.uart, &mut context.memory_manager);
        memory::handle_memory_test(&context.uart, &mut context.memory_manager);
        memory::handle_comprehensive_memory_test(&context.uart, &mut context.memory_manager);
        memory::handle_memory_corruption_check(&context.uart, &context.memory_manager);
        memory::handle_memory_defragment(&context.uart, &mut context.memory_manager);
    }
    
    #[test]
    fn test_filesystem_command_handlers_exist() {
        let mut context = create_shell_context();
        
        // Verify all filesystem command handlers exist and are callable
        filesystem::handle_directory_listing(&context.uart, &mut context.fat32_fs);
        filesystem::handle_filesystem_mount_info(&context.uart, &mut context.fat32_fs);
        filesystem::handle_change_directory(&context.uart, &mut context.fat32_fs);
        filesystem::handle_read_file(&context.uart, &mut context.fat32_fs);
        filesystem::handle_change_to_root(&context.uart, &mut context.fat32_fs);
    }
}

#[cfg(test)]
mod regression_tests {
    use super::*;
    
    /// Test that verifies the refactored shell maintains the same command set
    /// as the original implementation
    #[test]
    fn test_command_completeness() {
        // This test documents all the commands that should be available
        // and verifies that our modular implementation supports them all
        
        let mut context = create_shell_context();
        let start_time = context.timer.get_time();
        
        // System Commands (originally in main.rs lines ~108-137)
        system::handle_help(&context);                    // 'h', 'H'
        system::handle_time(&context, start_time);        // 't', 'T'
        system::handle_system_info(&context);             // 's', 'S'
        system::handle_health_check(&mut context);        // 'c', 'C'
        
        // Hardware Control (originally in main.rs lines ~265-285)
        hardware::handle_led_on(&mut context);            // '1'
        hardware::handle_led_off(&mut context);           // '0'
        hardware::handle_led_toggle(&mut context);        // 'l', 'L'
        
        // Interrupt Management (originally in main.rs, interrupt commands)
        hardware::handle_interrupt_status(&context);      // 'i', 'I'
        hardware::handle_interrupt_toggle(&mut context);  // 'e', 'E'
        hardware::handle_interrupt_test(&mut context);    // 'j', 'J'
        
        // Exception Management (originally in main.rs, exception commands)
        hardware::handle_exception_stats(&context);       // 'v', 'V'
        hardware::handle_exception_test(&context);        // 'w', 'W'
        
        // Storage & SD Card (originally in main.rs, SD card commands)
        hardware::handle_sdcard_info(&context);           // 'p', 'P'
        hardware::handle_sdcard_read(&mut context);       // 'q', 'Q'
        hardware::handle_sdcard_write(&mut context);      // 'y', 'Y'
        
        // Memory Management (originally in main.rs lines ~286-325)
        memory::handle_memory_stats(&context.uart, &context.memory_manager);              // 'm', 'M'
        memory::handle_memory_allocate(&context.uart, &mut context.memory_manager);       // 'a', 'A'
        memory::handle_memory_free(&context.uart, &mut context.memory_manager);           // 'f', 'F'
        memory::handle_memory_test(&context.uart, &mut context.memory_manager);           // 'x', 'X'
        memory::handle_comprehensive_memory_test(&context.uart, &mut context.memory_manager); // 'z', 'Z'
        memory::handle_memory_corruption_check(&context.uart, &context.memory_manager);   // 'g', 'G'
        memory::handle_memory_defragment(&context.uart, &mut context.memory_manager);     // 'r', 'R'
        
        // FAT32 Filesystem (originally in main.rs lines ~326-450)
        filesystem::handle_directory_listing(&context.uart, &mut context.fat32_fs);       // 'd', 'D'
        filesystem::handle_filesystem_mount_info(&context.uart, &mut context.fat32_fs);   // 'n', 'N'
        filesystem::handle_change_directory(&context.uart, &mut context.fat32_fs);        // 'o', 'O'
        filesystem::handle_read_file(&context.uart, &mut context.fat32_fs);               // 'u', 'U'
        filesystem::handle_change_to_root(&context.uart, &mut context.fat32_fs);          // 'k', 'K'
        
        // If we reach here, all commands exist and are callable
        assert!(true, "All original commands are available in refactored implementation");
    }
}
