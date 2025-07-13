use crate::{
    memory::protection::{
        get_advanced_protection_stats, setup_advanced_stack_protection,
        verify_advanced_stack_canary,
    },
    shell::core::ShellContext,
};

/// Handle stack protection commands
pub fn cmd_advanced_protection_stack(args: &[&str], context: &mut ShellContext) {
    if args.len() < 2 {
        context.uart.puts("Usage: stack [setup|verify|info]\r\n");
        context
            .uart
            .puts("  setup <pid> <start> <size>  - Setup stack protection\r\n");
        context
            .uart
            .puts("  verify <pid> <canary>       - Verify stack canary\r\n");
        context
            .uart
            .puts("  info                        - Show stack protection info\r\n");
        return;
    }

    match args[1] {
        "setup" => {
            if args.len() < 5 {
                context
                    .uart
                    .puts("Usage: stack setup <process_id> <start_addr> <size>\r\n");
                return;
            }

            let pid = parse_decimal(args[2]);
            let start = parse_hex_address(args[3]);
            let size = parse_hex_address(args[4]);

            match setup_advanced_stack_protection(pid as usize, start, size) {
                Ok(canary) => {
                    context.uart.puts("Stack protection setup successfully\r\n");
                    context.uart.puts("Stack canary: 0x");
                    context.uart.put_hex(canary);
                    context.uart.puts("\r\n");
                }
                Err(e) => {
                    context.uart.puts("Error setting up stack protection: ");
                    context.uart.puts(e);
                    context.uart.puts("\r\n");
                }
            }
        }
        "verify" => {
            if args.len() < 4 {
                context
                    .uart
                    .puts("Usage: stack verify <process_id> <canary>\r\n");
                return;
            }

            let pid = parse_decimal(args[2]);
            let canary = parse_hex_address(args[3]);

            if verify_advanced_stack_canary(pid as usize, canary) {
                context.uart.puts("Stack canary verification: PASSED\r\n");
            } else {
                context.uart.puts("Stack canary verification: FAILED\r\n");
            }
        }
        "info" => {
            let stats = get_advanced_protection_stats();
            context.uart.puts("Stack Protection Information:\r\n");
            context.uart.puts("============================\r\n");
            context.uart.puts("Protected stacks: ");
            context.uart.put_hex(stats.protected_stacks as u64);
            context.uart.puts("\r\n");
            context.uart.puts("Stack canaries active: ");
            context.uart.put_hex(stats.stack_canaries_active as u64);
            context.uart.puts("\r\n");
            context.uart.puts("Stack violations detected: ");
            context.uart.put_hex(stats.stack_violations as u64);
            context.uart.puts("\r\n");
        }
        _ => {
            context.uart.puts("Unknown stack command\r\n");
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

/// Parse decimal number from string
fn parse_decimal(s: &str) -> u32 {
    s.parse::<u32>().unwrap_or(0)
}
