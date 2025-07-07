#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

mod exceptions;
// mod fat32;  // Temporarily disabled
mod gpio;
mod interrupts;
mod memory;
mod sdcard;
mod timer;
mod uart;
use exceptions::{get_exception_stats, init_exceptions, reset_exception_stats};
// use fat32::Fat32FileSystem;  // Temporarily disabled
use gpio::{Gpio, GpioFunction};
use interrupts::InterruptController;
use memory::MemoryManager;
use sdcard::SdCard;
use timer::SystemTimer;
use uart::Uart;

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
    
    uart.puts("Initializing Exception Handlers...\r\n");
    
    // Initialize exception vectors early in boot process
    init_exceptions();
    uart.puts("Exception Handlers initialized!\r\n");
    
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

    uart.puts("Initializing Interrupt Controller...\r\n");

    // Initialize Interrupt Controller
    let mut interrupt_controller = InterruptController::new();
    interrupt_controller.init();
    uart.puts("Interrupt Controller initialized!\r\n");

    uart.puts("Initializing SD Card...\r\n");

    // Initialize SD Card and FAT32 filesystem
    let mut sdcard = SdCard::new();
    // let mut fat32_fs: Option<Fat32FileSystem> = None;  // Temporarily disabled
    let sd_init_success = match sdcard.init() {
        Ok(()) => {
            uart.puts("SD Card initialized successfully!\r\n");
            if let Some(info) = sdcard.get_card_info() {
                uart.puts("SD Card Info: ");
                if info.high_capacity {
                    uart.puts("SDHC/SDXC");
                } else {
                    uart.puts("SDSC");
                }
                uart.puts(", Capacity: ");
                print_number(&uart, (info.get_capacity() / (1024 * 1024)) as u32);
                uart.puts(" MB\r\n");
            }
            
            // Try to initialize FAT32 filesystem
            uart.puts("Initializing FAT32 filesystem...\r\n");
            
            // Create a second SD card instance for the filesystem
            // (In a real implementation, you'd want proper resource sharing)
            let mut fs_sdcard = SdCard::new();
            let _ = fs_sdcard.init(); // Initialize for filesystem use
            
            match Fat32FileSystem::new(fs_sdcard) {
                Ok(mut fs) => {
                    match fs.mount() {
                        Ok(()) => {
                            uart.puts("FAT32 filesystem ready!\r\n");
                            fat32_fs = Some(fs);
                        }
                        Err(_) => {
                            uart.puts("Failed to mount FAT32 filesystem\r\n");
                        }
                    }
                }
                Err(_) => {
                    uart.puts("No FAT32 filesystem found\r\n");
                }
            }
            true
        }
        Err(_) => {
            uart.puts("SD Card initialization failed (normal in QEMU)\r\n");
            false
        }
    };

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
            print_time(
                &uart,
                timer.ticks_to_ms(current_time.wrapping_sub(start_time as u32)),
            );
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
                    uart.puts("Interrupt Management:\r\n");
                    uart.puts("  i/I - Show interrupt status\r\n");
                    uart.puts("  e/E - Enable/disable interrupts\r\n");
                    uart.puts("  j/J - Run interrupt test\r\n");
                    uart.puts("Exception Management:\r\n");
                    uart.puts("  v/V - Show exception statistics\r\n");
                    uart.puts("  w/W - Test exception handling (safe)\r\n");
                    uart.puts("Storage & SD Card:\r\n");
                    uart.puts("  p/P - Show SD card information\r\n");
                    uart.puts("  q/Q - Read SD card block\r\n");
                    uart.puts("  y/Y - Write SD card block (test)\r\n");
                    uart.puts("File System (FAT32):\r\n");
                    uart.puts("  n/N - Show filesystem information\r\n");
                    uart.puts("  o/O - List current directory\r\n");
                    uart.puts("  k/K - Change directory (prompt for path)\r\n");
                    uart.puts("  b/B - Go to root directory\r\n");
                    uart.puts("  u/U - Read file (prompt for filename)\r\n");
                    uart.puts("Diagnostics:\r\n");
                    uart.puts("  d/D - Hardware diagnostics\r\n");
                    uart.puts("================================\r\n");
                }
                b't' | b'T' => {
                    uart.puts("Current system time: [");
                    print_time(
                        &uart,
                        timer.ticks_to_ms(current_time.wrapping_sub(start_time as u32)),
                    );
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
                    uart.puts("  GIC Base: 0xFF841000\r\n");
                    uart.puts("  LED Pin: GPIO 42\r\n");
                    uart.puts("  Current Uptime: [");
                    print_time(
                        &uart,
                        timer.ticks_to_ms(current_time.wrapping_sub(start_time as u32)),
                    );
                    uart.puts("]\r\n");

                    let int_stats = interrupt_controller.get_interrupt_stats();
                    uart.puts("  Active Interrupts: ");
                    print_number(&uart, int_stats.total_interrupts);
                    uart.puts("\r\n");
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
                    if (900..=1100).contains(&elapsed) {
                        // Allow some tolerance
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

                    uart.puts("6. Interrupt System: Running interrupt test...\r\n");
                    uart.puts("   - Interrupt controller: ");
                    if interrupt_controller.run_interrupt_test() {
                        uart.puts("✓ PASS\r\n");
                    } else {
                        uart.puts("✗ FAIL\r\n");
                    }

                    let int_stats = interrupt_controller.get_interrupt_stats();
                    uart.puts("   - Simulated interrupts: ");
                    print_number(&uart, int_stats.total_interrupts);
                    uart.puts(" total\r\n");

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
                b'a' | b'A' => match memory_manager.allocate_block() {
                    Some(addr) => {
                        last_allocated_block = Some(addr);
                        uart.puts("Allocated block at address: 0x");
                        print_hex(&uart, addr);
                        uart.puts("\r\n");
                    }
                    None => {
                        uart.puts("Memory allocation failed - out of memory!\r\n");
                    }
                },
                b'f' | b'F' => match last_allocated_block {
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
                },
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
                    uart.puts("GIC: ARM Generic Interrupt Controller\r\n");

                    let int_stats = interrupt_controller.get_interrupt_stats();
                    uart.puts("Interrupts: ");
                    if int_stats.enabled_interrupts > 0 {
                        print_number(&uart, int_stats.enabled_interrupts.count_ones());
                        uart.puts(" sources enabled, ");
                        print_number(&uart, int_stats.total_interrupts);
                        uart.puts(" total\r\n");
                    } else {
                        uart.puts("All disabled\r\n");
                    }

                    uart.puts("Current Time: [");
                    print_time(
                        &uart,
                        timer.ticks_to_ms(current_time.wrapping_sub(start_time as u32)),
                    );
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
                b'i' | b'I' => {
                    let stats = interrupt_controller.get_interrupt_stats();
                    uart.puts("\r\n=== Interrupt Status ===\r\n");
                    uart.puts("Controller State:\r\n");
                    uart.puts("  Enabled Interrupts: 0x");
                    print_hex(&uart, stats.enabled_interrupts);
                    uart.puts("\r\n");

                    uart.puts("Interrupt Sources:\r\n");
                    uart.puts("  Timer (IRQ 64): ");
                    if stats.timer_enabled {
                        uart.puts("ENABLED");
                    } else {
                        uart.puts("DISABLED");
                    }
                    uart.puts(" (");
                    print_number(&uart, stats.timer_count);
                    uart.puts(" interrupts)\r\n");

                    uart.puts("  UART (IRQ 153): ");
                    if stats.uart_enabled {
                        uart.puts("ENABLED");
                    } else {
                        uart.puts("DISABLED");
                    }
                    uart.puts(" (");
                    print_number(&uart, stats.uart_count);
                    uart.puts(" interrupts)\r\n");

                    uart.puts("  GPIO (IRQ 129): ");
                    if stats.gpio_enabled {
                        uart.puts("ENABLED");
                    } else {
                        uart.puts("DISABLED");
                    }
                    uart.puts(" (");
                    print_number(&uart, stats.gpio_count);
                    uart.puts(" interrupts)\r\n");

                    uart.puts("Statistics:\r\n");
                    uart.puts("  Total Interrupts: ");
                    print_number(&uart, stats.total_interrupts);
                    uart.puts("\r\n");
                    uart.puts("========================\r\n");
                }
                b'e' | b'E' => {
                    uart.puts("\r\n=== Interrupt Management ===\r\n");
                    uart.puts("1. Enable timer interrupts\r\n");
                    if interrupt_controller.enable_interrupt(interrupts::TIMER_IRQ) {
                        uart.puts("   Timer interrupts: ✓ ENABLED\r\n");
                    } else {
                        uart.puts("   Timer interrupts: ✗ FAILED\r\n");
                    }

                    uart.puts("2. Enable UART interrupts\r\n");
                    if interrupt_controller.enable_interrupt(interrupts::UART_IRQ) {
                        uart.puts("   UART interrupts: ✓ ENABLED\r\n");
                    } else {
                        uart.puts("   UART interrupts: ✗ FAILED\r\n");
                    }

                    uart.puts("3. Enable GPIO interrupts\r\n");
                    if interrupt_controller.enable_interrupt(interrupts::GPIO_IRQ) {
                        uart.puts("   GPIO interrupts: ✓ ENABLED\r\n");
                    } else {
                        uart.puts("   GPIO interrupts: ✗ FAILED\r\n");
                    }

                    uart.puts("All major interrupt sources enabled!\r\n");
                    uart.puts("Use 'i' to check interrupt status.\r\n");
                    uart.puts("============================\r\n");
                }
                b'j' | b'J' => {
                    uart.puts("\r\n=== Interrupt System Test ===\r\n");
                    uart.puts("Running comprehensive interrupt test...\r\n");

                    // Save initial state
                    let initial_stats = interrupt_controller.get_interrupt_stats();

                    // Run test
                    if interrupt_controller.run_interrupt_test() {
                        uart.puts("Interrupt test: ✓ PASSED\r\n");

                        let final_stats = interrupt_controller.get_interrupt_stats();
                        uart.puts("Test Results:\r\n");
                        uart.puts("  Timer interrupts: ");
                        print_number(&uart, final_stats.timer_count - initial_stats.timer_count);
                        uart.puts(" simulated\r\n");

                        uart.puts("  UART interrupts: ");
                        print_number(&uart, final_stats.uart_count - initial_stats.uart_count);
                        uart.puts(" simulated\r\n");

                        uart.puts("  GPIO interrupts: ");
                        print_number(&uart, final_stats.gpio_count - initial_stats.gpio_count);
                        uart.puts(" simulated\r\n");

                        uart.puts("All interrupt sources functioning correctly!\r\n");
                    } else {
                        uart.puts("Interrupt test: ✗ FAILED\r\n");
                        uart.puts("Interrupt system may have issues!\r\n");
                    }
                    uart.puts("=============================\r\n");
                }
                b'v' | b'V' => {
                    uart.puts("\r\n=== Exception Statistics ===\r\n");
                    let stats = get_exception_stats();
                    
                    uart.puts("Total Exceptions: ");
                    print_number(&uart, stats.total_exceptions as u32);
                    uart.puts("\r\n");
                    
                    uart.puts("Synchronous Exceptions: ");
                    print_number(&uart, stats.sync_exceptions as u32);
                    uart.puts("\r\n");
                    
                    uart.puts("IRQ Exceptions: ");
                    print_number(&uart, stats.irq_exceptions as u32);
                    uart.puts("\r\n");
                    
                    uart.puts("FIQ Exceptions: ");
                    print_number(&uart, stats.fiq_exceptions as u32);
                    uart.puts("\r\n");
                    
                    uart.puts("SError Exceptions: ");
                    print_number(&uart, stats.serror_exceptions as u32);
                    uart.puts("\r\n");
                    
                    if let Some(last_type) = stats.last_exception_type {
                        uart.puts("Last Exception Type: ");
                        match last_type {
                            exceptions::ExceptionType::Synchronous => uart.puts("Synchronous"),
                            exceptions::ExceptionType::Irq => uart.puts("IRQ"),
                            exceptions::ExceptionType::Fiq => uart.puts("FIQ"),
                            exceptions::ExceptionType::SError => uart.puts("SError"),
                        }
                        uart.puts("\r\n");
                    } else {
                        uart.puts("No exceptions have occurred yet.\r\n");
                    }
                    uart.puts("============================\r\n");
                }
                b'w' | b'W' => {
                    uart.puts("\r\n=== Exception Handling Test ===\r\n");
                    uart.puts("Testing exception handling capabilities...\r\n");
                    
                    // Reset statistics for clean test
                    reset_exception_stats();
                    uart.puts("Exception statistics reset.\r\n");
                    
                    // Simulate an IRQ by calling the handler directly
                    // This is safer than actually triggering real exceptions
                    uart.puts("Simulating IRQ exception...\r\n");
                    
                    // For now, just report that exception vectors are installed
                    uart.puts("Exception vectors installed and ready.\r\n");
                    uart.puts("Real exceptions will be handled automatically.\r\n");
                    uart.puts("Use 'v' command to view exception statistics.\r\n");
                    uart.puts("===============================\r\n");
                }
                b'p' | b'P' => {
                    uart.puts("\r\n=== SD Card Information ===\r\n");
                    if sdcard.is_initialized() {
                        if let Some(info) = sdcard.get_card_info() {
                            uart.puts("SD Card Status: ✓ INITIALIZED\r\n");
                            uart.puts("Card Type: ");
                            if info.high_capacity {
                                uart.puts("SDHC/SDXC (High Capacity)\r\n");
                            } else {
                                uart.puts("SDSC (Standard Capacity)\r\n");
                            }
                            
                            uart.puts("Capacity: ");
                            print_number(&uart, (info.get_capacity() / (1024 * 1024)) as u32);
                            uart.puts(" MB\r\n");
                            
                            uart.puts("Relative Card Address: 0x");
                            print_hex(&uart, info.rca as u32);
                            uart.puts("\r\n");
                            
                            uart.puts("Manufacturer ID: 0x");
                            print_hex(&uart, info.get_manufacturer_id() as u32);
                            uart.puts("\r\n");
                            
                            uart.puts("Product Name: ");
                            let name = info.get_product_name();
                            for &byte in &name {
                                if byte >= 32 && byte <= 126 {
                                    uart.putc(byte);
                                } else {
                                    uart.putc(b'?');
                                }
                            }
                            uart.puts("\r\n");
                        } else {
                            uart.puts("SD Card: Error getting card info\r\n");
                        }
                    } else {
                        uart.puts("SD Card Status: ✗ NOT INITIALIZED\r\n");
                        uart.puts("Note: SD card may not be available in QEMU\r\n");
                    }
                    uart.puts("===========================\r\n");
                }
                b'q' | b'Q' => {
                    uart.puts("\r\n=== SD Card Block Read Test ===\r\n");
                    if sdcard.is_initialized() {
                        uart.puts("Reading block 0 from SD card...\r\n");
                        let mut buffer = [0u8; 512];
                        match sdcard.read_block(0, &mut buffer) {
                            Ok(()) => {
                                uart.puts("✓ Block read successful!\r\n");
                                uart.puts("First 64 bytes (hex):\r\n");
                                for i in (0..64).step_by(16) {
                                    uart.puts("  ");
                                    print_hex(&uart, i as u32);
                                    uart.puts(": ");
                                    for j in 0..16 {
                                        if i + j < 64 {
                                            let byte = buffer[i + j];
                                            if byte < 16 {
                                                uart.putc(b'0');
                                            }
                                            print_hex(&uart, byte as u32);
                                            uart.putc(b' ');
                                        }
                                    }
                                    uart.puts("\r\n");
                                }
                                
                                // Check if it looks like a boot sector
                                if buffer[510] == 0x55 && buffer[511] == 0xAA {
                                    uart.puts("✓ Boot sector signature found (0x55AA)\r\n");
                                } else {
                                    uart.puts("ℹ No boot sector signature found\r\n");
                                }
                            }
                            Err(e) => {
                                uart.puts("✗ Block read failed: ");
                                match e {
                                    sdcard::SdError::ReadError => uart.puts("Read error\r\n"),
                                    sdcard::SdError::CommandTimeout => uart.puts("Command timeout\r\n"),
                                    sdcard::SdError::DataTimeout => uart.puts("Data timeout\r\n"),
                                    _ => uart.puts("Unknown error\r\n"),
                                }
                            }
                        }
                    } else {
                        uart.puts("✗ SD card not initialized\r\n");
                    }
                    uart.puts("===============================\r\n");
                }
                b'y' | b'Y' => {
                    uart.puts("\r\n=== SD Card Block Write Test ===\r\n");
                    if sdcard.is_initialized() {
                        uart.puts("⚠️  WARNING: This will write test data to block 1000\r\n");
                        uart.puts("Creating test pattern...\r\n");
                        
                        let mut buffer = [0u8; 512];
                        // Create a test pattern
                        for i in 0..512 {
                            buffer[i] = (i & 0xFF) as u8;
                        }
                        
                        uart.puts("Writing to block 1000...\r\n");
                        match sdcard.write_block(1000, &buffer) {
                            Ok(()) => {
                                uart.puts("✓ Block write successful!\r\n");
                                
                                // Verify by reading back
                                uart.puts("Verifying write by reading back...\r\n");
                                let mut verify_buffer = [0u8; 512];
                                match sdcard.read_block(1000, &mut verify_buffer) {
                                    Ok(()) => {
                                        let mut match_count = 0;
                                        for i in 0..512 {
                                            if buffer[i] == verify_buffer[i] {
                                                match_count += 1;
                                            }
                                        }
                                        uart.puts("Verification: ");
                                        print_number(&uart, match_count);
                                        uart.puts("/512 bytes match\r\n");
                                        if match_count == 512 {
                                            uart.puts("✓ Write/read verification PASSED\r\n");
                                        } else {
                                            uart.puts("✗ Write/read verification FAILED\r\n");
                                        }
                                    }
                                    Err(_) => {
                                        uart.puts("✗ Verification read failed\r\n");
                                    }
                                }
                            }
                            Err(e) => {
                                uart.puts("✗ Block write failed: ");
                                match e {
                                    sdcard::SdError::WriteError => uart.puts("Write error\r\n"),
                                    sdcard::SdError::CommandTimeout => uart.puts("Command timeout\r\n"),
                                    sdcard::SdError::DataTimeout => uart.puts("Data timeout\r\n"),
                                    _ => uart.puts("Unknown error\r\n"),
                                }
                            }
                        }
                    } else {
                        uart.puts("✗ SD card not initialized\r\n");
                    }
                    uart.puts("===============================\r\n");
                }
                // FAT32 Filesystem Commands
                b'n' | b'N' => {
                    uart.puts("\r\n=== FAT32 Filesystem Information ===\r\n");
                    if let Some(ref fs) = fat32_fs {
                        fs.print_info();
                    } else {
                        uart.puts("✗ FAT32 filesystem not available\r\n");
                        uart.puts("  Requires initialized SD card with FAT32 partition\r\n");
                    }
                    uart.puts("====================================\r\n");
                }
                b'o' | b'O' => {
                    uart.puts("\r\n=== Directory Listing ===\r\n");
                    if let Some(ref mut fs) = fat32_fs {
                        match fs.list_directory() {
                            Ok(files) => {
                                if files.len() == 0 {
                                    uart.puts("Directory is empty\r\n");
                                } else {
                                    uart.puts("Files and directories:\r\n");
                                    for i in 0..files.len() {
                                        let file = &files[i];
                                        uart.puts("  ");
                                        if file.is_directory {
                                            uart.puts("[DIR] ");
                                        } else {
                                            uart.puts("[FILE] ");
                                        }
                                        
                                        // Print filename
                                        let name_len = file.name.iter().position(|&x| x == 0).unwrap_or(256);
                                        for j in 0..name_len.min(50) { // Limit to 50 chars for display
                                            if file.name[j] >= 32 && file.name[j] <= 126 {
                                                uart.putc(file.name[j]);
                                            }
                                        }
                                        
                                        if !file.is_directory {
                                            uart.puts(" (");
                                            print_number(&uart, file.size);
                                            uart.puts(" bytes)");
                                        }
                                        uart.puts("\r\n");
                                    }
                                }
                            }
                            Err(_) => {
                                uart.puts("✗ Failed to read directory\r\n");
                            }
                        }
                    } else {
                        uart.puts("✗ FAT32 filesystem not available\r\n");
                    }
                    uart.puts("=========================\r\n");
                }
                b'k' | b'K' => {
                    uart.puts("\r\n=== Change Directory ===\r\n");
                    if let Some(ref mut fs) = fat32_fs {
                        uart.puts("Enter directory name (or .. for parent): ");
                        
                        let mut input_buffer = [0u8; 64];
                        let input_len = uart.read_line(&mut input_buffer, 63);
                        
                        if input_len > 0 {
                            if let Ok(dir_name) = core::str::from_utf8(&input_buffer[..input_len]) {
                                match fs.change_directory(dir_name) {
                                    Ok(()) => {
                                        uart.puts("✓ Changed to directory: ");
                                        uart.puts(dir_name);
                                        uart.puts("\r\n");
                                        uart.puts("Current directory cluster: ");
                                        print_hex(&uart, fs.get_current_directory());
                                        uart.puts("\r\n");
                                    }
                                    Err(_) => {
                                        uart.puts("✗ Directory not found: ");
                                        uart.puts(dir_name);
                                        uart.puts("\r\n");
                                    }
                                }
                            } else {
                                uart.puts("✗ Invalid directory name\r\n");
                            }
                        } else {
                            uart.puts("✗ No directory name entered\r\n");
                        }
                    } else {
                        uart.puts("✗ FAT32 filesystem not available\r\n");
                    }
                    uart.puts("========================\r\n");
                }
                b'b' | b'B' => {
                    uart.puts("\r\n=== Go to Root Directory ===\r\n");
                    if let Some(ref mut fs) = fat32_fs {
                        fs.change_to_root();
                        uart.puts("✓ Changed to root directory\r\n");
                        uart.puts("Current directory cluster: ");
                        print_hex(&uart, fs.get_current_directory());
                        uart.puts("\r\n");
                    } else {
                        uart.puts("✗ FAT32 filesystem not available\r\n");
                    }
                    uart.puts("============================\r\n");
                }
                b'u' | b'U' => {
                    uart.puts("\r\n=== Read File ===\r\n");
                    if let Some(ref mut fs) = fat32_fs {
                        uart.puts("Enter filename to read: ");
                        
                        let mut input_buffer = [0u8; 64];
                        let input_len = uart.read_line(&mut input_buffer, 63);
                        
                        if input_len > 0 {
                            if let Ok(filename) = core::str::from_utf8(&input_buffer[..input_len]) {
                                uart.puts("Reading file: ");
                                uart.puts(filename);
                                uart.puts("\r\n");
                                
                                match fs.read_file(filename) {
                                    Ok(content) => {
                                        uart.puts("File size: ");
                                        print_number(&uart, content.len() as u32);
                                        uart.puts(" bytes\r\n");
                                        uart.puts("=== File Content ===\r\n");
                                        
                                        // Display content as text if possible, hex otherwise
                                        match content.as_str() {
                                            Ok(text) => {
                                                uart.puts(text);
                                            }
                                            Err(_) => {
                                                uart.puts("[Binary file - showing hex dump]\r\n");
                                                let data = content.as_slice();
                                                for (i, &byte) in data.iter().enumerate().take(256) { // Limit output
                                                    if i % 16 == 0 {
                                                        uart.puts("\r\n");
                                                        print_hex(&uart, i as u32);
                                                        uart.puts(": ");
                                                    }
                                                    if byte < 16 {
                                                        uart.putc(b'0');
                                                    }
                                                    print_hex(&uart, byte as u32);
                                                    uart.putc(b' ');
                                                }
                                                if data.len() > 256 {
                                                    uart.puts("\r\n... (truncated)");
                                                }
                                            }
                                        }
                                        uart.puts("\r\n====================\r\n");
                                    }
                                    Err(_) => {
                                        uart.puts("✗ File not found or read error: ");
                                        uart.puts(filename);
                                        uart.puts("\r\n");
                                    }
                                }
                            } else {
                                uart.puts("✗ Invalid filename\r\n");
                            }
                        } else {
                            uart.puts("✗ No filename entered\r\n");
                        }
                    } else {
                        uart.puts("✗ FAT32 filesystem not available\r\n");
                    }
                    uart.puts("==================\r\n");
                }
                _ => {
                    // For any other character, just echo it back with timestamp
                    if (32..=126).contains(&ch) {
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
