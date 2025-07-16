//! Public API for Deferred Processing
//!
//! This module provides the public interface for the deferred processing
//! system, including initialization, work scheduling, and statistics.

use spin::Mutex;

use super::{
    deferred_manager::DeferredProcessingManager,
    deferred_stats::DeferredProcessingStats,
    softirq::SoftIrqType,
    work_item::{WorkFunction, WorkItem},
};
use crate::uart::Uart;

/// Global deferred processing manager
static DEFERRED_PROCESSING: Mutex<DeferredProcessingManager> =
    Mutex::new(DeferredProcessingManager::new());

/// Initialize deferred processing
pub fn init_deferred_processing() {
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
