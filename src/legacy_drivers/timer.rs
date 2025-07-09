// System Timer for Raspberry Pi 4/5
use core::ptr::{read_volatile, write_volatile};

// System Timer base addresses for Raspberry Pi 4/5
const TIMER_BASE: u32 = 0xFE003000;
#[allow(dead_code, clippy::identity_op)]
const TIMER_CS: u32 = TIMER_BASE + 0x00; // Control/Status
const TIMER_CLO: u32 = TIMER_BASE + 0x04; // Counter Lower 32 bits
const TIMER_CHI: u32 = TIMER_BASE + 0x08; // Counter Higher 32 bits
#[allow(dead_code)]
const TIMER_C0: u32 = TIMER_BASE + 0x0C; // Compare 0
#[allow(dead_code)]
const TIMER_C1: u32 = TIMER_BASE + 0x10; // Compare 1
#[allow(dead_code)]
const TIMER_C2: u32 = TIMER_BASE + 0x14; // Compare 2
#[allow(dead_code)]
const TIMER_C3: u32 = TIMER_BASE + 0x18; // Compare 3

// Control/Status register bits
#[allow(dead_code)]
const TIMER_CS_M0: u32 = 1 << 0; // Timer 0 matched
#[allow(dead_code)]
const TIMER_CS_M1: u32 = 1 << 1; // Timer 1 matched
#[allow(dead_code)]
const TIMER_CS_M2: u32 = 1 << 2; // Timer 2 matched
#[allow(dead_code)]
const TIMER_CS_M3: u32 = 1 << 3; // Timer 3 matched

// Timer frequency is 1MHz on Raspberry Pi
const TIMER_FREQ_HZ: u32 = 1_000_000;

pub struct SystemTimer {
    #[allow(dead_code)]
    base: u32,
}

impl SystemTimer {
    pub fn new() -> Self {
        Self { base: TIMER_BASE }
    }

    /// Get the current timer value (64-bit)
    pub fn get_time(&self) -> u64 {
        unsafe {
            let low = read_volatile(TIMER_CLO as *const u32);
            let high = read_volatile(TIMER_CHI as *const u32);
            ((high as u64) << 32) | (low as u64)
        }
    }

    /// Get the current timer value (32-bit, lower part only)
    pub fn get_time_32(&self) -> u32 {
        unsafe { read_volatile(TIMER_CLO as *const u32) }
    }

    /// Wait for a specific number of microseconds
    pub fn delay_us(&self, microseconds: u32) {
        let start = self.get_time_32();
        let target = start.wrapping_add(microseconds);

        // Handle wrap-around case
        if target < start {
            // Wait for wrap-around
            while self.get_time_32() >= start {}
        }

        // Wait until we reach the target time
        while self.get_time_32() < target {}
    }

    /// Wait for a specific number of milliseconds
    pub fn delay_ms(&self, milliseconds: u32) {
        self.delay_us(milliseconds * 1000);
    }

    /// Set up timer compare register for timer 1 (used for periodic interrupts)
    #[allow(dead_code)]
    pub fn set_timer1_compare(&self, microseconds_from_now: u32) {
        unsafe {
            let current = self.get_time_32();
            let target = current.wrapping_add(microseconds_from_now);
            write_volatile(TIMER_C1 as *mut u32, target);
        }
    }

    /// Check if timer 1 has matched (for polling)
    #[allow(dead_code)]
    pub fn timer1_matched(&self) -> bool {
        unsafe { (read_volatile(TIMER_CS as *const u32) & TIMER_CS_M1) != 0 }
    }

    /// Clear timer 1 match flag
    #[allow(dead_code)]
    pub fn clear_timer1_match(&self) {
        unsafe {
            write_volatile(TIMER_CS as *mut u32, TIMER_CS_M1);
        }
    }

    /// Get timer frequency in Hz
    #[allow(dead_code)]
    pub fn get_frequency(&self) -> u32 {
        TIMER_FREQ_HZ
    }

    /// Convert seconds to timer ticks
    #[allow(dead_code)]
    pub fn seconds_to_ticks(seconds: u32) -> u32 {
        seconds * TIMER_FREQ_HZ
    }

    /// Convert milliseconds to timer ticks
    #[allow(dead_code)]
    pub fn ms_to_ticks(milliseconds: u32) -> u32 {
        milliseconds * (TIMER_FREQ_HZ / 1000)
    }

    /// Convert timer ticks to milliseconds
    pub fn ticks_to_ms(&self, ticks: u32) -> u32 {
        ticks / (TIMER_FREQ_HZ / 1000)
    }
}
