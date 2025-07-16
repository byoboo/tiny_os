//! No-std Tests for Network Module
//!
//! Tests that work in the embedded no_std environment

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drivers::network::{
        ethernet::{EthernetController, EthernetStatus},
        protocols::{IoProtocol, ProtocolManager},
        wifi::{WiFiController, WiFiStatus},
        NetworkError, NetworkInterface, NetworkMetrics,
    };

    #[test]
    fn test_ethernet_controller_creation() {
        let controller = EthernetController::new();
        assert_eq!(controller.get_status(), EthernetStatus::Uninitialized);
    }

    #[test]
    fn test_ethernet_status_string_conversion() {
        assert_eq!(EthernetStatus::Uninitialized.as_str(), "Uninitialized");
        assert_eq!(EthernetStatus::LinkUp.as_str(), "Link Up");
        assert_eq!(EthernetStatus::LinkDown.as_str(), "Link Down");
        assert_eq!(EthernetStatus::Error.as_str(), "Error");
    }

    #[test]
    fn test_wifi_controller_creation() {
        let controller = WiFiController::new();
        assert_eq!(controller.get_status(), WiFiStatus::Uninitialized);
    }

    #[test]
    fn test_wifi_status_string_conversion() {
        assert_eq!(WiFiStatus::Uninitialized.as_str(), "Uninitialized");
        assert_eq!(WiFiStatus::Connected.as_str(), "Connected");
        assert_eq!(WiFiStatus::Disconnected.as_str(), "Disconnected");
        assert_eq!(WiFiStatus::Scanning.as_str(), "Scanning");
    }

    #[test]
    fn test_protocol_manager_creation() {
        let manager = ProtocolManager::new();
        assert!(!manager.is_protocol_available(IoProtocol::Usb3SuperSpeed));
        assert!(!manager.is_protocol_available(IoProtocol::SpiHighSpeed));
        assert!(!manager.is_protocol_available(IoProtocol::I2cFastModePlus));
    }

    #[test]
    fn test_io_protocol_string_conversion() {
        assert_eq!(IoProtocol::Usb3SuperSpeed.as_str(), "USB 3.0 SuperSpeed");
        assert_eq!(IoProtocol::PciExpress2.as_str(), "PCIe 2.0");
        assert_eq!(IoProtocol::SpiHighSpeed.as_str(), "SPI High-Speed");
        assert_eq!(IoProtocol::I2cFastModePlus.as_str(), "I2C Fast Mode+");
    }

    #[test]
    fn test_network_error_string_conversion() {
        assert_eq!(NetworkError::NotInitialized.as_str(), "Not Initialized");
        assert_eq!(NetworkError::HardwareError.as_str(), "Hardware Error");
        assert_eq!(NetworkError::Timeout.as_str(), "Timeout");
        assert_eq!(NetworkError::NoDevice.as_str(), "No Device");
    }

    #[test]
    fn test_network_metrics_default() {
        let metrics = NetworkMetrics::default();
        assert_eq!(metrics.bytes_transmitted, 0);
        assert_eq!(metrics.bytes_received, 0);
        assert_eq!(metrics.packets_transmitted, 0);
        assert_eq!(metrics.packets_received, 0);
        assert_eq!(metrics.errors, 0);
        assert_eq!(metrics.link_speed_mbps, 0);
    }

    #[test]
    fn test_ethernet_initialization() {
        let mut controller = EthernetController::new();
        assert_eq!(controller.get_status(), EthernetStatus::Uninitialized);

        // Test initialization
        let result = controller.init();
        assert!(result.is_ok());
        assert_eq!(controller.get_status(), EthernetStatus::Initialized);
    }

    #[test]
    fn test_wifi_initialization() {
        let mut controller = WiFiController::new();
        assert_eq!(controller.get_status(), WiFiStatus::Uninitialized);

        // Test initialization
        let result = controller.init();
        assert!(result.is_ok());
        assert_eq!(controller.get_status(), WiFiStatus::Initialized);
    }

    #[test]
    fn test_protocol_manager_initialization() {
        let mut manager = ProtocolManager::new();

        // Test USB3 initialization
        let result = manager.init_usb3();
        assert!(result.is_ok());

        // Test SPI initialization
        let result = manager.init_spi();
        assert!(result.is_ok());

        // Test I2C initialization
        let result = manager.init_i2c_fast();
        assert!(result.is_ok());
    }

    #[test]
    fn test_network_interface_enum() {
        // Test that NetworkInterface enum values exist
        let _ethernet = NetworkInterface::GigabitEthernet;
        let _wifi = NetworkInterface::WiFi6;
        let _bluetooth = NetworkInterface::Bluetooth5;
        let _usb = NetworkInterface::UsbEthernet;
    }

    #[test]
    fn test_protocol_performance_testing() {
        let mut manager = ProtocolManager::new();

        // Test performance testing for available protocols
        let result = manager.test_protocol_performance(IoProtocol::PciExpress2);
        assert!(result.is_ok());

        let metrics = result.unwrap();
        assert!(metrics.average_speed_mbps > 0);
    }
}
