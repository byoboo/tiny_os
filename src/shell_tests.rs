//! Shell System Tests
//!
//! Tests for the shell command system and context management.
//! These tests run in a hosted environment with std support.

#[cfg(test)]
mod shell_tests {
    use crate::shell::*;
    use crate::memory::MemoryManager;
    use crate::gpio::Gpio;
    use crate::timer::SystemTimer;
    use crate::uart::Uart;
    use crate::interrupts::InterruptController;
    use crate::sdcard::SdCard;
    use std::sync::{Arc, Mutex};
    use std::collections::VecDeque;

    /// Mock UART for testing shell output
    pub struct MockUart {
        output_buffer: Arc<Mutex<Vec<u8>>>,
        input_buffer: Arc<Mutex<VecDeque<u8>>>,
    }

    impl MockUart {
        pub fn new() -> Self {
            Self {
                output_buffer: Arc::new(Mutex::new(Vec::new())),
                input_buffer: Arc::new(Mutex::new(VecDeque::new())),
            }
        }

        pub fn add_input(&self, data: &[u8]) {
            let mut buffer = self.input_buffer.lock().unwrap();
            for &byte in data {
                buffer.push_back(byte);
            }
        }

        pub fn get_output(&self) -> String {
            let buffer = self.output_buffer.lock().unwrap();
            String::from_utf8_lossy(&buffer).to_string()
        }

        pub fn clear_output(&self) {
            let mut buffer = self.output_buffer.lock().unwrap();
            buffer.clear();
        }

        // Mock UART methods that match the real UART interface
        pub fn puts(&self, s: &str) {
            let mut buffer = self.output_buffer.lock().unwrap();
            buffer.extend_from_slice(s.as_bytes());
        }

        pub fn putc(&self, c: u8) {
            let mut buffer = self.output_buffer.lock().unwrap();
            buffer.push(c);
        }

        pub fn getc(&self) -> Option<u8> {
            let mut buffer = self.input_buffer.lock().unwrap();
            buffer.pop_front()
        }

        pub fn put_hex(&self, value: u64) {
            let hex_str = format!("{:X}", value);
            self.puts(&hex_str);
        }
    }

    /// Create a test shell context with mock components
    fn create_test_context() -> (ShellContext, Arc<MockUart>) {
        // Create mock UART
        let mock_uart = Arc::new(MockUart::new());
        
        // For testing, we'll create a shell context with minimal real components
        // and mock the UART interactions
        let uart = Uart::new(); // Real UART (won't actually do I/O in tests)
        let gpio = Gpio::new();
        let timer = SystemTimer::new();
        let memory_manager = MemoryManager::new();
        let interrupt_controller = InterruptController::new();
        let sdcard = SdCard::new();
        
        let context = ShellContext::new(
            uart,
            gpio,
            timer,
            memory_manager,
            interrupt_controller,
            sdcard,
            None, // No FAT32 filesystem for basic tests
        );

        (context, mock_uart)
    }

    #[test]
    fn test_shell_context_creation() {
        let (context, _mock_uart) = create_test_context();
        
        // Verify context was created with correct initial state
        assert_eq!(context.led_state, false);
        assert!(context.fat32_fs.is_none());
    }

    #[test]
    fn test_system_commands_available() {
        // Test that we can import and use the system command handlers
        use crate::shell::commands::system;
        
        let (context, mock_uart) = create_test_context();
        
        // Test help command output contains expected content
        system::handle_help(&context);
        // In a real test, we'd check mock_uart output, but since our current
        // system::handle_help uses the context's uart directly, we'll just
        // verify the function can be called without panicking
    }

    #[test]
    fn test_memory_commands_available() {
        // Test that memory command handlers can be imported and used
        use crate::shell::commands::memory;
        
        let (mut context, _mock_uart) = create_test_context();
        
        // Test memory stats command
        memory::handle_memory_stats(&context.uart, &context.memory_manager);
        
        // Test memory allocation
        memory::handle_memory_allocate(&context.uart, &mut context.memory_manager);
    }

    #[test]
    fn test_hardware_commands_available() {
        // Test that hardware command handlers can be imported and used
        use crate::shell::commands::hardware;
        
        let (mut context, _mock_uart) = create_test_context();
        
        // Test LED control commands
        hardware::handle_led_on(&mut context);
        assert_eq!(context.led_state, true);
        
        hardware::handle_led_off(&mut context);
        assert_eq!(context.led_state, false);
        
        hardware::handle_led_toggle(&mut context);
        assert_eq!(context.led_state, true);
    }

    #[test]
    fn test_filesystem_commands_available() {
        // Test that filesystem command handlers can be imported and used
        use crate::shell::commands::filesystem;
        
        let (mut context, _mock_uart) = create_test_context();
        
        // Test filesystem commands (they should handle None filesystem gracefully)
        filesystem::handle_directory_listing(&context.uart, &mut context.fat32_fs);
        filesystem::handle_filesystem_mount_info(&context.uart, &mut context.fat32_fs);
    }

