//! Deferred Interrupt Processing for TinyOS Phase 2
//!
//! This module implements interrupt bottom-half processing, work queues,
//! and soft IRQ mechanism for performance optimization.

use crate::uart::Uart;
use spin::Mutex;

/// Maximum number of work items in the queue
const MAX_WORK_ITEMS: usize = 32;

/// Work item function type
pub type WorkFunction = fn(&mut WorkItem);

/// Work item for deferred processing
#[derive(Clone, Copy)]
pub struct WorkItem {
    /// Function to execute
    pub work_fn: Option<WorkFunction>,
    /// Data for the work function (generic u64)
    pub data: u64,
    /// Additional context data
    pub context: u64,
    /// Work item ID for tracking
    pub id: u32,
    /// Whether this work item is valid
    pub is_valid: bool,
}

impl WorkItem {
    pub const fn new(work_fn: WorkFunction, data: u64, context: u64, id: u32) -> Self {
        Self {
            work_fn: Some(work_fn),
            data,
            context,
            id,
            is_valid: true,
        }
    }

    pub const fn invalid() -> Self {
        Self {
            work_fn: None,
            data: 0,
            context: 0,
            id: 0,
            is_valid: false,
        }
    }

    /// Execute the work item
    pub fn execute(&mut self) {
        if let Some(work_fn) = self.work_fn {
            work_fn(self);
        }
    }
}

/// Work queue for deferred processing
pub struct WorkQueue {
    /// Array of work items
    items: [WorkItem; MAX_WORK_ITEMS],
    /// Head pointer (next item to process)
    head: usize,
    /// Tail pointer (next insertion point)
    tail: usize,
    /// Number of items in queue
    count: usize,
    /// Next work item ID
    next_id: u32,
    /// Statistics
    stats: WorkQueueStats,
}

impl WorkQueue {
    pub const fn new() -> Self {
        Self {
            items: [WorkItem::invalid(); MAX_WORK_ITEMS],
            head: 0,
            tail: 0,
            count: 0,
            next_id: 1,
            stats: WorkQueueStats::new(),
        }
    }

    /// Add work item to queue
    pub fn schedule_work(&mut self, work_fn: WorkFunction, data: u64, context: u64) -> bool {
        if self.count >= MAX_WORK_ITEMS {
            self.stats.queue_full_events += 1;
            return false;
        }

        let work_item = WorkItem::new(work_fn, data, context, self.next_id);
        self.next_id = self.next_id.wrapping_add(1);

        self.items[self.tail] = work_item;
        self.tail = (self.tail + 1) % MAX_WORK_ITEMS;
        self.count += 1;

        self.stats.items_scheduled += 1;
        true
    }

    /// Process one work item
    pub fn process_work(&mut self) -> bool {
        if self.count == 0 {
            return false;
        }

        let mut work_item = self.items[self.head];
        self.head = (self.head + 1) % MAX_WORK_ITEMS;
        self.count -= 1;

        if work_item.is_valid {
            work_item.execute();
            self.stats.items_processed += 1;
        }

        true
    }

    /// Process all pending work items
    pub fn process_all_work(&mut self) -> u32 {
        let mut processed = 0;

        while self.process_work() {
            processed += 1;
        }

        processed
    }

    /// Get queue statistics
    pub fn get_stats(&self) -> WorkQueueStats {
        self.stats
    }

    /// Get current queue length
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Check if queue is full
    pub fn is_full(&self) -> bool {
        self.count >= MAX_WORK_ITEMS
    }
}

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

/// Work queue statistics
#[derive(Debug, Clone, Copy)]
pub struct WorkQueueStats {
    pub items_scheduled: u64,
    pub items_processed: u64,
    pub queue_full_events: u64,
}

impl WorkQueueStats {
    pub const fn new() -> Self {
        Self {
            items_scheduled: 0,
            items_processed: 0,
            queue_full_events: 0,
        }
    }
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

/// Deferred processing statistics
#[derive(Debug, Clone, Copy)]
pub struct DeferredProcessingStats {
    pub total_processing_cycles: u64,
    pub total_items_processed: u64,
    pub max_processing_time: u64,
}

impl DeferredProcessingStats {
    pub const fn new() -> Self {
        Self {
            total_processing_cycles: 0,
            total_items_processed: 0,
            max_processing_time: 0,
        }
    }
}

/// Global deferred processing manager
static DEFERRED_PROCESSING: Mutex<DeferredProcessingManager> = Mutex::new(DeferredProcessingManager::new());

/// Initialize deferred processing
pub fn init_deferred_processing() {
    let mut uart = Uart::new();
    uart.init();
    uart.puts("Deferred interrupt processing initialized\r\n");
}

/// Schedule work for deferred processing
pub fn schedule_work(work_fn: WorkFunction, data: u64, context: u64) -> bool {
    DEFERRED_PROCESSING.lock().schedule_work(work_fn, data, context)
}

/// Schedule soft IRQ work
pub fn schedule_softirq(
    soft_irq_type: SoftIrqType,
    work_fn: WorkFunction,
    data: u64,
    context: u64,
) -> bool {
    DEFERRED_PROCESSING.lock().schedule_softirq(soft_irq_type, work_fn, data, context)
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
    let mut uart = Uart::new();
    uart.init();
    uart.puts("Timer work executed (ID: ");
    uart.put_hex(work_item.id as u64);
    uart.puts(")\r\n");
}

/// Test work function - network work
fn network_work(work_item: &mut WorkItem) {
    let mut uart = Uart::new();
    uart.init();
    uart.puts("Network work executed (data: ");
    uart.put_hex(work_item.data);
    uart.puts(")\r\n");
}

/// Test deferred processing functionality
pub fn test_deferred_processing() -> bool {
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
