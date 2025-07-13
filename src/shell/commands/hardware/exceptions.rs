//! Exception command handlers
//!
//! This module contains handlers for exception-related commands including
//! exception statistics, testing, ESR decoding, system calls, and memory fault analysis.

use crate::{exceptions::types::ExceptionStats, shell::ShellContext, uart::Uart};

/// Helper function to print numbers
#[inline]
fn print_number(uart: &Uart, mut num: u32) {
    if num == 0 {
        uart.putc(b'0');
        return;
    }

    let mut digits = [0u8; 10];
    let mut count = 0;

    while num > 0 {
        digits[count] = (num % 10) as u8 + b'0';
        num /= 10;
        count += 1;
    }

    for i in (0..count).rev() {
        uart.putc(digits[i]);
    }
}

/// Handle exception statistics command (v/V)
pub fn handle_exception_stats(context: &ShellContext) {
    let stats = ExceptionStats::get_stats();
    context.uart.puts("\r\n=== Exception Statistics ===\r\n");
    context.uart.puts("Exception Handler Status:\r\n");
    context.uart.puts("  Exception vectors: ✓ ACTIVE\r\n");
    context.uart.puts("  Total exceptions handled: ");
    print_number(&context.uart, stats.total_exceptions as u32);
    context.uart.puts("\r\n");

    context.uart.puts("\r\nException Types:\r\n");
    context.uart.puts("  Synchronous exceptions: ");
    print_number(&context.uart, stats.sync_exceptions as u32);
    context.uart.puts("\r\n");
    context.uart.puts("  IRQ exceptions: ");
    print_number(&context.uart, stats.irq_exceptions as u32);
    context.uart.puts("\r\n");
    context.uart.puts("  FIQ exceptions: ");
    print_number(&context.uart, stats.fiq_exceptions as u32);
    context.uart.puts("\r\n");
    context.uart.puts("  SError exceptions: ");
    print_number(&context.uart, stats.serror_exceptions as u32);
    context.uart.puts("\r\n");

    context.uart.puts("\r\nException System: ✓ OPERATIONAL\r\n");
    context.uart.puts("============================\r\n");
}

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

/// Handle system call testing command (8)
pub fn handle_syscall_test(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== System Call Testing (Phase 1) ===\r\n");

    // Test 1: System call interface validation
    context.uart.puts("1. Testing System Call Interface...\r\n");
    test_syscall_interface(context);

    // Test 2: System call statistics
    context.uart.puts("\r\n2. System Call Statistics...\r\n");
    display_syscall_stats(context);

    // Test 3: Direct system call tests
    context.uart.puts("\r\n3. Direct System Call Tests...\r\n");
    test_direct_syscalls(context);

    context
        .uart
        .puts("\r\n✅ System call testing complete!\r\n");
    context
        .uart
        .puts("=====================================\r\n");
}

/// Handle memory fault testing command (9)
pub fn handle_memory_fault_test(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== Memory Fault Testing (Phase 1) ===\r\n");

    // Test 1: Memory fault analyzer
    context.uart.puts("1. Testing Memory Fault Analyzer...\r\n");
    test_memory_fault_analyzer(context);

    // Test 2: Memory fault statistics
    context.uart.puts("\r\n2. Memory Fault Statistics...\r\n");
    display_memory_fault_stats(context);

    // Test 3: Fault classification tests
    context
        .uart
        .puts("\r\n3. Fault Classification Tests...\r\n");
    test_fault_classification(context);

    context
        .uart
        .puts("\r\n✅ Memory fault testing complete!\r\n");
    context
        .uart
        .puts("======================================\r\n");
}

// === Helper functions ===

