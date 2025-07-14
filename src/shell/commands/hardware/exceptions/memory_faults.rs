//! Memory Faults Module
//!
//! This module provides memory fault testing and analysis functionality
//! for shell commands, including fault classification and statistics.

use crate::shell::ShellContext;
use super::utils::print_number;

/// Handle memory fault testing command (9)
pub fn handle_memory_fault_test(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== Memory Fault Testing (Phase 1) ===\r\n");

    // Test 1: Memory fault analyzer
    context.uart.puts("1. Testing Memory Fault Analyzer...\r\n");
    test_memory_fault_analysis(context);

    // Test 2: Memory fault statistics
    context.uart.puts("\r\n2. Memory Fault Statistics...\r\n");
    display_memory_fault_stats(context);

    // Test 3: Fault classification tests
    context
        .uart
        .puts("\r\n3. Fault Classification Tests...\r\n");
    test_memory_fault_analysis(context);

    context
        .uart
        .puts("\r\n✅ Memory fault testing complete!\r\n");
    context
        .uart
        .puts("======================================\r\n");
}

/// Test memory fault analyzer
fn test_memory_fault_analysis(context: &ShellContext) {
    use crate::exceptions::memory_faults::test_memory_fault_analysis;

    context
        .uart
        .puts("   Running memory fault analyzer tests...\r\n");
    let result = test_memory_fault_analysis();

    if result {
        context
            .uart
            .puts("   ✅ Memory fault analyzer tests passed\r\n");
    } else {
        context
            .uart
            .puts("   ❌ Some memory fault analyzer tests failed\r\n");
    }
}

/// Display memory fault statistics
fn display_memory_fault_stats(context: &ShellContext) {
    use crate::exceptions::memory_faults::get_memory_fault_stats;

    let stats = get_memory_fault_stats();
    context.uart.puts("   Total memory faults: ");
    print_number(&context.uart, stats.total_faults as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Data aborts: ");
    print_number(&context.uart, stats.data_aborts as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Instruction aborts: ");
    print_number(&context.uart, stats.instruction_aborts as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Permission faults: ");
    print_number(&context.uart, stats.permission_faults as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Translation faults: ");
    print_number(&context.uart, stats.translation_faults as u32);
    context.uart.puts("\r\n");
}

/// Test fault classification
fn test_memory_fault_analysis_cmd(context: &ShellContext) {
    use crate::exceptions::memory_faults::test_memory_fault_analysis;

    context.uart.puts("   Testing fault classification...\r\n");
    let result = test_memory_fault_analysis();

    if result {
        context
            .uart
            .puts("   ✅ Fault classification tests passed\r\n");
    } else {
        context
            .uart
            .puts("   ❌ Some fault classification tests failed\r\n");
    }
}
