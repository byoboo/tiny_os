use crate::memory::dynamic::{fast_context_switch, get_dynamic_memory_stats, is_dynamic_memory_enabled};
use crate::shell::core::ShellContext;

/// Hardware-assisted context switching and comprehensive statistics
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
