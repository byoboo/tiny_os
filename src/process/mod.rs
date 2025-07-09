// TinyOS Process Management Module
// Phase 3: Process Management Foundation

pub mod context;
pub mod privilege;
pub mod scheduler;

// Re-export key types and functions
pub use context::{ProcessContext, ProcessState, ContextSwitchResult};
pub use privilege::{PrivilegeLevel, PrivilegeManager, EL0ToEL1Transition};
pub use scheduler::{Scheduler, Task, TaskId, SchedulerStats};

/// Process management initialization
pub fn init_process_management() {
    privilege::init_privilege_management();
    scheduler::init_scheduler();
}

/// Process management statistics
#[derive(Debug, Clone, Copy)]
pub struct ProcessManagementStats {
    pub context_switches: u64,
    pub privilege_escalations: u64,
    pub privilege_violations: u64,
    pub tasks_created: u64,
    pub tasks_destroyed: u64,
    pub scheduler_preemptions: u64,
}

impl ProcessManagementStats {
    pub const fn new() -> Self {
        Self {
            context_switches: 0,
            privilege_escalations: 0,
            privilege_violations: 0,
            tasks_created: 0,
            tasks_destroyed: 0,
            scheduler_preemptions: 0,
        }
    }
}

/// Global process management statistics
static mut PROCESS_STATS: ProcessManagementStats = ProcessManagementStats::new();

/// Get process management statistics
pub fn get_process_stats() -> ProcessManagementStats {
    unsafe { PROCESS_STATS }
}

/// Record a context switch
pub fn record_context_switch() {
    unsafe {
        PROCESS_STATS.context_switches += 1;
    }
}

/// Record a privilege escalation
pub fn record_privilege_escalation() {
    unsafe {
        PROCESS_STATS.privilege_escalations += 1;
    }
}

/// Record a privilege violation
pub fn record_privilege_violation() {
    unsafe {
        PROCESS_STATS.privilege_violations += 1;
    }
}

/// Record task creation
pub fn record_task_creation() {
    unsafe {
        PROCESS_STATS.tasks_created += 1;
    }
}

/// Record task destruction
pub fn record_task_destruction() {
    unsafe {
        PROCESS_STATS.tasks_destroyed += 1;
    }
}

/// Record scheduler preemption
pub fn record_scheduler_preemption() {
    unsafe {
        PROCESS_STATS.scheduler_preemptions += 1;
    }
}
