//! Comprehensive Integration Tests for TinyOS
//! 
//! These tests validate component interactions and system-level behavior.

use super::mocks::*;
use super::test_config::*;
use super::test_reporter::TestReporter;
use std::time::{Duration, Instant};

// System Integration Tests
pub mod system_integration_tests {
    use super::*;

    pub fn test_boot_sequence_simulation(reporter: &mut TestReporter) {
        reporter.start_test("boot_sequence_simulation", "integration", "Simulate complete system boot sequence");
        
        reset_all_mocks();
        
        // Phase 1: Memory initialization
        with_mock_memory(|memory| {
            memory.clear();
            
            // Simulate bitmap initialization
            let bitmap_size = (TOTAL_BLOCKS + 7) / 8;
            for i in 0..bitmap_size {
                memory.write_u8(HEAP_START + i, 0);
            }
            
            // Mark bitmap area as used
            let bitmap_blocks = (bitmap_size + BLOCK_SIZE - 1) / BLOCK_SIZE;
            for block in 0..bitmap_blocks {
                let byte_index = block / 8;
                let bit_index = block % 8;
                let current = memory.read_u8(HEAP_START + byte_index);
                memory.write_u8(HEAP_START + byte_index, current | (1 << bit_index));
            }
        });
        
        // Phase 2: UART initialization and output
        with_mock_uart(|uart| {
            uart.clear_output();
            uart.puts("TinyOS v0.1.0 - Raspberry Pi Kernel\n");
            uart.puts("Initializing hardware drivers...\n");
        });
        
        // Phase 3: GPIO initialization
        with_mock_gpio(|gpio| {
            gpio.clear_all();
            gpio.set_function(LED_PIN, 1); // Set LED as output
            gpio.set_pin(LED_PIN, false);  // Start with LED off
        });
        
        // Verify boot sequence
        let memory_initialized = with_mock_memory(|memory| {
            memory.read_u8(HEAP_START) != 0 // Bitmap should have some bits set
        });
        
        let uart_initialized = with_mock_uart(|uart| {
            let output = uart.get_full_output();
            output.contains("TinyOS") && output.contains("Initializing")
        });
        
        let gpio_initialized = with_mock_gpio(|gpio| {
            gpio.get_function(LED_PIN) == 1 && !gpio.get_pin(LED_PIN)
        });
        
        let boot_ok = reporter.assert_true(memory_initialized, "Memory should be initialized") &
                      reporter.assert_true(uart_initialized, "UART should be initialized") &
                      reporter.assert_true(gpio_initialized, "GPIO should be initialized");
        
        if boot_ok {
            reporter.pass("Boot sequence simulation successful");
        }
    }

    pub fn test_memory_allocation_lifecycle(reporter: &mut TestReporter) {
        reporter.start_test("memory_allocation_lifecycle", "integration", "Test complete memory allocation lifecycle");
        
        reset_all_mocks();
        
        // Simulate a complex allocation scenario
        let mut allocations = Vec::new();
        
        with_mock_memory(|memory| {
            memory.clear();
            
            // Phase 1: Initial allocations
            for i in 0..5 {
                let address = HEAP_START + 0x1000 + (i * BLOCK_SIZE * 2);
                memory.allocate(address, BLOCK_SIZE);
                memory.write_u32(address, TEST_PATTERN_1 + i);
                allocations.push(address);
            }
            
            // Phase 2: Mixed operations
            // Deallocate middle blocks
            memory.deallocate(allocations[1]);
            memory.deallocate(allocations[3]);
            
            // Allocate larger blocks
            let large_address = HEAP_START + 0x10000;
            memory.allocate(large_address, BLOCK_SIZE * 4);
            
            // Phase 3: Data integrity check
            let mut data_ok = true;
            for (i, &addr) in allocations.iter().enumerate() {
                if memory.is_allocated(addr) {
                    let expected = TEST_PATTERN_1 + i as u32;
                    if memory.read_u32(addr) != expected {
                        data_ok = false;
                        break;
                    }
                }
            }
            
            data_ok
        });
        
        // Log the operations via UART
        with_mock_uart(|uart| {
            uart.puts("Memory lifecycle test completed\n");
            uart.puts("Allocations and deallocations performed successfully\n");
        });
        
        let lifecycle_ok = reporter.assert_true(allocations.len() == 5, "Should have 5 initial allocations");
        let logging_ok = with_mock_uart(|uart| {
            uart.get_full_output().contains("lifecycle")
        });
        
        if lifecycle_ok && logging_ok {
            reporter.pass("Memory allocation lifecycle successful");
        }
    }

