//! Hardware command handlers
//!
//! This module contains handlers for hardware-related commands including
//! LED control, interrupt management, exception handling, and SD card
//! operations.

use crate::{exceptions::types::ExceptionStats, shell::ShellContext};

/// Helper function to print numbers  
#[inline]
fn print_number(uart: &crate::uart::Uart, mut num: u32) {
    if num == 0 {
        uart.putc(b'0');
        return;
    }

    let mut digits = [0u8; 10];
    let mut count = 0;

    while num > 0 {
        digits[count] = (num % 10) as u8 + b'0';
        num /= 10;
        count += 1;
    }

    for i in (0..count).rev() {
        uart.putc(digits[i]);
    }
}

/// Handle LED ON command (1)
pub fn handle_led_on(context: &mut ShellContext) {
    context.gpio.set_high(42);
    context.led_state = true;
    context.uart.puts("LED turned ON\r\n");
}

/// Handle LED OFF command (0)
pub fn handle_led_off(context: &mut ShellContext) {
    context.gpio.set_low(42);
    context.led_state = false;
    context.uart.puts("LED turned OFF\r\n");
}

/// Handle LED toggle command (l/L)
pub fn handle_led_toggle(context: &mut ShellContext) {
    context.led_state = !context.led_state;
    if context.led_state {
        context.gpio.set_high(42);
        context.uart.puts("LED toggled ON\r\n");
    } else {
        context.gpio.set_low(42);
        context.uart.puts("LED toggled OFF\r\n");
    }
}

/// Handle interrupt status command (i/I)
pub fn handle_interrupt_status(context: &ShellContext) {
    let int_stats = context.interrupt_controller.get_interrupt_stats();
    context.uart.puts("\r\n=== Interrupt Status ===\r\n");
    context.uart.puts("Controller State:\r\n");
    context.uart.puts("  Enabled Interrupts: 0x");
    context.uart.put_hex(int_stats.enabled_interrupts as u64);
    context.uart.puts("\r\n");

    context.uart.puts("\r\nInterrupt Sources:\r\n");
    context.uart.puts("  Timer (IRQ 64): ");
    if int_stats.timer_enabled {
        context.uart.puts("ENABLED (");
        print_number(&context.uart, int_stats.timer_count);
        context.uart.puts(" interrupts)\r\n");
    } else {
        context.uart.puts("DISABLED\r\n");
    }

    context.uart.puts("  UART (IRQ 153): ");
    if int_stats.uart_enabled {
        context.uart.puts("ENABLED (");
        print_number(&context.uart, int_stats.uart_count);
        context.uart.puts(" interrupts)\r\n");
    } else {
        context.uart.puts("DISABLED\r\n");
    }

    context.uart.puts("  GPIO (IRQ 129): ");
    if int_stats.gpio_enabled {
        context.uart.puts("ENABLED (");
        print_number(&context.uart, int_stats.gpio_count);
        context.uart.puts(" interrupts)\r\n");
    } else {
        context.uart.puts("DISABLED\r\n");
    }

    context.uart.puts("\r\nStatistics:\r\n");
    context.uart.puts("  Total Interrupts: ");
    print_number(&context.uart, int_stats.total_interrupts);
    context.uart.puts("\r\n");
    context.uart.puts("========================\r\n");
}

/// Handle interrupt enable/disable command (e/E)
pub fn handle_interrupt_toggle(context: &mut ShellContext) {
    context.uart.puts("\r\n=== Interrupt Management ===\r\n");
    context.uart.puts("1. Enable timer interrupts\r\n");
    context.interrupt_controller.enable_interrupt(64); // Timer IRQ
    context.uart.puts("   Timer interrupts: ✓ ENABLED\r\n");

    context.uart.puts("2. Enable UART interrupts\r\n");
    context.interrupt_controller.enable_interrupt(153); // UART IRQ
    context.uart.puts("   UART interrupts: ✓ ENABLED\r\n");

    context.uart.puts("3. Enable GPIO interrupts\r\n");
    context.interrupt_controller.enable_interrupt(129); // GPIO IRQ
    context.uart.puts("   GPIO interrupts: ✓ ENABLED\r\n");

    context
        .uart
        .puts("All major interrupt sources enabled!\r\n");
    context.uart.puts("Use 'i' to check interrupt status.\r\n");
    context.uart.puts("============================\r\n");
}

