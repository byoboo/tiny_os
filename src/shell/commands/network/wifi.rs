//! WiFi Shell Commands
//! 
//! WiFi-specific command implementations

use crate::shell::ShellContext;
use crate::drivers::network::get_network_controller;

/// WiFi command handler
pub fn cmd_wifi(args: &[&str], context: &mut ShellContext) {
    if args.is_empty() {
        show_wifi_status(context);
        return;
    }

    match args[0] {
        "status" => show_wifi_status(context),
        "scan" => scan_wifi_networks(context),
        "connect" => connect_wifi(args, context),
        "disconnect" => disconnect_wifi(context),
        "help" => show_wifi_help(context),
        _ => {
            context.uart.puts("Unknown wifi command. Use 'network wifi help' for options.\n");
        }
    }
}

/// Show WiFi status
fn show_wifi_status(context: &mut ShellContext) {
    context.uart.puts("=== WiFi Status ===\n");
    
    if let Some(controller) = get_network_controller() {
        let wifi = controller.get_wifi();
        let status = wifi.get_status();
        
        context.uart.puts(&format!("Status: {:?}\n", status));
        
        if let Some(network) = wifi.get_current_network() {
            context.uart.puts(&format!("Connected to: {:?}\n", network));
        }
        
        let metrics = wifi.get_metrics();
        context.uart.puts(&format!("Packets TX: {}\n", metrics.packets_transmitted));
        context.uart.puts(&format!("Packets RX: {}\n", metrics.packets_received));
    } else {
        context.uart.puts("Network controller not initialized\n");
    }
}

/// Scan for WiFi networks
fn scan_wifi_networks(context: &mut ShellContext) {
    context.uart.puts("=== WiFi Network Scan ===\n");
    
    if let Some(controller) = get_network_controller() {
        let wifi = controller.get_wifi();
        
        match wifi.scan_networks() {
            Ok(networks) => {
                if networks.is_empty() {
                    context.uart.puts("No networks found\n");
                } else {
                    context.uart.puts(&format!("Found {} networks:\n", networks.len()));
                    for network in networks {
                        context.uart.puts(&format!("  {:?}\n", network));
                    }
                }
            }
            Err(e) => {
                context.uart.puts(&format!("Scan failed: {:?}\n", e));
            }
        }
    } else {
        context.uart.puts("Network controller not initialized\n");
    }
}

/// Connect to WiFi network
fn connect_wifi(args: &[&str], context: &mut ShellContext) {
    if args.len() < 3 {
        context.uart.puts("Usage: network wifi connect <ssid> <password>\n");
        return;
    }
    
    let ssid = args[1];
    let password = args[2];
    
    context.uart.puts(&format!("Connecting to WiFi network: {}\n", ssid));
    
    if let Some(controller) = get_network_controller() {
        let wifi = controller.get_wifi();
        
        match wifi.connect(ssid, password) {
            Ok(()) => {
                context.uart.puts("Connected successfully\n");
            }
            Err(e) => {
                context.uart.puts(&format!("Connection failed: {:?}\n", e));
            }
        }
    } else {
        context.uart.puts("Network controller not initialized\n");
    }
}

/// Disconnect from WiFi
fn disconnect_wifi(context: &mut ShellContext) {
    context.uart.puts("Disconnecting from WiFi...\n");
    
    if let Some(controller) = get_network_controller() {
        let wifi = controller.get_wifi();
        
        match wifi.disconnect() {
            Ok(()) => {
                context.uart.puts("Disconnected successfully\n");
            }
            Err(e) => {
                context.uart.puts(&format!("Disconnect failed: {:?}\n", e));
            }
        }
    } else {
        context.uart.puts("Network controller not initialized\n");
    }
}

/// Show WiFi help
fn show_wifi_help(context: &mut ShellContext) {
    context.uart.puts("=== WiFi Commands ===\n");
    context.uart.puts("  network wifi status       - Show WiFi status\n");
    context.uart.puts("  network wifi scan          - Scan for networks\n");
    context.uart.puts("  network wifi connect <ssid> <password> - Connect to network\n");
    context.uart.puts("  network wifi disconnect    - Disconnect from network\n");
    context.uart.puts("  network wifi help          - Show this help\n");
}