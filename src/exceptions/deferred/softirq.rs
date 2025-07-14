//! Soft IRQ Module
//!
//! This module implements soft IRQ management for deferred interrupt processing,
//! providing prioritized handling of interrupt bottom-halves.

use super::work_item::WorkFunction;
use super::work_queue::WorkQueue;

/// Soft IRQ types
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SoftIrqType {
    Timer = 0,
    Network = 1,
    Block = 2,
    Tasklet = 3,
    Scheduler = 4,
}

/// Soft IRQ statistics
#[derive(Debug, Clone, Copy)]
pub struct SoftIrqStats {
    pub softirqs_raised: u64,
    pub softirqs_processed: u64,
}

impl SoftIrqStats {
    pub const fn new() -> Self {
        Self {
            softirqs_raised: 0,
            softirqs_processed: 0,
        }
    }
}

/// Soft IRQ manager
pub struct SoftIrqManager {
    /// Pending soft IRQs bitmask
    pending: u32,
    /// Work queues for each soft IRQ type
    work_queues: [WorkQueue; 5],
    /// Statistics
    stats: SoftIrqStats,
}

impl SoftIrqManager {
    /// Create a new soft IRQ manager
    pub const fn new() -> Self {
        Self {
            pending: 0,
            work_queues: [
                WorkQueue::new(),
                WorkQueue::new(),
                WorkQueue::new(),
                WorkQueue::new(),
                WorkQueue::new(),
            ],
            stats: SoftIrqStats::new(),
        }
    }

    /// Raise a soft IRQ
    pub fn raise_softirq(&mut self, soft_irq_type: SoftIrqType) {
        let bit = 1 << (soft_irq_type as u32);
        if (self.pending & bit) == 0 {
            self.pending |= bit;
            self.stats.softirqs_raised += 1;
        }
    }

    /// Schedule work for a soft IRQ
    pub fn schedule_softirq_work(
        &mut self,
        soft_irq_type: SoftIrqType,
        work_fn: WorkFunction,
        data: u64,
        context: u64,
    ) -> bool {
        let queue_index = soft_irq_type as usize;
        if queue_index < self.work_queues.len() {
            let success = self.work_queues[queue_index].schedule_work(work_fn, data, context);
            if success {
                self.raise_softirq(soft_irq_type);
            }
            success
        } else {
            false
        }
    }

    /// Process pending soft IRQs
    pub fn process_softirqs(&mut self) -> u32 {
        let mut processed = 0;

        for i in 0..5 {
            let bit = 1 << i;
            if (self.pending & bit) != 0 {
                // Process this soft IRQ
                let items_processed = self.work_queues[i].process_all_work();
                if items_processed > 0 {
                    processed += items_processed;
                    self.stats.softirqs_processed += 1;
                }

                // Clear the pending bit if queue is empty
                if self.work_queues[i].is_empty() {
                    self.pending &= !bit;
                }
            }
        }

        processed
    }

    /// Check if any soft IRQs are pending
    pub fn has_pending_softirqs(&self) -> bool {
        self.pending != 0
    }

    /// Get soft IRQ statistics
    pub fn get_stats(&self) -> SoftIrqStats {
        self.stats
    }
}
