//! Advanced Command Router
//!
//! This module handles routing for advanced commands that require
//! complex submenu interactions and multi-level user input.

use crate::shell::{commands, core::ShellContext};

/// Route process management submenu commands
pub fn route_process_management(context: &mut ShellContext) {
    context.uart.puts("\r\nProcess Management Commands:\r\n");
    context.uart.puts("  1 - Process Context Test\r\n");
    context.uart.puts("  2 - Privilege Level Test\r\n");
    context.uart.puts("  3 - Scheduler Test\r\n");
    context.uart.puts("  4 - Process Stats\r\n");
    context.uart.puts("  5 - Scheduler Stats\r\n");
    context.uart.puts("  6 - Privilege Stats\r\n");
    context.uart.puts("Select option: ");

    if let Some(option) = context.uart.getc() {
        match option {
            b'1' => commands::process::handle_process_context_test(context),
            b'2' => commands::process::handle_privilege_test(context),
            b'3' => commands::process::handle_scheduler_test(context),
            b'4' => commands::process::handle_process_stats(context),
            b'5' => commands::process::handle_scheduler_stats(context),
            b'6' => commands::process::handle_privilege_stats(context),
            _ => context.uart.puts("Invalid option\r\n"),
        }
    }
}

/// Route exception management submenu commands
pub fn route_exception_management(context: &mut ShellContext) {
    context.uart.puts("\r\nException Management Commands:\r\n");
    context.uart.puts("  1 - Exception Statistics\r\n");
    context.uart.puts("  2 - MMU Exception Statistics\r\n");
    context.uart.puts("  3 - MMU Control (on/off)\r\n");
    context.uart.puts("  4 - Exception Testing (safe)\r\n");
    context.uart.puts("  5 - Reset Exception Stats\r\n");
    context.uart.puts("Select option: ");

    if let Some(option) = context.uart.getc() {
        match option {
            b'1' => commands::exceptions::cmd_exception_stats(&["ex"], context),
            b'2' => commands::exceptions::cmd_mmu_stats(&["mmu"], context),
            b'3' => {
                context.uart.puts("Enable (1) or Disable (2) MMU handling? ");
                if let Some(choice) = context.uart.getc() {
                    match choice {
                        b'1' => commands::exceptions::cmd_mmu_control(&["mmuctl", "on"], context),
                        b'2' => commands::exceptions::cmd_mmu_control(&["mmuctl", "off"], context),
                        _ => context.uart.puts("Invalid choice\r\n"),
                    }
                }
            }
            b'4' => {
                context.uart.puts("Test type: (1) Alignment, (2) Null deref: ");
                if let Some(choice) = context.uart.getc() {
                    match choice {
                        b'1' => commands::exceptions::cmd_test_exceptions(&["extest", "alignment"], context),
                        b'2' => commands::exceptions::cmd_test_exceptions(&["extest", "nullderef"], context),
                        _ => context.uart.puts("Invalid choice\r\n"),
                    }
                }
            }
            b'5' => commands::exceptions::cmd_reset_exception_stats(&["exreset"], context),
            _ => context.uart.puts("Invalid option\r\n"),
        }
    }
}

/// Route virtual memory management submenu commands
pub fn route_virtual_memory_management(context: &mut ShellContext) {
    context.uart.puts("\r\nVirtual Memory Management Commands:\r\n");
    context.uart.puts("  1 - Virtual Memory Status\r\n");
    context.uart.puts("  2 - Enable MMU\r\n");
    context.uart.puts("  3 - Disable MMU\r\n");
    context.uart.puts("  4 - Translate Address\r\n");
    context.uart.puts("  5 - Flush TLB\r\n");
    context.uart.puts("  6 - Virtual Memory Test\r\n");
    context.uart.puts("Select option: ");

    if let Some(option) = context.uart.getc() {
        match option {
            b'1' => commands::exceptions::cmd_virtual_memory_status(&["vmstatus"], context),
            b'2' => commands::exceptions::cmd_mmu_control(&["mmuenable", "on"], context),
            b'3' => commands::exceptions::cmd_mmu_control(&["mmudisable", "off"], context),
            b'4' => {
                context.uart.puts("Enter virtual address (hex): ");
                // For now, use a test address - in real implementation would parse input
                commands::exceptions::cmd_translate_address(&["translate", "0x10000000"], context);
            }
            b'5' => commands::exceptions::cmd_mmu_control(&["flush_tlb"], context),
            b'6' => commands::exceptions::cmd_virtual_memory_test(&["vmtest"], context),
            _ => context.uart.puts("Invalid option\r\n"),
        }
    }
}
