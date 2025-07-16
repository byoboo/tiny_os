//! Real-time Scheduler
//!
//! Microsecond-precision real-time task scheduling
//! Extracted from week6_security.rs

use core::ptr::{read_volatile, write_volatile};

use super::{RealTimeMetrics, SecurityError};

/// Generic Timer Physical Base
const GENERIC_TIMER_BASE: usize = 0xFF840000;

/// Real-time Priority Classes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RtPriority {
    /// System critical (highest)
    SystemCritical = 0,
    /// Hardware interrupt handling
    HardwareInterrupt = 1,
    /// Real-time tasks
    RealTime = 2,
    /// High priority system tasks
    HighPriority = 3,
    /// Normal priority tasks
    Normal = 4,
    /// Low priority background tasks
    Background = 5,
}

/// Real-time task descriptor
#[derive(Clone)]
pub struct RtTask {
    pub id: u32,
    pub priority: RtPriority,
    pub deadline_us: u64,
    pub period_us: u64,
    pub execution_time_us: u64,
    pub last_execution: u64,
    pub missed_deadlines: u32,
}

impl RtTask {
    pub fn new(id: u32, priority: RtPriority, deadline_us: u64, period_us: u64) -> Self {
        Self {
            id,
            priority,
            deadline_us,
            period_us,
            execution_time_us: 0,
            last_execution: 0,
            missed_deadlines: 0,
        }
    }
}

/// Real-time scheduler for microsecond-precision timing
pub struct RealTimeScheduler {
    timer_base: usize,
    tasks: [Option<RtTask>; 32], // Support up to 32 RT tasks
    task_count: usize,
    current_task: Option<u32>,
    metrics: RealTimeMetrics,
}

impl RealTimeScheduler {
    pub fn new() -> Self {
        Self {
            timer_base: GENERIC_TIMER_BASE,
            tasks: [const { None }; 32],
            task_count: 0,
            current_task: None,
            metrics: RealTimeMetrics::default(),
        }
    }

    /// Initialize real-time scheduler
    pub fn init(&mut self) -> Result<(), SecurityError> {
        unsafe {
            // Configure generic timer for microsecond precision
            let timer_control = self.timer_base + 0x00;
            write_volatile(timer_control as *mut u32, 0x0000_0001); // Enable timer

            // Set timer frequency (assuming 1MHz for microsecond precision)
            let timer_freq = self.timer_base + 0x08;
            write_volatile(timer_freq as *mut u32, 1_000_000); // 1MHz
        }

        Ok(())
    }

    /// Add real-time task
    pub fn add_task(&mut self, task: RtTask) -> Result<(), SecurityError> {
        if self.task_count >= 32 {
            return Err(SecurityError::ConfigurationError);
        }

        self.tasks[self.task_count] = Some(task);
        self.task_count += 1;

        Ok(())
    }

    /// Remove real-time task
    pub fn remove_task(&mut self, task_id: u32) -> Result<(), SecurityError> {
        for i in 0..self.task_count {
            if let Some(ref task) = self.tasks[i] {
                if task.id == task_id {
                    self.tasks[i] = None;

                    // Compact the array
                    for j in i..self.task_count - 1 {
                        self.tasks[j] = self.tasks[j + 1].take();
                    }
                    self.task_count -= 1;
                    return Ok(());
                }
            }
        }

        Err(SecurityError::ConfigurationError)
    }

    /// Get current time in microseconds
    pub fn get_current_time_us(&self) -> u64 {
        unsafe {
            let timer_value = self.timer_base + 0x04;
            read_volatile(timer_value as *const u64)
        }
    }

    /// Schedule next task (Earliest Deadline First)
    pub fn schedule_next(&mut self) -> Option<u32> {
        let current_time = self.get_current_time_us();
        let mut next_task_id = None;
        let mut earliest_deadline = u64::MAX;

        // Find task with earliest deadline
        for i in 0..self.task_count {
            if let Some(ref task) = self.tasks[i] {
                let next_deadline = task.last_execution + task.period_us;

                if next_deadline <= current_time && next_deadline < earliest_deadline {
                    earliest_deadline = next_deadline;
                    next_task_id = Some(task.id);
                }
            }
        }

        if let Some(task_id) = next_task_id {
            self.current_task = Some(task_id);
            self.metrics.task_switches += 1;
        }

        next_task_id
    }

    /// Mark task as completed
    pub fn complete_task(
        &mut self,
        task_id: u32,
        execution_time_us: u64,
    ) -> Result<(), SecurityError> {
        let current_time = self.get_current_time_us();

        for i in 0..self.task_count {
            if let Some(ref mut task) = self.tasks[i] {
                if task.id == task_id {
                    task.last_execution = current_time;
                    task.execution_time_us = execution_time_us;

                    // Check if deadline was missed
                    if current_time > task.last_execution + task.deadline_us {
                        task.missed_deadlines += 1;
                        self.metrics.missed_deadlines += 1;
                    }

                    // Update metrics
                    self.metrics.average_latency_us =
                        (self.metrics.average_latency_us + execution_time_us) / 2;

                    if execution_time_us > self.metrics.max_latency_us {
                        self.metrics.max_latency_us = execution_time_us;
                    }

                    return Ok(());
                }
            }
        }

        Err(SecurityError::ConfigurationError)
    }

    /// Get real-time metrics
    pub fn get_metrics(&self) -> &RealTimeMetrics {
        &self.metrics
    }

    /// Get task by ID
    pub fn get_task(&self, task_id: u32) -> Option<&RtTask> {
        for i in 0..self.task_count {
            if let Some(ref task) = self.tasks[i] {
                if task.id == task_id {
                    return Some(task);
                }
            }
        }
        None
    }

    /// Get current task ID
    pub fn get_current_task(&self) -> Option<u32> {
        self.current_task
    }

    /// Run scheduler analysis
    pub fn analyze_schedulability(&self) -> Result<bool, SecurityError> {
        let mut total_utilization = 0.0;

        // Calculate total CPU utilization
        for i in 0..self.task_count {
            if let Some(ref task) = self.tasks[i] {
                let utilization = task.execution_time_us as f64 / task.period_us as f64;
                total_utilization += utilization;
            }
        }

        // Check if system is schedulable (utilization < 1.0)
        Ok(total_utilization < 1.0)
    }
}
