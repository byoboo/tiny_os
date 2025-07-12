//! Dynamic Memory Management Shell Commands
//!
//! This module provides shell commands for interacting with the dynamic memory
//! management system. It includes commands for monitoring statistics, managing
//! dynamic stacks, controlling lazy page allocation, and handling memory
//! pressure situations.
//!
//! # Commands
//!
//! - `*s` - Show dynamic memory statistics
//! - `*d` - Show detailed dynamic stack information
//! - `*l` - Show lazy page allocation information
//! - `*p` - Show memory pressure status
//! - `*c` - Test context switching
//! - `*t` - Run dynamic memory stress tests
//! - `*r` - Show comprehensive dynamic memory report

use crate::{
    memory::dynamic::{
        add_lazy_page, check_dynamic_memory_pressure, create_dynamic_stack, fast_context_switch,
        get_dynamic_memory_stats, is_dynamic_memory_enabled, PressureLevel,
    },
    shell::ShellContext,
};

/// Dynamic memory management command handler
pub fn cmd_dynamic_memory(args: &[&str], context: &mut ShellContext) {
    if args.is_empty() {
        show_dynamic_memory_help(context);
        return;
    }

    match args[0] {
        "status" => cmd_dynamic_memory_status(args, context),
        "growth" => cmd_dynamic_memory_growth(args, context),
        "lazy" => cmd_dynamic_memory_lazy(args, context),
        "pressure" => cmd_dynamic_memory_pressure(args, context),
        "optimize" => cmd_dynamic_memory_optimize(args, context),
        "context" => cmd_dynamic_memory_context(args, context),
        "stats" => cmd_dynamic_memory_stats(args, context),
        "help" => show_dynamic_memory_help(context),
        _ => {
            context
                .uart
                .puts("Unknown dynamic memory command. Use 'help' for available commands.\r\n");
        }
    }
}

/// Show dynamic memory help
fn show_dynamic_memory_help(context: &mut ShellContext) {
    context.uart.puts("Dynamic Memory Management Commands:\r\n");
    context
        .uart
        .puts("  status          - Show dynamic memory system status\r\n");
    context
        .uart
        .puts("  growth          - Manage dynamic stack growth\r\n");
    context
        .uart
        .puts("  lazy            - Lazy page allocation management\r\n");
    context
        .uart
        .puts("  pressure        - Memory pressure monitoring\r\n");
    context
        .uart
        .puts("  optimize        - Memory optimization controls\r\n");
    context
        .uart
        .puts("  context         - Hardware-assisted context switching\r\n");
    context
        .uart
        .puts("  stats           - Show detailed statistics\r\n");
    context.uart.puts("  help            - Show this help\r\n");
}

/// Show dynamic memory status
pub fn cmd_dynamic_memory_status(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("Dynamic Memory Management Status:\r\n");
    context.uart.puts("================================\r\n");

    // Check if dynamic memory management is enabled
    if is_dynamic_memory_enabled() {
        context.uart.puts("Status: ENABLED\r\n");
    } else {
        context.uart.puts("Status: DISABLED\r\n");
        return;
    }

    // Get and display statistics
    match get_dynamic_memory_stats() {
        Ok(stats) => {
            context.uart.puts("Active Dynamic Stacks: ");
            context.uart.put_hex(stats.active_dynamic_stacks.into());
            context.uart.puts("\r\n");

            context.uart.puts("Total Lazy Pages: ");
            context.uart.put_hex(stats.total_lazy_pages.into());
            context.uart.puts("\r\n");

            context.uart.puts("Allocated Lazy Pages: ");
            context.uart.put_hex(stats.allocated_lazy_pages.into());
            context.uart.puts("\r\n");

            context.uart.puts("Context Switches: ");
            context.uart.put_hex(stats.context_switch_count.into());
            context.uart.puts("\r\n");
        }
        Err(e) => {
            context.uart.puts("Error getting statistics: ");
            context.uart.puts(e);
            context.uart.puts("\r\n");
        }
    }
}

/// Manage dynamic stack growth
pub fn cmd_dynamic_memory_growth(args: &[&str], context: &mut ShellContext) {
    context.uart.puts("Dynamic Stack Growth Management:\r\n");
    context.uart.puts("===============================\r\n");

    if args.len() < 2 {
        context.uart.puts("Usage: growth <create|status>\r\n");
        context
            .uart
            .puts("  create <base> <initial> <max> - Create dynamic stack\r\n");
        context
            .uart
            .puts("  status                        - Show stack status\r\n");
        return;
    }

    match args[1] {
        "create" => {
            if args.len() < 5 {
                context
                    .uart
                    .puts("Usage: growth create <base_addr> <initial_size> <max_size>\r\n");
                return;
            }

            // Parse addresses (simplified for demo)
            let base_addr = 0x100000; // Use fixed address for demo
            let initial_size = 4096; // 4KB initial
            let max_size = 65536; // 64KB max

            match create_dynamic_stack(base_addr, initial_size, max_size) {
                Ok(stack_id) => {
                    context.uart.puts("Dynamic stack created with ID: ");
                    context.uart.put_hex(stack_id.into());
                    context.uart.puts("\r\n");
                }
                Err(e) => {
                    context.uart.puts("Error creating dynamic stack: ");
                    context.uart.puts(e);
                    context.uart.puts("\r\n");
                }
            }
        }
        "status" => match get_dynamic_memory_stats() {
            Ok(stats) => {
                context.uart.puts("Stack Growth Events: ");
                context.uart.put_hex(stats.total_stack_growth_events.into());
                context.uart.puts("\r\n");

                context.uart.puts("Stack Shrink Events: ");
                context.uart.put_hex(stats.total_stack_shrink_events.into());
                context.uart.puts("\r\n");

                context.uart.puts("Total Dynamic Stacks: ");
                context.uart.put_hex(stats.total_dynamic_stacks.into());
                context.uart.puts("\r\n");
            }
            Err(e) => {
                context.uart.puts("Error getting stack statistics: ");
                context.uart.puts(e);
                context.uart.puts("\r\n");
            }
        },
        _ => {
            context
                .uart
                .puts("Unknown growth command. Use 'help' for usage.\r\n");
        }
    }
}

