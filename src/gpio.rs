// GPIO and hardware control for Raspberry Pi 4/5
use core::ptr::{read_volatile, write_volatile};

// GPIO base addresses for Raspberry Pi 4/5
const GPIO_BASE: u32 = 0xFE200000;
const GPIO_FSEL: u32 = GPIO_BASE + 0x00;      // Function select
const GPIO_SET: u32 = GPIO_BASE + 0x1C;       // Set pins high
const GPIO_CLR: u32 = GPIO_BASE + 0x28;       // Set pins low

// Activity LED is typically on GPIO 42 (Pi 4) or GPIO 47 (Pi 5)
const ACT_LED_GPIO: u32 = 42;

pub struct Gpio {
    base: u32,
}

impl Gpio {
    pub fn new() -> Self {
        Self { base: GPIO_BASE }
    }

    /// Set GPIO pin function (input, output, alt functions)
    pub fn set_function(&self, pin: u32, function: GpioFunction) {
        if pin > 53 { return; } // Pi has 54 GPIO pins (0-53)
        
        let reg_index = pin / 10;
        let bit_offset = (pin % 10) * 3;
        
        unsafe {
            let reg_addr = (GPIO_FSEL + reg_index * 4) as *mut u32;
            let mut reg_val = read_volatile(reg_addr);
            
            // Clear the 3 bits for this pin
            reg_val &= !(0b111 << bit_offset);
            // Set the new function
            reg_val |= (function as u32) << bit_offset;
            
            write_volatile(reg_addr, reg_val);
        }
    }

    /// Set a GPIO pin high
    pub fn set_high(&self, pin: u32) {
        if pin > 53 { return; }
        
        let reg_index = pin / 32;
        let bit_offset = pin % 32;
        
        unsafe {
            let reg_addr = (GPIO_SET + reg_index * 4) as *mut u32;
            write_volatile(reg_addr, 1 << bit_offset);
        }
    }

    /// Set a GPIO pin low
    pub fn set_low(&self, pin: u32) {
        if pin > 53 { return; }
        
        let reg_index = pin / 32;
        let bit_offset = pin % 32;
        
        unsafe {
            let reg_addr = (GPIO_CLR + reg_index * 4) as *mut u32;
            write_volatile(reg_addr, 1 << bit_offset);
        }
    }
}

#[repr(u32)]
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

/// Simple delay function (very rough, depends on CPU speed)
pub fn delay_cycles(cycles: u32) {
    for _ in 0..cycles {
        unsafe {
            asm!("nop");
        }
    }
}

use core::arch::asm;
