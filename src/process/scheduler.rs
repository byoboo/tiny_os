// TinyOS Basic Task Scheduler
// Phase 3.3: Basic Task Scheduler

use crate::process::context::{ProcessContext, ProcessState};

/// Task ID type
pub type TaskId = u32;

/// Simple array-based queue for tasks (no_std compatible)
#[derive(Debug, Clone)]
pub struct TaskQueue {
    tasks: [Option<Task>; 16], // Maximum 16 tasks per priority level
    head: usize,
    tail: usize,
    count: usize,
}

impl TaskQueue {
    pub const fn new() -> Self {
        const NONE_TASK: Option<Task> = None;
        Self {
            tasks: [NONE_TASK; 16],
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn push_back(&mut self, task: Task) -> Result<(), &'static str> {
        if self.count >= 16 {
            return Err("Task queue full");
        }

        self.tasks[self.tail] = Some(task);
        self.tail = (self.tail + 1) % 16;
        self.count += 1;
        Ok(())
    }

    pub fn pop_front(&mut self) -> Option<Task> {
        if self.count == 0 {
            return None;
        }

        let task = self.tasks[self.head].take();
        self.head = (self.head + 1) % 16;
        self.count -= 1;
        task
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn iter(&self) -> TaskQueueIter {
        TaskQueueIter {
            queue: self,
            index: 0,
            current: self.head,
        }
    }

    pub fn remove(&mut self, task_id: TaskId) -> Option<Task> {
        for i in 0..self.count {
            let pos = (self.head + i) % 16;
            if let Some(ref task) = self.tasks[pos] {
                if task.id == task_id {
                    let removed_task = self.tasks[pos].take();

                    // Shift remaining tasks
                    for j in i..self.count.saturating_sub(1) {
                        let current_pos = (self.head + j) % 16;
                        let next_pos = (self.head + j + 1) % 16;
                        self.tasks[current_pos] = self.tasks[next_pos].take();
                    }

                    self.count -= 1;
                    if self.count == 0 {
                        self.head = 0;
                        self.tail = 0;
                    } else {
                        self.tail = if self.tail == 0 { 15 } else { self.tail - 1 };
                    }

                    return removed_task;
                }
            }
        }
        None
    }
}

pub struct TaskQueueIter<'a> {
    queue: &'a TaskQueue,
    index: usize,
    current: usize,
}

impl<'a> Iterator for TaskQueueIter<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.queue.count {
            return None;
        }

        if let Some(ref task) = self.queue.tasks[self.current] {
            self.current = (self.current + 1) % 16;
            self.index += 1;
            Some(task)
        } else {
            None
        }
    }
}

/// Get current system time (placeholder implementation)
fn get_system_time() -> u64 {
    // TODO: Implement actual system time
    // For now, return a simple counter
    static mut TIME_COUNTER: u64 = 0;
    unsafe {
        TIME_COUNTER += 1;
        TIME_COUNTER
    }
}

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    /// Idle task priority
    Idle = 0,
    /// Low priority
    Low = 1,
    /// Normal priority
    Normal = 2,
    /// High priority  
    High = 3,
    /// Real-time priority
    RealTime = 4,
}

impl TaskPriority {
    /// Get default time slice for this priority
    pub fn default_time_slice(self) -> u32 {
        match self {
            TaskPriority::Idle => 100,
            TaskPriority::Low => 500,
            TaskPriority::Normal => 1000,
            TaskPriority::High => 2000,
            TaskPriority::RealTime => 5000,
        }
    }
}