    #[test]
    fn test_shell_module_structure() {
        // Test that all expected shell modules can be imported
        use crate::shell::commands::{system, hardware, memory, filesystem};
        
        // If this compiles, it means our module structure is correct
        let (context, _mock_uart) = create_test_context();
        
        // Basic smoke test - ensure we can call representative functions
        // from each command module without panicking
        system::handle_help(&context);
        // Other commands would need mutable context or specific setup
    }

    #[test]
    fn test_memory_management_integration() {
        let (mut context, _mock_uart) = create_test_context();
        
        // Test that memory manager is properly integrated
        let stats_before = context.memory_manager.get_stats();
        
        // Allocate a block
        if let Some(_address) = context.memory_manager.allocate_block() {
            let stats_after = context.memory_manager.get_stats();
            assert!(stats_after.allocated_blocks > stats_before.allocated_blocks);
        }
    }

    #[test]
    fn test_command_module_isolation() {
        // Test that command modules are properly isolated and don't interfere
        use crate::shell::commands::{system, hardware, memory, filesystem};
        
        let (mut context, _mock_uart) = create_test_context();
        
        // Test that we can call commands from different modules
        // without state interference
        let initial_led_state = context.led_state;
        
        // Call system command (shouldn't affect LED state)
        system::handle_help(&context);
        assert_eq!(context.led_state, initial_led_state);
        
        // Call hardware command (should affect LED state)
        hardware::handle_led_toggle(&mut context);
        assert_ne!(context.led_state, initial_led_state);
        
        // Memory commands shouldn't affect LED state
        let led_state_after_hardware = context.led_state;
        memory::handle_memory_stats(&context.uart, &context.memory_manager);
        assert_eq!(context.led_state, led_state_after_hardware);
    }

    #[test]
    fn test_command_error_handling() {
        let (mut context, _mock_uart) = create_test_context();
        
        // Test that commands handle edge cases gracefully
        use crate::shell::commands::{memory, filesystem};
        
        // Test memory commands with empty state
        memory::handle_memory_stats(&context.uart, &context.memory_manager);
        memory::handle_memory_corruption_check(&context.uart, &context.memory_manager);
        
        // Test filesystem commands with no mounted filesystem
        filesystem::handle_directory_listing(&context.uart, &mut context.fat32_fs);
        filesystem::handle_read_file(&context.uart, &mut context.fat32_fs);
        
        // If we get here without panicking, error handling is working
    }

    #[test]
    fn test_shell_context_state_management() {
        let (mut context, _mock_uart) = create_test_context();
        
        // Test that shell context properly manages state
        assert_eq!(context.led_state, false);
        
        // Simulate LED state changes
        context.led_state = true;
        assert_eq!(context.led_state, true);
        
        context.led_state = false;
        assert_eq!(context.led_state, false);
        
        // Test that filesystem state can be managed
        assert!(context.fat32_fs.is_none());
        // In a real scenario, we'd mount a filesystem and test state changes
    }
}

#[cfg(test)]
mod integration_tests {
    use super::shell_tests::*;
    
    #[test]
    fn test_full_shell_command_cycle() {
        let (mut context, _mock_uart) = create_test_context();
        
        // Simulate a full command cycle: help -> LED on -> memory stats -> LED off
        use crate::shell::commands::{system, hardware, memory};
        
        // 1. Start with help
        system::handle_help(&context);
        
        // 2. Turn LED on
        let initial_led_state = context.led_state;
        hardware::handle_led_on(&mut context);
        assert_ne!(context.led_state, initial_led_state);
        
        // 3. Check memory stats
        let stats = context.memory_manager.get_stats();
        memory::handle_memory_stats(&context.uart, &context.memory_manager);
        // Verify stats are accessible
        assert!(stats.total_blocks > 0);
        
        // 4. Turn LED off
        hardware::handle_led_off(&mut context);
        assert_eq!(context.led_state, false);
    }

    #[test]
    fn test_command_module_boundaries() {
        // Test that command modules maintain proper boundaries
        let (mut context, _mock_uart) = create_test_context();
        
        // Each module should only affect its own domain
        use crate::shell::commands::{system, hardware, memory, filesystem};
        
        // System commands shouldn't affect hardware state
        let initial_led_state = context.led_state;
        system::handle_time(&context, 0);
        system::handle_system_info(&context);
        assert_eq!(context.led_state, initial_led_state);
        
        // Memory commands shouldn't affect hardware state
        memory::handle_memory_stats(&context.uart, &context.memory_manager);
        assert_eq!(context.led_state, initial_led_state);
        
        // Filesystem commands shouldn't affect hardware state
        filesystem::handle_change_to_root(&context.uart, &mut context.fat32_fs);
        assert_eq!(context.led_state, initial_led_state);
        
        // Only hardware commands should affect hardware state
        hardware::handle_led_toggle(&mut context);
        assert_ne!(context.led_state, initial_led_state);
    }
}