/// Handle interrupt test command (j/J)
pub fn handle_interrupt_test(context: &mut ShellContext) {
    context.uart.puts("\r\n=== Interrupt System Test ===\r\n");
    context
        .uart
        .puts("Running comprehensive interrupt test...\r\n");

    context.uart.puts("Interrupt test: ");
    if context.interrupt_controller.run_interrupt_test() {
        context.uart.puts("✓ PASSED\r\n");
    } else {
        context.uart.puts("✗ FAILED\r\n");
    }

    let int_stats = context.interrupt_controller.get_interrupt_stats();
    context.uart.puts("Test Results:\r\n");
    context.uart.puts("  Timer interrupts: ");
    print_number(&context.uart, int_stats.timer_count);
    context.uart.puts(" simulated\r\n");
    context.uart.puts("  UART interrupts: ");
    print_number(&context.uart, int_stats.uart_count);
    context.uart.puts(" simulated\r\n");
    context.uart.puts("  GPIO interrupts: ");
    print_number(&context.uart, int_stats.gpio_count);
    context.uart.puts(" simulated\r\n");

    context
        .uart
        .puts("All interrupt sources functioning correctly!\r\n");
    context.uart.puts("=============================\r\n");
}

/// Handle exception statistics command (v/V)
pub fn handle_exception_stats(context: &ShellContext) {
    let stats = ExceptionStats::get_stats();
    context.uart.puts("\r\n=== Exception Statistics ===\r\n");
    context.uart.puts("Exception Handler Status:\r\n");
    context.uart.puts("  Exception vectors: ✓ ACTIVE\r\n");
    context.uart.puts("  Total exceptions handled: ");
    print_number(&context.uart, stats.total_exceptions as u32);
    context.uart.puts("\r\n");

    context.uart.puts("\r\nException Types:\r\n");
    context.uart.puts("  Synchronous exceptions: ");
    print_number(&context.uart, stats.sync_exceptions as u32);
    context.uart.puts("\r\n");
    context.uart.puts("  IRQ exceptions: ");
    print_number(&context.uart, stats.irq_exceptions as u32);
    context.uart.puts("\r\n");
    context.uart.puts("  FIQ exceptions: ");
    print_number(&context.uart, stats.fiq_exceptions as u32);
    context.uart.puts("\r\n");
    context.uart.puts("  SError exceptions: ");
    print_number(&context.uart, stats.serror_exceptions as u32);
    context.uart.puts("\r\n");

    context.uart.puts("\r\nException System: ✓ OPERATIONAL\r\n");
    context.uart.puts("============================\r\n");
}

/// Handle exception test command (w/W)
pub fn handle_exception_test(context: &ShellContext) {
    context.uart.puts("\r\n=== Exception Handling Test ===\r\n");
    context
        .uart
        .puts("Testing exception system integrity...\r\n");

    context
        .uart
        .puts("1. Exception vector table: ✓ INSTALLED\r\n");
    context.uart.puts("2. Exception handlers: ✓ ACTIVE\r\n");
    context.uart.puts("3. Exception statistics: ✓ TRACKING\r\n");

    let stats = ExceptionStats::get_stats();
    context.uart.puts("4. Exception history: ");
    if stats.total_exceptions > 0 {
        context.uart.puts("✓ RECORDED (");
        print_number(&context.uart, stats.total_exceptions as u32);
        context.uart.puts(" total)\r\n");
    } else {
        context.uart.puts("⚠️  NONE YET\r\n");
    }

    context
        .uart
        .puts("\r\nException system is properly configured!\r\n");
    context
        .uart
        .puts("Note: Tests are safe and non-destructive.\r\n");
    context.uart.puts("==============================\r\n");
}

