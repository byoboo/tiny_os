//! Exception handling module exports
//!
//! This module provides the main interface for TinyOS exception handling system.
//! It includes comprehensive ESR_EL1 decoding, exception handlers, system call
//! interface, memory fault analysis, advanced IRQ management, and system initialization.

pub mod deferred_processing;
pub mod esr_decoder;
pub mod handler;
pub mod init;
pub mod irq_integration;
pub mod memory_faults;
pub mod nested_irq;
pub mod syscall;
pub mod types;

// Re-export main types for easy access
pub use deferred_processing::{schedule_work, schedule_softirq, process_pending_work, SoftIrqType, get_deferred_stats};
pub use esr_decoder::{EsrDecoder, EsrInfo, EsrDetails, ExceptionClass, DataFaultStatus};
pub use handler::{handle_sync_exception, handle_irq_exception, handle_fiq_exception, handle_serror_exception};
pub use init::init_exceptions;
pub use irq_integration::{init_irq_integration, handle_irq_integration, get_irq_stats, IrqInfo, IrqSource};
pub use memory_faults::{MemoryFaultAnalyzer, MemoryFaultInfo, MemoryFaultType, MemoryAccessType, get_memory_fault_stats, handle_memory_fault_with_mmu};
pub use nested_irq::{InterruptPriority, CriticalSection, enter_interrupt_with_priority, exit_current_interrupt, get_nested_interrupt_stats};
pub use syscall::{SyscallNumber, SyscallResult, handle_syscall, make_syscall, get_syscall_stats};
pub use types::{ExceptionType, ExceptionLevel, ExceptionContext, ExceptionStats, EXCEPTION_STATS};
