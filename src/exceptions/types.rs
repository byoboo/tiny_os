//! Core exception types and data structures for TinyOS
//!
//! This module defines the fundamental types used throughout the
//! exception handling system.

use core::{
    option::Option::{self, None, Some},
    prelude::rust_2021::*,
};

use spin::Mutex;

/// Exception types in ARM64
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

/// Exception levels in ARM64
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

/// Exception statistics tracking
#[derive(Debug, Clone)]
pub struct ExceptionStats {
    pub sync_exceptions: u64,
    pub irq_exceptions: u64,
    pub fiq_exceptions: u64,
    pub serror_exceptions: u64,
    pub total_exceptions: u64,
    pub last_exception_type: Option<ExceptionType>,
    pub last_exception_level: Option<ExceptionLevel>,
}

/// Global exception statistics
pub static EXCEPTION_STATS: Mutex<ExceptionStats> = Mutex::new(ExceptionStats::new());

impl ExceptionStats {
    pub const fn new() -> Self {
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

    pub fn record_exception(&mut self, exc_type: ExceptionType, exc_level: ExceptionLevel) {
        self.total_exceptions += 1;
        self.last_exception_type = Some(exc_type);
        self.last_exception_level = Some(exc_level);

        match exc_type {
            ExceptionType::Synchronous => {
                self.sync_exceptions += 1;
            }
            ExceptionType::Irq => {
                self.irq_exceptions += 1;
            }
            ExceptionType::Fiq => {
                self.fiq_exceptions += 1;
            }
            ExceptionType::SError => {
                self.serror_exceptions += 1;
            }
        }
    }

    /// Get current exception statistics
    pub fn get_stats() -> ExceptionStats {
        EXCEPTION_STATS.lock().clone()
    }

    /// Record an exception occurrence
    pub fn record_exception_occurrence(exc_type: ExceptionType, exc_level: ExceptionLevel) {
        EXCEPTION_STATS.lock().record_exception(exc_type, exc_level);
    }

    /// Reset exception statistics
    pub fn reset_stats() {
        *EXCEPTION_STATS.lock() = ExceptionStats::new();
    }
}
