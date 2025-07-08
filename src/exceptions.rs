//! ARM64 Exception Handling for TinyOS
//!
//! This module implements the ARM64 exception vector table and exception
//! handlers. ARM64 has a comprehensive exception model with different exception
//! levels (EL0-EL3) and various exception types.

// Allow various warnings for this low-level exception handling code
#![allow(
    static_mut_refs,
    dead_code,
    clippy::new_without_default,
    clippy::missing_transmute_annotations,
    clippy::unnecessary_cast
)]

#[cfg(target_arch = "aarch64")]
use core::arch::global_asm;
use core::{
    include_str,
    mem::transmute,
    option::Option::{self, None, Some},
    prelude::rust_2021::*,
};

use crate::uart::Uart;

// Exception types in ARM64
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum ExceptionType {
    /// Synchronous exception (SVC, undefined instruction, etc.)
    Synchronous = 0,
    /// IRQ (Interrupt Request)
    Irq = 1,
    /// FIQ (Fast Interrupt Request)
    Fiq = 2,
    /// SError (System Error)
    SError = 3,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum ExceptionLevel {
    /// Exception from current EL with SP_EL0
    CurrentSpEl0 = 0,
    /// Exception from current EL with SP_ELx
    CurrentSpElx = 1,
    /// Exception from lower EL using AArch64
    LowerAArch64 = 2,
    /// Exception from lower EL using AArch32
    LowerAArch32 = 3,
}

/// Exception context - registers saved during exception entry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExceptionContext {
    // General purpose registers x0-x30
    pub gpr: [u64; 31],
    // Stack pointer
    pub sp: u64,
    // Exception Link Register (return address)
    pub elr: u64,
    // Saved Program Status Register
    pub spsr: u64,
    // Exception Syndrome Register
    pub esr: u64,
    // Fault Address Register
    pub far: u64,
}

impl ExceptionContext {
    pub fn new() -> Self {
        Self {
            gpr: [0; 31],
            sp: 0,
            elr: 0,
            spsr: 0,
            esr: 0,
            far: 0,
        }
    }
}

/// Global exception statistics
static mut EXCEPTION_STATS: ExceptionStats = ExceptionStats::new();

#[derive(Debug)]
pub struct ExceptionStats {
    pub sync_exceptions: u64,
    pub irq_exceptions: u64,
    pub fiq_exceptions: u64,
    pub serror_exceptions: u64,
    pub total_exceptions: u64,
    pub last_exception_type: Option<ExceptionType>,
    pub last_exception_level: Option<ExceptionLevel>,
}

impl ExceptionStats {
    const fn new() -> Self {
        Self {
            sync_exceptions: 0,
            irq_exceptions: 0,
            fiq_exceptions: 0,
            serror_exceptions: 0,
            total_exceptions: 0,
            last_exception_type: None,
            last_exception_level: None,
        }
    }

    fn record_exception(&mut self, exc_type: ExceptionType, exc_level: ExceptionLevel) {
        self.total_exceptions += 1;
        self.last_exception_type = Some(exc_type);
        self.last_exception_level = Some(exc_level);

        match exc_type {
            ExceptionType::Synchronous => self.sync_exceptions += 1,
            ExceptionType::Irq => self.irq_exceptions += 1,
            ExceptionType::Fiq => self.fiq_exceptions += 1,
            ExceptionType::SError => self.serror_exceptions += 1,
        }
    }
}

/// Initialize the exception vector table
#[cfg(target_arch = "aarch64")]
pub fn init_exceptions() {
    unsafe {
        // Set the Vector Base Address Register (VBAR_EL1) to point to our vector table
        core::arch::asm!(
            "adrp x0, exception_vector_table",
            "add x0, x0, :lo12:exception_vector_table",
            "msr vbar_el1, x0",
            "isb",
            options(nomem, nostack)
        );
    }
}

/// Initialize the exception vector table (mock for non-aarch64 targets)
#[cfg(not(target_arch = "aarch64"))]
pub fn init_exceptions() {
    // Mock implementation for testing on non-aarch64 targets
}

/// Get current exception statistics
pub fn get_exception_stats() -> &'static ExceptionStats {
    unsafe { (&raw const EXCEPTION_STATS).as_ref().unwrap() }
}

/// Reset exception statistics
pub fn reset_exception_stats() {
    unsafe {
        EXCEPTION_STATS = ExceptionStats::new();
    }
}

// Exception handlers called from assembly
#[no_mangle]
pub extern "C" fn handle_sync_exception(ctx: &mut ExceptionContext, exc_level: u32) {
    unsafe {
        EXCEPTION_STATS.record_exception(ExceptionType::Synchronous, transmute(exc_level));
    }

    let uart = Uart::new();
    uart.puts("EXCEPTION: Synchronous exception occurred!\r\n");
    uart.puts("ELR_EL1: 0x");
    uart.put_hex(ctx.elr);
    uart.puts("\r\nESR_EL1: 0x");
    uart.put_hex(ctx.esr);
    uart.puts("\r\nFAR_EL1: 0x");
    uart.put_hex(ctx.far);
    uart.puts("\r\nSPSR_EL1: 0x");
    uart.put_hex(ctx.spsr);
    uart.puts("\r\n");

    // Decode exception syndrome
    let ec = (ctx.esr >> 26) & 0x3F; // Exception Class
    let iss = ctx.esr & 0x1FFFFFF; // Instruction Specific Syndrome

    uart.puts("Exception Class: ");
    match ec {
        0x15 => uart.puts("SVC instruction execution in AArch64"),
        0x20 => uart.puts("Instruction Abort from a lower Exception level"),
        0x21 => uart.puts("Instruction Abort taken without a change in Exception level"),
        0x24 => uart.puts("Data Abort from a lower Exception level"),
        0x25 => uart.puts("Data Abort taken without a change in Exception level"),
        0x0E => uart.puts("Illegal Execution state"),
        _ => {
            uart.puts("Unknown (0x");
            uart.put_hex(ec as u64);
            uart.puts(")");
        }
    }
    uart.puts("\r\nISS: 0x");
    uart.put_hex(iss as u64);
    uart.puts("\r\n");

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

#[no_mangle]
pub extern "C" fn handle_fiq_exception(_ctx: &mut ExceptionContext, exc_level: u32) {
    unsafe {
        EXCEPTION_STATS.record_exception(ExceptionType::Fiq, transmute(exc_level));
    }

    let uart = Uart::new();
    uart.puts("FIQ received\r\n");
}

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

// Include the exception vector table assembly
#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("exception_vectors.s"));
