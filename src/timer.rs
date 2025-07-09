// TinyOS Timer Module
// Simple timer implementation for process management

/// Global timer counter
static mut TIMER_COUNTER: u64 = 0;

/// Initialize timer
pub fn init() {
    // TODO: Initialize hardware timer
    unsafe {
        TIMER_COUNTER = 0;
    }
}

/// Get current system time (in arbitrary units)
pub fn get_system_time() -> u64 {
    unsafe {
        TIMER_COUNTER += 1;
        TIMER_COUNTER
    }
}

/// Update timer (called from interrupt handler)
pub fn update_timer() {
    unsafe {
        TIMER_COUNTER += 1;
    }
}

/// Get elapsed time since start
pub fn get_elapsed_time() -> u64 {
    unsafe { TIMER_COUNTER }
}
