// TinyOS Process Management Module
// Phase 3: Process Management Foundation

pub mod context;
pub mod privilege;
pub mod scheduler;

// Re-export key types and functions
pub use context::{ContextSwitchResult, ProcessContext, ProcessState};
pub use privilege::{EL0ToEL1Transition, PrivilegeLevel, PrivilegeManager};
pub use scheduler::{Scheduler, SchedulerStats, Task, TaskId};

/// Process management initialization
pub fn init_process_management() {
    privilege::init_privilege_management();
    scheduler::init_scheduler();
}

/// Process management statistics
use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
pub struct ProcessManagementStats {
    pub context_switches: AtomicU64,
    pub privilege_escalations: AtomicU64,
    pub privilege_violations: AtomicU64,
    pub tasks_created: AtomicU64,
    pub tasks_destroyed: AtomicU64,
    pub scheduler_preemptions: AtomicU64,
}

impl ProcessManagementStats {
    pub const fn new() -> Self {
        Self {
            context_switches: AtomicU64::new(0),
            privilege_escalations: AtomicU64::new(0),
            privilege_violations: AtomicU64::new(0),
            tasks_created: AtomicU64::new(0),
            tasks_destroyed: AtomicU64::new(0),
            scheduler_preemptions: AtomicU64::new(0),
        }
    }
}

/// Global process management statistics
static PROCESS_STATS: ProcessManagementStats = ProcessManagementStats::new();

/// Get process management statistics
pub fn get_process_stats() -> &'static ProcessManagementStats {
    &PROCESS_STATS
}

/// Record a context switch
pub fn record_context_switch() {
    PROCESS_STATS
        .context_switches
        .fetch_add(1, Ordering::SeqCst);
}

/// Record a privilege escalation
pub fn record_privilege_escalation() {
    PROCESS_STATS
        .privilege_escalations
        .fetch_add(1, Ordering::SeqCst);
}

/// Record a privilege violation
pub fn record_privilege_violation() {
    PROCESS_STATS
        .privilege_violations
        .fetch_add(1, Ordering::SeqCst);
}

/// Record task creation
pub fn record_task_creation() {
    PROCESS_STATS.tasks_created.fetch_add(1, Ordering::SeqCst);
}

/// Record task destruction
pub fn record_task_destruction() {
    PROCESS_STATS.tasks_destroyed.fetch_add(1, Ordering::SeqCst);
}

/// Record scheduler preemption
pub fn record_scheduler_preemption() {
    PROCESS_STATS
        .scheduler_preemptions
        .fetch_add(1, Ordering::SeqCst);
}
