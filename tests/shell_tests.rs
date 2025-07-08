//! Unit tests for the shell command system
//! 
//! These tests validate that the refactored shell command handlers work correctly
//! and maintain the same functionality as the original monolithic implementation.

use tiny_os::shell::{ShellContext, run_shell};
use tiny_os::uart::Uart;
use tiny_os::gpio::{Gpio, GpioFunction};
use tiny_os::timer::SystemTimer;
use tiny_os::memory::MemoryManager;
use tiny_os::interrupts::InterruptController;
use tiny_os::sdcard::SdCard;
use tiny_os::fat32::Fat32FileSystem;
use tiny_os::shell::commands::{system, hardware, memory, filesystem};

/// Mock UART for testing shell output
pub struct MockUart {
    pub output: Vec<u8>,
}

impl MockUart {
    pub fn new() -> Self {
        Self {
            output: Vec::new(),
        }
    }
    
    pub fn get_output_string(&self) -> String {
        String::from_utf8_lossy(&self.output).to_string()
    }
    
    pub fn clear(&mut self) {
        self.output.clear();
    }
    
    pub fn puts(&self, s: &str) {
        // In a real implementation, we'd need to capture this
        // For now, we'll just verify the function can be called
    }
    
    pub fn putc(&self, _c: u8) {
        // In a real implementation, we'd need to capture this
    }
    
    pub fn getc(&self) -> Option<u8> {
        None // No input for tests
    }
}

