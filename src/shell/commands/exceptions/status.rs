// TinyOS Shell Virtual Memory Status Commands
// Focused module for virtual memory status reporting

use crate::{
    memory::{get_virtual_memory_stats, is_mmu_enabled_global},
    shell::ShellContext,
};

/// Handle virtual memory status command
pub fn cmd_virtual_memory_status(args: &[&str], context: &mut ShellContext) {
    if args.len() != 1 {
        context.uart.puts("Usage: vm\r\n");
        return;
    }

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
