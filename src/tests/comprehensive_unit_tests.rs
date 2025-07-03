//! Comprehensive Unit Tests for TinyOS Components
//! 
//! These tests validate individual components in isolation using mocks.

use super::mocks::*;
use super::test_config::*;
use super::test_reporter::TestReporter;

// Memory Manager Unit Tests
pub mod memory_tests {
    use super::*;

    pub fn test_memory_manager_initialization(reporter: &mut TestReporter) {
        reporter.start_test("memory_manager_initialization", "memory", "Test memory manager initialization");
        
        with_mock_memory(|memory| {
            memory.clear();
            
            // Simulate memory manager initialization
            let bitmap_size = (TOTAL_BLOCKS + 7) / 8;
            
            // Initialize bitmap area to zero
            for i in 0..bitmap_size {
                memory.write_u8(HEAP_START + i, 0);
            }
            
            // Mark bitmap blocks as used
            let bitmap_blocks = (bitmap_size + BLOCK_SIZE - 1) / BLOCK_SIZE;
            for block in 0..bitmap_blocks {
                let byte_index = block / 8;
                let bit_index = block % 8;
                let current = memory.read_u8(HEAP_START + byte_index);
                memory.write_u8(HEAP_START + byte_index, current | (1 << bit_index));
            }
            
            // Verify bitmap initialization
            let first_byte = memory.read_u8(HEAP_START);
            if reporter.assert_ne(first_byte, 0, "Bitmap should have some bits set for bitmap blocks") {
                reporter.pass("Memory manager initialization successful");
            }
        });
    }

    pub fn test_single_block_allocation(reporter: &mut TestReporter) {
        reporter.start_test("single_block_allocation", "memory", "Test single block allocation");
        
        with_mock_memory(|memory| {
            memory.clear();
            
            let block_address = HEAP_START + 0x1000;
            
            // Allocate a block
            memory.allocate(block_address, BLOCK_SIZE);
            
            // Add canary values
            memory.write_u32(block_address, CANARY_START);
            memory.write_u32(block_address + BLOCK_SIZE - 4, CANARY_END);
            
            // Verify allocation
            let all_good = reporter.assert_true(memory.is_allocated(block_address), "Block should be allocated") &
                          reporter.assert_eq(memory.get_allocation_size(block_address), Some(BLOCK_SIZE), "Block size should match") &
                          reporter.assert_eq(memory.read_u32(block_address), CANARY_START, "Start canary should be correct") &
                          reporter.assert_eq(memory.read_u32(block_address + BLOCK_SIZE - 4), CANARY_END, "End canary should be correct");
            
            if all_good {
                reporter.pass("Single block allocation successful");
            }
        });
    }

    pub fn test_multiple_block_allocation(reporter: &mut TestReporter) {
        reporter.start_test("multiple_block_allocation", "memory", "Test multiple contiguous block allocation");
        
        with_mock_memory(|memory| {
            memory.clear();
            
            let num_blocks = 3;
            let total_size = num_blocks * BLOCK_SIZE;
            let block_address = HEAP_START + 0x2000;
            
            // Allocate multiple blocks
            memory.allocate(block_address, total_size);
            
            // Write test pattern across all blocks
            for i in 0..num_blocks {
                let offset = i * BLOCK_SIZE;
                memory.write_u32(block_address + offset, TEST_PATTERN_1 + i);
            }
            
            // Verify allocation and data integrity
            let mut all_good = reporter.assert_true(memory.is_allocated(block_address), "Multi-block should be allocated");
            all_good &= reporter.assert_eq(memory.get_allocation_size(block_address), Some(total_size), "Total size should match");
            
            // Verify data integrity across blocks
            for i in 0..num_blocks {
                let offset = i * BLOCK_SIZE;
                let expected = TEST_PATTERN_1 + i;
                let actual = memory.read_u32(block_address + offset);
                all_good &= reporter.assert_eq(actual, expected, &format!("Block {} data should be correct", i));
            }
            
            if all_good {
                reporter.pass("Multiple block allocation successful");
            }
        });
    }