/// Manage lazy page allocation
pub fn cmd_dynamic_memory_lazy(args: &[&str], context: &mut ShellContext) {
    context.uart.puts("Lazy Page Allocation Management:\r\n");
    context.uart.puts("===============================\r\n");

    if args.len() < 2 {
        context.uart.puts("Usage: lazy <add|status>\r\n");
        context.uart.puts("  add <address>  - Add lazy page\r\n");
        context
            .uart
            .puts("  status         - Show lazy page status\r\n");
        return;
    }

    match args[1] {
        "add" => {
            // Use a demo address
            let virtual_address = 0x200000;

            match add_lazy_page(virtual_address) {
                Ok(page_index) => {
                    context.uart.puts("Lazy page added at index: ");
                    context.uart.put_hex((page_index as u32).into());
                    context.uart.puts("\r\n");
                }
                Err(e) => {
                    context.uart.puts("Error adding lazy page: ");
                    context.uart.puts(e);
                    context.uart.puts("\r\n");
                }
            }
        }
        "status" => match get_dynamic_memory_stats() {
            Ok(stats) => {
                context.uart.puts("Total Lazy Pages: ");
                context.uart.put_hex(stats.total_lazy_pages.into());
                context.uart.puts("\r\n");

                context.uart.puts("Allocated Lazy Pages: ");
                context.uart.put_hex(stats.allocated_lazy_pages.into());
                context.uart.puts("\r\n");

                context.uart.puts("Lazy Page Faults: ");
                context.uart.put_hex(stats.total_lazy_page_faults.into());
                context.uart.puts("\r\n");
            }
            Err(e) => {
                context.uart.puts("Error getting lazy page statistics: ");
                context.uart.puts(e);
                context.uart.puts("\r\n");
            }
        },
        _ => {
            context
                .uart
                .puts("Unknown lazy command. Use 'help' for usage.\r\n");
        }
    }
}

/// Monitor memory pressure
pub fn cmd_dynamic_memory_pressure(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("Memory Pressure Monitoring:\r\n");
    context.uart.puts("===========================\r\n");

    // Simulate memory pressure check with current memory usage
    let available_memory = 1024 * 1024 * 8; // 8MB available (demo)

    match check_dynamic_memory_pressure(available_memory) {
        Ok(pressure) => {
            context.uart.puts("Current Pressure Level: ");
            match pressure {
                PressureLevel::Low => context.uart.puts("LOW\r\n"),
                PressureLevel::Medium => context.uart.puts("MEDIUM\r\n"),
                PressureLevel::High => context.uart.puts("HIGH\r\n"),
                PressureLevel::Critical => context.uart.puts("CRITICAL\r\n"),
            }

            context.uart.puts("Available Memory: ");
            context.uart.put_hex((available_memory as u32).into());
            context.uart.puts(" bytes\r\n");
        }
        Err(e) => {
            context.uart.puts("Error checking memory pressure: ");
            context.uart.puts(e);
            context.uart.puts("\r\n");
        }
    }

    // Show pressure statistics
    match get_dynamic_memory_stats() {
        Ok(stats) => {
            context.uart.puts("Pressure Events: ");
            context.uart.put_hex(stats.memory_pressure_events.into());
            context.uart.puts("\r\n");

            context.uart.puts("Optimization Events: ");
            context.uart.put_hex(stats.optimization_events.into());
            context.uart.puts("\r\n");
        }
        Err(e) => {
            context.uart.puts("Error getting pressure statistics: ");
            context.uart.puts(e);
            context.uart.puts("\r\n");
        }
    }
}