/// Task structure
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

        let kernel_stack = stack_base + stack_size;
        let user_stack = stack_base + (stack_size / 2);

        let mut context = ProcessContext::new(id, user_stack, kernel_stack, entry_point);
        context.priority = priority as u8;
        context.set_time_slice(priority.default_time_slice());

        Self {
            id,
            name: task_name,
            priority,
            context,
            entry_point,
            stack_base,
            stack_size,
            creation_time: get_system_time(),
            run_time: 0,
            last_run: 0,
            flags: 0,
            user_page_table_id: None,
        }
    }

    /// Get task name as string
    pub fn get_name(&self) -> &str {
        let end = self
            .name
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.name.len());
        core::str::from_utf8(&self.name[..end]).unwrap_or("<invalid>")
    }

    /// Check if task is ready to run
    pub fn is_ready(&self) -> bool {
        self.context.is_ready()
    }

    /// Check if task is running
    pub fn is_running(&self) -> bool {
        self.context.is_running()
    }

    /// Check if task is blocked
    pub fn is_blocked(&self) -> bool {
        self.context.is_blocked()
    }

    /// Check if task is terminated
    pub fn is_terminated(&self) -> bool {
        self.context.is_terminated()
    }

    /// Set task state
    pub fn set_state(&mut self, state: ProcessState) {
        self.context.set_state(state);
    }

    /// Get task state
    pub fn get_state(&self) -> ProcessState {
        self.context.get_state()
    }

    /// Update run time
    pub fn update_run_time(&mut self, time: u64) {
        self.run_time += time;
        self.last_run = get_system_time();
    }

    /// Reset time slice
    pub fn reset_time_slice(&mut self) {
        self.context
            .set_time_slice(self.priority.default_time_slice());
    }

    /// Check if time slice expired
    pub fn time_slice_expired(&mut self) -> bool {
        self.context.decrement_time_slice()
    }

    /// Set user space page table ID
    pub fn set_user_page_table_id(&mut self, page_table_id: usize) {
        self.user_page_table_id = Some(page_table_id);
    }

    /// Get user space page table ID
    pub fn get_user_page_table_id(&self) -> Option<usize> {
        self.user_page_table_id
    }

    /// Check if task has user space page table
    pub fn has_user_page_table(&self) -> bool {
        self.user_page_table_id.is_some()
    }

    /// Clear user space page table ID
    pub fn clear_user_page_table_id(&mut self) {
        self.user_page_table_id = None;
    }
}

/// Scheduler statistics
#[derive(Debug, Clone, Copy)]
pub struct SchedulerStats {
    /// Total context switches
    pub context_switches: u64,

    /// Total preemptions
    pub preemptions: u64,

    /// Tasks created
    pub tasks_created: u64,

    /// Tasks destroyed
    pub tasks_destroyed: u64,

    /// Scheduler invocations
    pub scheduler_calls: u64,

    /// Idle time
    pub idle_time: u64,

    /// Total run time
    pub total_run_time: u64,
}

impl SchedulerStats {
    pub const fn new() -> Self {
        Self {
            context_switches: 0,
            preemptions: 0,
            tasks_created: 0,
            tasks_destroyed: 0,
            scheduler_calls: 0,
            idle_time: 0,
            total_run_time: 0,
        }
    }
}

/// Simple task list for no_std environment
#[derive(Debug)]
struct TaskList {
    tasks: [Option<Task>; 32],
    count: usize,
}

impl TaskList {
    pub const fn new() -> Self {
        const NONE_TASK: Option<Task> = None;
        Self {
            tasks: [NONE_TASK; 32],
            count: 0,
        }
    }

    pub fn push(&mut self, task: Task) -> Result<(), &'static str> {
        if self.count >= 32 {
            return Err("Task list full");
        }

        for slot in &mut self.tasks {
            if slot.is_none() {
                *slot = Some(task);
                self.count += 1;
                return Ok(());
            }
        }

