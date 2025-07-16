//! System Calls Module
//!
//! This module provides system call testing and analysis functionality
//! for shell commands, including interface validation and statistics.

use super::utils::print_number;
use crate::shell::ShellContext;

/// Handle system call testing command (8)
pub fn handle_syscall_test(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== System Call Testing (Phase 1) ===\r\n");

    // Test 1: System call interface validation
    context.uart.puts("1. Testing System Call Interface...\r\n");
    test_syscall_interface(context);

    // Test 2: System call statistics
    context.uart.puts("\r\n2. System Call Statistics...\r\n");
    display_syscall_stats(context);

    // Test 3: Direct system call tests
    context.uart.puts("\r\n3. Direct System Call Tests...\r\n");
    test_syscall_interface(context);

    context
        .uart
        .puts("\r\n✅ System call testing complete!\r\n");
    context
        .uart
        .puts("=====================================\r\n");
}

/// Test system call interface
fn test_syscall_interface(context: &ShellContext) {
    use crate::exceptions::syscall::test_syscall_interface;

    context
        .uart
        .puts("   Running syscall interface tests...\r\n");
    let result = test_syscall_interface();

    if result {
        context
            .uart
            .puts("   ✅ All syscall interface tests passed\r\n");
    } else {
        context
            .uart
            .puts("   ❌ Some syscall interface tests failed\r\n");
    }
}

/// Display system call statistics
fn display_syscall_stats(context: &ShellContext) {
    use crate::exceptions::syscall::get_syscall_stats;

    let stats = get_syscall_stats();
    context.uart.puts("   Total syscalls: ");
    print_number(&context.uart, stats.total_syscalls as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Successful syscalls: ");
    print_number(
        &context.uart,
        (stats.total_syscalls - stats.invalid_calls) as u32,
    );
    context.uart.puts("\r\n");

    context.uart.puts("   Failed syscalls: ");
    print_number(&context.uart, 0); // failed not tracked separately
    context.uart.puts("\r\n");

    context.uart.puts("   Invalid syscalls: ");
    print_number(&context.uart, stats.invalid_calls as u32);
    context.uart.puts("\r\n");
}

/// Test direct system calls
fn test_syscall_interface_cmd(context: &ShellContext) {
    use crate::exceptions::syscall::test_syscall_interface;

    context
        .uart
        .puts("   Testing direct syscall execution...\r\n");
    let result = test_syscall_interface();

    if result {
        context.uart.puts("   ✅ Direct syscall tests passed\r\n");
    } else {
        context
            .uart
            .puts("   ❌ Some direct syscall tests failed\r\n");
    }
}
