// TinyOS Shell Exception Testing Commands
// Focused module for exception testing operations

use crate::{
    memory::{is_mmu_enabled_global, translate_address_global},
    shell::ShellContext,
};

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

/// Handle virtual memory test command
pub fn cmd_virtual_memory_test(args: &[&str], context: &mut ShellContext) {
    if args.len() != 1 {
        context.uart.puts("Usage: vmtest\r\n");
        return;
    }

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
