//! ARM64 Exception Handlers for TinyOS
//!
//! This module implements the main exception handlers called from the
//! exception vector table. It provides comprehensive exception handling
//! with detailed ESR_EL1 decoding and reporting.

// Allow various warnings for this low-level code
#![allow(
    static_mut_refs,
    dead_code,
    clippy::new_without_default,
    clippy::missing_transmute_annotations,
    clippy::unnecessary_cast
)]

use core::mem::transmute;

// Import our ESR decoding system
use super::esr_decoder::{EsrDecoder, EsrDetails, EsrInfo, ExceptionClass};
// Import types from the main exceptions module
use super::types::{ExceptionContext, ExceptionStats, ExceptionType};
use super::{
    deferred_processing::process_pending_work,
    irq_integration::handle_irq_integration,
    memory_faults::{MemoryFaultAnalyzer, MEMORY_FAULT_STATS},
    nested_irq::{enter_interrupt_with_priority, exit_current_interrupt, InterruptPriority},
    syscall::handle_syscall,
};
use crate::uart::Uart;

/// Handle synchronous exceptions with comprehensive ESR decoding
#[no_mangle]
pub extern "C" fn handle_sync_exception(ctx: &mut ExceptionContext, exc_level: u32) {
    ExceptionStats::record_exception_occurrence(ExceptionType::Synchronous, unsafe {
        transmute(exc_level)
    });

    let uart = Uart::new();
    uart.puts("EXCEPTION: Synchronous exception occurred!\r\n");

    // Use our enhanced ESR decoder for detailed analysis
    let decoder = EsrDecoder::new();
    let esr_info = decoder.decode_esr(ctx.esr as u32);

    // Print basic exception information
    uart.puts("ELR_EL1: 0x");
    uart.put_hex(ctx.elr);
    uart.puts("\r\nESR_EL1: 0x");
    uart.put_hex(ctx.esr);
    uart.puts("\r\nFAR_EL1: 0x");
    uart.put_hex(ctx.far);
    uart.puts("\r\nSPSR_EL1: 0x");
    uart.put_hex(ctx.spsr);
    uart.puts("\r\n");

    // Enhanced exception reporting using ESR decoder
    report_exception_details(&uart, &esr_info);

    // Handle specific exception types
    match esr_info.exception_class {
        ExceptionClass::Svc64 => {
            uart.puts("System call detected - SVC instruction\r\n");
            handle_system_call(ctx, &esr_info);
        }
        ExceptionClass::DataAbortLower | ExceptionClass::DataAbortSame => {
            uart.puts("Data abort detected\r\n");
            handle_data_abort(ctx, &esr_info);
        }
        ExceptionClass::InstructionAbortLower | ExceptionClass::InstructionAbortSame => {
            uart.puts("Instruction abort detected\r\n");
            handle_instruction_abort(ctx, &esr_info);
        }
        ExceptionClass::IllegalExecution => {
            uart.puts("Illegal execution state\r\n");
            handle_illegal_execution_state(ctx, &esr_info);
        }
        _ => {
            uart.puts("Unhandled exception type\r\n");
            handle_unhandled_exception(ctx, &esr_info);
        }
    }

    // For now, halt the system on synchronous exceptions
    uart.puts("System halted due to synchronous exception.\r\n");
    #[cfg(target_arch = "aarch64")]
    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        // Mock halt for testing
        panic!("System halted due to synchronous exception");
    }
}

/// Handle IRQ exceptions with Phase 2 enhancements
#[no_mangle]
pub extern "C" fn handle_irq_exception(ctx: &mut ExceptionContext, exc_level: u32) {
    ExceptionStats::record_exception_occurrence(ExceptionType::Irq, unsafe {
        transmute(exc_level)
    });

    // Enter interrupt with normal priority
    if !enter_interrupt_with_priority(InterruptPriority::Normal) {
        // Interrupt was masked, shouldn't happen for IRQ
        let uart = Uart::new();
        uart.puts("IRQ masked - should not happen\r\n");
        return;
    }

    // Handle the IRQ through the integration layer
    let irq_info = handle_irq_integration(ctx);

    if irq_info.is_valid {
        let uart = Uart::new();
        uart.puts("IRQ handled: ");
        match irq_info.source {
            super::irq_integration::IrqSource::Timer => uart.puts("Timer"),
            super::irq_integration::IrqSource::Uart => uart.puts("UART"),
            super::irq_integration::IrqSource::Gpio => uart.puts("GPIO"),
            super::irq_integration::IrqSource::Unknown => uart.puts("Unknown"),
        }
        uart.puts(" (ID: ");
        uart.put_hex(irq_info.interrupt_id as u64);
        uart.puts(")\r\n");
    }

    // Process any pending deferred work
    process_pending_work();

    // Exit interrupt context
    exit_current_interrupt();
}

