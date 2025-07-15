//! Protocol Shell Commands
//! 
//! High-speed I/O protocol command implementations

use crate::shell::ShellContext;
use crate::drivers::network::{get_network_controller, protocols::IoProtocol};

/// Protocol command handler
pub fn cmd_protocols(args: &[&str], context: &mut ShellContext) {
    if args.is_empty() {
        show_protocols_status(context);
        return;
    }

    match args[0] {
        "status" => show_protocols_status(context),
        "test" => test_protocol_performance(args, context),
        "usb3" => show_usb3_status(context),
        "spi" => show_spi_status(context),
        "i2c" => show_i2c_status(context),
        "help" => show_protocols_help(context),
        _ => {
            context.uart.puts("Unknown protocol command. Use 'network protocols help' for options.\n");
        }
    }
}

/// Show protocols status
fn show_protocols_status(context: &mut ShellContext) {
    context.uart.puts("=== I/O Protocols Status ===\n");
    
    if let Some(controller) = get_network_controller() {
        let protocols = controller.get_protocols();
        
        context.uart.puts(&format!("USB 3.0: {}\n", 
            if protocols.is_protocol_available(IoProtocol::Usb3SuperSpeed) { "Available" } else { "Not Available" }));
        context.uart.puts(&format!("SPI High-Speed: {}\n", 
            if protocols.is_protocol_available(IoProtocol::SpiHighSpeed) { "Available" } else { "Not Available" }));
        context.uart.puts(&format!("I2C Fast Mode+: {}\n", 
            if protocols.is_protocol_available(IoProtocol::I2cFastModePlus) { "Available" } else { "Not Available" }));
        context.uart.puts(&format!("PCIe 2.0: {}\n", 
            if protocols.is_protocol_available(IoProtocol::PciExpress2) { "Available" } else { "Not Available" }));
    } else {
        context.uart.puts("Network controller not initialized\n");
    }
}

/// Test protocol performance
fn test_protocol_performance(args: &[&str], context: &mut ShellContext) {
    if args.len() < 2 {
        context.uart.puts("Usage: network protocols test <usb3|spi|i2c|pcie>\n");
        return;
    }
    
    let protocol = match args[1] {
        "usb3" => IoProtocol::Usb3SuperSpeed,
        "spi" => IoProtocol::SpiHighSpeed,
        "i2c" => IoProtocol::I2cFastModePlus,
        "pcie" => IoProtocol::PciExpress2,
        _ => {
            context.uart.puts("Unknown protocol. Use usb3, spi, i2c, or pcie.\n");
            return;
        }
    };
    
    context.uart.puts(&format!("Testing {:?} performance...\n", protocol));
    
    if let Some(controller) = get_network_controller() {
        let protocols = controller.get_protocols();
        
        match protocols.test_protocol_performance(protocol) {
            Ok(metrics) => {
                context.uart.puts(&format!("Average Speed: {} Mbps\n", metrics.average_speed_mbps));
                context.uart.puts(&format!("Transfers: {}\n", metrics.transfers_completed));
                context.uart.puts(&format!("Bytes: {}\n", metrics.bytes_transferred));
                context.uart.puts(&format!("Errors: {}\n", metrics.errors));
            }
            Err(e) => {
                context.uart.puts(&format!("Test failed: {:?}\n", e));
            }
        }
    } else {
        context.uart.puts("Network controller not initialized\n");
    }
}

/// Show USB 3.0 status
fn show_usb3_status(context: &mut ShellContext) {
    context.uart.puts("=== USB 3.0 Status ===\n");
    
    if let Some(controller) = get_network_controller() {
        let protocols = controller.get_protocols();
        let available = protocols.is_protocol_available(IoProtocol::Usb3SuperSpeed);
        
        context.uart.puts(&format!("USB 3.0 SuperSpeed: {}\n", if available { "Available" } else { "Not Available" }));
        
        if available {
            let metrics = protocols.get_protocol_metrics(IoProtocol::Usb3SuperSpeed);
            context.uart.puts(&format!("Theoretical Speed: 5000 Mbps\n"));
            context.uart.puts(&format!("Transfers: {}\n", metrics.transfers_completed));
        }
    } else {
        context.uart.puts("Network controller not initialized\n");
    }
}

/// Show SPI status
fn show_spi_status(context: &mut ShellContext) {
    context.uart.puts("=== SPI Status ===\n");
    
    if let Some(controller) = get_network_controller() {
        let protocols = controller.get_protocols();
        let available = protocols.is_protocol_available(IoProtocol::SpiHighSpeed);
        
        context.uart.puts(&format!("SPI High-Speed: {}\n", if available { "Available" } else { "Not Available" }));
        
        if available {
            context.uart.puts("SPI0: 31.25 MHz\n");
            context.uart.puts("SPI1: 31.25 MHz\n");
        }
    } else {
        context.uart.puts("Network controller not initialized\n");
    }
}

/// Show I2C status
fn show_i2c_status(context: &mut ShellContext) {
    context.uart.puts("=== I2C Status ===\n");
    
    if let Some(controller) = get_network_controller() {
        let protocols = controller.get_protocols();
        let available = protocols.is_protocol_available(IoProtocol::I2cFastModePlus);
        
        context.uart.puts(&format!("I2C Fast Mode+: {}\n", if available { "Available" } else { "Not Available" }));
        
        if available {
            context.uart.puts("Speed: 1 MHz\n");
        }
    } else {
        context.uart.puts("Network controller not initialized\n");
    }
}

/// Show protocols help
fn show_protocols_help(context: &mut ShellContext) {
    context.uart.puts("=== Protocol Commands ===\n");
    context.uart.puts("  network protocols status  - Show protocols status\n");
    context.uart.puts("  network protocols test    - Test protocol performance\n");
    context.uart.puts("  network protocols usb3    - Show USB 3.0 status\n");
    context.uart.puts("  network protocols spi     - Show SPI status\n");
    context.uart.puts("  network protocols i2c     - Show I2C status\n");
    context.uart.puts("  network protocols help    - Show this help\n");
}