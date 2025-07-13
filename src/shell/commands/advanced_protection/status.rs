use crate::{
    memory::protection::{get_advanced_protection_stats, get_aslr_offset},
    shell::core::ShellContext,
};

/// Show advanced protection status
pub fn cmd_advanced_protection_status(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("Advanced Memory Protection Status:\r\n");
    context.uart.puts("==================================\r\n");

    let stats = get_advanced_protection_stats();
    context.uart.puts("Page Permissions:\r\n");
    context.uart.puts("  Total protected pages: ");
    context.uart.put_hex(stats.total_protected_pages as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Read-only pages: ");
    context.uart.put_hex(stats.read_only_pages as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Non-executable pages: ");
    context.uart.put_hex(stats.non_executable_pages as u64);
    context.uart.puts("\r\n");

    context.uart.puts("Stack Protection:\r\n");
    context.uart.puts("  Protected stacks: ");
    context.uart.put_hex(stats.protected_stacks as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Stack canaries active: ");
    context.uart.put_hex(stats.stack_canaries_active as u64);
    context.uart.puts("\r\n");

    context.uart.puts("ASLR:\r\n");
    context.uart.puts("  Current ASLR offset: 0x");
    context.uart.put_hex(get_aslr_offset());
    context.uart.puts("\r\n");

    context.uart.puts("Fault Handling:\r\n");
    context.uart.puts("  Permission faults: ");
    context.uart.put_hex(stats.permission_faults as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Stack violations: ");
    context.uart.put_hex(stats.stack_violations as u64);
    context.uart.puts("\r\n");
}
