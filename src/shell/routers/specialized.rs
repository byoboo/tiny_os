//! Specialized Command Router
//!
//! This module handles routing for specialized subsystems including
//! stack management, COW operations, testing framework, and advanced features.

use crate::shell::{commands, core::ShellContext};

/// Route stack management submenu commands
pub fn route_stack_management(context: &mut ShellContext) {
    context.uart.puts("\r\nStack Management Commands:\r\n");
    context.uart.puts("  1 - Stack Status\r\n");
    context.uart.puts("  2 - Allocate Kernel Stack\r\n");
    context.uart.puts("  3 - Allocate User Stack\r\n");
    context.uart.puts("  4 - Deallocate Stack\r\n");
    context.uart.puts("  5 - Switch Stack\r\n");
    context.uart.puts("  6 - Stack Test\r\n");
    context.uart.puts("Select option: ");

    if let Some(option) = context.uart.getc() {
        match option {
            b'1' => commands::system::cmd_stack_status(&["stack_status"], context),
            b'2' => commands::system::cmd_stack_alloc(&["stack_alloc", "kernel"], context),
            b'3' => commands::system::cmd_stack_alloc(&["stack_alloc", "user"], context),
            b'4' => {
                context.uart.puts("Enter stack ID: ");
                // For now, test with stack ID 1
                commands::system::cmd_stack_dealloc(&["stack_dealloc", "1"], context);
            }
            b'5' => {
                context.uart.puts("Enter stack ID: ");
                // For now, test with stack ID 0
                commands::system::cmd_stack_switch(&["stack_switch", "0"], context);
            }
            b'6' => commands::system::cmd_stack_test(&["stack_test"], context),
            _ => context.uart.puts("Invalid option\r\n"),
        }
    }
}

/// Route COW management submenu commands
pub fn route_cow_management(context: &mut ShellContext) {
    context.uart.puts("\r\nCOW Management Commands:\r\n");
    context.uart.puts("  1 - COW Status\r\n");
    context.uart.puts("  2 - COW Statistics\r\n");
    context.uart.puts("  3 - Create COW Mapping\r\n");
    context.uart.puts("  4 - Protect COW Page\r\n");
    context.uart.puts("  5 - Unprotect COW Page\r\n");
    context.uart.puts("  6 - COW Test\r\n");
    context.uart.puts("Select option: ");

    if let Some(option) = context.uart.getc() {
        match option {
            b'1' => commands::system::cmd_cow_status(&["cow_status"], context),
            b'2' => commands::system::cmd_cow_stats(&["cow_stats"], context),
            b'3' => commands::system::cmd_cow_create(&["cow_create"], context),
            b'4' => commands::system::cmd_cow_protect(&["cow_protect"], context),
            b'5' => commands::system::cmd_cow_unprotect(&["cow_unprotect"], context),
            b'6' => commands::system::cmd_cow_test(&["cow_test"], context),
            _ => context.uart.puts("Invalid option\r\n"),
        }
    }
}

/// Route testing framework submenu commands
pub fn route_testing_framework(context: &mut ShellContext) {
    context.uart.puts("\r\nTesting Framework Commands:\r\n");
    context.uart.puts("  1 - Kernel Tests\r\n");
    context.uart.puts("  2 - MMU Tests\r\n");
    context.uart.puts("  3 - Process Tests\r\n");
    context.uart.puts("  4 - Syscall Tests\r\n");
    context.uart.puts("  5 - Integration Tests\r\n");
    context.uart.puts("  6 - All Tests\r\n");
    context.uart.puts("Select option: ");

    if let Some(option) = context.uart.getc() {
        match option {
            b'1' => commands::testing::handle_kernel_tests(context),
            b'2' => commands::testing::handle_mmu_tests(context),
            b'3' => commands::testing::handle_process_tests(context),
            b'4' => commands::testing::handle_syscall_tests(context),
            b'5' => commands::testing::handle_integration_tests(context),
            b'6' => commands::testing::handle_all_tests(context),
            _ => context.uart.puts("Invalid option\r\n"),
        }
    }
}

/// Route user space page table management commands
pub fn route_user_space_management(context: &mut ShellContext) {
    context
        .uart
        .puts("\r\nUser Space Page Table Management:\r\n");
    context.uart.puts("  1 - User Space Status\r\n");
    context.uart.puts("  2 - Create User Page Table\r\n");
    context.uart.puts("  3 - Destroy User Page Table\r\n");
    context.uart.puts("  4 - Switch User Page Table\r\n");
    context.uart.puts("  5 - VMA Management\r\n");
    context.uart.puts("  6 - User Space Test\r\n");
    context.uart.puts("  7 - Initialize User Space Manager\r\n");
    context.uart.puts("Select option: ");

    if let Some(option) = context.uart.getc() {
        match option {
            b'1' => commands::user_space::handle_user_space_status(context),
            b'2' => commands::user_space::handle_create_user_page_table(context),
            b'3' => commands::user_space::handle_destroy_user_page_table(context),
            b'4' => commands::user_space::handle_switch_user_page_table(context),
            b'5' => commands::user_space::handle_vma_management(context),
            b'6' => commands::user_space::handle_user_space_test(context),
            b'7' => commands::user_space::handle_user_space_init(context),
            _ => context.uart.puts("Invalid option\r\n"),
        }
    }
}

