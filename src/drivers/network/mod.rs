//! Network Driver Module
//!
//! Consolidated network functionality from Week 5 implementation
//! Provides Ethernet, WiFi, and high-speed I/O protocol support

pub mod controller;
pub mod ethernet;
pub mod protocols;
pub mod wifi;

#[cfg(test)]
mod tests;

pub use controller::NetworkController;
pub use ethernet::{EthernetController, EthernetStatus};
pub use protocols::{IoProtocol, ProtocolManager};
pub use wifi::{WiFiController, WiFiStatus};

/// Network interface types supported by the system
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkInterface {
    /// Gigabit Ethernet (built-in)
    GigabitEthernet,
    /// WiFi 6 (802.11ax)
    WiFi6,
    /// Bluetooth 5.0
    Bluetooth5,
    /// USB Ethernet adapter
    UsbEthernet,
}

/// Common network errors
#[derive(Clone, Copy, PartialEq)]
pub enum NetworkError {
    NotInitialized,
    HardwareError,
    ConfigurationError,
    Timeout,
    NoDevice,
    InvalidInterface,
}

impl NetworkError {
    /// Convert to string representation for no_std compatibility
    pub fn as_str(&self) -> &'static str {
        match self {
            NetworkError::NotInitialized => "Not Initialized",
            NetworkError::HardwareError => "Hardware Error",
            NetworkError::ConfigurationError => "Configuration Error",
            NetworkError::Timeout => "Timeout",
            NetworkError::NoDevice => "No Device",
            NetworkError::InvalidInterface => "Invalid Interface",
        }
    }
}

/// Network performance metrics
#[derive(Debug, Default)]
pub struct NetworkMetrics {
    pub bytes_transmitted: u64,
    pub bytes_received: u64,
    pub packets_transmitted: u64,
    pub packets_received: u64,
    pub errors: u32,
    pub link_speed_mbps: u32,
}