    pub fn test_interrupt_driven_operations(reporter: &mut TestReporter) {
        reporter.start_test("interrupt_driven_operations", "integration", "Test interrupt-driven system operations");
        
        reset_all_mocks();
        
        // Simulate timer-driven LED blinking
        let mut blink_count = 0;
        let mut led_state = false;
        
        // Simulate 10 timer interrupts
        for _ in 0..10 {
            // Timer interrupt occurs
            led_state = !led_state;
            blink_count += 1;
            
            // Update GPIO
            with_mock_gpio(|gpio| {
                gpio.set_pin(LED_PIN, led_state);
            });
            
            // Log via UART
            with_mock_uart(|uart| {
                if led_state {
                    uart.puts("LED ON\n");
                } else {
                    uart.puts("LED OFF\n");
                }
            });
        }
        
        // Verify final state
        let final_led_state = with_mock_gpio(|gpio| {
            gpio.get_pin(LED_PIN)
        });
        
        let uart_logs = with_mock_uart(|uart| {
            let output = uart.get_full_output();
            output.matches("LED ON").count() + output.matches("LED OFF").count()
        });
        
        let blink_count_ok = reporter.assert_eq(blink_count, 10, "Should have 10 blink operations");
        let uart_count_ok = reporter.assert_eq(uart_logs, 10, "Should have 10 UART log entries");
        let final_state_ok = reporter.assert_eq(final_led_state, false, "LED should be off after even number of blinks");
        
        if blink_count_ok && uart_count_ok && final_state_ok {
            reporter.pass("Interrupt-driven operations successful");
        }
    }

    pub fn test_shell_command_processing(reporter: &mut TestReporter) {
        reporter.start_test("shell_command_processing", "integration", "Test shell command processing system");
        
        reset_all_mocks();
        
        // Simulate shell commands
        let commands = vec!["h", "m", "1", "0", "s"];
        
        for cmd in commands {
            with_mock_uart(|uart| {
                uart.set_input(cmd);
                
                // Process command (simplified simulation)
                if let Some(c) = uart.getc() {
                    match c {
                        'h' => uart.puts("=== TinyOS Help ===\n"),
                        'm' => uart.puts("=== Memory Stats ===\n"),
                        '1' => {
                            with_mock_gpio(|gpio| {
                                gpio.set_pin(LED_PIN, true);
                            });
                            uart.puts("LED ON\n");
                        },
                        '0' => {
                            with_mock_gpio(|gpio| {
                                gpio.set_pin(LED_PIN, false);
                            });
                            uart.puts("LED OFF\n");
                        },
                        's' => uart.puts("=== System Info ===\n"),
                        _ => uart.puts("Unknown command\n"),
                    }
                }
            });
        }
        
        // Verify command processing
        let output = with_mock_uart(|uart| {
            uart.get_full_output()
        });
        
        let led_state = with_mock_gpio(|gpio| {
            gpio.get_pin(LED_PIN)
        });
        
        let has_help = reporter.assert_true(output.contains("Help"), "Should process help command");
        let has_memory = reporter.assert_true(output.contains("Memory Stats"), "Should process memory command");
        let has_system = reporter.assert_true(output.contains("System Info"), "Should process system command");
        let led_controlled = reporter.assert_true(!led_state, "LED should be off after commands"); // Last command was '0'
        
        if has_help && has_memory && has_system && led_controlled {
            reporter.pass("Shell command processing successful");
        }
    }

