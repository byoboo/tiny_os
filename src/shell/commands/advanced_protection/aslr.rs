use crate::{memory::protection::get_aslr_offset, shell::core::ShellContext};

/// Handle ASLR commands
pub fn cmd_advanced_protection_aslr(_args: &[&str], context: &mut ShellContext) {
    context
        .uart
        .puts("Address Space Layout Randomization (ASLR):\r\n");
    context
        .uart
        .puts("=========================================\r\n");

    let offset = get_aslr_offset();
    context.uart.puts("Current ASLR offset: 0x");
    context.uart.put_hex(offset);
    context.uart.puts("\r\n");

    context.uart.puts("ASLR provides randomization of:\r\n");
    context.uart.puts("  - Process base addresses\r\n");
    context.uart.puts("  - Stack locations\r\n");
    context.uart.puts("  - Heap placement\r\n");
    context.uart.puts("  - Library loading addresses\r\n");
}
