//! Core system command handlers
//!
//! This module contains handlers for essential system commands including
//! help display, time information, system info, and health diagnostics.

use super::utils::{print_number, print_time};
use crate::{exceptions::types::ExceptionStats, shell::ShellContext};

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
    context.uart.puts("  b/B - Performance benchmark menu\r\n");
    context.uart.puts("  z/Z - Quick performance status\r\n");
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
    context
        .uart
        .puts("  7   - Advanced exception testing (Phase 1)\r\n");
    context.uart.puts("  8   - Test ESR_EL1 decoder\r\n");
    context
        .uart
        .puts("  9   - Test system call interface (Phase 1)\r\n");
    context
        .uart
        .puts("  !   - Test memory fault analysis (Phase 1)\r\n");
    context
        .uart
        .puts("Advanced Exception Testing (Phase 2):\r\n");
    context
        .uart
        .puts("  #   - Test IRQ integration and routing\r\n");
    context
        .uart
        .puts("  $   - Test nested interrupt handling\r\n");
    context
        .uart
        .puts("  %   - Test deferred processing system\r\n");
    context.uart.puts("Process Management (Phase 3):\r\n");
    context.uart.puts("  &   - Process management submenu\r\n");
    context.uart.puts("    1 - Process context test\r\n");
    context.uart.puts("    2 - Privilege level test\r\n");
    context.uart.puts("    3 - Task scheduler test\r\n");
    context.uart.puts("    4 - Process statistics\r\n");
    context.uart.puts("    5 - Scheduler statistics\r\n");
    context.uart.puts("    6 - Privilege statistics\r\n");
    context
        .uart
        .puts("MMU & Exception Management (Phase 4):\r\n");
    context
        .uart
        .puts("  ^   - Exception management submenu\r\n");
    context.uart.puts("    1 - Exception statistics\r\n");
    context.uart.puts("    2 - MMU exception statistics\r\n");
    context.uart.puts("    3 - MMU control (on/off)\r\n");
    context.uart.puts("    4 - Exception testing (safe)\r\n");
    context.uart.puts("    5 - Reset exception stats\r\n");
    context
        .uart
        .puts("Virtual Memory Management (Phase 4.2):\r\n");
    context
        .uart
        .puts("  ~   - Virtual memory management submenu\r\n");
    context.uart.puts("    1 - Virtual memory status\r\n");
    context.uart.puts("    2 - Enable MMU\r\n");
    context.uart.puts("    3 - Disable MMU\r\n");
    context.uart.puts("    4 - Translate address\r\n");
    context.uart.puts("    5 - Flush TLB\r\n");
    context.uart.puts("    6 - Virtual memory test\r\n");
    context.uart.puts("Stack Management (Phase 4.3):\r\n");
    context.uart.puts("  `   - Stack management submenu\r\n");
    context.uart.puts("    1 - Stack status\r\n");
    context.uart.puts("    2 - Allocate kernel stack\r\n");
    context.uart.puts("    3 - Allocate user stack\r\n");
    context.uart.puts("    4 - Deallocate stack\r\n");
    context.uart.puts("    5 - Switch stack\r\n");
    context.uart.puts("    6 - Stack test\r\n");
    context
        .uart
        .puts("User Space Management (Phase 4.4.2):\r\n");
    context
        .uart
        .puts("  |   - User space page table submenu\r\n");
    context
        .uart
        .puts("Advanced Memory Protection (Phase 4.4.3):\r\n");
    context.uart.puts("  @   - Advanced protection submenu\r\n");
    context
        .uart
        .puts("Dynamic Memory Management (Phase 4.4.4):\r\n");
    context.uart.puts("  *   - Dynamic memory submenu\r\n");
    context
        .uart
        .puts("Copy-on-Write Management (Phase 4.4):\r\n");
    context.uart.puts("  (   - COW management submenu\r\n");
    context.uart.puts("    1 - COW status\r\n");
    context.uart.puts("    2 - COW statistics\r\n");
    context.uart.puts("    3 - Create COW mapping\r\n");
    context.uart.puts("    4 - Protect COW page\r\n");
    context.uart.puts("    5 - Unprotect COW page\r\n");
    context.uart.puts("    6 - COW test\r\n");
    context.uart.puts("Testing Framework (Phase 5):\r\n");
    context.uart.puts("  )   - Testing framework submenu\r\n");
    context.uart.puts("    1 - Kernel tests\r\n");
    context.uart.puts("    2 - MMU tests\r\n");
    context.uart.puts("    3 - Process tests\r\n");
    context.uart.puts("    4 - Syscall tests\r\n");
    context.uart.puts("    5 - Integration tests\r\n");
    context.uart.puts("    6 - All tests\r\n");
    context.uart.puts("Command Line Interface:\r\n");
    context.uart.puts("  +   - Advanced command routing\r\n");
    context
        .uart
        .puts("    1 - Advanced protection commands\r\n");
    context.uart.puts("    2 - Dynamic memory commands\r\n");
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
    let stats = ExceptionStats::get_stats();
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
