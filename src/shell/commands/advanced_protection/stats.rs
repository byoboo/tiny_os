use crate::{memory::protection::get_advanced_protection_stats, shell::core::ShellContext};

/// Show advanced protection statistics
pub fn cmd_advanced_protection_stats(_args: &[&str], context: &mut ShellContext) {
    let stats = get_advanced_protection_stats();

    context
        .uart
        .puts("Advanced Memory Protection Statistics:\r\n");
    context
        .uart
        .puts("=====================================\r\n");

    context.uart.puts("Page Protection:\r\n");
    context.uart.puts("  Total protected pages: ");
    context.uart.put_hex(stats.total_protected_pages as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Read-only pages: ");
    context.uart.put_hex(stats.read_only_pages as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Non-executable pages: ");
    context.uart.put_hex(stats.non_executable_pages as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  User-protected pages: ");
    context.uart.put_hex(stats.user_protected_pages as u64);
    context.uart.puts("\r\n");

    context.uart.puts("Stack Protection:\r\n");
    context.uart.puts("  Protected stacks: ");
    context.uart.put_hex(stats.protected_stacks as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Stack canaries active: ");
    context.uart.put_hex(stats.stack_canaries_active as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Stack violations: ");
    context.uart.put_hex(stats.stack_violations as u64);
    context.uart.puts("\r\n");

    context.uart.puts("Control Flow Integrity:\r\n");
    context.uart.puts("  CFI violations: ");
    context.uart.put_hex(stats.cfi_violations as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  ROP attacks blocked: ");
    context.uart.put_hex(stats.rop_attacks_blocked as u64);
    context.uart.puts("\r\n");

    context.uart.puts("Fault Handling:\r\n");
    context.uart.puts("  Permission faults: ");
    context.uart.put_hex(stats.permission_faults as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Faults handled: ");
    context.uart.put_hex(stats.faults_handled as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Faults terminated: ");
    context.uart.put_hex(stats.faults_terminated as u64);
    context.uart.puts("\r\n");
}
