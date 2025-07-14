//! Deferred Processing Module
//!
//! This module provides comprehensive deferred interrupt processing functionality
//! including work queues, soft IRQs, and bottom-half handling for TinyOS.
//!
//! The module is organized into focused submodules:
//! - `work_item`: Core work item structure and operations
//! - `work_queue`: Work queue management and processing
//! - `softirq`: Soft IRQ types and manager implementation
//! - `statistics`: Performance metrics and tracking
//! - `manager`: Main coordinator and global access functions

pub mod work_item;
pub mod work_queue;
pub mod softirq;
pub mod statistics;
pub mod manager;

// Re-export commonly used types and functions
pub use work_item::{WorkItem, WorkFunction, MAX_WORK_ITEMS};
pub use work_queue::{WorkQueue, WorkQueueStats};
pub use softirq::{SoftIrqType, SoftIrqManager, SoftIrqStats};
pub use statistics::DeferredProcessingStats;
pub use manager::{
    DeferredProcessingManager,
    init_deferred_processing,
    schedule_work,
    schedule_softirq,
    process_pending_work,
    has_pending_work,
    get_deferred_stats,
    test_deferred_processing,
};