/// Route advanced memory protection commands
pub fn route_advanced_protection(context: &mut ShellContext) {
    context.uart.puts("\r\nAdvanced Memory Protection:\r\n");
    context.uart.puts("  1 - Protection Status\r\n");
    context.uart.puts("  2 - Page Permissions\r\n");
    context.uart.puts("  3 - ASLR Information\r\n");
    context.uart.puts("  4 - Stack Protection\r\n");
    context.uart.puts("  5 - Protection Test\r\n");
    context.uart.puts("  6 - Protection Statistics\r\n");
    context.uart.puts("  h - Help\r\n");
    context.uart.puts("Select option: ");

    if let Some(option) = context.uart.getc() {
        match option {
            b'1' => commands::advanced_protection::cmd_advanced_protection_status(&[], context),
            b'2' => handle_page_permissions_submenu(context),
            b'3' => commands::advanced_protection::cmd_advanced_protection_aslr(&[], context),
            b'4' => handle_stack_protection_submenu(context),
            b'5' => commands::advanced_protection::cmd_advanced_protection_test(&[], context),
            b'6' => commands::advanced_protection::cmd_advanced_protection_stats(&[], context),
            b'h' => show_advanced_protection_help(context),
            _ => context.uart.puts("Invalid option\r\n"),
        }
    }
}

/// Handle page permissions submenu
fn handle_page_permissions_submenu(context: &mut ShellContext) {
    context.uart.puts("Page permissions commands:\r\n");
    context.uart.puts("  s - Set permissions\r\n");
    context.uart.puts("  g - Get permissions\r\n");
    context.uart.puts("Select: ");
    if let Some(perm_option) = context.uart.getc() {
        match perm_option {
            b's' => {
                context.uart.puts("Set permissions (addr perms): ");
                let args = ["permissions", "set", "0x1000000", "rw"];
                commands::advanced_protection::cmd_advanced_protection_permissions(&args, context);
            }
            b'g' => {
                context.uart.puts("Get permissions (addr): ");
                let args = ["permissions", "get", "0x1000000"];
                commands::advanced_protection::cmd_advanced_protection_permissions(&args, context);
            }
            _ => context.uart.puts("Invalid option\r\n"),
        }
    }
}

/// Handle stack protection submenu
fn handle_stack_protection_submenu(context: &mut ShellContext) {
    context.uart.puts("Stack protection commands:\r\n");
    context.uart.puts("  s - Setup protection\r\n");
    context.uart.puts("  i - Stack info\r\n");
    context.uart.puts("Select: ");
    if let Some(stack_option) = context.uart.getc() {
        match stack_option {
            b's' => {
                let args = ["stack", "setup", "1", "0x2000000", "0x10000"];
                commands::advanced_protection::cmd_advanced_protection_stack(&args, context);
            }
            b'i' => {
                let args = ["stack", "info"];
                commands::advanced_protection::cmd_advanced_protection_stack(&args, context);
            }
            _ => context.uart.puts("Invalid option\r\n"),
        }
    }
}

/// Show advanced protection help
fn show_advanced_protection_help(context: &mut ShellContext) {
    context.uart.puts("Advanced Memory Protection Help:\r\n");
    context.uart.puts("  1 - Show protection system status\r\n");
    context.uart.puts("  2 - Manage page permissions\r\n");
    context.uart.puts("  3 - View ASLR information\r\n");
    context.uart.puts("  4 - Stack protection features\r\n");
    context.uart.puts("  5 - Run protection tests\r\n");
    context.uart.puts("  6 - Show protection statistics\r\n");
}

/// Route dynamic memory management commands
pub fn route_dynamic_memory_management(context: &mut ShellContext) {
    context.uart.puts("\r\nDynamic Memory Management:\r\n");
    context.uart.puts("  1 - System Status\r\n");
    context.uart.puts("  2 - Stack Growth Management\r\n");
    context.uart.puts("  3 - Lazy Page Allocation\r\n");
    context.uart.puts("  4 - Memory Pressure Monitoring\r\n");
    context.uart.puts("  5 - Memory Optimization\r\n");
    context.uart.puts("  6 - Dynamic Memory Statistics\r\n");
    context.uart.puts("Select option: ");

    if let Some(option) = context.uart.getc() {
        match option {
            b'1' => commands::dynamic_memory::cmd_dynamic_memory_status(&[], context),
            b'2' => commands::dynamic_memory::cmd_dynamic_memory_growth(&["growth"], context),
            b'3' => commands::dynamic_memory::cmd_dynamic_memory_lazy(&["lazy"], context),
            b'4' => commands::dynamic_memory::cmd_dynamic_memory_pressure(&["pressure"], context),
            b'5' => {
                commands::dynamic_memory::cmd_dynamic_memory_optimize(&["optimization"], context)
            }
            b'6' => commands::dynamic_memory::cmd_dynamic_memory_stats(&[], context),
            _ => context.uart.puts("Invalid option\r\n"),
        }
    }
}
