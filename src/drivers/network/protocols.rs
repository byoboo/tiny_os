//! High-Speed I/O Protocol Manager
//!
//! USB 3.0, SPI, and I2C protocol support
//! Extracted from week5_network.rs

use core::ptr::{read_volatile, write_volatile};

use super::NetworkError;

/// USB 3.0 Controller Base Address
#[cfg(feature = "raspi3")]
const USB3_XHCI_BASE: usize = 0x3D500000;
#[cfg(not(feature = "raspi3"))]
const USB3_XHCI_BASE: usize = 0xFD500000;

/// SPI Master Controller Addresses
#[cfg(feature = "raspi3")]
const SPI0_BASE: usize = 0x3F204000;
#[cfg(not(feature = "raspi3"))]
const SPI0_BASE: usize = 0xFE204000;

#[cfg(feature = "raspi3")]
const SPI1_BASE: usize = 0x3F215080;
#[cfg(not(feature = "raspi3"))]
const SPI1_BASE: usize = 0xFE215080;

/// High-Speed I/O Protocols
#[derive(Clone, Copy, PartialEq)]
pub enum IoProtocol {
    /// USB 3.0 SuperSpeed
    Usb3SuperSpeed,
    /// PCIe 2.0 (from Week 4)
    PciExpress2,
    /// SPI high-speed mode
    SpiHighSpeed,
    /// I2C fast mode plus
    I2cFastModePlus,
}

impl IoProtocol {
    /// Convert to string representation for no_std compatibility
    pub fn as_str(&self) -> &'static str {
        match self {
            IoProtocol::Usb3SuperSpeed => "USB 3.0 SuperSpeed",
            IoProtocol::PciExpress2 => "PCIe 2.0",
            IoProtocol::SpiHighSpeed => "SPI High-Speed",
            IoProtocol::I2cFastModePlus => "I2C Fast Mode+",
        }
    }
}

/// Protocol performance metrics
#[derive(Debug, Default)]
pub struct ProtocolMetrics {
    pub transfers_completed: u64,
    pub bytes_transferred: u64,
    pub errors: u32,
    pub average_speed_mbps: u32,
}

/// High-speed I/O protocol manager
pub struct ProtocolManager {
    usb3_base: usize,
    spi0_base: usize,
    spi1_base: usize,
    usb3_enabled: bool,
    spi_enabled: bool,
    i2c_enabled: bool,
}

impl ProtocolManager {
    pub fn new() -> Self {
        Self {
            usb3_base: USB3_XHCI_BASE,
            spi0_base: SPI0_BASE,
            spi1_base: SPI1_BASE,
            usb3_enabled: false,
            spi_enabled: false,
            i2c_enabled: false,
        }
    }

    /// Initialize USB 3.0 controller
    pub fn init_usb3(&mut self) -> Result<(), NetworkError> {
        unsafe {
            // Enable USB 3.0 xHCI controller
            let command_reg = self.usb3_base + 0x20;
            write_volatile(command_reg as *mut u32, 0x0000_0001);

            // Check if controller is operational
            let status_reg = self.usb3_base + 0x04;
            let status = read_volatile(status_reg as *const u32);

            if status & 0x1 != 0 {
                self.usb3_enabled = true;
                Ok(())
            } else {
                Err(NetworkError::HardwareError)
            }
        }
    }

    /// Initialize SPI controllers
    pub fn init_spi(&mut self) -> Result<(), NetworkError> {
        unsafe {
            // Initialize SPI0
            let spi0_cs = self.spi0_base + 0x00;
            write_volatile(spi0_cs as *mut u32, 0x0000_0000); // Clear CS

            // Configure for high-speed mode
            let spi0_clk = self.spi0_base + 0x08;
            write_volatile(spi0_clk as *mut u32, 0x0000_0002); // 125MHz / 4 = 31.25MHz

            self.spi_enabled = true;
        }
        Ok(())
    }

    /// Initialize I2C fast mode plus
    pub fn init_i2c_fast(&mut self) -> Result<(), NetworkError> {
        // Placeholder for I2C initialization
        // Would configure I2C for fast mode plus (1 MHz)
        self.i2c_enabled = true;
        Ok(())
    }

    /// Get protocol availability
    pub fn is_protocol_available(&self, protocol: IoProtocol) -> bool {
        match protocol {
            IoProtocol::Usb3SuperSpeed => self.usb3_enabled,
            IoProtocol::SpiHighSpeed => self.spi_enabled,
            IoProtocol::I2cFastModePlus => self.i2c_enabled,
            IoProtocol::PciExpress2 => true, // Assume PCIe is available from Week 4
        }
    }

    /// Get protocol metrics
    pub fn get_protocol_metrics(&self, protocol: IoProtocol) -> ProtocolMetrics {
        // Placeholder - would return actual metrics for each protocol
        ProtocolMetrics::default()
    }

    /// Test protocol performance (placeholder)
    pub fn test_protocol_performance(
        &mut self,
        protocol: IoProtocol,
    ) -> Result<ProtocolMetrics, NetworkError> {
        if !self.is_protocol_available(protocol) {
            return Err(NetworkError::NoDevice);
        }

        // Placeholder for performance testing
        let mut metrics = ProtocolMetrics::default();

        match protocol {
            IoProtocol::Usb3SuperSpeed => {
                metrics.average_speed_mbps = 5000; // 5 Gbps theoretical
            }
            IoProtocol::SpiHighSpeed => {
                metrics.average_speed_mbps = 31; // 31.25 MHz
            }
            IoProtocol::I2cFastModePlus => {
                metrics.average_speed_mbps = 1; // 1 MHz
            }
            IoProtocol::PciExpress2 => {
                metrics.average_speed_mbps = 2500; // 2.5 GT/s
            }
        }

        Ok(metrics)
    }
}
