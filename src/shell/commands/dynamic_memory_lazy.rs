use crate::memory::dynamic::{add_lazy_page, get_dynamic_memory_stats};
use crate::shell::core::ShellContext;

/// Manage lazy page allocation and monitoring
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
