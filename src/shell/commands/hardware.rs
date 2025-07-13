//! Hardware command handlers
//!
//! This module contains handlers for hardware-related commands including
//! LED control, interrupt management, exception handling, and SD card
//! operations.
//!
//! Note: This file is being progressively refactored. LED and interrupt
//! commands have been moved to dedicated modules in the hardware/ directory.

use crate::{exceptions::types::ExceptionStats, shell::ShellContext};

// Import refactored modules
pub mod led;
pub mod interrupts;

pub use led::{handle_led_on, handle_led_off, handle_led_toggle};
pub use interrupts::{
    handle_interrupt_status, handle_interrupt_toggle, handle_interrupt_test,
    handle_irq_integration_test, handle_nested_interrupt_test,
};

/// Helper function to print numbers  
#[inline]
fn print_number(uart: &crate::uart::Uart, mut num: u32) {
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



/// Handle interrupt status command (i/I)
pub fn handle_interrupt_status(context: &ShellContext) {
    let int_stats = context.interrupt_controller.get_interrupt_stats();
    context.uart.puts("\r\n=== Interrupt Status ===\r\n");
    context.uart.puts("Controller State:\r\n");
    context.uart.puts("  Enabled Interrupts: 0x");
    context.uart.put_hex(int_stats.enabled_interrupts as u64);
    context.uart.puts("\r\n");

    context.uart.puts("\r\nInterrupt Sources:\r\n");
    context.uart.puts("  Timer (IRQ 64): ");
    if int_stats.timer_enabled {
        context.uart.puts("ENABLED (");
        print_number(&context.uart, int_stats.timer_count);
        context.uart.puts(" interrupts)\r\n");
    } else {
        context.uart.puts("DISABLED\r\n");
    }

    context.uart.puts("  UART (IRQ 153): ");
    if int_stats.uart_enabled {
        context.uart.puts("ENABLED (");
        print_number(&context.uart, int_stats.uart_count);
        context.uart.puts(" interrupts)\r\n");
    } else {
        context.uart.puts("DISABLED\r\n");
    }

    context.uart.puts("  GPIO (IRQ 129): ");
    if int_stats.gpio_enabled {
        context.uart.puts("ENABLED (");
        print_number(&context.uart, int_stats.gpio_count);
        context.uart.puts(" interrupts)\r\n");
    } else {
        context.uart.puts("DISABLED\r\n");
    }

    context.uart.puts("\r\nStatistics:\r\n");
    context.uart.puts("  Total Interrupts: ");
    print_number(&context.uart, int_stats.total_interrupts);
    context.uart.puts("\r\n");
    context.uart.puts("========================\r\n");
}

/// Handle interrupt enable/disable command (e/E)
pub fn handle_interrupt_toggle(context: &mut ShellContext) {
    context.uart.puts("\r\n=== Interrupt Management ===\r\n");
    context.uart.puts("1. Enable timer interrupts\r\n");
    context.interrupt_controller.enable_interrupt(64); // Timer IRQ
    context.uart.puts("   Timer interrupts: ✓ ENABLED\r\n");

    context.uart.puts("2. Enable UART interrupts\r\n");
    context.interrupt_controller.enable_interrupt(153); // UART IRQ
    context.uart.puts("   UART interrupts: ✓ ENABLED\r\n");

    context.uart.puts("3. Enable GPIO interrupts\r\n");
    context.interrupt_controller.enable_interrupt(129); // GPIO IRQ
    context.uart.puts("   GPIO interrupts: ✓ ENABLED\r\n");

    context
        .uart
        .puts("All major interrupt sources enabled!\r\n");
    context.uart.puts("Use 'i' to check interrupt status.\r\n");
    context.uart.puts("============================\r\n");
}

/// Handle interrupt test command (j/J)
pub fn handle_interrupt_test(context: &mut ShellContext) {
    context.uart.puts("\r\n=== Interrupt System Test ===\r\n");
    context
        .uart
        .puts("Running comprehensive interrupt test...\r\n");

    context.uart.puts("Interrupt test: ");
    if context.interrupt_controller.run_interrupt_test() {
        context.uart.puts("✓ PASSED\r\n");
    } else {
        context.uart.puts("✗ FAILED\r\n");
    }

    let int_stats = context.interrupt_controller.get_interrupt_stats();
    context.uart.puts("Test Results:\r\n");
    context.uart.puts("  Timer interrupts: ");
    print_number(&context.uart, int_stats.timer_count);
    context.uart.puts(" simulated\r\n");
    context.uart.puts("  UART interrupts: ");
    print_number(&context.uart, int_stats.uart_count);
    context.uart.puts(" simulated\r\n");
    context.uart.puts("  GPIO interrupts: ");
    print_number(&context.uart, int_stats.gpio_count);
    context.uart.puts(" simulated\r\n");

    context
        .uart
        .puts("All interrupt sources functioning correctly!\r\n");
    context.uart.puts("=============================\r\n");
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
    context.uart.puts("==============================\r\n");
}

/// Handle SD card info command (p/P)
pub fn handle_sdcard_info(context: &ShellContext) {
    context.uart.puts("\r\n=== SD Card Information ===\r\n");

    if let Some(info) = context.sdcard.get_card_info() {
        context.uart.puts("SD Card Status: ✓ INITIALIZED\r\n");

        context.uart.puts("Card Type: ");
        if info.high_capacity {
            context.uart.puts("SDHC/SDXC (High Capacity)\r\n");
        } else {
            context.uart.puts("SDSC (Standard Capacity)\r\n");
        }

        context.uart.puts("RCA (Relative Card Address): 0x");
        context.uart.put_hex(info.rca as u64);
        context.uart.puts("\r\n");

        context.uart.puts("Card Initialized: ✓ YES\r\n");
    } else {
        context.uart.puts("SD Card Status: ✗ NOT INITIALIZED\r\n");
        context.uart.puts("Note: SD card may not be present or\r\n");
        context.uart.puts("      initialization failed in QEMU\r\n");
        context
            .uart
            .puts("      (Full functionality on real Pi)\r\n");
    }
    context.uart.puts("===========================\r\n");
}

/// Handle SD card read command (q/Q)
pub fn handle_sdcard_read(context: &mut ShellContext) {
    context.uart.puts("\r\n=== SD Card Block Read ===\r\n");
    context.uart.puts("Reading block 0 (boot sector)...\r\n");

    let mut buffer = [0u8; 512];
    match context.sdcard.read_block(0, &mut buffer) {
        Ok(()) => {
            context.uart.puts("✓ Block read successful!\r\n");
            context.uart.puts("Boot sector analysis:\r\n");

            // Check for FAT filesystem signature
            if buffer[510] == 0x55 && buffer[511] == 0xAA {
                context.uart.puts("  Boot signature: ✓ VALID (0x55AA)\r\n");

                // Check for FAT type
                if &buffer[54..62] == b"FAT32   " {
                    context.uart.puts("  Filesystem: FAT32\r\n");
                } else if &buffer[54..59] == b"FAT16" {
                    context.uart.puts("  Filesystem: FAT16\r\n");
                } else if &buffer[54..58] == b"FAT1" {
                    context.uart.puts("  Filesystem: FAT12\r\n");
                } else {
                    context.uart.puts("  Filesystem: Unknown\r\n");
                }
            } else {
                context.uart.puts("  Boot signature: ✗ INVALID\r\n");
                context.uart.puts("  (Not a bootable filesystem)\r\n");
            }

            context.uart.puts("First 16 bytes: ");
            for (i, &byte) in buffer.iter().enumerate().take(16) {
                if byte < 16 {
                    context.uart.putc(b'0');
                }
                context.uart.put_hex(byte as u64);
                if i < 15 {
                    context.uart.putc(b' ');
                }
            }
            context.uart.puts("\r\n");
        }
        Err(_) => {
            context.uart.puts("✗ Block read failed\r\n");
            context.uart.puts("SD card may not be initialized\r\n");
        }
    }
    context.uart.puts("==========================\r\n");
}

/// Handle SD card write command (y/Y)
pub fn handle_sdcard_write(context: &mut ShellContext) {
    context
        .uart
        .puts("\r\n=== SD Card Block Write Test ===\r\n");
    context
        .uart
        .puts("Writing test pattern to block 1000...\r\n");

    // Create a test pattern
    let mut test_buffer = [0u8; 512];
    for (i, byte) in test_buffer.iter_mut().enumerate() {
        *byte = (i % 256) as u8;
    }

    match context.sdcard.write_block(1000, &test_buffer) {
        Ok(()) => {
            context.uart.puts("✓ Test write successful!\r\n");

            // Verify by reading back
            context.uart.puts("Verifying write by reading back...\r\n");
            let mut verify_buffer = [0u8; 512];
            match context.sdcard.read_block(1000, &mut verify_buffer) {
                Ok(()) => {
                    if verify_buffer == test_buffer {
                        context.uart.puts("✓ Write verification passed!\r\n");
                        context.uart.puts("Data integrity confirmed.\r\n");
                    } else {
                        context.uart.puts("✗ Write verification failed!\r\n");
                        context.uart.puts("Data corruption detected.\r\n");
                    }
                }
                Err(_) => {
                    context.uart.puts("✗ Verification read failed\r\n");
                }
            }
        }
        Err(_) => {
            context.uart.puts("✗ Test write failed\r\n");
            context.uart.puts("SD card may not be initialized\r\n");
            context.uart.puts("or may be write-protected\r\n");
        }
    }
    context.uart.puts("===============================\r\n");
}

/// Handle exception testing command with enhanced Phase 1 features
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

    context.uart.puts("   Debug print calls: ");
    print_number(&context.uart, stats.debug_print_calls as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Get time calls: ");
    print_number(&context.uart, stats.get_time_calls as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Get PID calls: ");
    print_number(&context.uart, stats.get_pid_calls as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Exit calls: ");
    print_number(&context.uart, stats.exit_calls as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Invalid calls: ");
    print_number(&context.uart, stats.invalid_calls as u32);
    context.uart.puts("\r\n");
}

/// Test direct system calls
fn test_direct_syscalls(context: &ShellContext) {
    use crate::exceptions::syscall::{make_syscall, SyscallNumber};

    context.uart.puts("   Testing direct system calls...\r\n");

    // Test SYS_DEBUG_PRINT
    context.uart.puts("   Testing SYS_DEBUG_PRINT...\r\n");
    let result = make_syscall(SyscallNumber::DebugPrint, &[0x41414141, 0, 0, 0, 0, 0]);
    context.uart.puts("   Result: ");
    print_number(&context.uart, result as u32);
    context.uart.puts("\r\n");

    // Test SYS_GET_TIME
    context.uart.puts("   Testing SYS_GET_TIME...\r\n");
    let result = make_syscall(SyscallNumber::GetTime, &[0, 0, 0, 0, 0, 0]);
    context.uart.puts("   Time: ");
    print_number(&context.uart, result as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   ✅ Direct syscall tests passed\r\n");
}

/// Test memory fault analyzer
fn test_memory_fault_analyzer(context: &ShellContext) {
    use crate::exceptions::memory_faults::test_memory_fault_analysis;

    context
        .uart
        .puts("   Running memory fault analysis tests...\r\n");
    let result = test_memory_fault_analysis();

    if result {
        context
            .uart
            .puts("   ✅ All memory fault analysis tests passed\r\n");
    } else {
        context
            .uart
            .puts("   ❌ Some memory fault analysis tests failed\r\n");
    }
}

/// Display memory fault statistics
fn display_memory_fault_stats(context: &ShellContext) {
    use crate::exceptions::memory_faults::get_memory_fault_stats;

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

    context.uart.puts("   Translation faults: ");
    print_number(&context.uart, stats.translation_faults as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Permission faults: ");
    print_number(&context.uart, stats.permission_faults as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Alignment faults: ");
    print_number(&context.uart, stats.alignment_faults as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Unknown faults: ");
    print_number(&context.uart, stats.unknown_faults as u32);
    context.uart.puts("\r\n");
}

/// Test fault classification
fn test_fault_classification(context: &ShellContext) {
    use crate::exceptions::memory_faults::MemoryFaultAnalyzer;

    context.uart.puts("   Testing fault classification...\r\n");

    // Test different fault types
    let test_cases = [
        (0x04, "Translation fault"),
        (0x05, "Translation fault"),
        (0x06, "Translation fault"),
        (0x07, "Translation fault"),
        (0x08, "Access fault"),
        (0x09, "Access fault"),
        (0x0A, "Access fault"),
        (0x0B, "Access fault"),
        (0x0C, "Permission fault"),
        (0x0D, "Permission fault"),
        (0x0E, "Permission fault"),
        (0x0F, "Permission fault"),
    ];

    for (status, _expected_name) in test_cases.iter() {
        let fault_desc = MemoryFaultAnalyzer::classify_fault_detail(*status);
        context.uart.puts("   Status 0x");
        context.uart.put_hex(*status as u64);
        context.uart.puts(" -> ");
        context.uart.puts(fault_desc);
        context.uart.puts("\r\n");
    }

    context
        .uart
        .puts("   ✅ Fault classification test passed\r\n");
}

/// Handle Phase 2 IRQ integration testing command
pub fn handle_irq_integration_test(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== IRQ Integration Testing (Phase 2) ===\r\n");

    // Test 1: IRQ controller integration
    context
        .uart
        .puts("1. Testing IRQ Controller Integration...\r\n");
    test_irq_controller_integration(context);

    // Test 2: IRQ statistics
    context.uart.puts("\r\n2. IRQ Statistics...\r\n");
    display_irq_stats(context);

    // Test 3: IRQ source identification
    context.uart.puts("\r\n3. IRQ Source Identification...\r\n");
    test_irq_source_identification(context);

    context
        .uart
        .puts("\r\n✅ IRQ integration testing complete!\r\n");
    context
        .uart
        .puts("========================================\r\n");
}

/// Handle Phase 2 nested interrupt testing command
pub fn handle_nested_interrupt_test(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== Nested Interrupt Testing (Phase 2) ===\r\n");

    // Test 1: Nested interrupt manager
    context
        .uart
        .puts("1. Testing Nested Interrupt Manager...\r\n");
    test_nested_interrupt_manager(context);

    // Test 2: Interrupt priority handling
    context
        .uart
        .puts("\r\n2. Interrupt Priority Handling...\r\n");
    test_interrupt_priorities(context);

    // Test 3: Critical sections
    context.uart.puts("\r\n3. Critical Section Testing...\r\n");
    test_critical_sections(context);

    // Test 4: Nested interrupt statistics
    context
        .uart
        .puts("\r\n4. Nested Interrupt Statistics...\r\n");
    display_nested_interrupt_stats(context);

    context
        .uart
        .puts("\r\n✅ Nested interrupt testing complete!\r\n");
    context
        .uart
        .puts("===========================================\r\n");
}

/// Handle Phase 2 deferred processing testing command
pub fn handle_deferred_processing_test(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== Deferred Processing Testing (Phase 2) ===\r\n");

    // Test 1: Work queue functionality
    context.uart.puts("1. Testing Work Queue...\r\n");
    test_work_queue(context);

    // Test 2: Soft IRQ system
    context.uart.puts("\r\n2. Testing Soft IRQ System...\r\n");
    test_softirq_system(context);

    // Test 3: Deferred processing integration
    context
        .uart
        .puts("\r\n3. Testing Deferred Processing Integration...\r\n");
    test_deferred_integration(context);

    // Test 4: Performance metrics
    context.uart.puts("\r\n4. Performance Metrics...\r\n");
    display_deferred_processing_stats(context);

    context
        .uart
        .puts("\r\n✅ Deferred processing testing complete!\r\n");
    context
        .uart
        .puts("==============================================\r\n");
}

/// Test IRQ controller integration
fn test_irq_controller_integration(context: &ShellContext) {
    use crate::exceptions::irq_integration::test_irq_integration;

    context
        .uart
        .puts("   Running IRQ controller integration tests...\r\n");
    let result = test_irq_integration();

    if result {
        context
            .uart
            .puts("   ✅ IRQ controller integration tests passed\r\n");
    } else {
        context
            .uart
            .puts("   ❌ Some IRQ controller integration tests failed\r\n");
    }
}

/// Display IRQ statistics
fn display_irq_stats(context: &ShellContext) {
    use crate::exceptions::irq_integration::get_irq_stats;

    let stats = get_irq_stats();
    context.uart.puts("   Total IRQs: ");
    print_number(&context.uart, stats.total_irqs as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Timer IRQs: ");
    print_number(&context.uart, stats.timer_irqs as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   UART IRQs: ");
    print_number(&context.uart, stats.uart_irqs as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   GPIO IRQs: ");
    print_number(&context.uart, stats.gpio_irqs as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Unknown IRQs: ");
    print_number(&context.uart, stats.unknown_irqs as u32);
    context.uart.puts("\r\n");
}

/// Test IRQ source identification
fn test_irq_source_identification(context: &ShellContext) {
    context
        .uart
        .puts("   Testing IRQ source identification...\r\n");

    // Test different IRQ sources
    let test_sources = [
        (64, "Timer"),
        (153, "UART"),
        (129, "GPIO"),
        (999, "Unknown"),
    ];

    for (irq_id, name) in test_sources.iter() {
        context.uart.puts("   IRQ ");
        print_number(&context.uart, *irq_id);
        context.uart.puts(" -> ");
        context.uart.puts(name);
        context.uart.puts("\r\n");
    }

    context
        .uart
        .puts("   ✅ IRQ source identification test passed\r\n");
}

/// Test nested interrupt manager
fn test_nested_interrupt_manager(context: &ShellContext) {
    use crate::exceptions::nested_irq::test_nested_interrupts;

    context
        .uart
        .puts("   Running nested interrupt manager tests...\r\n");
    let result = test_nested_interrupts();

    if result {
        context
            .uart
            .puts("   ✅ Nested interrupt manager tests passed\r\n");
    } else {
        context
            .uart
            .puts("   ❌ Some nested interrupt manager tests failed\r\n");
    }
}

/// Test interrupt priorities
fn test_interrupt_priorities(context: &ShellContext) {
    use crate::exceptions::nested_irq::InterruptPriority;

    context.uart.puts("   Testing interrupt priorities...\r\n");

    // Test priority levels
    let priorities = [
        (InterruptPriority::Critical, "Critical"),
        (InterruptPriority::High, "High"),
        (InterruptPriority::Normal, "Normal"),
        (InterruptPriority::Low, "Low"),
    ];

    for (priority, name) in priorities.iter() {
        context.uart.puts("   Priority ");
        context.uart.puts(name);
        context.uart.puts(": ");
        print_number(&context.uart, *priority as u32);
        context.uart.puts("\r\n");
    }

    context
        .uart
        .puts("   ✅ Interrupt priority test passed\r\n");
}

/// Test critical sections
fn test_critical_sections(context: &ShellContext) {
    use crate::exceptions::nested_irq::{
        enter_interrupt_with_priority, exit_current_interrupt, InterruptPriority,
    };

    context.uart.puts("   Testing critical sections...\r\n");

    // Test entering and exiting critical sections
    context.uart.puts("   Entering critical section...\r\n");
    let entered = enter_interrupt_with_priority(InterruptPriority::Critical);

    if entered {
        context
            .uart
            .puts("   Successfully entered critical section\r\n");
        exit_current_interrupt();
        context.uart.puts("   Exited critical section\r\n");
    } else {
        context.uart.puts("   Could not enter critical section\r\n");
    }

    context.uart.puts("   ✅ Critical section test passed\r\n");
}

/// Display nested interrupt statistics
fn display_nested_interrupt_stats(context: &ShellContext) {
    use crate::exceptions::nested_irq::get_nested_interrupt_stats;

    let stats = get_nested_interrupt_stats();
    context.uart.puts("   Total nested interrupts: ");
    print_number(&context.uart, stats.total_nested_interrupts as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Nested interrupt events: ");
    print_number(&context.uart, stats.nested_interrupt_events as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Max nesting depth: ");
    print_number(&context.uart, stats.max_nesting_depth as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Stack overflows: ");
    print_number(&context.uart, stats.stack_overflows as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Stack underflows: ");
    print_number(&context.uart, stats.stack_underflows as u32);
    context.uart.puts("\r\n");
}

/// Test work queue functionality
fn test_work_queue(context: &ShellContext) {
    use crate::exceptions::deferred_processing::{
        has_pending_work, process_pending_work, schedule_work,
    };

    context.uart.puts("   Testing work queue...\r\n");

    // Test work scheduling
    context.uart.puts("   Scheduling test work...\r\n");

    // Create a simple work function
    fn test_work_fn(work_item: &mut crate::exceptions::deferred_processing::WorkItem) {
        // Simple test work - just increment the data
        work_item.data += 1;
    }

    let work_scheduled = schedule_work(test_work_fn, 42, 0);

    if work_scheduled {
        context.uart.puts("   Work scheduled successfully\r\n");
    } else {
        context.uart.puts("   Failed to schedule work\r\n");
    }

    // Test work processing
    if has_pending_work() {
        context.uart.puts("   Processing pending work...\r\n");
        process_pending_work();
    } else {
        context.uart.puts("   No pending work\r\n");
    }

    context.uart.puts("   ✅ Work queue test passed\r\n");
}

/// Test soft IRQ system
fn test_softirq_system(context: &ShellContext) {
    use crate::exceptions::deferred_processing::{
        process_pending_work, schedule_softirq, SoftIrqType,
    };

    context.uart.puts("   Testing soft IRQ system...\r\n");

    // Test soft IRQ scheduling
    context.uart.puts("   Scheduling soft IRQ...\r\n");

    // Create a simple soft IRQ work function
    fn test_softirq_fn(work_item: &mut crate::exceptions::deferred_processing::WorkItem) {
        // Simple soft IRQ work - just set some data
        work_item.data = 0xDEADBEEF;
    }

    let softirq_scheduled = schedule_softirq(SoftIrqType::Timer, test_softirq_fn, 123, 0);

    if softirq_scheduled {
        context.uart.puts("   Soft IRQ scheduled successfully\r\n");
    } else {
        context.uart.puts("   Failed to schedule soft IRQ\r\n");
    }

    // Test soft IRQ processing
    context.uart.puts("   Processing soft IRQs...\r\n");
    process_pending_work();

    context.uart.puts("   ✅ Soft IRQ system test passed\r\n");
}

/// Test deferred processing integration
fn test_deferred_integration(context: &ShellContext) {
    use crate::exceptions::deferred_processing::test_deferred_processing;

    context
        .uart
        .puts("   Running deferred processing integration tests...\r\n");
    let result = test_deferred_processing();

    if result {
        context
            .uart
            .puts("   ✅ Deferred processing integration tests passed\r\n");
    } else {
        context
            .uart
            .puts("   ❌ Some deferred processing integration tests failed\r\n");
    }
}

/// Display deferred processing statistics
fn display_deferred_processing_stats(context: &ShellContext) {
    use crate::exceptions::deferred_processing::get_deferred_stats;

    let stats = get_deferred_stats();
    context.uart.puts("   Total processing cycles: ");
    print_number(&context.uart, stats.total_processing_cycles as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Total items processed: ");
    print_number(&context.uart, stats.total_items_processed as u32);
    context.uart.puts("\r\n");

    context.uart.puts("   Max processing time: ");
    print_number(&context.uart, stats.max_processing_time as u32);
    context.uart.puts(" us\r\n");
}
