//! Central Command Router
//!
//! This module provides the central command routing logic that dispatches
//! incoming commands to the appropriate specialized routers based on command
//! type.

use crate::shell::{commands, core::ShellContext, routers};

/// Main command routing function
///
/// This function receives a command character and routes it to the appropriate
/// handler based on the command type and complexity.
pub fn route_command(ch: u8, context: &mut ShellContext, start_time: u64) {
    // Try basic system commands first
    if routers::route_system_commands(ch, context, start_time) {
        return;
    }

    // Try basic hardware commands
    if routers::route_hardware_commands(ch, context) {
        return;
    }

    // Try enhanced hardware commands
    if routers::route_enhanced_hardware_commands(ch, context) {
        return;
    }

    // Try basic memory commands
    if routers::route_memory_commands(ch, context) {
        return;
    }

    // Handle complex submenu commands
    match ch {
        // Process Management (Phase 3)
        b'&' => routers::route_process_management(context),

        // Exception Management (Phase 4)
        b'^' => routers::route_exception_management(context),

        // Virtual Memory Management (Phase 4.2)
        b'~' => routers::route_virtual_memory_management(context),

        // Stack Management (Phase 4.3)
        b'`' => routers::route_stack_management(context),

        // User Space Management (Phase 4.4.2)
        b'|' => routers::route_user_space_management(context),

        // Advanced Memory Protection (Phase 4.4.3)
        b'@' => routers::route_advanced_protection(context),

        // Dynamic Memory Management (Phase 4.4.4)
        b'*' => routers::route_dynamic_memory_management(context),

        // COW Management (Phase 4.4)
        b'(' => routers::route_cow_management(context),

        // Testing Framework (Phase 5)
        b')' => routers::route_testing_framework(context),

        // Advanced Command Routing
        b'+' => route_advanced_command_interface(context),

        // Direct process management test commands (for automated testing)
        b'[' => commands::process::handle_process_context_test(context),
        b'\\' => commands::process::handle_privilege_test(context),
        b']' => commands::process::handle_scheduler_test(context),

        // Filesystem commands
        b'd' | b'D' => {
            commands::filesystem::handle_directory_listing(&context.uart, &mut context.fat32_fs)
        }
        b'n' | b'N' => {
            commands::filesystem::handle_filesystem_mount_info(&context.uart, &mut context.fat32_fs)
        }
        b'o' | b'O' => {
            commands::filesystem::handle_change_directory(&context.uart, &mut context.fat32_fs)
        }
        b'u' | b'U' => commands::filesystem::handle_read_file(&context.uart, &mut context.fat32_fs),
        b'k' | b'K' => {
            commands::filesystem::handle_change_to_root(&context.uart, &mut context.fat32_fs)
        }
        
        // Text Editor (Week 7 Feature)
        b'E' => {
            commands::editor::cmd_edit(&[], context);
        }

        // Printable character feedback
        _ => {
            if ch.is_ascii_graphic() {
                context.uart.puts("Unknown command: ");
                context.uart.putc(ch);
                context.uart.puts("\r\n");
            } else {
                context
                    .uart
                    .puts("Non-printable character (control code)\r\n");
            }
        }
    }
}

/// Handle advanced command line interface routing
fn route_advanced_command_interface(context: &mut ShellContext) {
    context.uart.puts("\r\nAdvanced Command Interface:\r\n");
    context.uart.puts("  1 - Advanced Protection Commands\r\n");
    context.uart.puts("  2 - Dynamic Memory Commands\r\n");
    context.uart.puts("  3 - Text Editor (Week 7 Feature)\r\n");
    context.uart.puts("Select option: ");

    if let Some(option) = context.uart.getc() {
        match option {
            b'1' => {
                context.uart.puts("Enter advanced protection command: ");
                // For now, show status - in real implementation would parse input
                commands::advanced_protection::cmd_advanced_protection(&["status"], context);
            }
            b'2' => {
                context.uart.puts("Enter dynamic memory command: ");
                // For now, show status - in real implementation would parse input
                commands::dynamic_memory::cmd_dynamic_memory(&["status"], context);
            }
            b'3' => {
                context.uart.puts("Launching Text Editor...\r\n");
                commands::editor::cmd_edit(&[], context);
            }
            _ => context.uart.puts("Invalid option\r\n"),
        }
    }
}