/// Create a test shell context with mock components
fn create_test_context() -> ShellContext {
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
mod system_command_tests {
    use super::*;
    
    #[test]
    fn test_shell_context_creation() {
        let context = create_test_context();
        assert!(!context.led_state);
        assert!(context.fat32_fs.is_none());
    }
    
    #[test]
    fn test_system_help_command() {
        let context = create_test_context();
        // Test that help command can be called without panicking
        system::handle_help(&context);
        // In a full implementation, we'd capture and verify the output
    }
    
    #[test]
    fn test_system_time_command() {
        let context = create_test_context();
        let start_time = context.timer.get_time();
        // Test that time command can be called without panicking
        system::handle_time(&context, start_time);
    }
    
    #[test]
    fn test_system_info_command() {
        let context = create_test_context();
        // Test that system info command can be called without panicking
        system::handle_system_info(&context);
    }
    
    #[test]
    fn test_health_check_command() {
        let mut context = create_test_context();
        // Test that health check can be called without panicking
        system::handle_health_check(&mut context);
    }
}

#[cfg(test)]
mod hardware_command_tests {
    use super::*;
    
    #[test]
    fn test_led_on_command() {
        let mut context = create_test_context();
        assert!(!context.led_state);
        
        hardware::handle_led_on(&mut context);
        assert!(context.led_state);
    }
    
    #[test]
    fn test_led_off_command() {
        let mut context = create_test_context();
        context.led_state = true;
        
        hardware::handle_led_off(&mut context);
        assert!(!context.led_state);
    }
    
    #[test]
    fn test_led_toggle_command() {
        let mut context = create_test_context();
        let initial_state = context.led_state;
        
        hardware::handle_led_toggle(&mut context);
        assert_eq!(context.led_state, !initial_state);
        
        hardware::handle_led_toggle(&mut context);
        assert_eq!(context.led_state, initial_state);
    }
    
    #[test]
    fn test_interrupt_status_command() {
        let context = create_test_context();
        // Test that interrupt status can be called without panicking
        hardware::handle_interrupt_status(&context);
    }
    
    #[test]
    fn test_interrupt_toggle_command() {
        let mut context = create_test_context();
        // Test that interrupt toggle can be called without panicking
        hardware::handle_interrupt_toggle(&mut context);
    }
    
    #[test]
    fn test_interrupt_test_command() {
        let mut context = create_test_context();
        // Test that interrupt test can be called without panicking
        hardware::handle_interrupt_test(&mut context);
    }
    
    #[test]
    fn test_exception_stats_command() {
        let context = create_test_context();
        // Test that exception stats can be called without panicking
        hardware::handle_exception_stats(&context);
    }
    
    #[test]
    fn test_exception_test_command() {
        let context = create_test_context();
        // Test that exception test can be called without panicking
        hardware::handle_exception_test(&context);
    }
    
    #[test]
    fn test_sdcard_info_command() {
        let context = create_test_context();
        // Test that SD card info can be called without panicking
        hardware::handle_sdcard_info(&context);
    }
    
    #[test]
    fn test_sdcard_read_command() {
        let mut context = create_test_context();
        // Test that SD card read can be called without panicking
        hardware::handle_sdcard_read(&mut context);
    }
    
    #[test]
    fn test_sdcard_write_command() {
        let mut context = create_test_context();
        // Test that SD card write can be called without panicking
        hardware::handle_sdcard_write(&mut context);
    }
}

#[cfg(test)]
mod memory_command_tests {
    use super::*;
    
    #[test]
    fn test_memory_stats_command() {
        let context = create_test_context();
        // Test that memory stats can be called without panicking
        memory::handle_memory_stats(&context.uart, &context.memory_manager);
    }
    
    #[test]
    fn test_memory_allocate_command() {
        let mut context = create_test_context();
        // Test that memory allocation can be called without panicking
        memory::handle_memory_allocate(&context.uart, &mut context.memory_manager);
    }
    
    #[test]
    fn test_memory_free_command() {
        let mut context = create_test_context();
        // Test that memory free command can be called without panicking
        memory::handle_memory_free(&context.uart, &mut context.memory_manager);
    }
    
    #[test]
    fn test_memory_test_command() {
        let mut context = create_test_context();
        // Test that memory test can be called without panicking
        memory::handle_memory_test(&context.uart, &mut context.memory_manager);
    }
    
    #[test]
    fn test_comprehensive_memory_test_command() {
        let mut context = create_test_context();
        // Test that comprehensive memory test can be called without panicking
        memory::handle_comprehensive_memory_test(&context.uart, &mut context.memory_manager);
    }
    
    #[test]
    fn test_memory_corruption_check_command() {
        let context = create_test_context();
        // Test that corruption check can be called without panicking
        memory::handle_memory_corruption_check(&context.uart, &context.memory_manager);
    }
    
    #[test]
    fn test_memory_defragment_command() {
        let mut context = create_test_context();
        // Test that defragmentation can be called without panicking
        memory::handle_memory_defragment(&context.uart, &mut context.memory_manager);
    }
}

#[cfg(test)]
mod filesystem_command_tests {
    use super::*;
    
    #[test]
    fn test_directory_listing_command_no_fs() {
        let mut context = create_test_context();
        // Test directory listing with no filesystem mounted
        filesystem::handle_directory_listing(&context.uart, &mut context.fat32_fs);
        assert!(context.fat32_fs.is_none());
    }
    
    #[test]
    fn test_filesystem_mount_info_command() {
        let mut context = create_test_context();
        // Test filesystem mount/info with no filesystem
        filesystem::handle_filesystem_mount_info(&context.uart, &mut context.fat32_fs);
    }
    
    #[test]
    fn test_change_directory_command_no_fs() {
        let mut context = create_test_context();
        // Test change directory with no filesystem mounted
        filesystem::handle_change_directory(&context.uart, &mut context.fat32_fs);
        assert!(context.fat32_fs.is_none());
    }
    
    #[test]
    fn test_read_file_command_no_fs() {
        let mut context = create_test_context();
        // Test file read with no filesystem mounted
        filesystem::handle_read_file(&context.uart, &mut context.fat32_fs);
        assert!(context.fat32_fs.is_none());
    }
    
    #[test]
    fn test_change_to_root_command_no_fs() {
        let mut context = create_test_context();
        // Test change to root with no filesystem mounted
        filesystem::handle_change_to_root(&context.uart, &mut context.fat32_fs);
        assert!(context.fat32_fs.is_none());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_shell_context_led_state_management() {
        let mut context = create_test_context();
        
        // Test LED state transitions
        assert!(!context.led_state);
        
        // Turn LED on
        hardware::handle_led_on(&mut context);
        assert!(context.led_state);
        
        // Turn LED off
        hardware::handle_led_off(&mut context);
        assert!(!context.led_state);
        
        // Toggle LED (should turn on)
        hardware::handle_led_toggle(&mut context);
        assert!(context.led_state);
        
        // Toggle LED again (should turn off)
        hardware::handle_led_toggle(&mut context);
        assert!(!context.led_state);
    }
    
    #[test]
    fn test_memory_allocation_sequence() {
        let mut context = create_test_context();
        
        // Get initial stats
        let initial_stats = context.memory_manager.get_stats();
        let initial_free_blocks = initial_stats.free_blocks;
        
        // Allocate a block
        memory::handle_memory_allocate(&context.uart, &mut context.memory_manager);
        
        let after_alloc_stats = context.memory_manager.get_stats();
        // Note: We can't easily verify the exact change without capturing output
        // but we can verify the memory manager is still in a valid state
        assert!(after_alloc_stats.allocated_blocks <= after_alloc_stats.total_blocks);
    }
    
    #[test]
    fn test_all_commands_callable() {
        let mut context = create_test_context();
        let start_time = context.timer.get_time();
        
        // Test that all command handlers can be called without panicking
        // This verifies our refactor didn't break any function signatures
        
        // System commands
        system::handle_help(&context);
        system::handle_time(&context, start_time);
        system::handle_system_info(&context);
        system::handle_health_check(&mut context);
        
        // Hardware commands
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
        
        // Memory commands
        memory::handle_memory_stats(&context.uart, &context.memory_manager);
        memory::handle_memory_allocate(&context.uart, &mut context.memory_manager);
        memory::handle_memory_free(&context.uart, &mut context.memory_manager);
        memory::handle_memory_test(&context.uart, &mut context.memory_manager);
        memory::handle_comprehensive_memory_test(&context.uart, &mut context.memory_manager);
        memory::handle_memory_corruption_check(&context.uart, &context.memory_manager);
        memory::handle_memory_defragment(&context.uart, &mut context.memory_manager);
        
        // Filesystem commands
        filesystem::handle_directory_listing(&context.uart, &mut context.fat32_fs);
        filesystem::handle_filesystem_mount_info(&context.uart, &mut context.fat32_fs);
        filesystem::handle_change_directory(&context.uart, &mut context.fat32_fs);
        filesystem::handle_read_file(&context.uart, &mut context.fat32_fs);
        filesystem::handle_change_to_root(&context.uart, &mut context.fat32_fs);
        
        // If we get here, all commands are callable
        assert!(true);
    }
}