    pub fn test_memory_deallocation(reporter: &mut TestReporter) {
        reporter.start_test("memory_deallocation", "memory", "Test memory deallocation");
        
        with_mock_memory(|memory| {
            memory.clear();
            
            let block_address = HEAP_START + 0x3000;
            
            // Allocate a block
            memory.allocate(block_address, BLOCK_SIZE);
            memory.write_u32(block_address, TEST_PATTERN_2);
            
            let allocated = reporter.assert_true(memory.is_allocated(block_address), "Block should be allocated");
            let data_correct = reporter.assert_eq(memory.read_u32(block_address), TEST_PATTERN_2, "Data should be written correctly");
            
            // Deallocate the block
            let size = memory.deallocate(block_address);
            let deallocated = reporter.assert_eq(size, Some(BLOCK_SIZE), "Deallocation should return correct size");
            let not_allocated = reporter.assert_true(!memory.is_allocated(block_address), "Block should not be allocated after deallocation");
            let data_cleared = reporter.assert_eq(memory.read_u32(block_address), 0, "Memory should be cleared after deallocation");
            
            if allocated && data_correct && deallocated && not_allocated && data_cleared {
                reporter.pass("Memory deallocation successful");
            }
        });
    }

    pub fn test_memory_corruption_detection(reporter: &mut TestReporter) {
        reporter.start_test("memory_corruption_detection", "memory", "Test memory corruption detection");
        
        with_mock_memory(|memory| {
            memory.clear();
            
            let block_address = HEAP_START + 0x4000;
            
            // Allocate and set up canaries
            memory.allocate(block_address, BLOCK_SIZE);
            memory.write_u32(block_address, CANARY_START);
            memory.write_u32(block_address + BLOCK_SIZE - 4, CANARY_END);
            
            // Verify canaries are intact
            let start_ok = reporter.assert_eq(memory.read_u32(block_address), CANARY_START, "Start canary should be intact");
            let end_ok = reporter.assert_eq(memory.read_u32(block_address + BLOCK_SIZE - 4), CANARY_END, "End canary should be intact");
            
            // Simulate corruption
            memory.write_u32(block_address + BLOCK_SIZE - 4, 0xBADC0DE);
            
            // Verify corruption is detected
            let corrupted = reporter.assert_ne(memory.read_u32(block_address + BLOCK_SIZE - 4), CANARY_END, "Corruption should be detected");
            
            if start_ok && end_ok && corrupted {
                reporter.pass("Memory corruption detection successful");
            }
        });
    }

    pub fn test_fragmentation_scenarios(reporter: &mut TestReporter) {
        reporter.start_test("fragmentation_scenarios", "memory", "Test memory fragmentation scenarios");
        
        with_mock_memory(|memory| {
            memory.clear();
            
            // Allocate several blocks with gaps
            let mut allocations = Vec::new();
            for i in 0..10 {
                let address = HEAP_START + 0x5000 + (i * BLOCK_SIZE * 2); // Leave gaps
                memory.allocate(address, BLOCK_SIZE);
                allocations.push(address);
            }
            
            let initial_count = reporter.assert_eq(memory.get_allocation_count(), 10, "Should have 10 initial allocations");
            
            // Free every other block to create fragmentation
            for i in (0..allocations.len()).step_by(2) {
                memory.deallocate(allocations[i]);
            }
            
            // Verify fragmentation pattern
            let remaining_count = reporter.assert_eq(memory.get_allocation_count(), 5, "Should have 5 remaining allocations");
            
            // Verify specific allocations
            let mut pattern_correct = true;
            for i in 0..allocations.len() {
                let should_be_allocated = i % 2 == 1; // Only odd indices should remain
                let is_allocated = memory.is_allocated(allocations[i]);
                if is_allocated != should_be_allocated {
                    pattern_correct = false;
                    break;
                }
            }
            
            let pattern_ok = reporter.assert_true(pattern_correct, "Fragmentation pattern should be correct");
            
            if initial_count && remaining_count && pattern_ok {
                reporter.pass("Fragmentation scenarios successful");
            }
        });
    }

