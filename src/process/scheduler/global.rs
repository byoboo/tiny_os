//! Global Scheduler Interface
//!
//! This module provides the global scheduler instance and
//! system-wide scheduling functions with thread-safe access.

use spin::Mutex;

use super::{
    core::Scheduler,
    stats::SchedulerStats,
    task::{TaskId, TaskPriority},
};

/// Global scheduler instance
static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());

/// Initialize scheduler
pub fn init_scheduler() {
    SCHEDULER.lock().init();
}

/// Create a new task
pub fn create_task(
    name: &str,
    priority: TaskPriority,
    entry_point: u64,
    stack_base: u64,
    stack_size: u64,
) -> TaskId {
    SCHEDULER
        .lock()
        .create_task(name, priority, entry_point, stack_base, stack_size)
}

/// Destroy a task
pub fn destroy_task(task_id: TaskId) -> Result<(), &'static str> {
    SCHEDULER.lock().destroy_task(task_id)
}

/// Schedule next task
pub fn schedule() -> Option<TaskId> {
    SCHEDULER.lock().schedule().map(|task| task.id)
}

/// Handle timer preemption
pub fn handle_timer_preemption() -> bool {
    SCHEDULER.lock().handle_timer_preemption()
}

/// Block current task
pub fn block_current_task() {
    SCHEDULER.lock().block_current_task()
}

/// Unblock a task
pub fn unblock_task(task_id: TaskId) -> Result<(), &'static str> {
    SCHEDULER.lock().unblock_task(task_id)
}

/// Get current task ID
pub fn get_current_task_id() -> Option<TaskId> {
    SCHEDULER.lock().get_current_task().map(|task| task.id)
}

/// Get scheduler statistics
pub fn get_scheduler_stats() -> SchedulerStats {
    SCHEDULER.lock().get_stats()
}

/// Get task count
pub fn get_task_count() -> usize {
    SCHEDULER.lock().get_task_count()
}

/// Get ready task count for a specific priority
pub fn get_ready_count(priority: TaskPriority) -> usize {
    SCHEDULER.lock().get_ready_count(priority)
}

/// Enable/disable scheduler
pub fn set_scheduler_enabled(enabled: bool) {
    SCHEDULER.lock().set_enabled(enabled)
}

/// Check if scheduler is enabled
pub fn is_scheduler_enabled() -> bool {
    SCHEDULER.lock().is_enabled()
}

/// Find a task by ID
pub fn find_task_by_id(task_id: TaskId) -> bool {
    SCHEDULER.lock().find_task(task_id).is_some()
}

/// Get next available task ID
pub fn get_next_task_id() -> TaskId {
    SCHEDULER.lock().get_next_task_id()
}

/// Execute a function with the scheduler lock
pub fn with_scheduler<F, R>(f: F) -> R
where
    F: FnOnce(&mut Scheduler) -> R,
{
    f(&mut SCHEDULER.lock())
}

/// Check if any tasks are ready
pub fn has_ready_tasks() -> bool {
    SCHEDULER.lock().get_task_count() > 0
}

/// Get current task name (copies to a provided buffer)
pub fn get_current_task_name(buffer: &mut [u8]) -> Option<usize> {
    if let Some(task) = SCHEDULER.lock().get_current_task() {
        let name = task.get_name();
        let copy_len = core::cmp::min(name.len(), buffer.len());
        buffer[..copy_len].copy_from_slice(name.as_bytes());
        Some(copy_len)
    } else {
        None
    }
}

/// Force a context switch (yield)
pub fn yield_task() {
    // This would trigger a context switch
    // Implementation depends on the specific architecture
}

/// Emergency scheduler reset (for debugging)
pub fn reset_scheduler() {
    let mut scheduler = SCHEDULER.lock();
    *scheduler = Scheduler::new();
    scheduler.init();
}