/// Handle FIQ exceptions
#[no_mangle]
pub extern "C" fn handle_fiq_exception(_ctx: &mut ExceptionContext, exc_level: u32) {
    ExceptionStats::record_exception_occurrence(ExceptionType::Fiq, unsafe {
        transmute(exc_level)
    });

    let uart = Uart::new();
    uart.puts("FIQ received\r\n");
}

/// Handle SError exceptions
#[no_mangle]
pub extern "C" fn handle_serror_exception(ctx: &mut ExceptionContext, exc_level: u32) {
    ExceptionStats::record_exception_occurrence(ExceptionType::SError, unsafe {
        core::mem::transmute(exc_level)
    });

    let uart = Uart::new();
    uart.puts("CRITICAL: SError exception occurred!\r\n");
    uart.puts("ELR_EL1: 0x");
    uart.put_hex(ctx.elr);
    uart.puts("\r\nESR_EL1: 0x");
    uart.put_hex(ctx.esr);
    uart.puts("\r\nFAR_EL1: 0x");
    uart.put_hex(ctx.far);
    uart.puts("\r\n");

    // SError is critical - halt the system
    uart.puts("System halted due to SError.\r\n");
    #[cfg(target_arch = "aarch64")]
    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        // Mock halt for testing
        panic!("System halted due to SError");
    }
}

/// Report detailed exception information using ESR decoder
fn report_exception_details(uart: &Uart, esr_info: &EsrInfo) {
    uart.puts("Exception Details:\r\n");
    uart.puts("  Class: ");
    uart.puts(esr_info.exception_class.description());
    uart.puts("\r\n  ISS: 0x");
    uart.put_hex(esr_info.iss as u64);
    uart.puts("\r\n  IL: ");
    uart.puts(if esr_info.instruction_length {
        "32-bit"
    } else {
        "16-bit"
    });
    uart.puts(" instruction\r\n");

    // Print additional details based on exception class
    match &esr_info.details {
        EsrDetails::DataAbort {
            dfsc,
            wnr,
            s1ptw: _,
            cm,
            ea: _,
            fnv,
            set: _,
            ar: _,
            sf: _,
        } => {
            uart.puts("  Data Fault Status: ");
            uart.puts(dfsc.description());
            uart.puts("\r\n  Write not Read: ");
            uart.puts(if *wnr { "true" } else { "false" });
            uart.puts("\r\n  Fault Address Valid: ");
            uart.puts(if !*fnv { "true" } else { "false" });
            uart.puts("\r\n  Access Size: ");
            uart.put_hex(0); // Access size not available in new structure
            uart.puts("\r\n  Cache Maintenance: ");
            uart.puts(if *cm { "true" } else { "false" });
            uart.puts("\r\n");
        }
        EsrDetails::SystemCall { imm16 } => {
            uart.puts("  System Call Number: ");
            uart.put_hex(*imm16 as u64);
            uart.puts("\r\n");
        }
        EsrDetails::InstructionAbort { ifsc, .. } => {
            uart.puts("  Instruction Fault Status: ");
            uart.put_hex(*ifsc as u64);
            uart.puts("\r\n");
        }
        EsrDetails::Unknown => {
            uart.puts("  Unknown exception details\r\n");
        }
        _ => {
            uart.puts("  Additional details not implemented\r\n");
        }
    }
}

/// Handle system calls (SVC instructions)
fn handle_system_call(ctx: &mut ExceptionContext, esr_info: &EsrInfo) {
    let uart = Uart::new();

    if let EsrDetails::SystemCall { imm16 } = &esr_info.details {
        uart.puts("System call number: ");
        uart.put_hex(*imm16 as u64);
        uart.puts("\r\n");

        // Extract system call arguments from general registers
        // In ARM64, syscall arguments are passed in x0-x5
        let args = [
            ctx.gpr[0], // x0
            ctx.gpr[1], // x1
            ctx.gpr[2], // x2
            ctx.gpr[3], // x3
            ctx.gpr[4], // x4
            ctx.gpr[5], // x5
        ];

        // Call the system call handler
        let result = handle_syscall(*imm16 as u64, &args);

        // Store the result in x0 (return value register)
        ctx.gpr[0] = result as i64 as u64;

        uart.puts("System call completed with result: ");
        uart.put_hex(result as u64);
        uart.puts("\r\n");
    }
}

