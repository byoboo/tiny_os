use crate::{
    memory::dynamic::{get_dynamic_memory_stats, is_dynamic_memory_enabled},
    shell::core::ShellContext,
};

/// Detailed dynamic memory status and basic statistics
pub fn cmd_dynamic_memory_status(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("Dynamic Memory System Status:\r\n");
    context.uart.puts("============================\r\n");

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
