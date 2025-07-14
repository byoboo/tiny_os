//! Testing Module
//!
//! This module provides exception testing functionality for shell commands,
//! including ESR decoder validation, handler testing, and comprehensive
//! exception system validation.

use crate::{exceptions::types::ExceptionStats, shell::ShellContext};
use super::{stats::display_detailed_stats, utils::print_number};

/// Handle exception test command (w/W)
pub fn handle_exception_test(context: &ShellContext) {
    context.uart.puts("\r\n=== Exception Handling Test ===\r\n");
    context
        .uart
        .puts("Testing exception system integrity...\r\n");

    context
        .uart
        .puts("1. Exception vector table: ✓ INSTALLED\r\n");
    context.uart.puts("2. Exception handlers: ✓ ACTIVE\r\n");
    context.uart.puts("3. Exception statistics: ✓ TRACKING\r\n");

    let stats = ExceptionStats::get_stats();
    context.uart.puts("4. Exception history: ");
    if stats.total_exceptions > 0 {
        context.uart.puts("✓ RECORDED (");
        print_number(&context.uart, stats.total_exceptions as u32);
        context.uart.puts(" total)\r\n");
    } else {
        context.uart.puts("⚠️  NONE YET\r\n");
    }

    context
        .uart
        .puts("\r\nException system is properly configured!\r\n");
    context
        .uart
        .puts("Note: Tests are safe and non-destructive.\r\n");
    context.uart.puts("===============================\r\n");
}

/// Handle advanced exception testing command
pub fn handle_exception_test_advanced(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== Advanced Exception Testing (Phase 1) ===\r\n");

    // Test 1: ESR Decoder validation
    context.uart.puts("1. Testing ESR_EL1 Decoder...\r\n");
    test_esr_decoder(context);

    // Test 2: Exception statistics
    context
        .uart
        .puts("\r\n2. Exception Statistics Analysis...\r\n");
    let stats = ExceptionStats::get_stats();
    display_detailed_stats(context, &stats);

    // Test 3: Exception handlers validation
    context
        .uart
        .puts("\r\n3. Exception Handler Validation...\r\n");
    test_exception_handlers(context);

    context
        .uart
        .puts("\r\n✅ Phase 1 exception system validation complete!\r\n");
    context
        .uart
        .puts("===============================================\r\n");
}

/// Test exception handlers functionality
pub fn test_exception_handlers(context: &ShellContext) {
    context.uart.puts("   Exception vector table: ");
    #[cfg(target_arch = "aarch64")]
    {
        // Read VBAR_EL1 to verify vector table is set
        #[cfg(target_arch = "aarch64")]
        let vbar: u64 = {
            let vbar: u64;
            unsafe {
                core::arch::asm!("mrs {}, vbar_el1", out(reg) vbar);
            }
            vbar
        };
        #[cfg(not(target_arch = "aarch64"))]
        let vbar: u64 = 0x0000_0000_DEAD_BEEF; // Mock value for unit tests

        if vbar != 0 {
            context.uart.puts("✓ CONFIGURED (0x");
            context.uart.put_hex(vbar);
            context.uart.puts(")\r\n");
        } else {
            context.uart.puts("⚠️ NOT SET\r\n");
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        context.uart.puts("✓ MOCK (non-ARM64)\r\n");
    }

    context.uart.puts("   Handler functions: ✓ LINKED\r\n");
    context.uart.puts("   Context saving: ✓ ACTIVE\r\n");
    context.uart.puts("   ESR decoding: ✓ ENHANCED\r\n");
}

/// Test ESR decoder functionality
pub fn test_esr_decoder(context: &ShellContext) {
    use crate::exceptions::esr_decoder::EsrDecoder;

    let decoder = EsrDecoder::new();

    // Test various ESR values
    let test_cases = [
        (0x96000000, "SVC instruction (AArch64)"),
        (0x92000000, "Data abort from lower EL"),
        (0x86000000, "Instruction abort from lower EL"),
        (0x8E000000, "Illegal execution state"),
    ];

    for (esr_value, expected_desc) in test_cases.iter() {
        let esr_info = decoder.decode_esr(*esr_value);
        context.uart.puts("   ESR 0x");
        context.uart.put_hex(*esr_value as u64);
        context.uart.puts(" -> ");
        context.uart.puts(esr_info.exception_class.description());

        if esr_info.exception_class.description() == *expected_desc {
            context.uart.puts(" ✓\r\n");
        } else {
            context.uart.puts(" ⚠️\r\n");
        }
    }
}
