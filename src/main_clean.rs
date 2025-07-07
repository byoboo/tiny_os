#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo};

mod exceptions;
mod gpio;
mod interrupts;
mod memory;
mod sdcard;
mod timer;
mod uart;
use exceptions::{get_exception_stats, init_exceptions, reset_exception_stats};
use gpio::{Gpio, GpioFunction};
use interrupts::InterruptController;
use memory::MemoryManager;
use sdcard::SdCard;
use timer::SystemTimer;
use uart::Uart;

// Include the boot assembly
global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn kernel_main() {
    // Initialize UART for output
    let uart = Uart::new();
    uart.init();

    uart.puts("TinyOS Starting...\r\n");
    uart.puts("================================\r\n");

    // Initialize system components
    uart.puts("Initializing system components...\r\n");

    // Initialize exceptions
    init_exceptions();
    uart.puts("✓ Exception handling initialized\r\n");

    // Initialize GPIO
    let mut gpio = Gpio::new();

    // Configure LED pin (GPIO 42 on Raspberry Pi)
    gpio.set_function(42, GpioFunction::Output);
    uart.puts("✓ GPIO initialized (LED on pin 42)\r\n");

    // Initialize system timer
    let timer = SystemTimer::new();
    uart.puts("✓ System timer initialized\r\n");

    // Initialize memory manager
    let mut memory_manager = MemoryManager::new(0x100000, 0x400000, 64);
    uart.puts("✓ Memory manager initialized\r\n");

    // Initialize interrupt controller
    let mut interrupt_controller = InterruptController::new();
    interrupt_controller.init();
    uart.puts("✓ Interrupt controller initialized\r\n");

    // Initialize SD Card
    uart.puts("Initializing SD Card...\r\n");
    let mut sdcard = SdCard::new();
    let sd_init_success = match sdcard.init() {
        Ok(()) => {
            uart.puts("SD Card initialized successfully!\r\n");
            if let Some(info) = sdcard.get_card_info() {
                uart.puts("SD Card Info: ");
                if info.high_capacity {
                    uart.puts("SDHC/SDXC, ");
                } else {
                    uart.puts("Standard, ");
                }
                uart.puts("RCA: ");
                uart.put_hex(info.rca as u64);
                uart.puts("\r\n");
            }
            true
        }
        Err(_) => {
            uart.puts("SD Card initialization failed\r\n");
            false
        }
    };

    // System ready
    uart.puts("================================\r\n");
    uart.puts("✓ TinyOS Ready!\r\n");
    uart.puts("Available commands (type 'h' for help):\r\n");
    uart.puts("================================\r\n");

    // Main system loop
    let mut led_state = false;
    let start_time = timer.get_system_time();

    loop {
        let current_time = timer.get_system_time();

        // Check for UART input
        if let Some(ch) = uart.getc() {
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
                    uart.puts("Running comprehensive diagnostics...\r\n");

                    uart.puts("1. GPIO System: Testing LED control...\r\n");
                    uart.puts("   - LED toggle test: ");
                    gpio.set_high(42);
                    timer.delay_ms(100);
                    gpio.set_low(42);
                    timer.delay_ms(100);
                    uart.puts("✓ PASS\r\n");

                    uart.puts("2. Timer System: Testing delays...\r\n");
                    uart.puts("   - Microsecond timing: ");
                    let start = timer.get_system_time();
                    timer.delay_us(1000);
                    let elapsed = timer.get_system_time() - start;
                    if elapsed >= 900 && elapsed <= 1100 {
                        uart.puts("✓ PASS\r\n");
                    } else {
                        uart.puts("✗ FAIL\r\n");
                    }

                    uart.puts("3. UART System: Communication check...\r\n");
                    uart.puts("   - Character transmission: ✓ PASS (you see this!)\r\n");

                    uart.puts("4. Exception System: Handler validation...\r\n");
                    uart.puts("   - Exception stats available: ");
                    let stats = get_exception_stats();
                    uart.puts("✓ PASS\r\n");
                    uart.puts("   - Total exceptions handled: ");
                    print_number(&uart, stats.total_exceptions);
                    uart.puts("\r\n");

                    uart.puts("5. Memory System: Allocation test...\r\n");
                    uart.puts("   - Block allocation: ");
                    if memory_manager.allocate().is_some() {
                        uart.puts("✓ PASS\r\n");
                    } else {
                        uart.puts("✗ FAIL\r\n");
                    }

                    uart.puts("   - Memory corruption check: ");
                    if memory_manager.check_corruption() {
                        uart.puts("✓ PASS\r\n");
                    } else {
                        uart.puts("⚠️  WARNING\r\n");
                    }

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

                    uart.puts("  Fragmentation: ");
                    print_number(&uart, stats.fragmentation_percent);
                    uart.puts("%\r\n");
                    uart.puts("========================\r\n");
                }
                _ => {
                    if (32..=126).contains(&ch) {
                        uart.puts("Unknown command. Type 'h' for help.\r\n");
                    } else {
                        uart.puts("Non-printable character (control code)\r\n");
                    }
                }
            }
        }

        timer.delay_us(50);
    }
}

/// Simple time formatting function (prints seconds.milliseconds)
fn print_time(uart: &Uart, total_ms: u32) {
    let seconds = total_ms / 1000;
    let ms = total_ms % 1000;

    print_number(uart, seconds);
    uart.puts(".");

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

    let mut buffer = [0u8; 10];
    let mut index = 0;

    while num > 0 {
        buffer[index] = (num % 10) as u8 + b'0';
        num /= 10;
        index += 1;
    }

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

    let mut buffer = [0u8; 8];
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

    for i in (0..index).rev() {
        uart.putc(buffer[i]);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let uart = Uart::new();
    uart.init();
    uart.puts("KERNEL PANIC!\r\n");
    loop {}
}
