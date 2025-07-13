// TinyOS Shell Exception Control Commands
// Focused module for MMU and exception control operations

use crate::{
    memory::{
        disable_mmu_global, enable_mmu_global, set_mmu_exception_handling_enabled,
    },
    shell::ShellContext,
};

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

/// Handle MMU control command (enable/disable)
pub fn cmd_mmu_enable_disable(args: &[&str], context: &mut ShellContext) {
    if args.len() != 2 {
        context.uart.puts("Usage: mmuctl <on|off>\r\n");
        return;
    }

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
