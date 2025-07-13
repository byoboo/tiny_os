// TinyOS Shell Statistics Commands
// Focused module for process and system statistics

use core::sync::atomic::Ordering;
use crate::{process, shell::ShellContext};

/// Handle process statistics display
pub fn handle_process_stats(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== Process Management Statistics ===\r\n");

    let stats = process::get_process_stats();

    context.uart.puts("Context Switches: ");
    context
        .uart
        .put_hex(stats.context_switches.load(Ordering::SeqCst));
    context.uart.puts("\r\n");

    context.uart.puts("Privilege Escalations: ");
    context
        .uart
        .put_hex(stats.privilege_escalations.load(Ordering::SeqCst));
    context.uart.puts("\r\n");

    context.uart.puts("Privilege Violations: ");
    context
        .uart
        .put_hex(stats.privilege_violations.load(Ordering::SeqCst));
    context.uart.puts("\r\n");

    context.uart.puts("Tasks Created: ");
    context
        .uart
        .put_hex(stats.tasks_created.load(Ordering::SeqCst));
    context.uart.puts("\r\n");

    context.uart.puts("Tasks Destroyed: ");
    context
        .uart
        .put_hex(stats.tasks_destroyed.load(Ordering::SeqCst));
    context.uart.puts("\r\n");

    context.uart.puts("Scheduler Preemptions: ");
    context
        .uart
        .put_hex(stats.scheduler_preemptions.load(Ordering::SeqCst));
    context.uart.puts("\r\n");
}
