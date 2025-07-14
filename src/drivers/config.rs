//! Driver Configuration Module
//!
//! This module provides configuration types and hardware version detection
//! for the driver system.

/// Hardware version enumeration for Raspberry Pi models
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HardwareVersionEnum {
    Pi3,
    Pi4,
    Pi5,
    Unknown,
}

impl Default for HardwareVersionEnum {
    fn default() -> Self {
        // Default to Pi4 for compatibility
        HardwareVersionEnum::Pi4
    }
}

/// Hardware version trait for driver abstraction
pub trait HardwareVersion {
    /// Get the hardware version
    fn version() -> HardwareVersionEnum;
    
    /// GPIO base address
    const GPIO_BASE: u32;
    
    /// EMMC base address  
    const EMMC_BASE: u32;
    
    /// Timer base address
    const TIMER_BASE: u32;
    
    /// UART base address
    const UART_BASE: u32;
}

/// Default hardware configuration detection
pub struct DefaultHardware;

impl DefaultHardware {
    /// Detect the current hardware version
    pub fn detect_version() -> HardwareVersionEnum {
        // For now, default to Pi4
        // This could be enhanced with actual hardware detection
        HardwareVersionEnum::Pi4
    }
}

impl HardwareVersion for DefaultHardware {
    fn version() -> HardwareVersionEnum {
        Self::detect_version()
    }
    
    // Pi 4/5 base addresses
    const GPIO_BASE: u32 = 0xFE200000;
    const EMMC_BASE: u32 = 0xFE300000;
    const TIMER_BASE: u32 = 0xFE003000;
    const UART_BASE: u32 = 0xFE201000;
}