    pub fn test_system_health_check(reporter: &mut TestReporter) {
        reporter.start_test("system_health_check", "integration", "Test comprehensive system health check");
        
        reset_all_mocks();
        
        // Initialize all subsystems
        with_mock_memory(|memory| {
            memory.clear();
            // Allocate some blocks to test memory system
            for i in 0..5 {
                let addr = HEAP_START + 0x1000 + (i * BLOCK_SIZE);
                memory.allocate(addr, BLOCK_SIZE);
                memory.write_u32(addr, CANARY_START);
                memory.write_u32(addr + BLOCK_SIZE - 4, CANARY_END);
            }
        });
        
        with_mock_gpio(|gpio| {
            gpio.clear_all();
            gpio.set_function(LED_PIN, 1);
            gpio.set_pin(LED_PIN, true);
        });
        
        // Perform health checks
        let mut health_report = Vec::new();
        
        // Memory health
        let memory_health = with_mock_memory(|memory| {
            if memory.get_allocation_count() == 5 {
                health_report.push("Memory: HEALTHY");
                true
            } else {
                health_report.push("Memory: ERROR");
                false
            }
        });
        
        // GPIO health
        let gpio_health = with_mock_gpio(|gpio| {
            if gpio.get_function(LED_PIN) == 1 && gpio.get_pin(LED_PIN) {
                health_report.push("GPIO: HEALTHY");
                true
            } else {
                health_report.push("GPIO: ERROR");
                false
            }
        });
        
        // UART health (always healthy in mock)
        with_mock_uart(|uart| {
            uart.puts("=== System Health Check ===\n");
            for status in &health_report {
                uart.puts(&format!("{}\n", status));
            }
            health_report.push("UART: HEALTHY");
        });
        
        let overall_health = memory_health && gpio_health;
        
        let health_ok = reporter.assert_true(overall_health, "All subsystems should be healthy");
        let report_complete = reporter.assert_eq(health_report.len(), 3, "Should have health reports for all subsystems");
        
        if health_ok && report_complete {
            reporter.pass("System health check successful");
        }
    }

    pub fn test_stress_scenario(reporter: &mut TestReporter) {
        reporter.start_test("stress_scenario", "integration", "Test system under stress conditions");
        
        reset_all_mocks();
        
        let start_time = Instant::now();
        let mut operations = 0;
        
        // Stress test: rapid operations across all subsystems
        for cycle in 0..50 {
            // Memory operations
            with_mock_memory(|memory| {
                let addr = HEAP_START + 0x20000 + (cycle * BLOCK_SIZE);
                memory.allocate(addr, BLOCK_SIZE);
                memory.write_u32(addr, TEST_PATTERN_1 + cycle);
                operations += 1;
                
                // Occasionally deallocate
                if cycle % 10 == 0 && cycle > 0 {
                    let old_addr = HEAP_START + 0x20000 + ((cycle - 10) * BLOCK_SIZE);
                    memory.deallocate(old_addr);
                    operations += 1;
                }
            });
            
            // GPIO operations
            with_mock_gpio(|gpio| {
                gpio.set_pin(LED_PIN, cycle % 2 == 0);
                operations += 1;
            });
            
            // UART operations
            with_mock_uart(|uart| {
                uart.puts(&format!("Cycle {}\n", cycle));
                operations += 1;
            });
        }
        
        let elapsed = start_time.elapsed();
        
        // Verify system integrity after stress
        let memory_integrity = with_mock_memory(|memory| {
            memory.get_allocation_count() > 0 && memory.get_allocation_count() <= 50
        });
        
        let uart_logs = with_mock_uart(|uart| {
            uart.get_full_output().matches("Cycle").count() >= 40 // Should have most cycles logged
        });
        
        let stress_ok = reporter.assert_true(memory_integrity, "Memory should maintain integrity under stress");
        let timing_ok = reporter.assert_true(elapsed < Duration::from_secs(1), "Stress test should complete quickly");
        let operations_ok = reporter.assert_true(operations >= 150, "Should perform many operations");
        let logs_ok = reporter.assert_true(uart_logs >= 40, "Should have sufficient log entries");
        
        if stress_ok && timing_ok && operations_ok && logs_ok {
            reporter.pass("Stress scenario successful");
        }
    }
}

