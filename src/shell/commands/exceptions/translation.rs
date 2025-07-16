// TinyOS Shell Address Translation Commands
// Focused module for address translation and TLB operations

use crate::{
    memory::{invalidate_tlb_global, translate_address_global},
    shell::ShellContext,
};

/// Handle address translation command
pub fn cmd_translate_address(args: &[&str], context: &mut ShellContext) {
    if args.len() != 2 {
        context.uart.puts("Usage: translate <hex_address>\r\n");
        context.uart.puts("Example: translate 0x80000\r\n");
        return;
    }

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

    context.uart.puts("Invalidating TLB...\r\n");
    invalidate_tlb_global();
    context.uart.puts("✓ TLB invalidated\r\n");
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
