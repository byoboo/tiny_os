use crate::{
    memory::dynamic::{check_dynamic_memory_pressure, get_dynamic_memory_stats, PressureLevel},
    shell::core::ShellContext,
};

/// Monitor memory pressure and handle optimization
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
