//! Core Scheduler Implementation
//!
//! This module provides the main scheduling logic including
//! task switching, preemption, and priority management.

use crate::process::context::ProcessState;

use super::{
    queue::TaskList,
    stats::SchedulerStats,
    task::{Task, TaskId, TaskPriority},
};

/// Basic round-robin scheduler with priority levels
pub struct Scheduler {
    /// Ready queue for each priority level
    ready_queues: [TaskList; 5],

    /// Currently running task
    current_task: Option<Task>,

    /// Next task ID to assign
    next_task_id: TaskId,

    /// Scheduler statistics
    stats: SchedulerStats,

    /// Scheduler enabled flag
    enabled: bool,

    /// Idle task
    idle_task: Option<Task>,
}

impl Scheduler {
    /// Create a new scheduler
    pub const fn new() -> Self {
        Self {
            ready_queues: [
                TaskList::new(), // Idle
                TaskList::new(), // Low
                TaskList::new(), // Normal
                TaskList::new(), // High
                TaskList::new(), // RealTime
            ],
            current_task: None,
            next_task_id: 1,
            stats: SchedulerStats::new(),
            enabled: false,
            idle_task: None,
        }
    }

    /// Initialize scheduler
    pub fn init(&mut self) {
        self.enabled = true;
        self.create_idle_task();
    }

    /// Create idle task
    fn create_idle_task(&mut self) {
        let idle_task = Task::new(
            0,
            "idle",
            TaskPriority::Idle,
            0x0000_0000_0000_0000, // Will be set to idle function
            0x7F00_0000,           // Idle stack base
            0x1000,                // 4KB stack
        );

        self.idle_task = Some(idle_task);
    }

    /// Create a new task
    pub fn create_task(
        &mut self,
        name: &str,
        priority: TaskPriority,
        entry_point: u64,
        stack_base: u64,
        stack_size: u64,
    ) -> TaskId {
        let task_id = self.next_task_id;
        self.next_task_id += 1;

        let task = Task::new(task_id, name, priority, entry_point, stack_base, stack_size);

        // Add to appropriate ready queue
        let priority_index = priority.as_index();
        if self.ready_queues[priority_index].push(task).is_ok() {
            self.stats.record_task_created();
        }

        task_id
    }

    /// Destroy a task
    pub fn destroy_task(&mut self, task_id: TaskId) -> Result<(), &'static str> {
        // Remove from ready queues
        for queue in &mut self.ready_queues {
            if queue.remove(task_id).is_some() {
                self.stats.record_task_destroyed();
                return Ok(());
            }
        }

        // Check if it's the current task
        if let Some(ref current) = self.current_task {
            if current.id == task_id {
                self.current_task = None;
                self.stats.record_task_destroyed();
                return Ok(());
            }
        }

        Err("Task not found")
    }

    /// Get next task to run (round-robin within priority levels)
    pub fn schedule(&mut self) -> Option<&mut Task> {
        if !self.enabled {
            return None;
        }

        self.stats.record_scheduler_call();

        // Check each priority level from highest to lowest
        for priority in (0..5).rev() {
            if let Some(mut task) = self.ready_queues[priority].pop() {
                task.set_state(ProcessState::Running);
                task.reset_time_slice();

                // If there was a previous task, put it back in ready queue
                if let Some(mut prev_task) = self.current_task.take() {
                    if !prev_task.is_terminated() {
                        prev_task.set_state(ProcessState::Ready);
                        let prev_priority = prev_task.priority.as_index();
                        let _ = self.ready_queues[prev_priority].push(prev_task);
                    }
                    self.stats.record_task_switch(false); // Involuntary switch
                }

                // Handle user space page table switching
                self.switch_user_page_table(&task);

                self.current_task = Some(task);
                return self.current_task.as_mut();
            }
        }

        // No ready tasks, run idle task
        if let Some(ref mut idle) = self.idle_task {
            idle.set_state(ProcessState::Running);
            self.stats.record_idle_cycle();
            Some(idle)
        } else {
            None
        }
    }

    /// Handle timer preemption
    pub fn handle_timer_preemption(&mut self) -> bool {
        if let Some(ref mut current) = self.current_task {
            if current.time_slice_expired() {
                self.stats.record_preemption();

                // Put current task back in ready queue
                current.set_state(ProcessState::Ready);
                let priority = current.priority.as_index();
                let task = self.current_task.take().unwrap();
                let _ = self.ready_queues[priority].push(task);

                return true; // Need to reschedule
            }
        }
        false
    }

    /// Block current task
    pub fn block_current_task(&mut self) {
        if let Some(ref mut current) = self.current_task {
            current.set_state(ProcessState::Blocked);
            self.stats.record_task_switch(true); // Voluntary switch
        }
    }

    /// Unblock a task
    pub fn unblock_task(&mut self, _task_id: TaskId) -> Result<(), &'static str> {
        // Find blocked task (not implemented - would need blocked queue)
        Err("Task blocking not fully implemented")
    }

    /// Get current task
    pub fn get_current_task(&self) -> Option<&Task> {
        self.current_task.as_ref()
    }

    /// Get mutable reference to current task
    pub fn get_current_task_mut(&mut self) -> Option<&mut Task> {
        self.current_task.as_mut()
    }

    /// Get scheduler statistics
    pub fn get_stats(&self) -> SchedulerStats {
        let mut stats = self.stats;
        
        // Update current task counts
        let ready_count: usize = self.ready_queues.iter().map(|q| q.len()).sum();
        let running_count = if self.current_task.is_some() { 1 } else { 0 };
        let blocked_count = 0; // Would need blocked queue implementation
        
        stats.update_task_counts(ready_count, blocked_count, running_count);
        stats
    }

    /// Get total task count
    pub fn get_task_count(&self) -> usize {
        let ready_count: usize = self.ready_queues.iter().map(|q| q.len()).sum();
        let current_count = if self.current_task.is_some() { 1 } else { 0 };
        ready_count + current_count
    }

    /// Get ready task count for a specific priority
    pub fn get_ready_count(&self, priority: TaskPriority) -> usize {
        self.ready_queues[priority.as_index()].len()
    }

    /// Enable/disable scheduler
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if scheduler is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Switch user page table (placeholder implementation)
    fn switch_user_page_table(&mut self, task: &Task) {
        if let Some(page_table_id) = task.get_user_page_table_id() {
            // TODO: Implement actual page table switching
            // This would involve:
            // 1. Save current page table context
            // 2. Load new page table for the task
            // 3. Flush TLB if necessary
            let _ = page_table_id; // Suppress unused warning
        }
    }

    /// Get next available task ID
    pub fn get_next_task_id(&self) -> TaskId {
        self.next_task_id
    }

    /// Find a task by ID across all queues
    pub fn find_task(&self, task_id: TaskId) -> Option<&Task> {
        // Check current task
        if let Some(ref current) = self.current_task {
            if current.id == task_id {
                return Some(current);
            }
        }

        // Check ready queues
        for queue in &self.ready_queues {
            if let Some(task) = queue.find(task_id) {
                return Some(task);
            }
        }

        // Check idle task
        if let Some(ref idle) = self.idle_task {
            if idle.id == task_id {
                return Some(idle);
            }
        }

        None
    }

    /// Get tasks by priority level
    pub fn get_tasks_by_priority(&self, priority: TaskPriority) -> &TaskList {
        &self.ready_queues[priority.as_index()]
    }
}
