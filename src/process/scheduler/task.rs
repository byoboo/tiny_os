//! Task Definitions and Management
//!
//! This module provides the core task structure and related types
//! for the TinyOS process scheduler.

use crate::process::context::{ProcessContext, ProcessState};

/// Task ID type
pub type TaskId = u32;

/// Task priority levels with associated time slices
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    RealTime = 4,
}

impl TaskPriority {
    /// Get default time slice for this priority level
    pub fn default_time_slice(self) -> u32 {
        match self {
            TaskPriority::Idle => 100,
            TaskPriority::Low => 500,
            TaskPriority::Normal => 1000,
            TaskPriority::High => 2000,
            TaskPriority::RealTime => 5000,
        }
    }

    /// Get priority index for queue array indexing
    pub fn as_index(self) -> usize {
        self as usize
    }
}

/// Task structure containing all task-related information
#[derive(Debug, Clone)]
pub struct Task {
    /// Task ID
    pub id: TaskId,

    /// Task name
    pub name: [u8; 32],

    /// Task priority
    pub priority: TaskPriority,

    /// Process context
    pub context: ProcessContext,

    /// Entry point
    pub entry_point: u64,

    /// Task stack base address
    pub stack_base: u64,

    /// Task stack size
    pub stack_size: u64,

    /// Task creation time
    pub creation_time: u64,

    /// Total run time
    pub run_time: u64,

    /// Last run time
    pub last_run: u64,

    /// Task flags
    pub flags: u32,

    /// User space page table ID (for user tasks)
    pub user_page_table_id: Option<usize>,
}

impl Task {
    /// Create a new task
    pub fn new(
        id: TaskId,
        name: &str,
        priority: TaskPriority,
        entry_point: u64,
        stack_base: u64,
        stack_size: u64,
    ) -> Self {
        let mut task_name = [0u8; 32];
        let name_bytes = name.as_bytes();
        let copy_len = core::cmp::min(name_bytes.len(), 31);
        task_name[..copy_len].copy_from_slice(&name_bytes[..copy_len]);

        let creation_time = get_current_time();

        Self {
            id,
            name: task_name,
            priority,
            context: ProcessContext::new(id, stack_base, stack_base + stack_size, entry_point),
            entry_point,
            stack_base,
            stack_size,
            creation_time,
            run_time: 0,
            last_run: creation_time,
            flags: 0,
            user_page_table_id: None,
        }
    }

    /// Get task name as string
    pub fn get_name(&self) -> &str {
        let null_pos = self.name.iter().position(|&b| b == 0).unwrap_or(32);
        core::str::from_utf8(&self.name[..null_pos]).unwrap_or("invalid")
    }

    /// Check if task is in Ready state
    pub fn is_ready(&self) -> bool {
        matches!(self.context.state, ProcessState::Ready)
    }

    /// Check if task is in Running state
    pub fn is_running(&self) -> bool {
        matches!(self.context.state, ProcessState::Running)
    }

    /// Check if task is in Blocked state
    pub fn is_blocked(&self) -> bool {
        matches!(self.context.state, ProcessState::Blocked)
    }

    /// Check if task is in Terminated state
    pub fn is_terminated(&self) -> bool {
        matches!(self.context.state, ProcessState::Terminated)
    }

    /// Set task state
    pub fn set_state(&mut self, state: ProcessState) {
        self.context.state = state;
    }

    /// Get task state
    pub fn get_state(&self) -> ProcessState {
        self.context.state
    }

    /// Update task run time
    pub fn update_run_time(&mut self, time: u64) {
        self.run_time += time - self.last_run;
        self.last_run = time;
    }

    /// Reset time slice for this task
    pub fn reset_time_slice(&mut self) {
        // Reset any time slice tracking (implementation specific)
        self.last_run = get_current_time();
    }

    /// Check if time slice has expired
    pub fn time_slice_expired(&mut self) -> bool {
        let current_time = get_current_time();
        let elapsed = current_time - self.last_run;
        elapsed >= self.priority.default_time_slice() as u64
    }

    /// Set user page table ID
    pub fn set_user_page_table_id(&mut self, page_table_id: usize) {
        self.user_page_table_id = Some(page_table_id);
    }

    /// Get user page table ID
    pub fn get_user_page_table_id(&self) -> Option<usize> {
        self.user_page_table_id
    }

    /// Check if task has user page table
    pub fn has_user_page_table(&self) -> bool {
        self.user_page_table_id.is_some()
    }

    /// Clear user page table ID
    pub fn clear_user_page_table_id(&mut self) {
        self.user_page_table_id = None;
    }
}

/// Get current system time (placeholder implementation)
fn get_current_time() -> u64 {
    // TODO: Implement actual system time
    // For now, return a simple counter
    static mut COUNTER: u64 = 0;
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}
