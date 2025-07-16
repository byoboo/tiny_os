//! Statistics Module
//!
//! This module provides exception statistics display and management
//! functionality for shell commands, including detailed breakdowns and
//! analysis.

use super::utils::print_number;
use crate::{exceptions::types::ExceptionStats, shell::ShellContext};

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

/// Display detailed exception statistics
pub fn display_detailed_stats(context: &ShellContext, stats: &ExceptionStats) {
    context.uart.puts("   Total exceptions: ");
    print_number(&context.uart, stats.total_exceptions as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   - Synchronous: ");
    print_number(&context.uart, stats.sync_exceptions as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   - IRQ: ");
    print_number(&context.uart, stats.irq_exceptions as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   - FIQ: ");
    print_number(&context.uart, stats.fiq_exceptions as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   - SError: ");
    print_number(&context.uart, stats.serror_exceptions as u32);
    context.uart.puts("\r\n");

    if let Some(last_type) = &stats.last_exception_type {
        context.uart.puts("   Last exception: ");
        match last_type {
            crate::exceptions::types::ExceptionType::Synchronous => {
                context.uart.puts("Synchronous")
            }
            crate::exceptions::types::ExceptionType::Irq => context.uart.puts("IRQ"),
            crate::exceptions::types::ExceptionType::Fiq => context.uart.puts("FIQ"),
            crate::exceptions::types::ExceptionType::SError => context.uart.puts("SError"),
        }
        context.uart.puts("\r\n");
    }
}
