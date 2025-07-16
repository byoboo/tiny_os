// TinyOS Shell Exception Statistics Commands
// Focused module for exception and MMU statistics display

use crate::{
    exceptions::{get_memory_fault_stats, types::ExceptionStats},
    memory::{get_mmu_exception_stats, is_mmu_exception_handling_enabled},
    shell::ShellContext,
};

/// Handle the 'ex' command - show exception statistics
pub fn cmd_exception_stats(args: &[&str], context: &mut ShellContext) {
    if args.len() != 1 {
        context.uart.puts("Usage: ex\r\n");
        return;
    }

    // Get exception statistics from the global static
    let stats = ExceptionStats::get_stats();
    let (sync_count, irq_count, fiq_count, serror_count, total_count) = (
        stats.sync_exceptions,
        stats.irq_exceptions,
        stats.fiq_exceptions,
        stats.serror_exceptions,
        stats.total_exceptions,
    );

    context.uart.puts("Exception Statistics:\r\n");
    context.uart.puts("=====================\r\n");

    // Format and display statistics
    let mut buffer = [0u8; 64];

    // Total exceptions
    let total_str = format_number(total_count, &mut buffer);
    context.uart.puts("Total exceptions: ");
    context.uart.puts(total_str);
    context.uart.puts("\r\n");

    // Synchronous exceptions
    let sync_str = format_number(sync_count, &mut buffer);
    context.uart.puts("Synchronous: ");
    context.uart.puts(sync_str);
    context.uart.puts("\r\n");

    // IRQ exceptions
    let irq_str = format_number(irq_count, &mut buffer);
    context.uart.puts("IRQ: ");
    context.uart.puts(irq_str);
    context.uart.puts("\r\n");

    // FIQ exceptions
    let fiq_str = format_number(fiq_count, &mut buffer);
    context.uart.puts("FIQ: ");
    context.uart.puts(fiq_str);
    context.uart.puts("\r\n");

    // System error exceptions
    let serr_str = format_number(serror_count, &mut buffer);
    context.uart.puts("System Error: ");
    context.uart.puts(serr_str);
    context.uart.puts("\r\n");

    // Memory fault statistics
    let memory_stats = get_memory_fault_stats();
    let mem_total_str = format_number(memory_stats.total_faults, &mut buffer);
    context.uart.puts("Memory Faults: ");
    context.uart.puts(mem_total_str);
    context.uart.puts("\r\n");

    context
        .uart
        .puts("\r\nNo current exception context available\r\n");
}

/// Handle the 'mmu' command - show MMU exception statistics
pub fn cmd_mmu_stats(args: &[&str], context: &mut ShellContext) {
    if args.len() != 1 {
        context.uart.puts("Usage: mmu\r\n");
        return;
    }

    let stats = get_mmu_exception_stats();
    let enabled = is_mmu_exception_handling_enabled();

    context.uart.puts("MMU Exception Statistics:\r\n");
    context.uart.puts("=========================\r\n");

    // MMU exception handling status
    context.uart.puts("Status: ");
    if enabled {
        context.uart.puts("ENABLED\r\n");
    } else {
        context.uart.puts("DISABLED\r\n");
    }

    let mut buffer = [0u8; 64];

    // Total MMU exceptions
    let total_str = format_number(stats.total_exceptions, &mut buffer);
    context.uart.puts("Total MMU exceptions: ");
    context.uart.puts(total_str);
    context.uart.puts("\r\n");

    // Page faults
    let pf_str = format_number(stats.page_faults, &mut buffer);
    context.uart.puts("Page faults: ");
    context.uart.puts(pf_str);
    context.uart.puts("\r\n");

    // Permission faults
    let perm_str = format_number(stats.permission_faults, &mut buffer);
    context.uart.puts("Permission faults: ");
    context.uart.puts(perm_str);
    context.uart.puts("\r\n");

    // Alignment faults
    let align_str = format_number(stats.alignment_faults, &mut buffer);
    context.uart.puts("Alignment faults: ");
    context.uart.puts(align_str);
    context.uart.puts("\r\n");

    // TLB misses
    let tlb_str = format_number(stats.tlb_misses, &mut buffer);
    context.uart.puts("TLB misses: ");
    context.uart.puts(tlb_str);
    context.uart.puts("\r\n");

    // Recovered faults
    let rec_str = format_number(stats.recovered_faults, &mut buffer);
    context.uart.puts("Recovered faults: ");
    context.uart.puts(rec_str);
    context.uart.puts("\r\n");

    // Fatal faults
    let fatal_str = format_number(stats.fatal_faults, &mut buffer);
    context.uart.puts("Fatal faults: ");
    context.uart.puts(fatal_str);
    context.uart.puts("\r\n");
}

/// Handle the 'exreset' command - reset exception statistics
pub fn cmd_reset_exception_stats(args: &[&str], context: &mut ShellContext) {
    if args.len() != 1 {
        context.uart.puts("Usage: exreset\r\n");
        return;
    }

    // Reset exception statistics using the interface
    ExceptionStats::reset_stats();

    context.uart.puts("Exception statistics reset\r\n");
}

/// Format a number to string (simple decimal formatting)
fn format_number(num: u64, buffer: &mut [u8]) -> &str {
    if num == 0 {
        return "0";
    }

    let mut n = num;
    let mut pos = buffer.len();

    while n > 0 && pos > 0 {
        pos -= 1;
        buffer[pos] = b'0' + (n % 10) as u8;
        n /= 10;
    }

    // Convert to string slice
    core::str::from_utf8(&buffer[pos..]).unwrap_or("???")
}
