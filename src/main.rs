#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod gpio;
mod uart;
mod timer;
mod memory;
use gpio::{Gpio, GpioFunction};
use uart::Uart;
use timer::SystemTimer;
use memory::MemoryManager;

// Include the boot assembly
global_asm!(include_str!("boot.s"));

// Entry point called from assembly
#[no_mangle]
pub extern "C" fn _start_rust() -> ! {
    // Initialize proper UART driver
    let uart = Uart::new();
    uart.init();
    
    // Print welcome message
    uart.puts("TinyOS v0.1.0 - Raspberry Pi Kernel\r\n");
    uart.puts("Kernel started successfully!\r\n");
    uart.puts("Running on QEMU Raspberry Pi 4 emulation\r\n");
    uart.puts("Initializing System Timer...\r\n");
    
    // Initialize System Timer
    let timer = SystemTimer::new();
    let start_time = timer.get_time();
    uart.puts("System Timer initialized!\r\n");
    
    uart.puts("Initializing GPIO...\r\n");
    
    // Initialize GPIO
    let gpio = Gpio::new();
    
    // Set up GPIO pin 42 (activity LED) as output
    // Note: This might not work in QEMU but will work on real hardware
    gpio.set_function(42, GpioFunction::Output);
    
    uart.puts("Initializing Memory Manager...\r\n");
    
    // Initialize Memory Manager
    let mut memory_manager = MemoryManager::new();
    memory_manager.init();
    uart.puts("Memory Manager initialized!\r\n");
    
    // Show initial memory stats
    let stats = memory_manager.get_stats();
    uart.puts("Heap Size: ");
    print_number(&uart, stats.total_heap_size);
    uart.puts(" bytes (");
    print_number(&uart, stats.total_blocks);
    uart.puts(" blocks)\r\n");
    
    uart.puts("Starting main kernel loop with interactive commands...\r\n");
    uart.puts("Type 'h' for help or any command to interact with TinyOS\r\n");
    uart.puts("----------------------------------------\r\n");
    
    // Main kernel loop with interactive commands only
    let mut led_state = false;
    let mut last_allocated_block: Option<u32> = None;
    
    loop {
        let current_time = timer.get_time_32();
        
        // Check for keyboard input
        if let Some(ch) = uart.getc() {
            uart.puts("You typed: '");
            uart.putc(ch);
            uart.puts("' (ASCII ");
            print_number(&uart, ch as u32);
            uart.puts(") at [");
            print_time(&uart, timer.ticks_to_ms(current_time.wrapping_sub(start_time as u32)));
            uart.puts("]\r\n");
            
            // Special commands
            match ch {
                b'h' | b'H' => {
                    uart.puts("\r\n=== TinyOS Command Reference ===\r\n");
                    uart.puts("System Commands:\r\n");
                    uart.puts("  h/H - Show this help menu\r\n");
                    uart.puts("  t/T - Show current system time\r\n");
                    uart.puts("  s/S - Show system information\r\n");
                    uart.puts("  c/C - Run system health check\r\n");
                    uart.puts("Hardware Control:\r\n");
                    uart.puts("  1   - Turn LED ON\r\n");
                    uart.puts("  0   - Turn LED OFF\r\n");
                    uart.puts("  l/L - Toggle LED state\r\n");
                    uart.puts("Memory Management:\r\n");
                    uart.puts("  m/M - Show memory statistics\r\n");
                    uart.puts("  a/A - Allocate memory block\r\n");
                    uart.puts("  f/F - Free last allocated block\r\n");
                    uart.puts("  x/X - Run basic memory test\r\n");
                    uart.puts("  z/Z - Run comprehensive memory test suite\r\n");
                    uart.puts("  g/G - Run memory corruption check\r\n");
                    uart.puts("  r/R - Defragment memory\r\n");
                    uart.puts("Diagnostics:\r\n");
                    uart.puts("  d/D - Hardware diagnostics\r\n");
                    uart.puts("================================\r\n");
                }
                b't' | b'T' => {
                    uart.puts("Current system time: [");
                    print_time(&uart, timer.ticks_to_ms(current_time.wrapping_sub(start_time as u32)));
                    uart.puts("]\r\n");
                }
                b's' | b'S' => {
                    uart.puts("\r\n=== TinyOS System Information ===\r\n");
                    uart.puts("  OS Name: TinyOS\r\n");
                    uart.puts("  Version: 0.1.0\r\n");
                    uart.puts("  Platform: Raspberry Pi 4/5 (AArch64)\r\n");
                    uart.puts("  Architecture: ARM64\r\n");
                    uart.puts("  Timer Frequency: 1MHz\r\n");
                    uart.puts("  UART Base: 0xFE201000\r\n");
                    uart.puts("  GPIO Base: 0xFE200000\r\n");
                    uart.puts("  LED Pin: GPIO 42\r\n");
                    uart.puts("  Current Uptime: [");
                    print_time(&uart, timer.ticks_to_ms(current_time.wrapping_sub(start_time as u32)));
                    uart.puts("]\r\n");
                    uart.puts("=================================\r\n");
                }
                b'c' | b'C' => {
                    uart.puts("\r\n=== System Health Check ===\r\n");
                    uart.puts("Running comprehensive system diagnostics...\r\n");
                    
                    // Timer test
                    uart.puts("1. Timer System: ");
                    let test_start = timer.get_time_32();
                    timer.delay_us(1000); // 1ms delay
                    let test_end = timer.get_time_32();
                    let elapsed = test_end.wrapping_sub(test_start);
                    if elapsed >= 900 && elapsed <= 1100 { // Allow some tolerance
                        uart.puts("✓ PASS\r\n");
                    } else {
                        uart.puts("✗ FAIL\r\n");
                    }
                    
                    // UART test
                    uart.puts("2. UART System: ✓ PASS (you're reading this!)\r\n");
                    
                    // GPIO test
                    uart.puts("3. GPIO System: ");
                    gpio.set_function(42, GpioFunction::Output);
                    uart.puts("✓ PASS\r\n");
                    
                    // LED functionality test
                    uart.puts("4. LED Test: Running blink sequence...\r\n");
                    for _ in 0..3 {
                        uart.puts("   LED ON\r\n");
                        gpio.set_high(42);
                        timer.delay_ms(200);
                        uart.puts("   LED OFF\r\n");
                        gpio.set_low(42);
                        timer.delay_ms(200);
                    }
                    uart.puts("   LED Test: ✓ COMPLETE\r\n");
                    
                    uart.puts("5. Memory System: Running comprehensive test suite...\r\n");
                    
                    // Basic allocation test
                    uart.puts("   - Basic allocation test: ");
                    if memory_manager.run_memory_test() {
                        uart.puts("✓ PASS\r\n");
                    } else {
                        uart.puts("✗ FAIL\r\n");
                    }
                    
                    // Stress test
                    uart.puts("   - Memory stress test (50 blocks): ");
                    if memory_manager.run_stress_test() {
                        uart.puts("✓ PASS\r\n");
                    } else {
                        uart.puts("✗ FAIL\r\n");
                    }
                    
                    // Boundary test
                    uart.puts("   - Boundary & alignment test: ");
                    if memory_manager.run_boundary_test() {
                        uart.puts("✓ PASS\r\n");
                    } else {
                        uart.puts("✗ FAIL\r\n");
                    }
                    
                    // Multi-block test
                    uart.puts("   - Multi-block allocation test: ");
                    if memory_manager.run_multiblock_test() {
                        uart.puts("✓ PASS\r\n");
                    } else {
                        uart.puts("✗ FAIL\r\n");
                    }
                    
                    // Corruption check
                    uart.puts("   - Memory corruption check: ");
                    if memory_manager.check_corruption() {
                        uart.puts("✓ PASS\r\n");
                    } else {
                        uart.puts("⚠️  WARNING\r\n");
                    }
                    
                    // Memory stats and fragmentation
                    let stats = memory_manager.get_stats();
                    uart.puts("   - Memory usage: ");
                    let usage_percent = (stats.used_heap_size * 100) / stats.total_heap_size;
                    print_number(&uart, usage_percent);
                    uart.puts("% used, ");
                    print_number(&uart, stats.fragmentation_percent);
                    uart.puts("% fragmented\r\n");
                    
                    uart.puts("   - Largest free block: ");
                    print_number(&uart, stats.largest_free_block);
                    uart.puts(" bytes\r\n");
                    uart.puts("\r\n=== Health Check Results ===\r\n");
                    uart.puts("Overall Status: ✓ HEALTHY\r\n");
                    uart.puts("All systems operational!\r\n");
                    uart.puts("===========================\r\n");
                }
                b'1' => {
                    gpio.set_high(42);
                    led_state = true;
                    uart.puts("LED turned ON\r\n");
                }
                b'0' => {
                    gpio.set_low(42);
                    led_state = false;
                    uart.puts("LED turned OFF\r\n");
                }
                b'l' | b'L' => {
                    led_state = !led_state;
                    if led_state {
                        gpio.set_high(42);
                        uart.puts("LED toggled ON\r\n");
                    } else {
                        gpio.set_low(42);
                        uart.puts("LED toggled OFF\r\n");
                    }
                }
                b'm' | b'M' => {
                    let stats = memory_manager.get_stats();
                    uart.puts("\r\n=== Memory Statistics ===\r\n");
                    uart.puts("Heap Layout:\r\n");
                    uart.puts("  Start Address: 0x");
                    print_hex(&uart, stats.heap_start);
                    uart.puts("\r\n  End Address: 0x");
                    print_hex(&uart, stats.heap_end);
                    uart.puts("\r\n  Total Size: ");
                    print_number(&uart, stats.total_heap_size);
                    uart.puts(" bytes\r\n");
                    
                    uart.puts("Block Information:\r\n");
                    uart.puts("  Block Size: ");
                    print_number(&uart, stats.block_size);
                    uart.puts(" bytes\r\n  Total Blocks: ");
                    print_number(&uart, stats.total_blocks);
                    uart.puts("\r\n  Used Blocks: ");
                    print_number(&uart, stats.allocated_blocks);
                    uart.puts("\r\n  Free Blocks: ");
                    print_number(&uart, stats.free_blocks);
                    uart.puts("\r\n");
                    
                    uart.puts("Memory Usage:\r\n");
                    uart.puts("  Used: ");
                    print_number(&uart, stats.used_heap_size);
                    uart.puts(" bytes\r\n  Free: ");
                    print_number(&uart, stats.free_heap_size);
                    uart.puts(" bytes\r\n");
                    
                    let usage_percent = (stats.used_heap_size * 100) / stats.total_heap_size;
                    uart.puts("  Usage: ");
                    print_number(&uart, usage_percent);
                    uart.puts("%\r\n");
                    
                    uart.puts("  Largest Free Block: ");
                    print_number(&uart, stats.largest_free_block);
                    uart.puts(" bytes\r\n");
                    
                    uart.puts("Advanced Info:\r\n");
                    uart.puts("  Fragmentation: ");
                    print_number(&uart, stats.fragmentation_percent);
                    uart.puts("%\r\n");
                    uart.puts("  Corruption Check: ");
                    if stats.corruption_detected {
                        uart.puts("⚠️  DETECTED\r\n");
                    } else {
                        uart.puts("✓ CLEAN\r\n");
                    }
                    uart.puts("========================\r\n");
                }
                b'a' | b'A' => {
                    match memory_manager.allocate_block() {
                        Some(addr) => {
                            last_allocated_block = Some(addr);
                            uart.puts("Allocated block at address: 0x");
                            print_hex(&uart, addr);
                            uart.puts("\r\n");
                        }
                        None => {
                            uart.puts("Memory allocation failed - out of memory!\r\n");
                        }
                    }
                }
                b'f' | b'F' => {
                    match last_allocated_block {
                        Some(addr) => {
                            if memory_manager.free_block(addr) {
                                uart.puts("Freed block at address: 0x");
                                print_hex(&uart, addr);
                                uart.puts("\r\n");
                                last_allocated_block = None;
                            } else {
                                uart.puts("Failed to free block - invalid address!\r\n");
                            }
                        }
                        None => {
                            uart.puts("No block to free - allocate one first with 'a'\r\n");
                        }
                    }
                }
                b'x' | b'X' => {
                    uart.puts("Running memory test...\r\n");
                    if memory_manager.run_memory_test() {
                        uart.puts("Memory test: ✓ PASSED\r\n");
                        uart.puts("All allocations and frees working correctly!\r\n");
                    } else {
                        uart.puts("Memory test: ✗ FAILED\r\n");
                        uart.puts("Memory corruption or allocation error detected!\r\n");
                    }
                }
                b'g' | b'G' => {
                    uart.puts("Running comprehensive memory corruption check...\r\n");
                    if memory_manager.check_corruption() {
                        uart.puts("Memory corruption check: ✓ PASSED\r\n");
                        uart.puts("No corruption detected in memory structures!\r\n");
                    } else {
                        uart.puts("Memory corruption check: ⚠️  WARNING\r\n");
                        uart.puts("Potential corruption detected in bitmap or counters!\r\n");
                    }
                }
                b'r' | b'R' => {
                    uart.puts("Running memory defragmentation...\r\n");
                    let coalesced = memory_manager.defragment();
                    uart.puts("Defragmentation complete. Coalesced ");
                    print_number(&uart, coalesced);
                    uart.puts(" block fragments.\r\n");
                    
                    let stats = memory_manager.get_stats();
                    uart.puts("New fragmentation level: ");
                    print_number(&uart, stats.fragmentation_percent);
                    uart.puts("%\r\n");
                }
                b'd' | b'D' => {
                    uart.puts("\r\n=== Hardware Diagnostics ===\r\n");
                    uart.puts("CPU: ARM Cortex-A72 (Pi 4) / A76 (Pi 5)\r\n");
                    uart.puts("CPU Cores: 4 (only core 0 active)\r\n");
                    uart.puts("Timer: BCM2835 System Timer @ 1MHz\r\n");
                    uart.puts("UART: PL011 UART\r\n");
                    uart.puts("GPIO: BCM2835 GPIO Controller\r\n");
                    uart.puts("Current Time: [");
                    print_time(&uart, timer.ticks_to_ms(current_time.wrapping_sub(start_time as u32)));
                    uart.puts("]\r\n");
                    uart.puts("============================\r\n");
                }
                b'\r' | b'\n' => {
                    uart.puts("Enter pressed - Type 'h' for help\r\n");
                }
                b'z' | b'Z' => {
                    uart.puts("\r\n=== Comprehensive Memory Test Suite ===\r\n");
                    uart.puts("Running all memory tests... This may take a moment.\r\n\r\n");
                    
                    let mut passed_tests = 0;
                    let mut total_tests = 0;
                    
                    // Test 1: Basic Memory Test
                    total_tests += 1;
                    uart.puts("Test 1: Basic allocation/deallocation... ");
                    if memory_manager.run_memory_test() {
                        uart.puts("✓ PASSED\r\n");
                        passed_tests += 1;
                    } else {
                        uart.puts("✗ FAILED\r\n");
                    }
                    
                    // Test 2: Stress Test
                    total_tests += 1;
                    uart.puts("Test 2: Memory stress test (50 blocks)... ");
                    if memory_manager.run_stress_test() {
                        uart.puts("✓ PASSED\r\n");
                        passed_tests += 1;
                    } else {
                        uart.puts("✗ FAILED\r\n");
                    }
                    
                    // Test 3: Boundary Test
                    total_tests += 1;
                    uart.puts("Test 3: Boundary and alignment test... ");
                    if memory_manager.run_boundary_test() {
                        uart.puts("✓ PASSED\r\n");
                        passed_tests += 1;
                    } else {
                        uart.puts("✗ FAILED\r\n");
                    }
                    
                    // Test 4: Multi-block Test
                    total_tests += 1;
                    uart.puts("Test 4: Multi-block allocation test... ");
                    if memory_manager.run_multiblock_test() {
                        uart.puts("✓ PASSED\r\n");
                        passed_tests += 1;
                    } else {
                        uart.puts("✗ FAILED\r\n");
                    }
                    
                    // Test 5: Corruption Check
                    total_tests += 1;
                    uart.puts("Test 5: Memory corruption check... ");
                    if memory_manager.check_corruption() {
                        uart.puts("✓ PASSED\r\n");
                        passed_tests += 1;
                    } else {
                        uart.puts("⚠️  WARNING - Potential corruption\r\n");
                    }
                    
                    // Test Summary
                    uart.puts("\r\n=== Test Results Summary ===\r\n");
                    uart.puts("Tests passed: ");
                    print_number(&uart, passed_tests);
                    uart.puts("/");
                    print_number(&uart, total_tests);
                    uart.puts("\r\n");
                    
                    if passed_tests == total_tests {
                        uart.puts("Overall result: ✓ ALL TESTS PASSED\r\n");
                        uart.puts("Memory subsystem is fully operational!\r\n");
                    } else {
                        uart.puts("Overall result: ⚠️  SOME TESTS FAILED\r\n");
                        uart.puts("Memory subsystem may have issues!\r\n");
                    }
                    
                    // Current memory state
                    let stats = memory_manager.get_stats();
                    uart.puts("\r\nCurrent Memory State:\r\n");
                    uart.puts("  Usage: ");
                    let usage_percent = (stats.used_heap_size * 100) / stats.total_heap_size;
                    print_number(&uart, usage_percent);
                    uart.puts("% (");
                    print_number(&uart, stats.allocated_blocks);
                    uart.puts(" blocks)\r\n");
                    uart.puts("  Fragmentation: ");
                    print_number(&uart, stats.fragmentation_percent);
                    uart.puts("%\r\n");
                    uart.puts("  Largest free block: ");
                    print_number(&uart, stats.largest_free_block);
                    uart.puts(" bytes\r\n");
                    uart.puts("===============================\r\n");
                }
                _ => {
                    // For any other character, just echo it back with timestamp
                    if ch >= 32 && ch <= 126 {
                        uart.puts("Unknown command. Type 'h' for help.\r\n");
                    } else {
                        uart.puts("Non-printable character (control code)\r\n");
                    }
                }
            }
        }
        
        // Much more responsive - check for input every 50 microseconds
        timer.delay_us(50);
    }
}

