//! Deferred Processing Module
//!
//! This module provides comprehensive deferred interrupt processing
//! functionality including work queues, soft IRQs, and bottom-half handling for
//! TinyOS.
//!
//! The module is organized into focused submodules:
//! - `work_item`: Core work item structure and operations
//! - `work_queue`: Work queue management and processing
//! - `softirq`: Soft IRQ types and manager implementation
//! - `statistics`: Performance metrics and tracking
//! - `manager`: Main coordinator and global access functions

pub mod manager;
pub mod softirq;
pub mod statistics;
pub mod work_item;
pub mod work_queue;

// Re-export commonly used types and functions
pub use manager::{
    get_deferred_stats, has_pending_work, init_deferred_processing, process_pending_work,
    schedule_softirq, schedule_work, test_deferred_processing, DeferredProcessingManager,
};
pub use softirq::{SoftIrqManager, SoftIrqStats, SoftIrqType};
pub use statistics::DeferredProcessingStats;
pub use work_item::{WorkFunction, WorkItem, MAX_WORK_ITEMS};
pub use work_queue::{WorkQueue, WorkQueueStats};
