//! Manager Module
//!
//! This module implements the main DeferredProcessingManager that coordinates
//! work queues, soft IRQs, and provides global access functions.

use spin::Mutex;

use super::work_item::{WorkItem, WorkFunction};
use super::work_queue::WorkQueue;
use super::softirq::{SoftIrqManager, SoftIrqType};
use super::statistics::DeferredProcessingStats;

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
    /// Create a new deferred processing manager
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

/// Global deferred processing manager instance
static DEFERRED_PROCESSING: Mutex<DeferredProcessingManager> =
    Mutex::new(DeferredProcessingManager::new());

/// Initialize deferred processing
pub fn init_deferred_processing() {
    use crate::drivers::uart::Uart;
    let mut uart = Uart::new();
    uart.init();
    uart.puts("Deferred interrupt processing initialized\r\n");
}

/// Schedule work for deferred processing
pub fn schedule_work(work_fn: WorkFunction, data: u64, context: u64) -> bool {
    DEFERRED_PROCESSING
        .lock()
        .schedule_work(work_fn, data, context)
}

/// Schedule soft IRQ work
pub fn schedule_softirq(
    soft_irq_type: SoftIrqType,
    work_fn: WorkFunction,
    data: u64,
    context: u64,
) -> bool {
    DEFERRED_PROCESSING
        .lock()
        .schedule_softirq(soft_irq_type, work_fn, data, context)
}

/// Process all pending deferred work
pub fn process_pending_work() {
    DEFERRED_PROCESSING.lock().process_deferred_work();
}

/// Check if there's pending work
pub fn has_pending_work() -> bool {
    DEFERRED_PROCESSING.lock().has_pending_work()
}

/// Get deferred processing statistics
pub fn get_deferred_stats() -> DeferredProcessingStats {
    DEFERRED_PROCESSING.lock().get_stats()
}

// Example work functions for testing

/// Test work function - timer work
fn timer_work(work_item: &mut WorkItem) {
    use crate::drivers::uart::Uart;
    let mut uart = Uart::new();
    uart.init();
    uart.puts("Timer work executed (ID: ");
    uart.put_hex(work_item.id as u64);
    uart.puts(")\r\n");
}

/// Test work function - network work  
fn network_work(work_item: &mut WorkItem) {
    use crate::drivers::uart::Uart;
    let mut uart = Uart::new();
    uart.init();
    uart.puts("Network work executed (data: ");
    uart.put_hex(work_item.data);
    uart.puts(")\r\n");
}

/// Test deferred processing functionality
pub fn test_deferred_processing() -> bool {
    use crate::drivers::uart::Uart;
    let mut uart = Uart::new();
    uart.init();
    uart.puts("Testing deferred interrupt processing...\r\n");

    // Test scheduling work
    if !schedule_work(timer_work, 0x1234, 0) {
        uart.puts("❌ Failed to schedule work\r\n");
        return false;
    }

    // Test scheduling soft IRQ work
    if !schedule_softirq(SoftIrqType::Network, network_work, 0x5678, 0) {
        uart.puts("❌ Failed to schedule soft IRQ work\r\n");
        return false;
    }

    // Test processing
    uart.puts("Processing deferred work...\r\n");
    process_pending_work();

    // Check statistics
    let stats = get_deferred_stats();
    uart.puts("Processing cycles: ");
    uart.put_hex(stats.total_processing_cycles);
    uart.puts(", Items processed: ");
    uart.put_hex(stats.total_items_processed);
    uart.puts("\r\n");

    uart.puts("✅ Deferred processing tests passed\r\n");
    true
}
