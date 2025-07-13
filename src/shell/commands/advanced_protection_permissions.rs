use crate::{
    memory::protection::{
        get_advanced_page_permissions, set_advanced_page_permissions, PagePermissions,
    },
    shell::core::ShellContext,
};

/// Handle page permissions commands
pub fn cmd_advanced_protection_permissions(args: &[&str], context: &mut ShellContext) {
    if args.len() < 2 {
        context
            .uart
            .puts("Usage: permissions [set|get] <address> [permissions]\r\n");
        context
            .uart
            .puts("  set <addr> <perms>  - Set page permissions\r\n");
        context
            .uart
            .puts("  get <addr>          - Get page permissions\r\n");
        context
            .uart
            .puts("Permissions: r (read), w (write), x (execute)\r\n");
        return;
    }

    match args[1] {
        "set" => {
            if args.len() < 4 {
                context
                    .uart
                    .puts("Usage: permissions set <address> <permissions>\r\n");
                return;
            }

            let addr = parse_hex_address(args[2]);
            let perms = parse_permissions(args[3]);

            match set_advanced_page_permissions(addr, perms) {
                Ok(_) => {
                    context.uart.puts("Page permissions set successfully\r\n");
                }
                Err(e) => {
                    context.uart.puts("Error setting permissions: ");
                    context.uart.puts(e);
                    context.uart.puts("\r\n");
                }
            }
        }
        "get" => {
            if args.len() < 3 {
                context.uart.puts("Usage: permissions get <address>\r\n");
                return;
            }

            let addr = parse_hex_address(args[2]);
            match get_advanced_page_permissions(addr) {
                Some(perms) => {
                    context.uart.puts("Page permissions at 0x");
                    context.uart.put_hex(addr);
                    context.uart.puts(": ");
                    display_permissions(perms, context);
                    context.uart.puts("\r\n");
                }
                None => {
                    context.uart.puts("No permissions found for address 0x");
                    context.uart.put_hex(addr);
                    context.uart.puts("\r\n");
                }
            }
        }
        _ => {
            context.uart.puts("Unknown permissions command\r\n");
        }
    }
}

/// Parse hex address from string
fn parse_hex_address(s: &str) -> u64 {
    if let Some(stripped) = s.strip_prefix("0x") {
        u64::from_str_radix(stripped, 16).unwrap_or(0)
    } else {
        u64::from_str_radix(s, 16).unwrap_or(0)
    }
}

/// Parse permissions from string
fn parse_permissions(s: &str) -> PagePermissions {
    let mut perms = PagePermissions::none();

    for c in s.chars() {
        match c {
            'r' => perms = perms.add_read(),
            'w' => perms = perms.add_write(),
            'x' => perms = perms.add_execute(),
            _ => {}
        }
    }

    perms
}

/// Display permissions
fn display_permissions(perms: PagePermissions, context: &mut ShellContext) {
    if perms.is_readable() {
        context.uart.puts("r");
    } else {
        context.uart.puts("-");
    }

    if perms.is_writable() {
        context.uart.puts("w");
    } else {
        context.uart.puts("-");
    }

    if perms.is_executable() {
        context.uart.puts("x");
    } else {
        context.uart.puts("-");
    }
}