/// Handle data aborts (memory access faults) with MMU integration
fn handle_data_abort(ctx: &mut ExceptionContext, esr_info: &EsrInfo) {
    let uart = Uart::new();

    if let EsrDetails::DataAbort {
        dfsc,
        wnr,
        s1ptw: _,
        cm: _,
        ea: _,
        fnv: _,
        set: _,
        ar: _,
        sf: _,
    } = &esr_info.details
    {
        uart.puts("Data abort analysis:\r\n");
        uart.puts("  Fault address: 0x");
        uart.put_hex(ctx.far);
        uart.puts("\r\n  Operation: ");
        uart.puts(if *wnr { "Write" } else { "Read" });
        uart.puts("\r\n  Fault type: ");
        uart.puts(dfsc.description());
        uart.puts("\r\n");

        // Use memory fault analyzer for detailed analysis
        let fault_info = MemoryFaultAnalyzer::analyze_fault(ctx.esr as u32);
        let _report = MemoryFaultAnalyzer::generate_fault_report(&fault_info);

        // Update statistics
        MEMORY_FAULT_STATS
            .lock()
            .record_fault(fault_info.fault_type);

        // Phase 4 MMU Integration: Check if we have a memory manager available
        // For now, we'll create a stub memory manager for demonstration
        // In a real system, this would be passed in or accessed globally
        let mut memory_manager = crate::memory::MemoryManager::new();

        // Determine if we're in user mode (simplified check)
        let user_mode = (ctx.spsr & 0xF) == 0x0; // EL0 = user mode

        // Call the integrated MMU memory fault handler
        use crate::exceptions::memory_faults::handle_memory_fault_with_mmu;
        let recovery_action = handle_memory_fault_with_mmu(
            ctx.esr as u32,
            ctx.far,
            ctx.elr,
            user_mode,
            &mut memory_manager,
        );

        uart.puts("MMU Recovery Action: ");
        match recovery_action {
            crate::memory::MmuRecoveryAction::Continue => uart.puts("Continue"),
            crate::memory::MmuRecoveryAction::Retry => uart.puts("Retry"),
            crate::memory::MmuRecoveryAction::TerminateProcess => uart.puts("Terminate Process"),
            crate::memory::MmuRecoveryAction::SystemPanic => uart.puts("System Panic"),
        }
        uart.puts("\r\n");

        uart.puts("Memory fault analysis completed\r\n");
    }
}

/// Handle instruction aborts (code execution faults) with MMU integration
fn handle_instruction_abort(ctx: &mut ExceptionContext, esr_info: &EsrInfo) {
    let uart = Uart::new();

    uart.puts("Instruction abort analysis:\r\n");

    if let EsrDetails::InstructionAbort {
        ifsc,
        s1ptw: _,
        ea: _,
        fnv: _,
        set: _,
    } = &esr_info.details
    {
        uart.puts("  Fault type: 0x");
        uart.put_hex(*ifsc as u64);
        uart.puts("\r\n");
    }

    // Phase 4 MMU Integration for instruction aborts
    let mut memory_manager = crate::memory::MemoryManager::new();
    let user_mode = (ctx.spsr & 0xF) == 0x0; // EL0 = user mode

    // Call the integrated MMU memory fault handler for instruction faults
    use crate::exceptions::memory_faults::handle_memory_fault_with_mmu;
    let recovery_action = handle_memory_fault_with_mmu(
        ctx.esr as u32,
        ctx.far,
        ctx.elr,
        user_mode,
        &mut memory_manager,
    );

    uart.puts("MMU Recovery Action: ");
    match recovery_action {
        crate::memory::MmuRecoveryAction::Continue => uart.puts("Continue"),
        crate::memory::MmuRecoveryAction::Retry => uart.puts("Retry"),
        crate::memory::MmuRecoveryAction::TerminateProcess => uart.puts("Terminate Process"),
        crate::memory::MmuRecoveryAction::SystemPanic => uart.puts("System Panic"),
    }
    uart.puts("\r\n");

    uart.puts("Instruction fault analysis completed\r\n");
}

/// Handle illegal execution state
fn handle_illegal_execution_state(_ctx: &mut ExceptionContext, _esr_info: &EsrInfo) {
    let uart = Uart::new();

    uart.puts("Illegal execution state detected\r\n");
    uart.puts("This typically indicates a serious system error\r\n");

    // TODO: Implement recovery mechanisms if possible
}

/// Handle unhandled exception types
fn handle_unhandled_exception(_ctx: &mut ExceptionContext, esr_info: &EsrInfo) {
    let uart = Uart::new();

    uart.puts("Unhandled exception type: ");
    uart.puts(esr_info.exception_class.description());
    uart.puts("\r\n");

    // TODO: Implement handlers for additional exception types
}
