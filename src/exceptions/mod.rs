//! Exception handling module exports
//!
//! This module provides the main interface for TinyOS exception handling
//! system. It includes comprehensive ESR_EL1 decoding, exception handlers,
//! system call interface, memory fault analysis, advanced IRQ management, and
//! system initialization.

pub mod deferred_processing;

// Deferred processing modules
pub mod deferred_api;
pub mod deferred_manager;
pub mod deferred_stats;
pub mod softirq;
pub mod work_item;
pub mod work_queue;

pub mod esr_decoder;
pub mod handler;
pub mod init;
pub mod irq_integration;
pub mod memory_faults;
pub mod nested_irq;
pub mod syscall;
pub mod types;

// Re-export main types for easy access
pub use deferred_api::{
    get_deferred_stats, init_deferred_processing, process_pending_work, schedule_softirq,
    schedule_work, test_deferred_processing,
};
pub use deferred_stats::{DeferredProcessingStats, SoftIrqStats, WorkQueueStats};
pub use esr_decoder::{DataFaultStatus, EsrDecoder, EsrDetails, EsrInfo, ExceptionClass};
pub use handler::{
    handle_fiq_exception, handle_irq_exception, handle_serror_exception, handle_sync_exception,
};
pub use init::init_exceptions;
pub use irq_integration::{
    get_irq_stats, handle_irq_integration, init_irq_integration, IrqInfo, IrqSource,
};
pub use memory_faults::{
    get_memory_fault_stats, handle_memory_fault_with_mmu, MemoryAccessType, MemoryFaultAnalyzer,
    MemoryFaultInfo, MemoryFaultType,
};
pub use nested_irq::{
    enter_interrupt_with_priority, exit_current_interrupt, get_nested_interrupt_stats,
    CriticalSection, InterruptPriority,
};
pub use softirq::SoftIrqType;
pub use syscall::{get_syscall_stats, handle_syscall, make_syscall, SyscallNumber, SyscallResult};
pub use types::{ExceptionContext, ExceptionLevel, ExceptionStats, ExceptionType, EXCEPTION_STATS};
pub use work_item::{WorkFunction, WorkItem};
