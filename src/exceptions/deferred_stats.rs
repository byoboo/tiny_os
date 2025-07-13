//! Statistics Types for Deferred Processing
//!
//! This module defines all statistics structures used throughout
//! the deferred processing system for monitoring and debugging.

/// Work queue statistics
#[derive(Debug, Clone, Copy)]
pub struct WorkQueueStats {
    pub items_scheduled: u64,
    pub items_processed: u64,
    pub queue_full_events: u64,
}

impl WorkQueueStats {
    pub const fn new() -> Self {
        Self {
            items_scheduled: 0,
            items_processed: 0,
            queue_full_events: 0,
        }
    }
}

/// Soft IRQ statistics
#[derive(Debug, Clone, Copy)]
pub struct SoftIrqStats {
    pub softirqs_raised: u64,
    pub softirqs_processed: u64,
}

impl SoftIrqStats {
    pub const fn new() -> Self {
        Self {
            softirqs_raised: 0,
            softirqs_processed: 0,
        }
    }
}

/// Deferred processing statistics
#[derive(Debug, Clone, Copy)]
pub struct DeferredProcessingStats {
    pub total_processing_cycles: u64,
    pub total_items_processed: u64,
    pub max_processing_time: u64,
}

impl DeferredProcessingStats {
    pub const fn new() -> Self {
        Self {
            total_processing_cycles: 0,
            total_items_processed: 0,
            max_processing_time: 0,
        }
    }
}
