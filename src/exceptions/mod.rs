//! Exception handling module exports
//!
//! This module provides the main interface for TinyOS exception handling system.
//! It includes comprehensive ESR_EL1 decoding, exception handlers, system call
//! interface, memory fault analysis, and system initialization.

pub mod esr_decoder;
pub mod handler;
pub mod init;
pub mod memory_faults;
pub mod syscall;
pub mod types;

// Re-export main types for easy access
pub use esr_decoder::{EsrDecoder, EsrInfo, EsrDetails, ExceptionClass, DataFaultStatus};
pub use handler::{handle_sync_exception, handle_irq_exception, handle_fiq_exception, handle_serror_exception};
pub use init::init_exceptions;
pub use memory_faults::{MemoryFaultAnalyzer, MemoryFaultInfo, MemoryFaultType, MemoryAccessType, get_memory_fault_stats};
pub use syscall::{SyscallNumber, SyscallResult, handle_syscall, make_syscall, get_syscall_stats};
pub use types::{ExceptionType, ExceptionLevel, ExceptionContext, ExceptionStats, EXCEPTION_STATS};
