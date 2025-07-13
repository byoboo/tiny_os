//! Work Item structures and implementations
//!
//! Defines the WorkItem struct for deferred processing tasks

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
    pub const fn new(work_fn: WorkFunction, data: u64, context: u64, id: u32) -> Self {
        Self {
            work_fn: Some(work_fn),
            data,
            context,
            id,
            is_valid: true,
        }
    }

    pub const fn invalid() -> Self {
        Self {
            work_fn: None,
            data: 0,
            context: 0,
            id: 0,
            is_valid: false,
        }
    }

    /// Execute the work item if it has a valid function
    pub fn execute(&mut self) -> bool {
        if let Some(work_fn) = self.work_fn {
            work_fn(self);
            true
        } else {
            false
        }
    }

    /// Check if this work item is valid and ready for execution
    pub fn is_ready(&self) -> bool {
        self.is_valid && self.work_fn.is_some()
    }

    /// Mark this work item as invalid
    pub fn invalidate(&mut self) {
        self.is_valid = false;
        self.work_fn = None;
    }

    /// Update the work item's data
    pub fn update_data(&mut self, data: u64, context: u64) {
        self.data = data;
        self.context = context;
    }
}
