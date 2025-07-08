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
        // This is a simple check to avoid hangs
        unsafe {
            // Try to read a register - if we get all 1s, probably not available
            let status = self.read_register(registers::STATUS);
            status != 0xFFFFFFFF && status != 0
        }
    }

    /// Send a command to the SD card
    pub fn send_command(&self, cmd: SdCommand, arg: u32) -> Result<u32, SdCardError> {
        if !self.is_available() {
            return Err(SdCardError::HardwareNotAvailable);
        }

        unsafe {
            // Set argument
            self.write_register(registers::ARG1, arg);

            // Send command (simplified - real implementation would set proper flags)
            let cmd_value = (cmd as u32) << 24;
            self.write_register(registers::CMDTM, cmd_value);

            // Wait for command complete (timeout after reasonable time)
            let mut timeout = 1000000;
            while timeout > 0 {
                let status = self.read_register(registers::INTERRUPT);
                if (status & 0x1) != 0 {
                    // Command complete
                    // Clear interrupt
                    self.write_register(registers::INTERRUPT, 0x1);
                    // Return response
                    return Ok(self.read_register(registers::RESP0));
                }
                timeout -= 1;
            }

            Err(SdCardError::Timeout)
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