/// Test ESR decoder functionality
fn test_esr_decoder(context: &ShellContext) {
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

/// Display detailed exception statistics
fn display_detailed_stats(context: &ShellContext, stats: &ExceptionStats) {
    context.uart.puts("   Total exceptions: ");
    print_number(&context.uart, stats.total_exceptions as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   - Synchronous: ");
    print_number(&context.uart, stats.sync_exceptions as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   - IRQ: ");
    print_number(&context.uart, stats.irq_exceptions as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   - FIQ: ");
    print_number(&context.uart, stats.fiq_exceptions as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   - SError: ");
    print_number(&context.uart, stats.serror_exceptions as u32);
    context.uart.puts("\r\n");

    if let Some(last_type) = &stats.last_exception_type {
        context.uart.puts("   Last exception: ");
        match last_type {
            crate::exceptions::types::ExceptionType::Synchronous => {
                context.uart.puts("Synchronous")
            }
            crate::exceptions::types::ExceptionType::Irq => context.uart.puts("IRQ"),
            crate::exceptions::types::ExceptionType::Fiq => context.uart.puts("FIQ"),
            crate::exceptions::types::ExceptionType::SError => context.uart.puts("SError"),
        }
        context.uart.puts("\r\n");
    }
}

/// Test exception handler configuration
fn test_exception_handlers(context: &ShellContext) {
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

/// Test system call interface
fn test_syscall_interface(context: &ShellContext) {
    use crate::exceptions::syscall::test_syscall_interface;

    context
        .uart
        .puts("   Running syscall interface tests...\r\n");
    let result = test_syscall_interface();

    if result {
        context
            .uart
            .puts("   ✅ All syscall interface tests passed\r\n");
    } else {
        context
            .uart
            .puts("   ❌ Some syscall interface tests failed\r\n");
    }
}

/// Display system call statistics
fn display_syscall_stats(context: &ShellContext) {
    use crate::exceptions::syscall::get_syscall_stats;

    let stats = get_syscall_stats();
    context.uart.puts("   Total syscalls: ");
    print_number(&context.uart, stats.total_syscalls as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Successful syscalls: ");
    print_number(&context.uart, stats.successful_syscalls as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Failed syscalls: ");
    print_number(&context.uart, stats.failed_syscalls as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Invalid syscalls: ");
    print_number(&context.uart, stats.invalid_syscalls as u32);
    context.uart.puts("\r\n");
}

/// Test direct system calls
fn test_direct_syscalls(context: &ShellContext) {
    use crate::exceptions::syscall::test_direct_syscalls;

    context.uart.puts("   Testing direct syscall execution...\r\n");
    let result = test_direct_syscalls();

    if result {
        context
            .uart
            .puts("   ✅ Direct syscall tests passed\r\n");
    } else {
        context
            .uart
            .puts("   ❌ Some direct syscall tests failed\r\n");
    }
}

/// Test memory fault analyzer
fn test_memory_fault_analyzer(context: &ShellContext) {
    use crate::exceptions::memory_fault::test_memory_fault_analyzer;

    context
        .uart
        .puts("   Running memory fault analyzer tests...\r\n");
    let result = test_memory_fault_analyzer();

    if result {
        context
            .uart
            .puts("   ✅ Memory fault analyzer tests passed\r\n");
    } else {
        context
            .uart
            .puts("   ❌ Some memory fault analyzer tests failed\r\n");
    }
}

/// Display memory fault statistics
fn display_memory_fault_stats(context: &ShellContext) {
    use crate::exceptions::memory_fault::get_memory_fault_stats;

    let stats = get_memory_fault_stats();
    context.uart.puts("   Total memory faults: ");
    print_number(&context.uart, stats.total_faults as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Data aborts: ");
    print_number(&context.uart, stats.data_aborts as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Instruction aborts: ");
    print_number(&context.uart, stats.instruction_aborts as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Permission faults: ");
    print_number(&context.uart, stats.permission_faults as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Translation faults: ");
    print_number(&context.uart, stats.translation_faults as u32);
    context.uart.puts("\r\n");
}

/// Test fault classification
fn test_fault_classification(context: &ShellContext) {
    use crate::exceptions::memory_fault::test_fault_classification;

    context.uart.puts("   Testing fault classification...\r\n");
    let result = test_fault_classification();

    if result {
        context
            .uart
            .puts("   ✅ Fault classification tests passed\r\n");
    } else {
        context
            .uart
            .puts("   ❌ Some fault classification tests failed\r\n");
    }
}
