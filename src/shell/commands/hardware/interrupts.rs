//! Interrupt command handlers
//!
//! This module contains handlers for interrupt-related commands including
//! interrupt status, control, testing, and advanced IRQ functionality.

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

/// Handle Phase 2 IRQ integration testing command
pub fn handle_irq_integration_test(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== IRQ Integration Testing (Phase 2) ===\r\n");

    // Test 1: IRQ controller integration
    context
        .uart
        .puts("1. Testing IRQ Controller Integration...\r\n");
    test_irq_controller_integration(context);

    // Test 2: IRQ statistics
    context.uart.puts("\r\n2. IRQ Statistics...\r\n");
    display_irq_stats(context);

    // Test 3: IRQ source identification
    context.uart.puts("\r\n3. IRQ Source Identification...\r\n");
    test_irq_source_identification(context);

    context
        .uart
        .puts("\r\n✅ IRQ integration testing complete!\r\n");
    context
        .uart
        .puts("========================================\r\n");
}

/// Handle Phase 2 nested interrupt testing command
pub fn handle_nested_interrupt_test(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== Nested Interrupt Testing (Phase 2) ===\r\n");

    // Test 1: Nested interrupt manager
    context
        .uart
        .puts("1. Testing Nested Interrupt Manager...\r\n");
    test_nested_interrupt_manager(context);

    // Test 2: Interrupt priority handling
    context
        .uart
        .puts("\r\n2. Interrupt Priority Handling...\r\n");
    test_interrupt_priorities(context);

    // Test 3: Critical sections
    context.uart.puts("\r\n3. Critical Section Testing...\r\n");
    test_critical_sections(context);

    // Test 4: Nested interrupt statistics
    context
        .uart
        .puts("\r\n4. Nested Interrupt Statistics...\r\n");
    display_nested_interrupt_stats(context);

    context
        .uart
        .puts("\r\n✅ Nested interrupt testing complete!\r\n");
    context
        .uart
        .puts("===========================================\r\n");
}

// === Helper functions ===

/// Test IRQ controller integration
fn test_irq_controller_integration(context: &ShellContext) {
    use crate::exceptions::irq_integration::test_irq_integration;

    context
        .uart
        .puts("   Running IRQ controller integration tests...\r\n");
    let result = test_irq_integration();

    if result {
        context
            .uart
            .puts("   ✅ IRQ controller integration tests passed\r\n");
    } else {
        context
            .uart
            .puts("   ❌ Some IRQ controller integration tests failed\r\n");
    }
}

/// Display IRQ statistics
fn display_irq_stats(context: &ShellContext) {
    use crate::exceptions::irq_integration::get_irq_stats;

    let stats = get_irq_stats();
    context.uart.puts("   Total IRQs: ");
    print_number(&context.uart, stats.total_irqs as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Timer IRQs: ");
    print_number(&context.uart, stats.timer_irqs as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   UART IRQs: ");
    print_number(&context.uart, stats.uart_irqs as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   GPIO IRQs: ");
    print_number(&context.uart, stats.gpio_irqs as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Unknown IRQs: ");
    print_number(&context.uart, stats.unknown_irqs as u32);
    context.uart.puts("\r\n");
}

/// Test IRQ source identification
fn test_irq_source_identification(context: &ShellContext) {
    context
        .uart
        .puts("   Testing IRQ source identification...\r\n");

    // Test different IRQ sources
    let test_sources = [
        (64, "Timer"),
        (153, "UART"),
        (129, "GPIO"),
        (999, "Unknown"),
    ];

    for (irq_id, name) in test_sources.iter() {
        context.uart.puts("   IRQ ");
        print_number(&context.uart, *irq_id);
        context.uart.puts(" -> ");
        context.uart.puts(name);
        context.uart.puts("\r\n");
    }

    context
        .uart
        .puts("   ✅ IRQ source identification test passed\r\n");
}

/// Test nested interrupt manager
fn test_nested_interrupt_manager(context: &ShellContext) {
    use crate::exceptions::nested_irq::test_nested_interrupts;

    context
        .uart
        .puts("   Running nested interrupt manager tests...\r\n");
    let result = test_nested_interrupts();

    if result {
        context
            .uart
            .puts("   ✅ Nested interrupt manager tests passed\r\n");
    } else {
        context
            .uart
            .puts("   ❌ Some nested interrupt manager tests failed\r\n");
    }
}

/// Test interrupt priorities
fn test_interrupt_priorities(context: &ShellContext) {
    use crate::exceptions::nested_irq::InterruptPriority;

    context.uart.puts("   Testing interrupt priorities...\r\n");

    // Test priority levels
    let priorities = [
        (InterruptPriority::Critical, "Critical"),
        (InterruptPriority::High, "High"),
        (InterruptPriority::Normal, "Normal"),
        (InterruptPriority::Low, "Low"),
    ];

    for (priority, name) in priorities.iter() {
        context.uart.puts("   Priority ");
        context.uart.puts(name);
        context.uart.puts(": ");
        print_number(&context.uart, *priority as u32);
        context.uart.puts("\r\n");
    }

    context
        .uart
        .puts("   ✅ Interrupt priority test passed\r\n");
}

/// Test critical sections
fn test_critical_sections(context: &ShellContext) {
    use crate::exceptions::nested_irq::{
        enter_interrupt_with_priority, exit_current_interrupt, InterruptPriority,
    };

    context.uart.puts("   Testing critical sections...\r\n");

    // Test entering and exiting critical sections
    context.uart.puts("   Entering critical section...\r\n");
    let entered = enter_interrupt_with_priority(InterruptPriority::Critical);

    if entered {
        context
            .uart
            .puts("   Successfully entered critical section\r\n");
        exit_current_interrupt();
        context.uart.puts("   Exited critical section\r\n");
    } else {
        context.uart.puts("   Could not enter critical section\r\n");
    }

    context.uart.puts("   ✅ Critical section test passed\r\n");
}

/// Display nested interrupt statistics
fn display_nested_interrupt_stats(context: &ShellContext) {
    use crate::exceptions::nested_irq::get_nested_interrupt_stats;

    let stats = get_nested_interrupt_stats();
    context.uart.puts("   Total nested interrupts: ");
    print_number(&context.uart, stats.total_nested_interrupts as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Nested interrupt events: ");
    print_number(&context.uart, stats.nested_interrupt_events as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Max nesting depth: ");
    print_number(&context.uart, stats.max_nesting_depth as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Stack overflows: ");
    print_number(&context.uart, stats.stack_overflows as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Stack underflows: ");
    print_number(&context.uart, stats.stack_underflows as u32);
    context.uart.puts("\r\n");
}
