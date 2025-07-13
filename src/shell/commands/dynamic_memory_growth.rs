use crate::{
    memory::dynamic::{create_dynamic_stack, get_dynamic_memory_stats},
    shell::core::ShellContext,
};

/// Manage dynamic stack growth operations
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
