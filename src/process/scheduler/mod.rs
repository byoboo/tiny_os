//! Process Scheduler Module
//!
//! This module provides a comprehensive task scheduling system including:
//! - Task definitions and lifecycle management
//! - Priority-based scheduling with round-robin within priorities
//! - Task queues and lists for efficient task management
//! - Performance statistics and monitoring
//! - Global scheduler interface with thread-safe access

pub mod core;
pub mod global;
pub mod queue;
pub mod stats;
pub mod task;

// Re-export key types and functions for compatibility
pub use core::Scheduler;
// For backward compatibility, re-export everything that was in the original module
pub use core::Scheduler as SchedulerCompat;

pub use global::{
    block_current_task, create_task, destroy_task, get_current_task_id, get_ready_count,
    get_scheduler_stats, get_task_count, handle_timer_preemption, has_ready_tasks, init_scheduler,
    is_scheduler_enabled, schedule, set_scheduler_enabled, unblock_task, with_scheduler,
    yield_task,
};
pub use queue::{TaskList, TaskQueue, TaskQueueIter};
pub use stats::SchedulerStats;
pub use task::{
    Task, Task as TaskCompat, TaskId, TaskId as TaskIdCompat, TaskPriority,
    TaskPriority as TaskPriorityCompat,
};
