//! ARM64 Exception Handlers for TinyOS
//!
//! This module implements the main exception handlers called from the
//! exception vector table. It provides comprehensive exception handling
//! with detailed ESR_EL1 decoding and reporting.

// Allow various warnings for this low-level exception handling code
#![allow(
    static_mut_refs,
    dead_code,
    clippy::new_without_default,
    clippy::missing_transmute_annotations,
    clippy::unnecessary_cast
)]

use core::mem::transmute;

use crate::uart::Uart;

// Import our ESR decoding system
use super::esr_decoder::{EsrDecoder, EsrInfo, EsrDetails, ExceptionClass};

// Import types from the main exceptions module
use super::types::{ExceptionContext, ExceptionType, EXCEPTION_STATS};

/// Handle synchronous exceptions with comprehensive ESR decoding
#[no_mangle]
pub extern "C" fn handle_sync_exception(ctx: &mut ExceptionContext, exc_level: u32) {
    unsafe {
        EXCEPTION_STATS.record_exception(ExceptionType::Synchronous, transmute(exc_level));
    }

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
        },
        ExceptionClass::DataAbortLower | ExceptionClass::DataAbortSame => {
            uart.puts("Data abort detected\r\n");
            handle_data_abort(ctx, &esr_info);
        },
        ExceptionClass::InstructionAbortLower | ExceptionClass::InstructionAbortSame => {
            uart.puts("Instruction abort detected\r\n");
            handle_instruction_abort(ctx, &esr_info);
        },
        ExceptionClass::IllegalExecution => {
            uart.puts("Illegal execution state\r\n");
            handle_illegal_execution_state(ctx, &esr_info);
        },
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

/// Handle IRQ exceptions
#[no_mangle]
pub extern "C" fn handle_irq_exception(_ctx: &mut ExceptionContext, exc_level: u32) {
    unsafe {
        EXCEPTION_STATS.record_exception(ExceptionType::Irq, transmute(exc_level));
    }

    // For now, just acknowledge and return
    // In a full implementation, this would dispatch to specific IRQ handlers
    let uart = Uart::new();
    uart.puts("IRQ received\r\n");
}

/// Handle FIQ exceptions
#[no_mangle]
pub extern "C" fn handle_fiq_exception(_ctx: &mut ExceptionContext, exc_level: u32) {
    unsafe {
        EXCEPTION_STATS.record_exception(ExceptionType::Fiq, transmute(exc_level));
    }

    let uart = Uart::new();
    uart.puts("FIQ received\r\n");
}

/// Handle SError exceptions
#[no_mangle]
pub extern "C" fn handle_serror_exception(ctx: &mut ExceptionContext, exc_level: u32) {
    unsafe {
        EXCEPTION_STATS.record_exception(ExceptionType::SError, core::mem::transmute(exc_level));
    }

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
    uart.puts(if esr_info.instruction_length { "32-bit" } else { "16-bit" });
    uart.puts(" instruction\r\n");
    
    // Print additional details based on exception class
    match &esr_info.details {
        EsrDetails::DataAbort { fault_address_valid, write_not_read, sign_extend: _, access_size, fault_status, cache_maintenance } => {
            uart.puts("  Data Fault Status: ");
            uart.puts(fault_status.description());
            uart.puts("\r\n  Write not Read: ");
            uart.puts(if *write_not_read { "true" } else { "false" });
            uart.puts("\r\n  Fault Address Valid: ");
            uart.puts(if *fault_address_valid { "true" } else { "false" });
            uart.puts("\r\n  Access Size: ");
            uart.put_hex(*access_size as u64);
            uart.puts("\r\n  Cache Maintenance: ");
            uart.puts(if *cache_maintenance { "true" } else { "false" });
            uart.puts("\r\n");
        },
        EsrDetails::SystemCall { immediate } => {
            uart.puts("  System Call Number: ");
            uart.put_hex(*immediate as u64);
            uart.puts("\r\n");
        },
        EsrDetails::InstructionAbort { fault_status } => {
            uart.puts("  Instruction Fault Status: ");
            uart.puts(fault_status.description());
            uart.puts("\r\n");
        },
        EsrDetails::Unknown => {
            uart.puts("  Unknown exception details\r\n");
        },
        _ => {
            uart.puts("  Additional details not implemented\r\n");
        },
    }
}

/// Handle system calls (SVC instructions)
fn handle_system_call(_ctx: &mut ExceptionContext, esr_info: &EsrInfo) {
    let uart = Uart::new();
    
    if let EsrDetails::SystemCall { immediate } = &esr_info.details {
        uart.puts("System call number: ");
        uart.put_hex(*immediate as u64);
        uart.puts("\r\n");
        
        // TODO: Implement system call dispatcher
        // For now, just log the call
        uart.puts("System call handling not yet implemented\r\n");
    }
}

/// Handle data aborts (memory access faults)
fn handle_data_abort(ctx: &mut ExceptionContext, esr_info: &EsrInfo) {
    let uart = Uart::new();
    
    if let EsrDetails::DataAbort { fault_address_valid: _, write_not_read, sign_extend: _, access_size: _, fault_status, cache_maintenance: _ } = &esr_info.details {
        uart.puts("Data abort analysis:\r\n");
        uart.puts("  Fault address: 0x");
        uart.put_hex(ctx.far);
        uart.puts("\r\n  Operation: ");
        uart.puts(if *write_not_read { "Write" } else { "Read" });
        uart.puts("\r\n  Fault type: ");
        uart.puts(fault_status.description());
        uart.puts("\r\n");
        
        // TODO: Implement memory fault recovery if possible
        uart.puts("Memory fault recovery not yet implemented\r\n");
    }
}

/// Handle instruction aborts (code execution faults)
fn handle_instruction_abort(_ctx: &mut ExceptionContext, esr_info: &EsrInfo) {
    let uart = Uart::new();
    
    uart.puts("Instruction abort analysis:\r\n");
    
    if let EsrDetails::InstructionAbort { fault_status } = &esr_info.details {
        uart.puts("  Fault type: ");
        uart.puts(fault_status.description());
        uart.puts("\r\n");
    }
    
    // TODO: Implement instruction fault analysis
    uart.puts("Instruction fault analysis not yet implemented\r\n");
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