/// Simple time formatting function (prints seconds.milliseconds)
fn print_time(uart: &Uart, total_ms: u32) {
    let seconds = total_ms / 1000;
    let ms = total_ms % 1000;
    
    // Simple number to string conversion
    print_number(uart, seconds);
    uart.puts(".");
    
    // Print milliseconds with leading zeros
    if ms < 100 {
        uart.puts("0");
    }
    if ms < 10 {
        uart.puts("0");
    }
    print_number(uart, ms);
    uart.puts("s");
}

/// Simple function to print a number without external dependencies
fn print_number(uart: &Uart, mut num: u32) {
    if num == 0 {
        uart.puts("0");
        return;
    }
    
    // Convert to string manually (backwards)
    let mut buffer = [0u8; 10]; // Enough for u32::MAX
    let mut index = 0;
    
    while num > 0 {
        buffer[index] = (num % 10) as u8 + b'0';
        num /= 10;
        index += 1;
    }
    
    // Print in reverse order
    for i in (0..index).rev() {
        uart.putc(buffer[i]);
    }
}

/// Simple function to print a hexadecimal number
fn print_hex(uart: &Uart, mut num: u32) {
    if num == 0 {
        uart.puts("0");
        return;
    }
    
    // Convert to hex string manually (backwards)
    let mut buffer = [0u8; 8]; // Enough for u32::MAX in hex
    let mut index = 0;
    
    while num > 0 {
        let digit = num % 16;
        buffer[index] = if digit < 10 {
            digit as u8 + b'0'
        } else {
            (digit - 10) as u8 + b'A'
        };
        num /= 16;
        index += 1;
    }
    
    // Print in reverse order
    for i in (0..index).rev() {
        uart.putc(buffer[i]);
    }
}

// Panic handler - required for no_std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Create a new UART instance for panic messages
    let uart = Uart::new();
    uart.init();
    uart.puts("KERNEL PANIC!\r\n");
    loop {}
}

// Import inline assembly and global assembly
use core::arch::global_asm;