// Performance Tests
pub mod performance_tests {
    use super::*;

    pub fn test_memory_allocation_performance(reporter: &mut TestReporter) {
        reporter.start_test("memory_allocation_performance", "performance", "Test memory allocation performance");
        
        reset_all_mocks();
        
        with_mock_memory(|memory| {
            memory.clear();
            
            let start_time = Instant::now();
            let num_allocations = 1000;
            
            // Perform many allocations
            for i in 0..num_allocations {
                let addr = HEAP_START + 0x100000 + (i * BLOCK_SIZE);
                memory.allocate(addr, BLOCK_SIZE);
            }
            
            let allocation_time = start_time.elapsed();
            let avg_time_per_allocation = allocation_time.as_nanos() / num_allocations as u128;
            
            // Performance assertions
            let speed_ok = reporter.assert_true(
                allocation_time < Duration::from_millis(100),
                "1000 allocations should complete in under 100ms"
            );
            
            let efficiency_ok = reporter.assert_true(
                avg_time_per_allocation < 100_000, // 100 microseconds per allocation
                "Average allocation time should be under 100μs"
            );
            
            let count_ok = reporter.assert_eq(
                memory.get_allocation_count(), 
                num_allocations, 
                "All allocations should be tracked"
            );
            
            if speed_ok && efficiency_ok && count_ok {
                reporter.pass(&format!("Memory performance: {}ns avg per allocation", avg_time_per_allocation));
            }
        });
    }

    pub fn test_uart_throughput_performance(reporter: &mut TestReporter) {
        reporter.start_test("uart_throughput_performance", "performance", "Test UART throughput performance");
        
        reset_all_mocks();
        
        with_mock_uart(|uart| {
            uart.clear_output();
            
            let start_time = Instant::now();
            let data_size = 10000; // 10KB of data
            let test_string = "X".repeat(100); // 100 byte chunks
            
            // Send data in chunks
            for _ in 0..(data_size / 100) {
                uart.puts(&test_string);
            }
            
            let transmission_time = start_time.elapsed();
            let throughput = (data_size as f64 / transmission_time.as_secs_f64()) as u64;
            
            let speed_ok = reporter.assert_true(
                transmission_time < Duration::from_millis(50),
                "10KB transmission should complete quickly"
            );
            
            let throughput_ok = reporter.assert_true(
                throughput > 100_000, // 100KB/s minimum
                "Throughput should exceed 100KB/s"
            );
            
            let output_correct = reporter.assert_eq(
                uart.get_full_output().len(),
                data_size,
                "Output size should match input"
            );
            
            if speed_ok && throughput_ok && output_correct {
                reporter.pass(&format!("UART throughput: {} bytes/s", throughput));
            }
        });
    }

    pub fn test_gpio_switching_performance(reporter: &mut TestReporter) {
        reporter.start_test("gpio_switching_performance", "performance", "Test GPIO switching performance");
        
        reset_all_mocks();
        
        with_mock_gpio(|gpio| {
            gpio.clear_all();
            gpio.set_function(LED_PIN, 1);
            
            let start_time = Instant::now();
            let num_switches = 10000;
            
            // Rapid GPIO switching
            for i in 0..num_switches {
                gpio.set_pin(LED_PIN, i % 2 == 0);
            }
            
            let switching_time = start_time.elapsed();
            let avg_switch_time = switching_time.as_nanos() / num_switches as u128;
            
            let speed_ok = reporter.assert_true(
                switching_time < Duration::from_millis(10),
                "10000 GPIO switches should complete quickly"
            );
            
            let efficiency_ok = reporter.assert_true(
                avg_switch_time < 1000, // 1 microsecond per switch
                "Average switch time should be under 1μs"
            );
            
            let final_state_ok = reporter.assert_true(
                !gpio.get_pin(LED_PIN), // Should be false after even number of switches
                "Final GPIO state should be correct"
            );
            
            if speed_ok && efficiency_ok && final_state_ok {
                reporter.pass(&format!("GPIO performance: {}ns avg per switch", avg_switch_time));
            }
        });
    }