        Err("No free slots")
    }

    pub fn pop(&mut self) -> Option<Task> {
        for slot in &mut self.tasks {
            if let Some(task) = slot.take() {
                self.count -= 1;
                return Some(task);
            }
        }
        None
    }

    pub fn remove(&mut self, task_id: TaskId) -> Option<Task> {
        for slot in &mut self.tasks {
            if let Some(task) = slot {
                if task.id == task_id {
                    let removed_task = slot.take().unwrap();
                    self.count -= 1;
                    return Some(removed_task);
                }
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.count
    }
}

/// Basic round-robin scheduler
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
        let priority_index = priority as usize;
        if self.ready_queues[priority_index].push(task).is_ok() {
            self.stats.tasks_created += 1;
            crate::process::record_task_creation();
        }

        task_id
    }

    /// Destroy a task
    pub fn destroy_task(&mut self, task_id: TaskId) -> Result<(), &'static str> {
        // Remove from ready queues
        for queue in &mut self.ready_queues {
            if queue.remove(task_id).is_some() {
                self.stats.tasks_destroyed += 1;
                crate::process::record_task_destruction();
                return Ok(());
            }
        }

        // Check if it's the current task
        if let Some(ref current) = self.current_task {
            if current.id == task_id {
                self.current_task = None;
                self.stats.tasks_destroyed += 1;
                crate::process::record_task_destruction();
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

        self.stats.scheduler_calls += 1;

        // Check each priority level from highest to lowest
        for priority in (0..5).rev() {
            if let Some(mut task) = self.ready_queues[priority].pop() {
                task.set_state(ProcessState::Running);
                task.reset_time_slice();

                // If there was a previous task, put it back in ready queue
                if let Some(mut prev_task) = self.current_task.take() {
                    if !prev_task.is_terminated() {
                        prev_task.set_state(ProcessState::Ready);
                        let prev_priority = prev_task.priority as usize;
                        let _ = self.ready_queues[prev_priority].push(prev_task);
                    }
                    self.stats.context_switches += 1;
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
            self.stats.idle_time += 1;
            Some(idle)
        } else {
            None
        }
    }

    /// Handle timer preemption
    pub fn handle_timer_preemption(&mut self) -> bool {
        if let Some(ref mut current) = self.current_task {
            if current.time_slice_expired() {
                self.stats.preemptions += 1;
                crate::process::record_scheduler_preemption();

                // Put current task back in ready queue
                current.set_state(ProcessState::Ready);
                let priority = current.priority as usize;
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

    /// Get current task (mutable)
    pub fn get_current_task_mut(&mut self) -> Option<&mut Task> {
        self.current_task.as_mut()
    }

    /// Get scheduler statistics
    pub fn get_stats(&self) -> SchedulerStats {
        self.stats
    }

    /// Get task count
    pub fn get_task_count(&self) -> usize {
        let mut count = 0;
        for queue in &self.ready_queues {
            count += queue.len();
        }
        if self.current_task.is_some() {
            count += 1;
        }
        count
    }

    /// Enable/disable scheduler
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if scheduler is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Switch user space page table for a task
    fn switch_user_page_table(&mut self, task: &Task) {
        if let Some(page_table_id) = task.get_user_page_table_id() {
            // Get user space manager and activate the page table
            if let Some(manager) = crate::memory::get_user_space_manager() {
                if let Err(e) = manager.activate_page_table(page_table_id) {
                    // Handle error - for now just continue without switching
                    // In a real implementation, might want to log this
                    let _ = e; // Suppress unused variable warning
                }
            }
        }
    }
}

/// Global scheduler instance
static mut SCHEDULER: Scheduler = Scheduler::new();

/// Initialize scheduler
pub fn init_scheduler() {
    unsafe {
        SCHEDULER.init();
    }
}

/// Create a new task
pub fn create_task(
    name: &str,
    priority: TaskPriority,
    entry_point: u64,
    stack_base: u64,
    stack_size: u64,
) -> TaskId {
    unsafe { SCHEDULER.create_task(name, priority, entry_point, stack_base, stack_size) }
}

/// Destroy a task
pub fn destroy_task(task_id: TaskId) -> Result<(), &'static str> {
    unsafe { SCHEDULER.destroy_task(task_id) }
}

/// Schedule next task
pub fn schedule() -> Option<TaskId> {
    unsafe { SCHEDULER.schedule().map(|task| task.id) }
}

/// Handle timer preemption
pub fn handle_timer_preemption() -> bool {
    unsafe { SCHEDULER.handle_timer_preemption() }
}

/// Block current task
pub fn block_current_task() {
    unsafe { SCHEDULER.block_current_task() }
}

/// Unblock a task
pub fn unblock_task(task_id: TaskId) -> Result<(), &'static str> {
    unsafe { SCHEDULER.unblock_task(task_id) }
}

/// Get current task ID
pub fn get_current_task_id() -> Option<TaskId> {
    unsafe { SCHEDULER.get_current_task().map(|task| task.id) }
}

/// Get scheduler statistics
pub fn get_scheduler_stats() -> SchedulerStats {
    unsafe { SCHEDULER.get_stats() }
}

/// Get task count
pub fn get_task_count() -> usize {
    unsafe { SCHEDULER.get_task_count() }
}

/// Enable/disable scheduler
pub fn set_scheduler_enabled(enabled: bool) {
    unsafe { SCHEDULER.set_enabled(enabled) }
}

/// Check if scheduler is enabled
pub fn is_scheduler_enabled() -> bool {
    unsafe { SCHEDULER.is_enabled() }
}
