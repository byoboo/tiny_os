//! Timer Hardware Registers and Low-level Access
//!
//! This module contains the hardware register definitions and low-level
//! memory-mapped I/O operations for the System Timer peripheral.

use core::ptr::{read_volatile, write_volatile};
use crate::drivers::config::HardwareVersion;

/// Timer register offsets from base address
pub mod registers {
    /// Control/Status register
    pub const CS: u32 = 0x00;
    /// Counter Lower 32 bits
    pub const CLO: u32 = 0x04;
    /// Counter Higher 32 bits
    pub const CHI: u32 = 0x08;
    /// Compare 0 register
    pub const C0: u32 = 0x0C;
    /// Compare 1 register
    pub const C1: u32 = 0x10;
    /// Compare 2 register
    pub const C2: u32 = 0x14;
    /// Compare 3 register
    pub const C3: u32 = 0x18;
}

/// Control/Status register bits
pub mod control_status {
    /// Timer 0 matched
    pub const M0: u32 = 1 << 0;
    /// Timer 1 matched
    pub const M1: u32 = 1 << 1;
    /// Timer 2 matched
    pub const M2: u32 = 1 << 2;
    /// Timer 3 matched
    pub const M3: u32 = 1 << 3;
}

/// Timer frequency constants
pub mod frequency {
    /// Timer frequency is 1MHz on Raspberry Pi
    pub const TIMER_FREQ_HZ: u32 = 1_000_000;
    /// Microseconds per second
    pub const US_PER_SEC: u32 = 1_000_000;
    /// Milliseconds per second
    pub const MS_PER_SEC: u32 = 1_000;
}

/// Low-level timer hardware access
pub struct TimerHardware<H: HardwareVersion> {
    _phantom: core::marker::PhantomData<H>,
}

impl<H: HardwareVersion> TimerHardware<H> {
    /// Create a new timer hardware interface
    #[inline]
    pub const fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    
    /// Get the base address for this hardware version
    #[inline]
    const fn base_addr() -> u32 {
        H::TIMER_BASE
    }
    
    /// Write to a timer register
    #[inline]
    pub unsafe fn write_register(&self, offset: u32, value: u32) {
        let addr = (Self::base_addr() + offset) as *mut u32;
        write_volatile(addr, value);
    }
    
    /// Read from a timer register
    #[inline]
    pub unsafe fn read_register(&self, offset: u32) -> u32 {
        let addr = (Self::base_addr() + offset) as *const u32;
        read_volatile(addr)
    }
    
    /// Get the current timer value (64-bit)
    #[inline]
    pub fn get_time_64(&self) -> u64 {
        unsafe {
            let low = self.read_register(registers::CLO);
            let high = self.read_register(registers::CHI);
            ((high as u64) << 32) | (low as u64)
        }
    }
    
    /// Get the current timer value (32-bit, lower part only)
    #[inline]
    pub fn get_time_32(&self) -> u32 {
        unsafe {
            self.read_register(registers::CLO)
        }
    }
    
    /// Set compare register for timer channel
    #[inline]
    pub fn set_compare(&self, channel: u8, value: u32) {
        if channel > 3 {
            return;
        }
        
        let offset = match channel {
            0 => registers::C0,
            1 => registers::C1,
            2 => registers::C2,
            3 => registers::C3,
            _ => return,
        };
        
        unsafe {
            self.write_register(offset, value);
        }
    }
    
    /// Check if timer channel has matched
    #[inline]
    pub fn has_matched(&self, channel: u8) -> bool {
        if channel > 3 {
            return false;
        }
        
        let bit = match channel {
            0 => control_status::M0,
            1 => control_status::M1,
            2 => control_status::M2,
            3 => control_status::M3,
            _ => return false,
        };
        
        unsafe {
            (self.read_register(registers::CS) & bit) != 0
        }
    }
    
    /// Clear timer channel match flag
    #[inline]
    pub fn clear_match(&self, channel: u8) {
        if channel > 3 {
            return;
        }
        
        let bit = match channel {
            0 => control_status::M0,
            1 => control_status::M1,
            2 => control_status::M2,
            3 => control_status::M3,
            _ => return,
        };
        
        unsafe {
            self.write_register(registers::CS, bit);
        }
    }
}
