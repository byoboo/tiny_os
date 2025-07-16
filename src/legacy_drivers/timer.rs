// TinyOS Timer Module
// Simple timer implementation for process management

use core::sync::atomic::{AtomicU64, Ordering};
use core::ptr::{read_volatile, write_volatile};

/// System Timer base address (BCM2835/BCM2711)
const TIMER_BASE: usize = 0x3F00_3000;

/// Timer registers
const TIMER_CS: usize = TIMER_BASE + 0x00; // Control/Status
const TIMER_CLO: usize = TIMER_BASE + 0x04; // Counter Lower 32 bits
const TIMER_CHI: usize = TIMER_BASE + 0x08; // Counter Higher 32 bits
const TIMER_C0: usize = TIMER_BASE + 0x0C; // Compare 0
const TIMER_C1: usize = TIMER_BASE + 0x10; // Compare 1
const TIMER_C2: usize = TIMER_BASE + 0x14; // Compare 2
const TIMER_C3: usize = TIMER_BASE + 0x18; // Compare 3

/// Timer control bits
const TIMER_CS_M0: u32 = 1 << 0; // Timer 0 match
const TIMER_CS_M1: u32 = 1 << 1; // Timer 1 match
const TIMER_CS_M2: u32 = 1 << 2; // Timer 2 match
const TIMER_CS_M3: u32 = 1 << 3; // Timer 3 match

/// Global timer counter
static TIMER_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Initialize timer
pub fn init() {
    // Initialize hardware timer
    unsafe {
        // Clear all timer interrupts
        write_volatile(TIMER_CS as *mut u32, TIMER_CS_M0 | TIMER_CS_M1 | TIMER_CS_M2 | TIMER_CS_M3);
        
        // Set up timer for 1ms intervals (1000 Hz)
        // ARM timer runs at 1MHz, so 1000 ticks = 1ms
        let current_time = get_hardware_time();
        write_volatile(TIMER_C1 as *mut u32, (current_time + 1000) as u32);
    }
    
    TIMER_COUNTER.store(0, Ordering::SeqCst);
}

/// Get current hardware timer value (64-bit)
fn get_hardware_time() -> u64 {
    unsafe {
        let hi1 = read_volatile(TIMER_CHI as *const u32);
        let lo = read_volatile(TIMER_CLO as *const u32);
        let hi2 = read_volatile(TIMER_CHI as *const u32);
        
        // Handle rollover - if high changed, re-read low
        if hi1 != hi2 {
            let lo2 = read_volatile(TIMER_CLO as *const u32);
            ((hi2 as u64) << 32) | (lo2 as u64)
        } else {
            ((hi1 as u64) << 32) | (lo as u64)
        }
    }
}

/// Get current system time (in microseconds)
pub fn get_system_time() -> u64 {
    get_hardware_time()
}

/// Update timer (called from interrupt handler)
pub fn update_timer() {
    TIMER_COUNTER.fetch_add(1, Ordering::SeqCst);
    
    // Acknowledge timer interrupt and set next interrupt
    unsafe {
        write_volatile(TIMER_CS as *mut u32, TIMER_CS_M1);
        let current_time = get_hardware_time();
        write_volatile(TIMER_C1 as *mut u32, (current_time + 1000) as u32);
    }
}

/// Get elapsed time since start (in timer ticks)
pub fn get_elapsed_time() -> u64 {
    TIMER_COUNTER.load(Ordering::SeqCst)
}

/// Get current time in milliseconds
pub fn get_time_ms() -> u64 {
    get_hardware_time() / 1000
}

/// Delay for specified microseconds
pub fn delay_us(us: u64) {
    let start = get_hardware_time();
    while get_hardware_time() - start < us {
        // Busy wait
    }
}

/// Delay for specified milliseconds
pub fn delay_ms(ms: u64) {
    delay_us(ms * 1000);
}