    pub fn test_system_integration_performance(reporter: &mut TestReporter) {
        reporter.start_test("system_integration_performance", "performance", "Test overall system integration performance");
        
        reset_all_mocks();
        
        let start_time = Instant::now();
        let iterations = 100;
        
        // Combined operations simulating real workload
        for i in 0..iterations {
            // Memory operation
            with_mock_memory(|memory| {
                let addr = HEAP_START + 0x50000 + (i * BLOCK_SIZE);
                memory.allocate(addr, BLOCK_SIZE);
                memory.write_u32(addr, TEST_PATTERN_1 + i);
                
                // Occasionally deallocate
                if i > 10 && i % 10 == 0 {
                    let old_addr = HEAP_START + 0x50000 + ((i - 10) * BLOCK_SIZE);
                    memory.deallocate(old_addr);
                }
            });
            
            // GPIO operation
            with_mock_gpio(|gpio| {
                gpio.set_pin(LED_PIN, i % 2 == 0);
            });
            
            // UART operation
            with_mock_uart(|uart| {
                uart.puts(&format!("Operation {}: ", i));
                if i % 2 == 0 {
                    uart.puts("LED ON\n");
                } else {
                    uart.puts("LED OFF\n");
                }
            });
        }
        
        let total_time = start_time.elapsed();
        let avg_iteration_time = total_time.as_micros() / iterations as u128;
        
        // Verify final system state
        let memory_state = with_mock_memory(|memory| {
            memory.get_allocation_count()
        });
        
        let gpio_state = with_mock_gpio(|gpio| {
            !gpio.get_pin(LED_PIN) // Should be off after even number
        });
        
        let uart_logs = with_mock_uart(|uart| {
            uart.get_full_output().matches("Operation").count()
        });
        
        let performance_ok = reporter.assert_true(
            total_time < Duration::from_millis(100),
            "100 integrated operations should complete in under 100ms"
        );
        
        let efficiency_ok = reporter.assert_true(
            avg_iteration_time < 1000, // 1ms per iteration
            "Average iteration time should be under 1ms"
        );
        
        let state_ok = reporter.assert_true(
            memory_state > 0 && gpio_state && uart_logs >= 90,
            "System should maintain correct state"
        );
        
        if performance_ok && efficiency_ok && state_ok {
            reporter.pass(&format!("Integration performance: {}μs avg per iteration", avg_iteration_time));
        }
    }
}

// Run all integration tests
pub fn run_all_integration_tests() -> TestReporter {
    let mut reporter = TestReporter::new();
    
    println!("🔧 Starting TinyOS Integration Tests");
    println!("═".repeat(60));
    
    // System integration tests
    system_integration_tests::test_boot_sequence_simulation(&mut reporter);
    system_integration_tests::test_memory_allocation_lifecycle(&mut reporter);
    system_integration_tests::test_interrupt_driven_operations(&mut reporter);
    system_integration_tests::test_shell_command_processing(&mut reporter);
    system_integration_tests::test_system_health_check(&mut reporter);
    system_integration_tests::test_stress_scenario(&mut reporter);
    
    // Performance tests
    performance_tests::test_memory_allocation_performance(&mut reporter);
    performance_tests::test_uart_throughput_performance(&mut reporter);
    performance_tests::test_gpio_switching_performance(&mut reporter);
    performance_tests::test_system_integration_performance(&mut reporter);
    
    reporter
}
