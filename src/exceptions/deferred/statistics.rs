//! Statistics Module
//!
//! This module provides comprehensive statistics tracking for deferred
//! processing, including performance metrics and processing counts.

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