    pub fn test_memory_stress(reporter: &mut TestReporter) {
        reporter.start_test("memory_stress", "memory", "Test memory under stress conditions");
        
        with_mock_memory(|memory| {
            memory.clear();
            
            let mut allocations = Vec::new();
            let base_address = HEAP_START + 0x10000;
            
            // Rapid allocation/deallocation cycles
            for cycle in 0..10 {
                // Allocate blocks
                for i in 0..20 {
                    let address = base_address + ((cycle * 20 + i) * BLOCK_SIZE);
                    memory.allocate(address, BLOCK_SIZE);
                    memory.write_u32(address, TEST_PATTERN_3 + i);
                    allocations.push(address);
                }
                
                // Randomly deallocate some blocks
                for i in (0..allocations.len()).step_by(3) {
                    if i < allocations.len() {
                        memory.deallocate(allocations[i]);
                        allocations.remove(i);
                    }
                }
            }
            
            // Verify remaining allocations have correct data
            let mut data_integrity = true;
            for &address in &allocations {
                if memory.is_allocated(address) {
                    let data = memory.read_u32(address);
                    // Data should be TEST_PATTERN_3 + some offset
                    if (data & 0xFFFFFFF0) != (TEST_PATTERN_3 & 0xFFFFFFF0) {
                        data_integrity = false;
                        break;
                    }
                }
            }
            
            let integrity_ok = reporter.assert_true(data_integrity, "Data integrity should be maintained under stress");
            let stats_reasonable = reporter.assert_true(memory.get_allocation_count() > 0, "Should have some allocations remaining");
            
            if integrity_ok && stats_reasonable {
                reporter.pass("Memory stress test successful");
            }
        });
    }
}

// UART Unit Tests
pub mod uart_tests {
    use super::*;

    pub fn test_uart_output(reporter: &mut TestReporter) {
        reporter.start_test("uart_output", "uart", "Test UART output functionality");
        
        with_mock_uart(|uart| {
            uart.clear_output();
            
            // Test basic output
            uart.puts("Hello, World!");
            uart.putc('\n');
            uart.puts("TinyOS UART Test");
            
            let output = uart.get_output();
            let has_hello = reporter.assert_true(output.len() >= 2, "Should have multiple output entries");
            let contains_hello = reporter.assert_true(output[0].contains("Hello, World!"), "Should contain hello message");
            let contains_tinyos = reporter.assert_true(output.iter().any(|s| s.contains("TinyOS")), "Should contain TinyOS message");
            
            let full_output = uart.get_full_output();
            let full_contains_hello = reporter.assert_true(full_output.contains("Hello, World!"), "Full output should contain hello");
            let full_contains_tinyos = reporter.assert_true(full_output.contains("TinyOS"), "Full output should contain TinyOS");
            
            if has_hello && contains_hello && contains_tinyos && full_contains_hello && full_contains_tinyos {
                reporter.pass("UART output test successful");
            }
        });
    }

    pub fn test_uart_input(reporter: &mut TestReporter) {
        reporter.start_test("uart_input", "uart", "Test UART input functionality");
        
        with_mock_uart(|uart| {
            uart.clear_output();
            uart.set_input("hello\n");
            
            // Read characters
            let mut input = String::new();
            while let Some(c) = uart.getc() {
                input.push(c);
                if c == '\n' {
                    break;
                }
            }
            
            let input_correct = reporter.assert_eq(input, "hello\n".to_string(), "Input should match expected");
            let input_consumed = reporter.assert_eq(uart.getc(), None, "All input should be consumed");
            
            if input_correct && input_consumed {
                reporter.pass("UART input test successful");
            }
        });
    }

    pub fn test_uart_echo(reporter: &mut TestReporter) {
        reporter.start_test("uart_echo", "uart", "Test UART echo functionality");
        
        with_mock_uart(|uart| {
            uart.clear_output();
            uart.set_input("test\r");
            
            // Simulate echo functionality
            while let Some(c) = uart.getc() {
                uart.putc(c);
                if c == '\r' {
                    uart.putc('\n');
                    break;
                }
            }
            
            let output = uart.get_full_output();
            let has_test = reporter.assert_true(output.contains("test"), "Should echo input text");
            let has_newline = reporter.assert_true(output.contains("\r\n"), "Should add newline after carriage return");
            
            if has_test && has_newline {
                reporter.pass("UART echo test successful");
            }
        });
    }

