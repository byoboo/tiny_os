//! Scheduler Statistics and Performance Tracking
//!
//! This module provides statistics collection and performance
//! monitoring for the task scheduler.

/// Scheduler performance statistics
#[derive(Debug, Clone, Copy)]
pub struct SchedulerStats {
    /// Total number of task switches
    pub task_switches: u64,
    
    /// Total number of tasks created
    pub tasks_created: u64,
    
    /// Total number of tasks destroyed
    pub tasks_destroyed: u64,
    
    /// Number of preemptions due to time slice expiration
    pub preemptions: u64,
    
    /// Number of voluntary context switches (blocking)
    pub voluntary_switches: u64,
    
    /// Number of involuntary context switches (preemption)
    pub involuntary_switches: u64,
    
    /// Total scheduler run time
    pub total_run_time: u64,
    
    /// Number of idle cycles
    pub idle_cycles: u64,
    
    /// Current number of ready tasks
    pub ready_tasks: usize,
    
    /// Current number of blocked tasks
    pub blocked_tasks: usize,
    
    /// Current number of running tasks (should be 0 or 1)
    pub running_tasks: usize,
    
    // Backward compatibility fields
    /// Context switches (alias for task_switches)
    pub context_switches: u64,
    
    /// Scheduler calls
    pub scheduler_calls: u64,
    
    /// Idle time
    pub idle_time: u64,
}

impl SchedulerStats {
    /// Create new scheduler statistics with default values
    pub const fn new() -> Self {
        Self {
            task_switches: 0,
            tasks_created: 0,
            tasks_destroyed: 0,
            preemptions: 0,
            voluntary_switches: 0,
            involuntary_switches: 0,
            total_run_time: 0,
            idle_cycles: 0,
            ready_tasks: 0,
            blocked_tasks: 0,
            running_tasks: 0,
            context_switches: 0,
            scheduler_calls: 0,
            idle_time: 0,
        }
    }

    /// Record a task switch
    pub fn record_task_switch(&mut self, voluntary: bool) {
        self.task_switches += 1;
        self.context_switches += 1; // Backward compatibility
        if voluntary {
            self.voluntary_switches += 1;
        } else {
            self.involuntary_switches += 1;
        }
    }

    /// Record a task creation
    pub fn record_task_created(&mut self) {
        self.tasks_created += 1;
    }

    /// Record a task destruction
    pub fn record_task_destroyed(&mut self) {
        self.tasks_destroyed += 1;
    }

    /// Record a preemption
    pub fn record_preemption(&mut self) {
        self.preemptions += 1;
        self.involuntary_switches += 1;
    }

    /// Record idle cycles
    pub fn record_idle_cycle(&mut self) {
        self.idle_cycles += 1;
        self.idle_time += 1; // Backward compatibility
    }

    /// Record a scheduler call
    pub fn record_scheduler_call(&mut self) {
        self.scheduler_calls += 1;
    }

    /// Update task counts
    pub fn update_task_counts(&mut self, ready: usize, blocked: usize, running: usize) {
        self.ready_tasks = ready;
        self.blocked_tasks = blocked;
        self.running_tasks = running;
    }

    /// Update total run time
    pub fn update_run_time(&mut self, time: u64) {
        self.total_run_time = time;
    }

    /// Get current active tasks
    pub fn get_active_tasks(&self) -> usize {
        self.ready_tasks + self.blocked_tasks + self.running_tasks
    }

    /// Get task switch efficiency (voluntary vs involuntary ratio)
    pub fn get_switch_efficiency(&self) -> f32 {
        if self.task_switches == 0 {
            return 1.0;
        }
        self.voluntary_switches as f32 / self.task_switches as f32
    }

    /// Get preemption rate (preemptions per task switch)
    pub fn get_preemption_rate(&self) -> f32 {
        if self.task_switches == 0 {
            return 0.0;
        }
        self.preemptions as f32 / self.task_switches as f32
    }

    /// Reset all statistics
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
