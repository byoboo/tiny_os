//! Exception system initialization for TinyOS
//!
//! This module handles the initialization of the ARM64 exception
//! vector table, exception handling system, and Phase 2 enhancements.

#[cfg(target_arch = "aarch64")]
use core::arch::global_asm;

use super::deferred_processing::init_deferred_processing;
// Import Phase 2 initialization functions
use super::nested_irq::init_nested_interrupts;

// Import the exception vector table assembly
#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("../exception_vectors.s"));

/// Initialize the exception vector table and Phase 2 enhancements
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

    // Initialize Phase 2 components
    init_nested_interrupts();
    init_deferred_processing();
}

/// Initialize the exception vector table (mock for non-aarch64 targets)
#[cfg(not(target_arch = "aarch64"))]
pub fn init_exceptions() {
    // Mock implementation for testing on non-aarch64 targets
    init_nested_interrupts();
    init_deferred_processing();
}
