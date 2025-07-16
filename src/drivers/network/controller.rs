//! Network Controller Integration
//!
//! Main network controller that manages all network interfaces
//! Refactored from week5_network.rs

use super::{
    ethernet::EthernetController, protocols::ProtocolManager, wifi::WiFiController, NetworkError,
    NetworkInterface, NetworkMetrics,
};

/// Main network controller for Pi 4/5
pub struct NetworkController {
    ethernet: EthernetController,
    wifi: WiFiController,
    protocols: ProtocolManager,
    is_pi5: bool,
    interface_status: [bool; 4], // Ethernet, WiFi, Bluetooth, USB
}

impl NetworkController {
    pub fn new() -> Self {
        Self {
            ethernet: EthernetController::new(),
            wifi: WiFiController::new(),
            protocols: ProtocolManager::new(),
            is_pi5: true, // Assume Pi 5 for advanced features
            interface_status: [false; 4],
        }
    }

    /// Initialize all network interfaces
    pub fn init_networking(&mut self) -> Result<(), NetworkError> {
        // Initialize Ethernet
        self.ethernet.init()?;
        self.interface_status[0] = true;

        // Initialize USB 3.0 for network adapters
        self.protocols.init_usb3()?;

        // Initialize high-speed protocols
        self.protocols.init_spi()?;
        self.protocols.init_i2c_fast()?;

        // Setup WiFi 6 if available
        if self.is_pi5 {
            self.wifi.init()?;
            self.interface_status[1] = true;
        }

        Ok(())
    }

    /// Get ethernet controller
    pub fn get_ethernet(&mut self) -> &mut EthernetController {
        &mut self.ethernet
    }

    /// Get WiFi controller
    pub fn get_wifi(&mut self) -> &mut WiFiController {
        &mut self.wifi
    }

    /// Get protocol manager
    pub fn get_protocols(&mut self) -> &mut ProtocolManager {
        &mut self.protocols
    }

    /// Get interface status
    pub fn get_interface_status(&self, interface: NetworkInterface) -> bool {
        match interface {
            NetworkInterface::GigabitEthernet => self.interface_status[0],
            NetworkInterface::WiFi6 => self.interface_status[1],
            NetworkInterface::Bluetooth5 => self.interface_status[2],
            NetworkInterface::UsbEthernet => self.interface_status[3],
        }
    }

    /// Get combined network metrics
    pub fn get_total_metrics(&self) -> NetworkMetrics {
        let ethernet_metrics = self.ethernet.get_metrics();
        let wifi_metrics = self.wifi.get_metrics();

        NetworkMetrics {
            bytes_transmitted: ethernet_metrics.bytes_transmitted + wifi_metrics.bytes_transmitted,
            bytes_received: ethernet_metrics.bytes_received + wifi_metrics.bytes_received,
            packets_transmitted: ethernet_metrics.packets_transmitted
                + wifi_metrics.packets_transmitted,
            packets_received: ethernet_metrics.packets_received + wifi_metrics.packets_received,
            errors: ethernet_metrics.errors + wifi_metrics.errors,
            link_speed_mbps: ethernet_metrics
                .link_speed_mbps
                .max(wifi_metrics.link_speed_mbps),
        }
    }

    /// Run network diagnostics
    pub fn run_diagnostics(&mut self) -> Result<(), NetworkError> {
        // Check ethernet link
        self.ethernet.check_link();

        // Test protocol performance
        let _ = self
            .protocols
            .test_protocol_performance(super::protocols::IoProtocol::Usb3SuperSpeed);

        Ok(())
    }
}

/// Global network controller instance
static mut NETWORK_CONTROLLER: Option<NetworkController> = None;
static mut NETWORK_CONTROLLER_INIT: bool = false;

/// Initialize global network controller
pub fn init_network_controller() -> Result<(), NetworkError> {
    unsafe {
        if !NETWORK_CONTROLLER_INIT {
            let mut controller = NetworkController::new();
            controller.init_networking()?;
            NETWORK_CONTROLLER = Some(controller);
            NETWORK_CONTROLLER_INIT = true;
        }
    }
    Ok(())
}

/// Get global network controller
pub fn get_network_controller() -> Option<&'static mut NetworkController> {
    unsafe {
        if NETWORK_CONTROLLER_INIT {
            NETWORK_CONTROLLER.as_mut()
        } else {
            None
        }
    }
}

/// Show Week 5 network capabilities
pub fn show_week5_capabilities() -> &'static str {
    "Week 5 Network Capabilities:\n\
     • Gigabit Ethernet (1000 Mbps)\n\
     • WiFi 6 (802.11ax)\n\
     • USB 3.0 SuperSpeed (5 Gbps)\n\
     • High-speed SPI (31.25 MHz)\n\
     • I2C Fast Mode Plus (1 MHz)"
}