/// Memory optimization controls
pub fn cmd_dynamic_memory_optimize(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("Memory Optimization Controls:\r\n");
    context.uart.puts("============================\r\n");

    // Trigger memory pressure check to potentially trigger optimization
    let available_memory = 1024 * 1024; // 1MB to trigger optimization

    match check_dynamic_memory_pressure(available_memory) {
        Ok(pressure) => {
            context
                .uart
                .puts("Triggered optimization with pressure level: ");
            match pressure {
                PressureLevel::Low => context.uart.puts("LOW\r\n"),
                PressureLevel::Medium => context.uart.puts("MEDIUM\r\n"),
                PressureLevel::High => context.uart.puts("HIGH\r\n"),
                PressureLevel::Critical => context.uart.puts("CRITICAL\r\n"),
            }
        }
        Err(e) => {
            context.uart.puts("Error triggering optimization: ");
            context.uart.puts(e);
            context.uart.puts("\r\n");
        }
    }

    // Show optimization statistics
    match get_dynamic_memory_stats() {
        Ok(stats) => {
            context.uart.puts("Total Optimization Events: ");
            context.uart.put_hex(stats.optimization_events.into());
            context.uart.puts("\r\n");
        }
        Err(e) => {
            context.uart.puts("Error getting optimization statistics: ");
            context.uart.puts(e);
            context.uart.puts("\r\n");
        }
    }
}

/// Hardware-assisted context switching
pub fn cmd_dynamic_memory_context(args: &[&str], context: &mut ShellContext) {
    context
        .uart
        .puts("Hardware-Assisted Context Switching:\r\n");
    context.uart.puts("===================================\r\n");

    if args.len() < 2 {
        context.uart.puts("Usage: context <switch|status>\r\n");
        context
            .uart
            .puts("  switch   - Perform demo context switch\r\n");
        context
            .uart
            .puts("  status   - Show context switching status\r\n");
        return;
    }

    match args[1] {
        "switch" => {
            // Perform a demo context switch
            let from_asid = 1;
            let to_asid = 2;

            match fast_context_switch(from_asid, to_asid) {
                Ok(()) => {
                    context.uart.puts("Fast context switch completed\r\n");
                    context.uart.puts("From ASID: ");
                    context.uart.put_hex((from_asid as u32).into());
                    context.uart.puts(" -> To ASID: ");
                    context.uart.put_hex((to_asid as u32).into());
                    context.uart.puts("\r\n");
                }
                Err(e) => {
                    context.uart.puts("Error performing context switch: ");
                    context.uart.puts(e);
                    context.uart.puts("\r\n");
                }
            }
        }
        "status" => match get_dynamic_memory_stats() {
            Ok(stats) => {
                context.uart.puts("Total Context Switches: ");
                context.uart.put_hex(stats.context_switch_count.into());
                context.uart.puts("\r\n");
            }
            Err(e) => {
                context
                    .uart
                    .puts("Error getting context switch statistics: ");
                context.uart.puts(e);
                context.uart.puts("\r\n");
            }
        },
        _ => {
            context
                .uart
                .puts("Unknown context command. Use 'help' for usage.\r\n");
        }
    }
}

/// Show detailed dynamic memory statistics
pub fn cmd_dynamic_memory_stats(_args: &[&str], context: &mut ShellContext) {
    context
        .uart
        .puts("Dynamic Memory Management Statistics:\r\n");
    context
        .uart
        .puts("====================================\r\n");

    if !is_dynamic_memory_enabled() {
        context
            .uart
            .puts("Dynamic memory management is not enabled\r\n");
        return;
    }

    match get_dynamic_memory_stats() {
        Ok(stats) => {
            context.uart.puts("Stack Management:\r\n");
            context.uart.puts("  Total Stacks: ");
            context.uart.put_hex(stats.total_dynamic_stacks.into());
            context.uart.puts("\r\n");

            context.uart.puts("  Active Stacks: ");
            context.uart.put_hex(stats.active_dynamic_stacks.into());
            context.uart.puts("\r\n");

            context.uart.puts("  Growth Events: ");
            context.uart.put_hex(stats.total_stack_growth_events.into());
            context.uart.puts("\r\n");

            context.uart.puts("  Shrink Events: ");
            context.uart.put_hex(stats.total_stack_shrink_events.into());
            context.uart.puts("\r\n");

            context.uart.puts("\r\nLazy Allocation:\r\n");
            context.uart.puts("  Total Pages: ");
            context.uart.put_hex(stats.total_lazy_pages.into());
            context.uart.puts("\r\n");

            context.uart.puts("  Allocated Pages: ");
            context.uart.put_hex(stats.allocated_lazy_pages.into());
            context.uart.puts("\r\n");

            context.uart.puts("  Page Faults: ");
            context.uart.put_hex(stats.total_lazy_page_faults.into());
            context.uart.puts("\r\n");

            context.uart.puts("\r\nPressure & Optimization:\r\n");
            context.uart.puts("  Pressure Events: ");
            context.uart.put_hex(stats.memory_pressure_events.into());
            context.uart.puts("\r\n");

            context.uart.puts("  Optimization Events: ");
            context.uart.put_hex(stats.optimization_events.into());
            context.uart.puts("\r\n");

            context.uart.puts("\r\nContext Switching:\r\n");
            context.uart.puts("  Context Switches: ");
            context.uart.put_hex(stats.context_switch_count.into());
            context.uart.puts("\r\n");
        }
        Err(e) => {
            context
                .uart
                .puts("Error getting dynamic memory statistics: ");
            context.uart.puts(e);
            context.uart.puts("\r\n");
        }
    }
}
