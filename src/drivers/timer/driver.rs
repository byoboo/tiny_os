//! High-level Timer Driver API
//!
//! This module provides a safe, high-level interface to the System Timer
//! peripheral with delay functions, time measurement, and timer channels.

use super::hardware::{frequency, TimerHardware};
use crate::drivers::{
    config::{DefaultHardware, HardwareVersion},
    traits::{DriverError, DriverStatus, Initialize, Status},
};

/// Timer driver configuration
#[derive(Debug, Clone, Copy)]
pub struct TimerConfig {
    /// Enable timer channels (bit mask for channels 0-3)
    pub enabled_channels: u8,
}

impl Default for TimerConfig {
    fn default() -> Self {
        Self {
            enabled_channels: 0b0010, // Enable channel 1 by default
        }
    }
}

/// High-level timer driver
pub struct TimerDriver<H: HardwareVersion = DefaultHardware> {
    hardware: TimerHardware<H>,
    status: DriverStatus,
}

impl<H: HardwareVersion> TimerDriver<H> {
    /// Create a new timer driver instance
    pub const fn new() -> Self {
        Self {
            hardware: TimerHardware::new(),
            status: DriverStatus::Uninitialized,
        }
    }

    /// Get the current timer value (64-bit)
    #[inline]
    pub fn get_time(&self) -> u64 {
        self.hardware.get_time_64()
    }

    /// Get the current timer value (32-bit, lower part only)
    #[inline]
    pub fn get_time_32(&self) -> u32 {
        self.hardware.get_time_32()
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
    #[inline]
    pub fn delay_ms(&self, milliseconds: u32) {
        self.delay_us(milliseconds * 1000);
    }

    /// Wait for a specific number of seconds
    #[inline]
    pub fn delay_s(&self, seconds: u32) {
        self.delay_us(seconds * frequency::US_PER_SEC);
    }

    /// Set up timer compare register for a channel
    pub fn set_timer_compare(
        &self,
        channel: u8,
        microseconds_from_now: u32,
    ) -> Result<(), DriverError> {
        if channel > 3 {
            return Err(DriverError::InvalidInput);
        }

        let current = self.get_time_32();
        let target = current.wrapping_add(microseconds_from_now);
        self.hardware.set_compare(channel, target);
        Ok(())
    }

    /// Check if timer channel has matched
    #[inline]
    pub fn has_timer_matched(&self, channel: u8) -> bool {
        self.hardware.has_matched(channel)
    }

    /// Clear timer channel match flag
    #[inline]
    pub fn clear_timer_match(&self, channel: u8) {
        self.hardware.clear_match(channel);
    }

    /// Get timer frequency in Hz
    #[inline]
    pub fn get_frequency(&self) -> u32 {
        frequency::TIMER_FREQ_HZ
    }

    /// Convert seconds to timer ticks
    #[inline]
    pub fn seconds_to_ticks(seconds: u32) -> u32 {
        seconds * frequency::TIMER_FREQ_HZ
    }

    /// Convert milliseconds to timer ticks
    #[inline]
    pub fn ms_to_ticks(milliseconds: u32) -> u32 {
        milliseconds * (frequency::TIMER_FREQ_HZ / frequency::MS_PER_SEC)
    }

    /// Convert microseconds to timer ticks
    #[inline]
    pub fn us_to_ticks(microseconds: u32) -> u32 {
        microseconds // 1:1 for 1MHz timer
    }

    /// Convert timer ticks to milliseconds
    #[inline]
    pub fn ticks_to_ms(&self, ticks: u32) -> u32 {
        ticks / (frequency::TIMER_FREQ_HZ / frequency::MS_PER_SEC)
    }

    /// Convert timer ticks to microseconds
    #[inline]
    pub fn ticks_to_us(&self, ticks: u32) -> u32 {
        ticks // 1:1 for 1MHz timer
    }

    /// Measure execution time of a closure in microseconds
    pub fn measure_us<F, R>(&self, f: F) -> (R, u32)
    where
        F: FnOnce() -> R,
    {
        let start = self.get_time_32();
        let result = f();
        let end = self.get_time_32();
        let elapsed = end.wrapping_sub(start);
        (result, elapsed)
    }

    /// Measure execution time of a closure in milliseconds
    pub fn measure_ms<F, R>(&self, f: F) -> (R, u32)
    where
        F: FnOnce() -> R,
    {
        let (result, us) = self.measure_us(f);
        (result, self.ticks_to_ms(us))
    }
}

impl<H: HardwareVersion> Initialize for TimerDriver<H> {
    type Config = TimerConfig;

