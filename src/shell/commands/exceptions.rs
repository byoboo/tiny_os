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

use crate::{
    exceptions::{get_memory_fault_stats, EXCEPTION_STATS},
    memory::{
        get_mmu_exception_stats, is_mmu_exception_handling_enabled,
        set_mmu_exception_handling_enabled,
    },
    shell::ShellContext,
};

/// Handle the 'ex' command - show exception statistics
pub fn cmd_exception_stats(args: &[&str], context: &mut ShellContext) {
    if args.len() != 1 {
        context.uart.puts("Usage: ex\r\n");
        return;
    }

    // Get exception statistics from the global static
    let (sync_count, irq_count, fiq_count, serror_count, total_count) = unsafe {
        let stats = &*core::ptr::addr_of!(EXCEPTION_STATS);
        (
            stats.sync_exceptions,
            stats.irq_exceptions,
            stats.fiq_exceptions,
            stats.serror_exceptions,
            stats.total_exceptions,
        )
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
        context
            .uart
            .puts("  on  - Enable MMU exception handling\r\n");
        context
            .uart
            .puts("  off - Disable MMU exception handling\r\n");
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
        context
            .uart
            .puts("  alignment - Test alignment fault (safe)\r\n");
        context
            .uart
            .puts("  nullderef - Test null pointer dereference (safe)\r\n");
        context
            .uart
            .puts("WARNING: These tests may cause system instability!\r\n");
        return;
    }

    match args[1] {
        "alignment" => {
            context.uart.puts("Testing alignment fault...\r\n");
            context
                .uart
                .puts("This would trigger an alignment exception\r\n");
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

// Phase 4.2 Virtual Memory Management Commands

/// Handle virtual memory status command
pub fn cmd_virtual_memory_status(args: &[&str], context: &mut ShellContext) {
    if args.len() != 1 {
        context.uart.puts("Usage: vm\r\n");
        return;
    }

    use crate::memory::{get_virtual_memory_stats, is_mmu_enabled_global};

    context.uart.puts("Virtual Memory Status:\r\n");
    context.uart.puts("======================\r\n");

    if let Some(stats) = get_virtual_memory_stats() {
        context.uart.puts("MMU Status: ");
        if stats.mmu_enabled {
            context.uart.puts("ENABLED\r\n");
        } else {
            context.uart.puts("DISABLED\r\n");
        }

        // Display page table addresses
        let mut buffer = [0u8; 32];

        context.uart.puts("Kernel Table: 0x");
        format_hex(stats.kernel_table_addr, &mut buffer);
        context
            .uart
            .puts(core::str::from_utf8(&buffer).unwrap_or("???"));
        context.uart.puts("\r\n");

        context.uart.puts("User Table: 0x");
        format_hex(stats.user_table_addr, &mut buffer);
        context
            .uart
            .puts(core::str::from_utf8(&buffer).unwrap_or("???"));
        context.uart.puts("\r\n");

        context.uart.puts("Next Table: 0x");
        format_hex(stats.next_table_addr, &mut buffer);
        context
            .uart
            .puts(core::str::from_utf8(&buffer).unwrap_or("???"));
        context.uart.puts("\r\n");
    } else {
        context
            .uart
            .puts("Virtual memory manager not initialized\r\n");
    }

    // Show current MMU enable status from hardware
    context.uart.puts("Hardware MMU: ");
    if is_mmu_enabled_global() {
        context.uart.puts("ACTIVE\r\n");
    } else {
        context.uart.puts("INACTIVE\r\n");
    }
}

/// Handle MMU control command (enable/disable)
pub fn cmd_mmu_enable_disable(args: &[&str], context: &mut ShellContext) {
    if args.len() != 2 {
        context.uart.puts("Usage: mmuctl <on|off>\r\n");
        return;
    }

    use crate::memory::{disable_mmu_global, enable_mmu_global};

    match args[1] {
        "on" | "enable" => {
            context.uart.puts("Enabling MMU...\r\n");
            match enable_mmu_global() {
                Ok(()) => {
                    context.uart.puts("✓ MMU enabled successfully\r\n");
                    context
                        .uart
                        .puts("Virtual memory translation is now active\r\n");
                }
                Err(e) => {
                    context.uart.puts("✗ MMU enable failed: ");
                    context.uart.puts(e);
                    context.uart.puts("\r\n");
                }
            }
        }
        "off" | "disable" => {
            context.uart.puts("Disabling MMU...\r\n");
            match disable_mmu_global() {
                Ok(()) => {
                    context.uart.puts("✓ MMU disabled successfully\r\n");
                    context
                        .uart
                        .puts("Virtual memory translation is now inactive\r\n");
                }
                Err(e) => {
                    context.uart.puts("✗ MMU disable failed: ");
                    context.uart.puts(e);
                    context.uart.puts("\r\n");
                }
            }
        }
        _ => {
            context.uart.puts("Invalid option. Use 'on' or 'off'\r\n");
        }
    }
}

/// Handle address translation command
pub fn cmd_translate_address(args: &[&str], context: &mut ShellContext) {
    if args.len() != 2 {
        context.uart.puts("Usage: translate <hex_address>\r\n");
        context.uart.puts("Example: translate 0x80000\r\n");
        return;
    }

    use crate::memory::translate_address_global;

    // Parse hex address
    let addr_str = args[1];
    let virtual_addr = if addr_str.starts_with("0x") || addr_str.starts_with("0X") {
        parse_hex(&addr_str[2..])
    } else {
        parse_hex(addr_str)
    };

    if let Some(virt_addr) = virtual_addr {
        context.uart.puts("Translating virtual address: 0x");
        let mut buffer = [0u8; 32];
        format_hex(virt_addr, &mut buffer);
        context
            .uart
            .puts(core::str::from_utf8(&buffer).unwrap_or("???"));
        context.uart.puts("\r\n");

        match translate_address_global(virt_addr) {
            Ok(phys_addr) => {
                context.uart.puts("Physical address: 0x");
                format_hex(phys_addr, &mut buffer);
                context
                    .uart
                    .puts(core::str::from_utf8(&buffer).unwrap_or("???"));
                context.uart.puts("\r\n");
            }
            Err(e) => {
                context.uart.puts("Translation failed: ");
                context.uart.puts(e);
                context.uart.puts("\r\n");
            }
        }
    } else {
        context.uart.puts("Invalid address format\r\n");
    }
}

/// Handle TLB invalidation command
pub fn cmd_invalidate_tlb(args: &[&str], context: &mut ShellContext) {
    if args.len() != 1 {
        context.uart.puts("Usage: tlbflush\r\n");
        return;
    }

    use crate::memory::invalidate_tlb_global;

    context.uart.puts("Invalidating TLB...\r\n");
    invalidate_tlb_global();
    context.uart.puts("✓ TLB invalidated\r\n");
}

/// Handle virtual memory test command
pub fn cmd_virtual_memory_test(args: &[&str], context: &mut ShellContext) {
    if args.len() != 1 {
        context.uart.puts("Usage: vmtest\r\n");
        return;
    }

    use crate::memory::{is_mmu_enabled_global, translate_address_global};

    context.uart.puts("Virtual Memory Test:\r\n");
    context.uart.puts("===================\r\n");

    // Test 1: Check MMU status
    context.uart.puts("1. MMU Status: ");
    if is_mmu_enabled_global() {
        context.uart.puts("✓ ENABLED\r\n");
    } else {
        context.uart.puts("⚠ DISABLED\r\n");
    }

    // Test 2: Test address translation for known addresses
    let test_addresses = [
        0x80000u64,    // Kernel start
        0x100000u64,   // Heap start
        0xFE000000u64, // Peripheral base
    ];

    let address_names = ["Kernel start", "Heap start", "Peripheral base"];

    for (i, &addr) in test_addresses.iter().enumerate() {
        context.uart.puts("2.");
        let mut buffer = [0u8; 4];
        format_number((i + 1) as u64, &mut buffer);
        context
            .uart
            .puts(core::str::from_utf8(&buffer).unwrap_or("?"));
        context.uart.puts(" ");
        context.uart.puts(address_names[i]);
        context.uart.puts(": ");

        match translate_address_global(addr) {
            Ok(phys_addr) => {
                context.uart.puts("0x");
                let mut hex_buffer = [0u8; 32];
                format_hex(phys_addr, &mut hex_buffer);
                context
                    .uart
                    .puts(core::str::from_utf8(&hex_buffer).unwrap_or("???"));
                context.uart.puts(" ✓\r\n");
            }
            Err(_) => {
                context.uart.puts("FAILED ✗\r\n");
            }
        }
    }

    context.uart.puts("\r\nVirtual memory test complete\r\n");
}

/// Format a 64-bit value as hexadecimal
fn format_hex(value: u64, buffer: &mut [u8]) -> usize {
    const HEX_CHARS: &[u8] = b"0123456789ABCDEF";
    let mut pos = 0;
    let mut val = value;

    // Handle zero case
    if val == 0 {
        buffer[0] = b'0';
        return 1;
    }

    // Convert to hex (reverse order)
    let mut temp_buffer = [0u8; 16];
    let mut temp_pos = 0;

    while val > 0 && temp_pos < 16 {
        temp_buffer[temp_pos] = HEX_CHARS[(val & 0xF) as usize];
        val >>= 4;
        temp_pos += 1;
    }

    // Reverse into final buffer
    for i in 0..temp_pos {
        if pos < buffer.len() {
            buffer[pos] = temp_buffer[temp_pos - 1 - i];
            pos += 1;
        }
    }

    pos
}

/// Parse hexadecimal string to u64
fn parse_hex(hex_str: &str) -> Option<u64> {
    let mut result = 0u64;

    for c in hex_str.chars() {
        let digit = match c {
            '0'..='9' => (c as u8) - b'0',
            'a'..='f' => (c as u8) - b'a' + 10,
            'A'..='F' => (c as u8) - b'A' + 10,
            _ => return None,
        };

        result = result.checked_mul(16)?.checked_add(digit as u64)?;
    }

    Some(result)
}
