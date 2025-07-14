//! Work Item Module
//!
//! This module implements work items for deferred interrupt processing,
//! providing the foundation for bottom-half processing and work queues.

/// Maximum number of work items in the queue
pub const MAX_WORK_ITEMS: usize = 32;

/// Work item function type
pub type WorkFunction = fn(&mut WorkItem);

/// Work item for deferred processing
#[derive(Clone, Copy)]
pub struct WorkItem {
    /// Function to execute
    pub work_fn: Option<WorkFunction>,
    /// Data for the work function (generic u64)
    pub data: u64,
    /// Additional context data
    pub context: u64,
    /// Work item ID for tracking
    pub id: u32,
    /// Whether this work item is valid
    pub is_valid: bool,
}

impl WorkItem {
    /// Create a new work item
    pub const fn new(work_fn: WorkFunction, data: u64, context: u64, id: u32) -> Self {
        Self {
            work_fn: Some(work_fn),
            data,
            context,
            id,
            is_valid: true,
        }
    }

    /// Create an invalid work item
    pub const fn invalid() -> Self {
        Self {
            work_fn: None,
            data: 0,
            context: 0,
            id: 0,
            is_valid: false,
        }
    }

    /// Execute the work item
    pub fn execute(&mut self) {
        if let Some(work_fn) = self.work_fn {
            work_fn(self);
        }
    }
}