    fn init(&mut self) -> Result<(), DriverError> {
        let config = TimerConfig::default();
        self.init_with_config(&config)
    }

    fn init_with_config(&mut self, _config: &Self::Config) -> Result<(), DriverError> {
        // System timer doesn't require special initialization
        // It's already running at boot
        self.status = DriverStatus::Ready;
        Ok(())
    }
}

impl<H: HardwareVersion> Status for TimerDriver<H> {
    fn status(&self) -> DriverStatus {
        self.status
    }
}

/// Timer channel wrapper for type-safe operations
pub struct TimerChannel<const CHANNEL: u8, H: HardwareVersion = DefaultHardware> {
    driver: *const TimerDriver<H>,
}

impl<const CHANNEL: u8, H: HardwareVersion> TimerChannel<CHANNEL, H> {
    /// Create a new timer channel reference
    ///
    /// # Safety
    /// The driver reference must be valid for the lifetime of this channel
    pub unsafe fn new(driver: &TimerDriver<H>) -> Result<Self, DriverError> {
        if CHANNEL > 3 {
            return Err(DriverError::InvalidInput);
        }

        Ok(Self {
            driver: driver as *const _,
        })
    }

    /// Get the channel number
    #[inline]
    pub const fn channel_number(&self) -> u8 {
        CHANNEL
    }

    /// Set compare value for this channel
    #[inline]
    pub fn set_compare(&self, microseconds_from_now: u32) -> Result<(), DriverError> {
        unsafe { (*self.driver).set_timer_compare(CHANNEL, microseconds_from_now) }
    }

    /// Check if this channel has matched
    #[inline]
    pub fn has_matched(&self) -> bool {
        unsafe { (*self.driver).has_timer_matched(CHANNEL) }
    }

    /// Clear match flag for this channel
    #[inline]
    pub fn clear_match(&self) {
        unsafe {
            (*self.driver).clear_timer_match(CHANNEL);
        }
    }

    /// Wait for this channel to match
    pub fn wait_for_match(&self) {
        while !self.has_matched() {
            // Busy wait
        }
        self.clear_match();
    }
}

/// Timer channels
impl<H: HardwareVersion> TimerDriver<H> {
    /// Get a type-safe reference to timer channel 0
    pub fn channel_0(&self) -> Result<TimerChannel<0, H>, DriverError> {
        unsafe { TimerChannel::new(self) }
    }

    /// Get a type-safe reference to timer channel 1
    pub fn channel_1(&self) -> Result<TimerChannel<1, H>, DriverError> {
        unsafe { TimerChannel::new(self) }
    }

    /// Get a type-safe reference to timer channel 2
    pub fn channel_2(&self) -> Result<TimerChannel<2, H>, DriverError> {
        unsafe { TimerChannel::new(self) }
    }

    /// Get a type-safe reference to timer channel 3
    pub fn channel_3(&self) -> Result<TimerChannel<3, H>, DriverError> {
        unsafe { TimerChannel::new(self) }
    }

    /// Get a type-safe reference to any timer channel
    pub fn channel<const CHANNEL: u8>(&self) -> Result<TimerChannel<CHANNEL, H>, DriverError> {
        unsafe { TimerChannel::new(self) }
    }
}

/// Type alias for the default timer driver
pub type SystemTimer = TimerDriver<DefaultHardware>;

/// Static system timer instance
static mut SYSTEM_TIMER: Option<SystemTimer> = None;

/// Initialize the system timer
pub fn init_system_timer() {
    unsafe {
        let mut timer = SystemTimer::new();
        let _ = timer.init();
        SYSTEM_TIMER = Some(timer);
    }
}

/// Get current system time (compatible with process management)
pub fn get_system_time() -> u64 {
    unsafe {
        if let Some(ref timer) = SYSTEM_TIMER {
            timer.get_time()
        } else {
            // Fallback counter if timer not initialized
            static mut COUNTER: u64 = 0;
            COUNTER += 1;
            COUNTER
        }
    }
}