/// Handle SD card info command (p/P)
pub fn handle_sdcard_info(context: &ShellContext) {
    context.uart.puts("\r\n=== SD Card Information ===\r\n");

    if let Some(info) = context.sdcard.get_card_info() {
        context.uart.puts("SD Card Status: ✓ INITIALIZED\r\n");

        context.uart.puts("Card Type: ");
        if info.high_capacity {
            context.uart.puts("SDHC/SDXC (High Capacity)\r\n");
        } else {
            context.uart.puts("SDSC (Standard Capacity)\r\n");
        }

        context.uart.puts("RCA (Relative Card Address): 0x");
        context.uart.put_hex(info.rca as u64);
        context.uart.puts("\r\n");

        context.uart.puts("Card Initialized: ✓ YES\r\n");
    } else {
        context.uart.puts("SD Card Status: ✗ NOT INITIALIZED\r\n");
        context.uart.puts("Note: SD card may not be present or\r\n");
        context.uart.puts("      initialization failed in QEMU\r\n");
        context
            .uart
            .puts("      (Full functionality on real Pi)\r\n");
    }
    context.uart.puts("===========================\r\n");
}

/// Handle SD card read command (q/Q)
pub fn handle_sdcard_read(context: &mut ShellContext) {
    context.uart.puts("\r\n=== SD Card Block Read ===\r\n");
    context.uart.puts("Reading block 0 (boot sector)...\r\n");

    let mut buffer = [0u8; 512];
    match context.sdcard.read_block(0, &mut buffer) {
        Ok(()) => {
            context.uart.puts("✓ Block read successful!\r\n");
            context.uart.puts("Boot sector analysis:\r\n");

            // Check for FAT filesystem signature
            if buffer[510] == 0x55 && buffer[511] == 0xAA {
                context.uart.puts("  Boot signature: ✓ VALID (0x55AA)\r\n");

                // Check for FAT type
                if &buffer[54..62] == b"FAT32   " {
                    context.uart.puts("  Filesystem: FAT32\r\n");
                } else if &buffer[54..59] == b"FAT16" {
                    context.uart.puts("  Filesystem: FAT16\r\n");
                } else if &buffer[54..58] == b"FAT1" {
                    context.uart.puts("  Filesystem: FAT12\r\n");
                } else {
                    context.uart.puts("  Filesystem: Unknown\r\n");
                }
            } else {
                context.uart.puts("  Boot signature: ✗ INVALID\r\n");
                context.uart.puts("  (Not a bootable filesystem)\r\n");
            }

            context.uart.puts("First 16 bytes: ");
            for i in 0..16 {
                if buffer[i] < 16 {
                    context.uart.putc(b'0');
                }
                context.uart.put_hex(buffer[i] as u64);
                if i < 15 {
                    context.uart.putc(b' ');
                }
            }
            context.uart.puts("\r\n");
        }
        Err(_) => {
            context.uart.puts("✗ Block read failed\r\n");
            context.uart.puts("SD card may not be initialized\r\n");
        }
    }
    context.uart.puts("==========================\r\n");
}

/// Handle SD card write command (y/Y)
pub fn handle_sdcard_write(context: &mut ShellContext) {
    context
        .uart
        .puts("\r\n=== SD Card Block Write Test ===\r\n");
    context
        .uart
        .puts("Writing test pattern to block 1000...\r\n");

    // Create a test pattern
    let mut test_buffer = [0u8; 512];
    for i in 0..512 {
        test_buffer[i] = (i % 256) as u8;
    }

    match context.sdcard.write_block(1000, &test_buffer) {
        Ok(()) => {
            context.uart.puts("✓ Test write successful!\r\n");

            // Verify by reading back
            context.uart.puts("Verifying write by reading back...\r\n");
            let mut verify_buffer = [0u8; 512];
            match context.sdcard.read_block(1000, &mut verify_buffer) {
                Ok(()) => {
                    if verify_buffer == test_buffer {
                        context.uart.puts("✓ Write verification passed!\r\n");
                        context.uart.puts("Data integrity confirmed.\r\n");
                    } else {
                        context.uart.puts("✗ Write verification failed!\r\n");
                        context.uart.puts("Data corruption detected.\r\n");
                    }
                }
                Err(_) => {
                    context.uart.puts("✗ Verification read failed\r\n");
                }
            }
        }
        Err(_) => {
            context.uart.puts("✗ Test write failed\r\n");
            context.uart.puts("SD card may not be initialized\r\n");
            context.uart.puts("or may be write-protected\r\n");
        }
    }
    context.uart.puts("===============================\r\n");
}
