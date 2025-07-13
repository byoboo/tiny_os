use crate::memory::dynamic::{is_dynamic_memory_enabled, enable_dynamic_memory, disable_dynamic_memory};
use crate::shell::core::ShellContext;

/// Core dynamic memory management commands
pub fn cmd_dynamic_memory(args: &[&str], context: &mut ShellContext) {
    context.uart.puts("Dynamic Memory Management System:\r\n");
    context.uart.puts("================================\r\n");

    if args.len() < 2 {
        context.uart.puts("Usage: dm <enable|disable|status|help>\r\n");
        context.uart.puts("  enable   - Enable dynamic memory management\r\n");
        context.uart.puts("  disable  - Disable dynamic memory management\r\n");
        context.uart.puts("  status   - Show system status\r\n");
        context.uart.puts("  help     - Show this help message\r\n");
        return;
    }

    match args[1] {
        "enable" => {
            if is_dynamic_memory_enabled() {
                context.uart.puts("Dynamic memory management is already enabled\r\n");
            } else {
                match enable_dynamic_memory() {
                    Ok(()) => {
                        context.uart.puts("Dynamic memory management enabled successfully\r\n");
                    }
                    Err(e) => {
                        context.uart.puts("Failed to enable dynamic memory management: ");
                        context.uart.puts(e);
                        context.uart.puts("\r\n");
                    }
                }
            }
        }
        "disable" => {
            if !is_dynamic_memory_enabled() {
                context.uart.puts("Dynamic memory management is already disabled\r\n");
            } else {
                match disable_dynamic_memory() {
                    Ok(()) => {
                        context.uart.puts("Dynamic memory management disabled successfully\r\n");
                    }
                    Err(e) => {
                        context.uart.puts("Failed to disable dynamic memory management: ");
                        context.uart.puts(e);
                        context.uart.puts("\r\n");
                    }
                }
            }
        }
        "status" => {
            if is_dynamic_memory_enabled() {
                context.uart.puts("Dynamic Memory Management: ENABLED\r\n");
                context.uart.puts("System is ready for dynamic operations\r\n");
            } else {
                context.uart.puts("Dynamic Memory Management: DISABLED\r\n");
                context.uart.puts("Enable system to access dynamic features\r\n");
            }
        }
        "help" => {
            context.uart.puts("Dynamic Memory Management Commands:\r\n");
            context.uart.puts("  dm enable/disable/status - Core system control\r\n");
            context.uart.puts("  dm_status               - Detailed status info\r\n");
            context.uart.puts("  dm_growth              - Stack growth management\r\n");
            context.uart.puts("  dm_lazy                - Lazy page allocation\r\n");
            context.uart.puts("  dm_pressure            - Memory pressure monitoring\r\n");
            context.uart.puts("  dm_optimize            - Memory optimization\r\n");
            context.uart.puts("  dm_context             - Context switching\r\n");
            context.uart.puts("  dm_stats               - Complete statistics\r\n");
        }
        _ => {
            context.uart.puts("Unknown command. Use 'dm help' for usage.\r\n");
        }
    }
}
