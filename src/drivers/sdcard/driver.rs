//! High-level SD Card Driver
//!
//! This module provides a safe, high-level interface to the SD card
//! with block-level read/write operations and error handling.

use crate::drivers::config::{DefaultHardware, HardwareVersion};
use crate::drivers::traits::{DriverError, DriverStatus, Initialize, Status};
use super::hardware::{SdCardError, SdCardHardware, SdCommand};

/// SD card information structure
#[derive(Debug, Clone, Copy)]
pub struct SdCardInfo {
    pub high_capacity: bool,
    pub rca: u32,
    pub ocr: u32,
    pub cid: [u32; 4],
    pub csd: [u32; 4],
    pub scr: [u32; 2],
}

impl SdCardInfo {
    /// Get card capacity in bytes (approximate)
    pub fn get_capacity(&self) -> u64 {
        if self.high_capacity {
            // SDHC/SDXC capacity calculation (simplified)
            let c_size = ((self.csd[1] & 0x3F) << 16) | ((self.csd[2] & 0xFFFF0000) >> 16);
            (c_size as u64 + 1) * 512 * 1024 // 512KB blocks
        } else {
            // Standard capacity calculation (simplified)
            1024 * 1024 * 1024 // Default to 1GB for demo
        }
    }
}

/// SD card driver configuration
#[derive(Debug, Clone, Copy)]
pub struct SdCardConfig {
    /// Block size (typically 512 bytes)
    pub block_size: u32,
    /// Enable high-speed mode
    pub high_speed: bool,
}

impl Default for SdCardConfig {
    fn default() -> Self {
        Self {
            block_size: 512,
            high_speed: false,
        }
    }
}

/// High-level SD card driver
pub struct SdCardDriver<H: HardwareVersion = DefaultHardware> {
    hardware: SdCardHardware<H>,
    status: DriverStatus,
    block_size: u32,
    card_initialized: bool,
}

impl<H: HardwareVersion> SdCardDriver<H> {
    /// Create a new SD card driver instance
    pub const fn new() -> Self {
        Self {
            hardware: SdCardHardware::new(),
            status: DriverStatus::Uninitialized,
            block_size: 512,
            card_initialized: false,
        }
    }
    
    /// Check if SD card hardware is available
    pub fn is_available(&self) -> bool {
        self.hardware.is_available()
    }
    
    /// Initialize the SD card
    pub fn init_card(&mut self) -> Result<(), SdCardError> {
        if !self.hardware.is_available() {
            return Err(SdCardError::HardwareNotAvailable);
        }
        
        // Reset card
        self.hardware.send_command(SdCommand::GoIdleState, 0)?;
        
        // Send interface condition
        let response = self.hardware.send_command(SdCommand::SendIfCond, 0x1AA)?;
        if (response & 0xFF) != 0xAA {
            return Err(SdCardError::HardwareError);
        }
        
        self.card_initialized = true;
        Ok(())
    }
    
    /// Read a single block from the SD card
    pub fn read_block(&self, block_addr: u32, buffer: &mut [u8]) -> Result<(), SdCardError> {
        if !self.card_initialized {
            return Err(SdCardError::NotInitialized);
        }
        
        if buffer.len() < self.block_size as usize {
            return Err(SdCardError::InvalidAddress);
        }
        
        // For now, just return a placeholder implementation
        // Real implementation would involve complex EMMC protocol
        let _response = self.hardware.send_command(SdCommand::ReadSingle, block_addr)?;
        
        // Fill buffer with test data for demo purposes
        for (i, byte) in buffer.iter_mut().enumerate().take(self.block_size as usize) {
            *byte = (i % 256) as u8;
        }
        
        Ok(())
    }
    
    /// Write a single block to the SD card
    pub fn write_block(&mut self, block_addr: u32, buffer: &[u8]) -> Result<(), SdCardError> {
        if !self.card_initialized {
            return Err(SdCardError::NotInitialized);
        }
        
        if buffer.len() < self.block_size as usize {
            return Err(SdCardError::InvalidAddress);
        }
        
        // For now, just return a placeholder implementation
        let _response = self.hardware.send_command(SdCommand::WriteSingle, block_addr)?;
        
        // In a real implementation, we would write the buffer data
        // to the EMMC data register
        Ok(())
    }
    
    /// Read multiple blocks from the SD card
    pub fn read_blocks(&self, start_block: u32, num_blocks: u32, buffer: &mut [u8]) -> Result<(), SdCardError> {
        let total_size = (num_blocks * self.block_size) as usize;
        if buffer.len() < total_size {
            return Err(SdCardError::InvalidAddress);
        }
        
        for block in 0..num_blocks {
            let block_addr = start_block + block;
            let offset = (block * self.block_size) as usize;
            let block_buffer = &mut buffer[offset..offset + self.block_size as usize];
            self.read_block(block_addr, block_buffer)?;
        }
        
        Ok(())
    }
    
    /// Write multiple blocks to the SD card
    pub fn write_blocks(&mut self, start_block: u32, num_blocks: u32, buffer: &[u8]) -> Result<(), SdCardError> {
        let total_size = (num_blocks * self.block_size) as usize;
        if buffer.len() < total_size {
            return Err(SdCardError::InvalidAddress);
        }
        
        for block in 0..num_blocks {
            let block_addr = start_block + block;
            let offset = (block * self.block_size) as usize;
            let block_buffer = &buffer[offset..offset + self.block_size as usize];
            self.write_block(block_addr, block_buffer)?;
        }
        
        Ok(())
    }
    
    /// Get the block size
    #[inline]
    pub fn block_size(&self) -> u32 {
        self.block_size
    }
    
    /// Check if the card is initialized
    #[inline]
    pub fn is_initialized(&self) -> bool {
        self.card_initialized
    }
    
    /// Get SD card information
    pub fn get_card_info(&self) -> Option<SdCardInfo> {
        if !self.card_initialized {
            return None;
        }
        
        // Return mock card info for compatibility
        // In a real implementation, this would read actual card registers
        Some(SdCardInfo {
            high_capacity: true,
            rca: 0x1234,
            ocr: 0x40FF8000,
            cid: [0, 0, 0, 0],
            csd: [0, 0, 0, 0],
            scr: [0, 0],
        })
    }
    
    /// Legacy init method for backward compatibility
    pub fn init(&mut self) -> Result<(), SdCardError> {
        self.init_card()
    }
}

impl<H: HardwareVersion> Initialize for SdCardDriver<H> {
    type Config = SdCardConfig;
    
    fn init(&mut self) -> Result<(), DriverError> {
        let config = SdCardConfig::default();
        self.init_with_config(&config)
    }
    
    fn init_with_config(&mut self, config: &Self::Config) -> Result<(), DriverError> {
        self.block_size = config.block_size;
        
        match self.init_card() {
            Ok(()) => {
                self.status = DriverStatus::Ready;
                Ok(())
            }
            Err(SdCardError::HardwareNotAvailable) => {
                // In QEMU or when no SD card is present, this is not an error
                self.status = DriverStatus::Ready;
                Ok(())
            }
            Err(_) => {
                self.status = DriverStatus::Error(DriverError::HardwareFault);
                Err(DriverError::HardwareFault)
            }
        }
    }
}

impl<H: HardwareVersion> Status for SdCardDriver<H> {
    fn status(&self) -> DriverStatus {
        self.status
    }
}

/// Type alias for the default SD card driver
pub type SdCard = SdCardDriver<DefaultHardware>;
