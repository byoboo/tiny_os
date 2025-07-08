//! GPIO Hardware Registers and Low-level Access
//!
//! This module contains the hardware register definitions and low-level
//! memory-mapped I/O operations for the GPIO peripheral.

use core::ptr::{read_volatile, write_volatile};
use crate::drivers::config::HardwareVersion;

/// GPIO register offsets from base address
pub mod registers {
    /// Function select registers (GPIO_FSEL0-5)
    pub const FSEL: u32 = 0x00;
    /// GPIO Pin Output Set registers (GPIO_SET0-1)
    pub const SET: u32 = 0x1C;
    /// GPIO Pin Output Clear registers (GPIO_CLR0-1)
    pub const CLR: u32 = 0x28;
    /// GPIO Pin Level registers (GPIO_LEV0-1)
    pub const LEV: u32 = 0x34;
}

/// GPIO function select values
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioFunction {
    Input = 0b000,
    Output = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010,
}

/// Low-level GPIO hardware access
pub struct GpioHardware<H: HardwareVersion> {
    _phantom: core::marker::PhantomData<H>,
}

impl<H: HardwareVersion> GpioHardware<H> {
    /// Create a new GPIO hardware interface
    #[inline]
    pub const fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
    
    /// Get the base address for this hardware version
    #[inline]
    const fn base_addr() -> u32 {
        H::GPIO_BASE
    }
    
    /// Write to a GPIO register
    #[inline]
    pub unsafe fn write_register(&self, offset: u32, value: u32) {
        let addr = (Self::base_addr() + offset) as *mut u32;
        write_volatile(addr, value);
    }
    
    /// Read from a GPIO register
    #[inline]
    pub unsafe fn read_register(&self, offset: u32) -> u32 {
        let addr = (Self::base_addr() + offset) as *const u32;
        read_volatile(addr)
    }
    
    /// Set GPIO pin function
    pub fn set_function(&self, pin: u32, function: GpioFunction) {
        if pin > 53 {
            return; // Pi has 54 GPIO pins (0-53)
        }
        
        let reg_index = pin / 10;
        let bit_offset = (pin % 10) * 3;
        
        unsafe {
            let reg_addr = registers::FSEL + reg_index * 4;
            let mut reg_val = self.read_register(reg_addr);
            
            // Clear the 3 bits for this pin
            reg_val &= !(0b111 << bit_offset);
            // Set the new function
            reg_val |= (function as u32) << bit_offset;
            
            self.write_register(reg_addr, reg_val);
        }
    }
    
    /// Set a GPIO pin high
    #[inline]
    pub fn set_high(&self, pin: u32) {
        if pin > 53 {
            return;
        }
        
        let reg_index = pin / 32;
        let bit_offset = pin % 32;
        
        unsafe {
            let reg_addr = registers::SET + reg_index * 4;
            self.write_register(reg_addr, 1 << bit_offset);
        }
    }
    
    /// Set a GPIO pin low
    #[inline]
    pub fn set_low(&self, pin: u32) {
        if pin > 53 {
            return;
        }
        
        let reg_index = pin / 32;
        let bit_offset = pin % 32;
        
        unsafe {
            let reg_addr = registers::CLR + reg_index * 4;
            self.write_register(reg_addr, 1 << bit_offset);
        }
    }
    
    /// Read the current level of a GPIO pin
    #[inline]
    pub fn read_pin(&self, pin: u32) -> bool {
        if pin > 53 {
            return false;
        }
        
        let reg_index = pin / 32;
        let bit_offset = pin % 32;
        
        unsafe {
            let reg_addr = registers::LEV + reg_index * 4;
            let reg_val = self.read_register(reg_addr);
            (reg_val & (1 << bit_offset)) != 0
        }
    }
}
