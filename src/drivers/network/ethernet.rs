//! Ethernet Controller Driver
//!
//! Gigabit Ethernet support for Raspberry Pi 4/5
//! Extracted from week5_network.rs

use core::ptr::{read_volatile, write_volatile};

use super::{NetworkError, NetworkMetrics};

/// Ethernet Controller Base Address (Pi 4/5)
const GENET_BASE: usize = 0xFD580000;

/// Ethernet controller status
#[derive(Clone, Copy, PartialEq)]
pub enum EthernetStatus {
    Uninitialized,
    Initialized,
    LinkUp,
    LinkDown,
    Error,
}

impl EthernetStatus {
    /// Convert to string representation for no_std compatibility
    pub fn as_str(&self) -> &'static str {
        match self {
            EthernetStatus::Uninitialized => "Uninitialized",
            EthernetStatus::Initialized => "Initialized",
            EthernetStatus::LinkUp => "Link Up",
            EthernetStatus::LinkDown => "Link Down",
            EthernetStatus::Error => "Error",
        }
    }
}

/// Ethernet Controller for Pi 4/5
pub struct EthernetController {
    base_address: usize,
    status: EthernetStatus,
    metrics: NetworkMetrics,
}

impl EthernetController {
    pub fn new() -> Self {
        Self {
            base_address: GENET_BASE,
            status: EthernetStatus::Uninitialized,
            metrics: NetworkMetrics::default(),
        }
    }

    /// Initialize Gigabit Ethernet controller
    pub fn init(&mut self) -> Result<(), NetworkError> {
        unsafe {
            // Enable Ethernet controller
            let ctrl_reg = self.base_address + 0x00;
            write_volatile(ctrl_reg as *mut u32, 0x8000_0001);

            // Configure for Gigabit speeds
            let speed_reg = self.base_address + 0x14;
            write_volatile(speed_reg as *mut u32, 0x0000_0003); // 1000 Mbps

            self.status = EthernetStatus::Initialized;
            self.metrics.link_speed_mbps = 1000;
        }
        Ok(())
    }

    /// Get current status
    pub fn get_status(&self) -> EthernetStatus {
        self.status
    }

    /// Get performance metrics
    pub fn get_metrics(&self) -> &NetworkMetrics {
        &self.metrics
    }

    /// Check link status
    pub fn check_link(&mut self) -> bool {
        unsafe {
            let status_reg = self.base_address + 0x10;
            let status = read_volatile(status_reg as *const u32);

            let link_up = (status & 0x1) != 0;
            self.status = if link_up {
                EthernetStatus::LinkUp
            } else {
                EthernetStatus::LinkDown
            };

            link_up
        }
    }

    /// Send packet (placeholder)
    pub fn send_packet(&mut self, _data: &[u8]) -> Result<(), NetworkError> {
        if self.status != EthernetStatus::LinkUp {
            return Err(NetworkError::NoDevice);
        }

        // Placeholder for actual packet transmission
        self.metrics.packets_transmitted += 1;
        Ok(())
    }

    /// Receive packet (placeholder)
    pub fn receive_packet(&mut self, _buffer: &mut [u8]) -> Result<usize, NetworkError> {
        if self.status != EthernetStatus::LinkUp {
            return Err(NetworkError::NoDevice);
        }

        // Placeholder for actual packet reception
        self.metrics.packets_received += 1;
        Ok(0)
    }
}
