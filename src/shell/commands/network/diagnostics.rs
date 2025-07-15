//! Network Diagnostics Commands
//! 
//! Network diagnostics and troubleshooting

use crate::shell::ShellContext;
use crate::drivers::network::get_network_controller;

/// Diagnostics command handler
pub fn cmd_diagnostics(args: &[&str], context: &mut ShellContext) {
    if args.is_empty() {
        run_full_diagnostics(context);
        return;
    }

    match args[0] {
        "full" => run_full_diagnostics(context),
        "connectivity" => test_connectivity(context),
        "performance" => test_performance(context),
        "help" => show_diagnostics_help(context),
        _ => {
            context.uart.puts("Unknown diagnostics command. Use 'network diagnostics help' for options.\n");
        }
    }
}

/// Run full network diagnostics
fn run_full_diagnostics(context: &mut ShellContext) {
    context.uart.puts("=== Network Diagnostics ===\n");
    
    if let Some(controller) = get_network_controller() {
        context.uart.puts("Running comprehensive diagnostics...\n");
        
        match controller.run_diagnostics() {
            Ok(()) => {
                context.uart.puts("✅ Diagnostics completed successfully\n");
                
                // Show summary
                let metrics = controller.get_total_metrics();
                context.uart.puts(&format!("Total TX: {} packets, {} bytes\n", 
                    metrics.packets_transmitted, metrics.bytes_transmitted));
                context.uart.puts(&format!("Total RX: {} packets, {} bytes\n", 
                    metrics.packets_received, metrics.bytes_received));
                context.uart.puts(&format!("Errors: {}\n", metrics.errors));
                context.uart.puts(&format!("Link Speed: {} Mbps\n", metrics.link_speed_mbps));
            }
            Err(e) => {
                context.uart.puts(&format!("❌ Diagnostics failed: {:?}\n", e));
            }
        }
    } else {
        context.uart.puts("Network controller not initialized\n");
    }
}

/// Test network connectivity
fn test_connectivity(context: &mut ShellContext) {
    context.uart.puts("=== Connectivity Test ===\n");
    
    if let Some(controller) = get_network_controller() {
        // Test ethernet connectivity
        let ethernet = controller.get_ethernet();
        let ethernet_link = ethernet.check_link();
        context.uart.puts(&format!("Ethernet Link: {}\n", if ethernet_link { "✅ Up" } else { "❌ Down" }));
        
        // Test WiFi connectivity
        let wifi = controller.get_wifi();
        let wifi_status = wifi.get_status();
        context.uart.puts(&format!("WiFi Status: {:?}\n", wifi_status));
        
        // Test protocols
        let protocols = controller.get_protocols();
        context.uart.puts(&format!("USB 3.0: {}\n", 
            if protocols.is_protocol_available(crate::drivers::network::protocols::IoProtocol::Usb3SuperSpeed) { "✅ Available" } else { "❌ Not Available" }));
        context.uart.puts(&format!("SPI: {}\n", 
            if protocols.is_protocol_available(crate::drivers::network::protocols::IoProtocol::SpiHighSpeed) { "✅ Available" } else { "❌ Not Available" }));
        context.uart.puts(&format!("I2C: {}\n", 
            if protocols.is_protocol_available(crate::drivers::network::protocols::IoProtocol::I2cFastModePlus) { "✅ Available" } else { "❌ Not Available" }));
    } else {
        context.uart.puts("Network controller not initialized\n");
    }
}

/// Test network performance
fn test_performance(context: &mut ShellContext) {
    context.uart.puts("=== Performance Test ===\n");
    
    if let Some(controller) = get_network_controller() {
        // Test ethernet performance
        let ethernet = controller.get_ethernet();
        let eth_metrics = ethernet.get_metrics();
        context.uart.puts(&format!("Ethernet Link Speed: {} Mbps\n", eth_metrics.link_speed_mbps));
        
        // Test protocol performance
        let protocols = controller.get_protocols();
        
        // Test USB 3.0
        if protocols.is_protocol_available(crate::drivers::network::protocols::IoProtocol::Usb3SuperSpeed) {
            match protocols.test_protocol_performance(crate::drivers::network::protocols::IoProtocol::Usb3SuperSpeed) {
                Ok(metrics) => {
                    context.uart.puts(&format!("USB 3.0 Speed: {} Mbps\n", metrics.average_speed_mbps));
                }
                Err(_) => {
                    context.uart.puts("USB 3.0 test failed\n");
                }
            }
        }
        
        // Test SPI
        if protocols.is_protocol_available(crate::drivers::network::protocols::IoProtocol::SpiHighSpeed) {
            match protocols.test_protocol_performance(crate::drivers::network::protocols::IoProtocol::SpiHighSpeed) {
                Ok(metrics) => {
                    context.uart.puts(&format!("SPI Speed: {} Mbps\n", metrics.average_speed_mbps));
                }
                Err(_) => {
                    context.uart.puts("SPI test failed\n");
                }
            }
        }
    } else {
        context.uart.puts("Network controller not initialized\n");
    }
}

/// Show diagnostics help
fn show_diagnostics_help(context: &mut ShellContext) {
    context.uart.puts("=== Diagnostics Commands ===\n");
    context.uart.puts("  network diagnostics full         - Run full diagnostics\n");
    context.uart.puts("  network diagnostics connectivity - Test connectivity\n");
    context.uart.puts("  network diagnostics performance  - Test performance\n");
    context.uart.puts("  network diagnostics help         - Show this help\n");
}