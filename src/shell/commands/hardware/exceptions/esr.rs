//! ESR (Exception Syndrome Register) Module
//!
//! This module provides ESR decoder testing and analysis functionality
//! for shell commands, including comprehensive ESR value testing.

use crate::shell::ShellContext;

/// Handle ESR decoder test command
pub fn handle_esr_test(context: &ShellContext) {
    context.uart.puts("\r\n=== ESR_EL1 Decoder Test ===\r\n");
    use crate::exceptions::esr_decoder::EsrDecoder;
    let decoder = EsrDecoder::new();

    context.uart.puts("Testing exception class decoding:\r\n");

    let test_values = [
        (0x96000000, "SVC64"),
        (0x92000000, "DataAbortLower"),
        (0x96000001, "SVC64 with immediate"),
        (0x86000000, "InstructionAbortLower"),
        (0x8E000000, "IllegalExecution"),
        (0xBE000000, "SError"),
    ];

    for (esr, _description) in test_values.iter() {
        let info = decoder.decode_esr(*esr);
        context.uart.puts("  ESR: 0x");
        context.uart.put_hex(*esr as u64);
        context.uart.puts("\r\n    Class: ");
        context.uart.puts(info.exception_class.description());
        context.uart.puts("\r\n    ISS: 0x");
        context.uart.put_hex(info.iss as u64);
        context.uart.puts("\r\n    IL: ");
        context.uart.puts(if info.instruction_length {
            "32-bit"
        } else {
            "16-bit"
        });
        context.uart.puts("\r\n\r\n");
    }

    context.uart.puts("ESR decoder test complete!\r\n");
    context.uart.puts("============================\r\n");
}
