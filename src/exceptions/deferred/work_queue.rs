//! Work Queue Module
//!
//! This module implements work queue management for deferred interrupt
//! processing, providing efficient queuing and processing of work items.

use super::work_item::{WorkFunction, WorkItem, MAX_WORK_ITEMS};

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
    /// Create a new work queue
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
