// TinyOS Timer Module
// Simple timer implementation for process management

use core::sync::atomic::{AtomicU64, Ordering};

/// Global timer counter
static TIMER_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Initialize timer
pub fn init() {
    // TODO: Initialize hardware timer
    TIMER_COUNTER.store(0, Ordering::SeqCst);
}

/// Get current system time (in arbitrary units)
pub fn get_system_time() -> u64 {
    TIMER_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// Update timer (called from interrupt handler)
pub fn update_timer() {
    TIMER_COUNTER.fetch_add(1, Ordering::SeqCst);
}

/// Get elapsed time since start
pub fn get_elapsed_time() -> u64 {
    TIMER_COUNTER.load(Ordering::SeqCst)
}
