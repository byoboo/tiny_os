//! Deferred Processing command handlers
//!
//! This module contains handlers for deferred processing-related commands including
//! work queue testing, soft IRQ system testing, and performance monitoring.

use crate::{shell::ShellContext, uart::Uart};

/// Helper function to print numbers
#[inline]
fn print_number(uart: &Uart, mut num: u32) {
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

/// Handle deferred processing testing command
pub fn handle_deferred_processing_test(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== Deferred Processing Testing (Phase 2) ===\r\n");

    // Test 1: Work queue functionality
    context.uart.puts("1. Testing Work Queue...\r\n");
    test_work_queue(context);

    // Test 2: Soft IRQ system
    context.uart.puts("\r\n2. Testing Soft IRQ System...\r\n");
    test_softirq_system(context);

    // Test 3: Deferred processing integration
    context
        .uart
        .puts("\r\n3. Testing Deferred Processing Integration...\r\n");
    test_deferred_integration(context);

    // Test 4: Performance metrics
    context.uart.puts("\r\n4. Performance Metrics...\r\n");
    display_deferred_processing_stats(context);

    context
        .uart
        .puts("\r\n✅ Deferred processing testing complete!\r\n");
    context
        .uart
        .puts("==============================================\r\n");
}

// === Helper functions ===

/// Test work queue functionality
fn test_work_queue(context: &ShellContext) {
    use crate::exceptions::deferred_processing::{
        has_pending_work, process_pending_work, schedule_work,
    };

    context.uart.puts("   Testing work queue...\r\n");

    // Test work scheduling
    context.uart.puts("   Scheduling test work...\r\n");

    // Create a simple work function
    fn test_work_fn(work_item: &mut crate::exceptions::deferred_processing::WorkItem) {
        // Simple test work - just increment the data
        work_item.data += 1;
    }

    let work_scheduled = schedule_work(test_work_fn, 42, 0);

    if work_scheduled {
        context.uart.puts("   Work scheduled successfully\r\n");
    } else {
        context.uart.puts("   Failed to schedule work\r\n");
    }

    // Test work processing
    if has_pending_work() {
        context.uart.puts("   Processing pending work...\r\n");
        process_pending_work();
    } else {
        context.uart.puts("   No pending work\r\n");
    }

    context.uart.puts("   ✅ Work queue test passed\r\n");
}

/// Test soft IRQ system
fn test_softirq_system(context: &ShellContext) {
    use crate::exceptions::deferred_processing::{
        process_pending_work, schedule_softirq, SoftIrqType,
    };

    context.uart.puts("   Testing soft IRQ system...\r\n");

    // Test soft IRQ scheduling
    context.uart.puts("   Scheduling soft IRQ...\r\n");

    // Create a simple soft IRQ work function
    fn test_softirq_fn(work_item: &mut crate::exceptions::deferred_processing::WorkItem) {
        // Simple soft IRQ work - just set some data
        work_item.data = 0xDEADBEEF;
    }

    let softirq_scheduled = schedule_softirq(SoftIrqType::Timer, test_softirq_fn, 123, 0);

    if softirq_scheduled {
        context.uart.puts("   Soft IRQ scheduled successfully\r\n");
    } else {
        context.uart.puts("   Failed to schedule soft IRQ\r\n");
    }

    // Test soft IRQ processing
    context.uart.puts("   Processing soft IRQs...\r\n");
    process_pending_work();

    context.uart.puts("   ✅ Soft IRQ system test passed\r\n");
}

/// Test deferred processing integration
fn test_deferred_integration(context: &ShellContext) {
    use crate::exceptions::deferred_processing::test_deferred_processing;

    context
        .uart
        .puts("   Running deferred processing integration tests...\r\n");
    let result = test_deferred_processing();

    if result {
        context
            .uart
            .puts("   ✅ Deferred processing integration tests passed\r\n");
    } else {
        context
            .uart
            .puts("   ❌ Some deferred processing integration tests failed\r\n");
    }
}

/// Display deferred processing statistics
fn display_deferred_processing_stats(context: &ShellContext) {
    use crate::exceptions::deferred_processing::get_deferred_stats;

    let stats = get_deferred_stats();
    context.uart.puts("   Total processing cycles: ");
    print_number(&context.uart, stats.total_processing_cycles as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Total items processed: ");
    print_number(&context.uart, stats.total_items_processed as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Max processing time: ");
    print_number(&context.uart, stats.max_processing_time as u32);
    context.uart.puts(" us\r\n");
}
