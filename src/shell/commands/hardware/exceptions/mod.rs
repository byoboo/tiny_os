//! Exception Commands Module
//!
//! This module provides exception-related shell commands for TinyOS,
//! organized into focused submodules for better maintainability.
//!
//! ## Module Organization
//! - `utils`: Utility functions (number printing, etc.)
//! - `stats`: Exception statistics display and analysis
//! - `testing`: Exception system testing and validation
//! - `esr`: ESR (Exception Syndrome Register) decoder testing
//! - `syscalls`: System call testing and analysis
//! - `memory_faults`: Memory fault testing and analysis

pub mod esr;
pub mod memory_faults;
pub mod stats;
pub mod syscalls;
pub mod testing;
pub mod utils;

// Re-export main command handlers for backwards compatibility
pub use esr::handle_esr_test;
pub use memory_faults::handle_memory_fault_test;
pub use stats::{display_detailed_stats, handle_exception_stats};
pub use syscalls::handle_syscall_test;
pub use testing::{
    handle_exception_test, handle_exception_test_advanced, test_esr_decoder,
    test_exception_handlers,
};
pub use utils::print_number;