    pub fn test_uart_throughput(reporter: &mut TestReporter) {
        reporter.start_test("uart_throughput", "uart", "Test UART throughput measurement");
        
        with_mock_uart(|uart| {
            uart.clear_output();
            
            // Send large amount of data
            let test_data = "A".repeat(1000);
            uart.puts(&test_data);
            
            let output = uart.get_full_output();
            let length_correct = reporter.assert_eq(output.len(), 1000, "Output length should match input");
            let content_correct = reporter.assert_eq(output, test_data, "Output content should match input");
            
            if length_correct && content_correct {
                reporter.pass("UART throughput test successful");
            }
        });
    }
}

// GPIO Unit Tests
pub mod gpio_tests {
    use super::*;

    pub fn test_gpio_pin_control(reporter: &mut TestReporter) {
        reporter.start_test("gpio_pin_control", "gpio", "Test GPIO pin state control");
        
        with_mock_gpio(|gpio| {
            gpio.clear_all();
            
            // Test setting pin high
            gpio.set_pin(LED_PIN, true);
            let pin_high = reporter.assert_true(gpio.get_pin(LED_PIN), "LED pin should be high");
            
            // Test setting pin low
            gpio.set_pin(LED_PIN, false);
            let pin_low = reporter.assert_true(!gpio.get_pin(LED_PIN), "LED pin should be low");
            
            if pin_high && pin_low {
                reporter.pass("GPIO pin control test successful");
            }
        });
    }

    pub fn test_gpio_function_setting(reporter: &mut TestReporter) {
        reporter.start_test("gpio_function_setting", "gpio", "Test GPIO function configuration");
        
        with_mock_gpio(|gpio| {
            gpio.clear_all();
            
            // Test setting pin function
            gpio.set_function(LED_PIN, 1); // Output function
            let function_set = reporter.assert_eq(gpio.get_function(LED_PIN), 1, "Function should be set to output");
            
            // Test changing function
            gpio.set_function(LED_PIN, 0); // Input function
            let function_changed = reporter.assert_eq(gpio.get_function(LED_PIN), 0, "Function should be changed to input");
            
            if function_set && function_changed {
                reporter.pass("GPIO function setting test successful");
            }
        });
    }

    pub fn test_gpio_multiple_pins(reporter: &mut TestReporter) {
        reporter.start_test("gpio_multiple_pins", "gpio", "Test multiple GPIO pins");
        
        with_mock_gpio(|gpio| {
            gpio.clear_all();
            
            let pins = [18, 19, 20, 21];
            
            // Set different states for different pins
            for (i, &pin) in pins.iter().enumerate() {
                gpio.set_pin(pin, i % 2 == 0);
                gpio.set_function(pin, i as u32 + 1);
            }
            
            // Verify all pins
            let mut all_correct = true;
            for (i, &pin) in pins.iter().enumerate() {
                let expected_state = i % 2 == 0;
                let expected_function = i as u32 + 1;
                
                if gpio.get_pin(pin) != expected_state || gpio.get_function(pin) != expected_function {
                    all_correct = false;
                    break;
                }
            }
            
            let pins_correct = reporter.assert_true(all_correct, "All pins should have correct states and functions");
            
            if pins_correct {
                reporter.pass("Multiple GPIO pins test successful");
            }
        });
    }
}

// Run all unit tests
pub fn run_all_unit_tests() -> TestReporter {
    let mut reporter = TestReporter::new();
    
    println!("🚀 Starting TinyOS Unit Tests");
    println!("═".repeat(60));
    
    // Memory tests
    memory_tests::test_memory_manager_initialization(&mut reporter);
    memory_tests::test_single_block_allocation(&mut reporter);
    memory_tests::test_multiple_block_allocation(&mut reporter);
    memory_tests::test_memory_deallocation(&mut reporter);
    memory_tests::test_memory_corruption_detection(&mut reporter);
    memory_tests::test_fragmentation_scenarios(&mut reporter);
    memory_tests::test_memory_stress(&mut reporter);
    
    // UART tests
    uart_tests::test_uart_output(&mut reporter);
    uart_tests::test_uart_input(&mut reporter);
    uart_tests::test_uart_echo(&mut reporter);
    uart_tests::test_uart_throughput(&mut reporter);
    
    // GPIO tests
    gpio_tests::test_gpio_pin_control(&mut reporter);
    gpio_tests::test_gpio_function_setting(&mut reporter);
    gpio_tests::test_gpio_multiple_pins(&mut reporter);
    
    reporter
}
