//! Deferred Processing Manager Core
//!
//! This module provides the main manager that coordinates work queues
//! and soft IRQs for the deferred processing system.

use super::{
    deferred_stats::DeferredProcessingStats,
    softirq::{SoftIrqManager, SoftIrqType},
    work_item::WorkFunction,
    work_queue::WorkQueue,
};

/// Global deferred processing manager
pub struct DeferredProcessingManager {
    /// Main work queue
    main_work_queue: WorkQueue,
    /// Soft IRQ manager
    softirq_manager: SoftIrqManager,
    /// Processing statistics
    stats: DeferredProcessingStats,
}

impl DeferredProcessingManager {
    pub const fn new() -> Self {
        Self {
            main_work_queue: WorkQueue::new(),
            softirq_manager: SoftIrqManager::new(),
            stats: DeferredProcessingStats::new(),
        }
    }

    /// Schedule deferred work
    pub fn schedule_work(&mut self, work_fn: WorkFunction, data: u64, context: u64) -> bool {
        self.main_work_queue.schedule_work(work_fn, data, context)
    }

    /// Schedule soft IRQ work
    pub fn schedule_softirq(
        &mut self,
        soft_irq_type: SoftIrqType,
        work_fn: WorkFunction,
        data: u64,
        context: u64,
    ) -> bool {
        self.softirq_manager
            .schedule_softirq_work(soft_irq_type, work_fn, data, context)
    }

    /// Process all deferred work
    pub fn process_deferred_work(&mut self) {
        let start_time = self.get_timestamp();

        // Process main work queue
        let main_processed = self.main_work_queue.process_all_work();

        // Process soft IRQs
        let softirq_processed = self.softirq_manager.process_softirqs();

        let end_time = self.get_timestamp();
        let processing_time = end_time.wrapping_sub(start_time);

        self.stats.total_processing_cycles += 1;
        self.stats.total_items_processed += (main_processed + softirq_processed) as u64;

        if processing_time > self.stats.max_processing_time {
            self.stats.max_processing_time = processing_time;
        }
    }

    /// Check if there's work to be done
    pub fn has_pending_work(&self) -> bool {
        !self.main_work_queue.is_empty() || self.softirq_manager.has_pending_softirqs()
    }

    /// Get processing statistics
    pub fn get_stats(&self) -> DeferredProcessingStats {
        self.stats
    }

    /// Simple timestamp function (using a counter for now)
    fn get_timestamp(&self) -> u64 {
        // TODO: Use actual timer when available
        use core::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        COUNTER.fetch_add(1, Ordering::SeqCst)
    }
}
