//! Exception system commands for the TinyOS shell
//!
//! This module provides shell commands for:
//! - Exception statistics and monitoring
//! - MMU exception handling control
//! - Exception testing and debugging
//!
//! Available commands:
//! - `ex`: Show exception statistics
//! - `mmu`: Show MMU exception statistics
//! - `exreset`: Reset exception statistics
//! - `extest`: Test exception handling (carefully)

use crate::exceptions::{EXCEPTION_STATS, get_memory_fault_stats};
use crate::memory::{get_mmu_exception_stats, is_mmu_exception_handling_enabled, set_mmu_exception_handling_enabled};
use crate::shell::ShellContext;

/// Handle the 'ex' command - show exception statistics
pub fn cmd_exception_stats(args: &[&str], context: &mut ShellContext) {
    if args.len() != 1 {
        context.uart.puts("Usage: ex\r\n");
        return;
    }

    // Get exception statistics from the global static
    let (sync_count, irq_count, fiq_count, serror_count, total_count) = unsafe {
        let stats = &*core::ptr::addr_of!(EXCEPTION_STATS);
        (stats.sync_exceptions, stats.irq_exceptions, stats.fiq_exceptions, stats.serror_exceptions, stats.total_exceptions)
    };
    
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
    
    context.uart.puts("\r\nNo current exception context available\r\n");
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

    // Reset exception statistics using the existing method
    unsafe {
        let stats = &mut *core::ptr::addr_of_mut!(EXCEPTION_STATS);
        stats.sync_exceptions = 0;
        stats.irq_exceptions = 0;
        stats.fiq_exceptions = 0;
        stats.serror_exceptions = 0;
        stats.total_exceptions = 0;
        stats.last_exception_type = None;
        stats.last_exception_level = None;
    }
    
    context.uart.puts("Exception statistics reset\r\n");
}

/// Handle the 'mmuctl' command - control MMU exception handling
pub fn cmd_mmu_control(args: &[&str], context: &mut ShellContext) {
    if args.len() != 2 {
        context.uart.puts("Usage: mmuctl <on|off>\r\n");
        context.uart.puts("  on  - Enable MMU exception handling\r\n");
        context.uart.puts("  off - Disable MMU exception handling\r\n");
        return;
    }

    match args[1] {
        "on" => {
            set_mmu_exception_handling_enabled(true);
            context.uart.puts("MMU exception handling enabled\r\n");
        }
        "off" => {
            set_mmu_exception_handling_enabled(false);
            context.uart.puts("MMU exception handling disabled\r\n");
        }
        _ => {
            context.uart.puts("Invalid argument. Use 'on' or 'off'\r\n");
        }
    }
}

/// Handle the 'extest' command - test exception handling (carefully)
pub fn cmd_test_exceptions(args: &[&str], context: &mut ShellContext) {
    if args.len() != 2 {
        context.uart.puts("Usage: extest <type>\r\n");
        context.uart.puts("  alignment - Test alignment fault (safe)\r\n");
        context.uart.puts("  nullderef - Test null pointer dereference (safe)\r\n");
        context.uart.puts("WARNING: These tests may cause system instability!\r\n");
        return;
    }

    match args[1] {
        "alignment" => {
            context.uart.puts("Testing alignment fault...\r\n");
            context.uart.puts("This would trigger an alignment exception\r\n");
            context.uart.puts("(Not implemented yet for safety)\r\n");
        }
        "nullderef" => {
            context.uart.puts("Testing null pointer dereference...\r\n");
            context.uart.puts("This would trigger a page fault\r\n");
            context.uart.puts("(Not implemented yet for safety)\r\n");
        }
        _ => {
            context.uart.puts("Unknown test type\r\n");
        }
    }
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
