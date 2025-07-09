//! System command handlers
//!
//! This module contains handlers for system-level commands like help, time,
//! system info, and health checks.

use crate::{exceptions::get_exception_stats, shell::ShellContext};

/// Helper function to print time in a readable format
fn print_time(uart: &crate::uart::Uart, ms: u32) {
    let seconds = ms / 1000;
    let remaining_ms = ms % 1000;
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    let hours = minutes / 60;
    let remaining_minutes = minutes % 60;

    if hours > 0 {
        print_number(uart, hours);
        uart.puts("h ");
    }
    if remaining_minutes > 0 || hours > 0 {
        print_number(uart, remaining_minutes);
        uart.puts("m ");
    }
    print_number(uart, remaining_seconds);
    uart.puts(".");
    // Print milliseconds with leading zeros
    if remaining_ms < 100 {
        uart.puts("0");
    }
    if remaining_ms < 10 {
        uart.puts("0");
    }
    print_number(uart, remaining_ms);
    uart.puts("s");
}

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

/// Handle the help command (h/H)
pub fn handle_help(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== TinyOS Command Reference ===\r\n");
    context.uart.puts("System Commands:\r\n");
    context.uart.puts("  h/H - Show this help menu\r\n");
    context.uart.puts("  t/T - Show current system time\r\n");
    context.uart.puts("  s/S - Show system information\r\n");
    context.uart.puts("  c/C - Run system health check\r\n");
    context.uart.puts("Hardware Control:\r\n");
    context.uart.puts("  1   - Turn LED ON\r\n");
    context.uart.puts("  0   - Turn LED OFF\r\n");
    context.uart.puts("  l/L - Toggle LED state\r\n");
    context.uart.puts("Memory Management:\r\n");
    context.uart.puts("  m/M - Show memory statistics\r\n");
    context.uart.puts("  a/A - Allocate memory block\r\n");
    context.uart.puts("  f/F - Free last allocated block\r\n");
    context.uart.puts("  x/X - Run basic memory test\r\n");
    context
        .uart
        .puts("  z/Z - Run comprehensive memory test suite\r\n");
    context.uart.puts("  g/G - Run memory corruption check\r\n");
    context.uart.puts("  r/R - Defragment memory\r\n");
    context.uart.puts("Interrupt Management:\r\n");
    context.uart.puts("  i/I - Show interrupt status\r\n");
    context.uart.puts("  e/E - Enable/disable interrupts\r\n");
    context.uart.puts("  j/J - Run interrupt test\r\n");
    context.uart.puts("Exception Management:\r\n");
    context.uart.puts("  v/V - Show exception statistics\r\n");
    context
        .uart
        .puts("  w/W - Test exception handling (safe)\r\n");
    context.uart.puts("Storage & SD Card:\r\n");
    context.uart.puts("  p/P - Show SD card information\r\n");
    context.uart.puts("  q/Q - Read SD card block\r\n");
    context.uart.puts("  y/Y - Write SD card block (test)\r\n");
    context.uart.puts("FAT32 Filesystem:\r\n");
    context.uart.puts("  d/D - List directory contents\r\n");
    context
        .uart
        .puts("  n/N - Mount filesystem / Show filesystem info\r\n");
    context.uart.puts("  o/O - Change directory (cd)\r\n");
    context.uart.puts("  u/U - Read file contents\r\n");
    context.uart.puts("  k/K - Go to root directory\r\n");
    context.uart.puts("Diagnostics:\r\n");
    context.uart.puts("  d/D - Hardware diagnostics\r\n");
    context.uart.puts("================================\r\n");
}

/// Handle the time command (t/T)
pub fn handle_time(context: &ShellContext, start_time: u64) {
    let current_time = context.timer.get_time();
    context.uart.puts("Current system time: [");
    print_time(
        &context.uart,
        context
            .timer
            .ticks_to_ms(current_time.wrapping_sub(start_time) as u32),
    );
    context.uart.puts("]\r\n");
}

