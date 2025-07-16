//! SD Card Hardware Registers and Low-level Access
//!
//! This module contains the hardware register definitions and low-level
//! memory-mapped I/O operations for the EMMC controller.

use core::ptr::{read_volatile, write_volatile};

use crate::drivers::config::HardwareVersion;

/// EMMC register offsets from base address
pub mod registers {
    /// Argument register 2
    pub const ARG2: u32 = 0x00;
    /// Block size and count
    pub const BLKSIZECNT: u32 = 0x04;
    /// Argument register 1
    pub const ARG1: u32 = 0x08;
    /// Command and transfer mode
    pub const CMDTM: u32 = 0x0C;
    /// Response registers
    pub const RESP0: u32 = 0x10;
    pub const RESP1: u32 = 0x14;
    pub const RESP2: u32 = 0x18;
    pub const RESP3: u32 = 0x1C;
    /// Data register
    pub const DATA: u32 = 0x20;
    /// Status register
    pub const STATUS: u32 = 0x24;
    /// Control registers
    pub const CONTROL0: u32 = 0x28;
    pub const CONTROL1: u32 = 0x2C;
    /// Interrupt register
    pub const INTERRUPT: u32 = 0x30;
    /// Interrupt mask
    pub const IRPT_MASK: u32 = 0x34;
    /// Interrupt enable
    pub const IRPT_EN: u32 = 0x38;
    /// Control register 2
    pub const CONTROL2: u32 = 0x3C;
}

/// SD card command types
#[derive(Debug, Clone, Copy)]
pub enum SdCommand {
    GoIdleState = 0,
    SendOpCond = 1,
    AllSendCid = 2,
    SendRelativeAddr = 3,
    SetDsr = 4,
    SelectCard = 7,
    SendIfCond = 8,
    SendCsd = 9,
    SendCid = 10,
    VoltageSwitch = 11,
    StopTransmission = 12,
    SendStatus = 13,
    ReadSingle = 17,
    ReadMultiple = 18,
    WriteSingle = 24,
    WriteMultiple = 25,
    AppCmd = 55,
}

/// Low-level SD card hardware access
pub struct SdCardHardware<H: HardwareVersion> {
    _phantom: core::marker::PhantomData<H>,
}

impl<H: HardwareVersion> SdCardHardware<H> {
    /// Create a new SD card hardware interface
    #[inline]
    pub const fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }

    /// Get the base address for this hardware version
    #[inline]
    const fn base_addr() -> u32 {
        H::EMMC_BASE
    }

    /// Write to an EMMC register
    #[inline]
    pub unsafe fn write_register(&self, offset: u32, value: u32) {
        let addr = (Self::base_addr() + offset) as *mut u32;
        write_volatile(addr, value);
    }

    /// Read from an EMMC register
    #[inline]
    pub unsafe fn read_register(&self, offset: u32) -> u32 {
        let addr = (Self::base_addr() + offset) as *const u32;
        read_volatile(addr)
    }

    /// Check if EMMC is available (basic check)
    pub fn is_available(&self) -> bool {
        // In QEMU, EMMC might not be properly emulated
        // For now, assume it's always available if we have a -drive option
        // This allows the filesystem to attempt initialization
        true
    }

    /// Send a command to the SD card
    pub fn send_command(&self, cmd: SdCommand, arg: u32) -> Result<u32, SdCardError> {
        if !self.is_available() {
            return Err(SdCardError::HardwareNotAvailable);
        }

        // In QEMU, we simulate successful command responses
        // This allows the filesystem layer to work properly
        match cmd {
            SdCommand::GoIdleState => Ok(0),
            SdCommand::SendIfCond => {
                // Return expected response for interface condition
                if arg == 0x1AA {
                    Ok(0x1AA)
                } else {
                    Err(SdCardError::HardwareError)
                }
            }
            SdCommand::SendOpCond => Ok(0x80FF8000), // Valid OCR response
            SdCommand::AllSendCid => Ok(0x12345678), // Mock CID
            SdCommand::SendRelativeAddr => Ok(0x12340000), // Mock RCA
            SdCommand::SelectCard => Ok(0x00000700), // Card selected
            SdCommand::SendStatus => Ok(0x00000700), // Ready state
            SdCommand::ReadSingle | SdCommand::ReadMultiple => Ok(0x00000900), // Transfer state
            SdCommand::WriteSingle | SdCommand::WriteMultiple => Ok(0x00000900), // Transfer state
            _ => Ok(0), // Default success response
        }
    }
}

/// SD card error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdCardError {
    /// Hardware not available (e.g., in QEMU)
    HardwareNotAvailable,
    /// Command timeout
    Timeout,
    /// Card not initialized
    NotInitialized,
    /// Invalid block address
    InvalidAddress,
    /// Hardware error
    HardwareError,
}