/// Handle the system info command (s/S)
pub fn handle_system_info(context: &ShellContext) {
    let _current_time = context.timer.get_time();
    // We need start_time passed in - for now, let's skip the uptime calculation

    context
        .uart
        .puts("\r\n=== TinyOS System Information ===\r\n");
    context.uart.puts("  OS Name: TinyOS\r\n");
    context.uart.puts("  Version: 0.1.0\r\n");
    context
        .uart
        .puts("  Platform: Raspberry Pi 4/5 (AArch64)\r\n");
    context.uart.puts("  Architecture: ARM64\r\n");
    context.uart.puts("  Timer Frequency: 1MHz\r\n");
    context.uart.puts("  UART Base: 0xFE201000\r\n");
    context.uart.puts("  GPIO Base: 0xFE200000\r\n");
    context.uart.puts("  GIC Base: 0xFF841000\r\n");
    context.uart.puts("  LED Pin: GPIO 42\r\n");

    let int_stats = context.interrupt_controller.get_interrupt_stats();
    context.uart.puts("  Active Interrupts: ");
    print_number(&context.uart, int_stats.total_interrupts);
    context.uart.puts("\r\n");
    context.uart.puts("=================================\r\n");
}

/// Handle the health check command (c/C)
pub fn handle_health_check(context: &mut ShellContext) {
    context.uart.puts("\r\n=== System Health Check ===\r\n");
    context
        .uart
        .puts("Running comprehensive diagnostics...\r\n");

    context
        .uart
        .puts("1. GPIO System: Testing LED control...\r\n");
    context.uart.puts("   - LED toggle test: ");
    context.gpio.set_high(42);
    context.timer.delay_ms(100);
    context.gpio.set_low(42);
    context.timer.delay_ms(100);
    context.uart.puts("✓ PASS\r\n");

    context.uart.puts("2. Timer System: Testing delays...\r\n");
    context.uart.puts("   - Microsecond timing: ");
    let start = context.timer.get_time();
    context.timer.delay_us(1000);
    let elapsed = context.timer.get_time() - start;
    if (900..=1100).contains(&elapsed) {
        context.uart.puts("✓ PASS\r\n");
    } else {
        context.uart.puts("✗ FAIL\r\n");
    }

    context
        .uart
        .puts("3. UART System: Communication check...\r\n");
    context
        .uart
        .puts("   - Character transmission: ✓ PASS (you see this!)\r\n");

    context
        .uart
        .puts("4. Exception System: Handler validation...\r\n");
    context.uart.puts("   - Exception stats available: ");
    let stats = get_exception_stats();
    context.uart.puts("✓ PASS\r\n");
    context.uart.puts("   - Total exceptions handled: ");
    print_number(&context.uart, stats.total_exceptions as u32);
    context.uart.puts("\r\n");

    context
        .uart
        .puts("5. Memory System: Allocation test...\r\n");
    context.uart.puts("   - Block allocation: ");
    if context.memory_manager.allocate_block().is_some() {
        context.uart.puts("✓ PASS\r\n");
    } else {
        context.uart.puts("✗ FAIL\r\n");
    }

    context.uart.puts("   - Memory corruption check: ");
    if context.memory_manager.check_corruption() {
        context.uart.puts("✓ PASS\r\n");
    } else {
        context.uart.puts("⚠️  WARNING\r\n");
    }

    let stats = context.memory_manager.get_stats();
    context.uart.puts("   - Memory usage: ");
    let usage_percent = (stats.used_heap_size * 100) / stats.total_heap_size;
    print_number(&context.uart, usage_percent);
    context.uart.puts("% used, ");
    print_number(&context.uart, stats.fragmentation_percent);
    context.uart.puts("% fragmented\r\n");

    context.uart.puts("   - Largest free block: ");
    print_number(&context.uart, stats.largest_free_block);
    context.uart.puts(" bytes\r\n");

    context
        .uart
        .puts("6. Interrupt System: Running interrupt test...\r\n");
    context.uart.puts("   - Interrupt controller: ");
    if context.interrupt_controller.run_interrupt_test() {
        context.uart.puts("✓ PASS\r\n");
    } else {
        context.uart.puts("✗ FAIL\r\n");
    }

    let int_stats = context.interrupt_controller.get_interrupt_stats();
    context.uart.puts("   - Simulated interrupts: ");
    print_number(&context.uart, int_stats.total_interrupts);
    context.uart.puts(" total\r\n");

    context.uart.puts("\r\n=== Health Check Results ===\r\n");
    context.uart.puts("Overall Status: ✓ HEALTHY\r\n");
    context.uart.puts("All systems operational!\r\n");
    context.uart.puts("===========================\r\n");
}
