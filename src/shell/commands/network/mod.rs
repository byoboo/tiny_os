//! Network Shell Commands Module
//! 
//! Refactored network commands from Week 5 implementation

pub mod ethernet;
pub mod wifi;
pub mod protocols;
pub mod diagnostics;

use crate::shell::ShellContext;
use crate::drivers::network::{get_network_controller, NetworkInterface};
use crate::utils::formatting::{write_number_with_text, write_bool_with_text};

/// Main network command handler
pub fn cmd_network(args: &[&str], context: &mut ShellContext) {
    if args.is_empty() {
        show_network_overview(context);
        return;
    }

    match args[0] {
        "overview" => show_network_overview(context),
        "status" => show_network_status(context),
        "ethernet" => {
            let eth_args = if args.len() > 1 { &args[1..] } else { &[] };
            ethernet::cmd_ethernet(eth_args, context);
        }
        "wifi" => {
            let wifi_args = if args.len() > 1 { &args[1..] } else { &[] };
            wifi::cmd_wifi(wifi_args, context);
        }
        "protocols" => {
            let proto_args = if args.len() > 1 { &args[1..] } else { &[] };
            protocols::cmd_protocols(proto_args, context);
        }
        "diagnostics" => {
            let diag_args = if args.len() > 1 { &args[1..] } else { &[] };
            diagnostics::cmd_diagnostics(diag_args, context);
        }
        "help" => show_network_help(context),
        _ => {
            context.uart.puts("Unknown network command. Use 'network help' for options.\n");
        }
    }
}

/// Show network overview
fn show_network_overview(context: &mut ShellContext) {
    context.uart.puts("=== TinyOS Network Subsystem ===\n");
    context.uart.puts("Week 5: Network and Advanced I/O\n\n");
    
    context.uart.puts("Available Interfaces:\n");
    context.uart.puts("  • Gigabit Ethernet (1000 Mbps)\n");
    context.uart.puts("  • WiFi 6 (802.11ax)\n");
    context.uart.puts("  • USB 3.0 SuperSpeed\n");
    context.uart.puts("  • High-speed SPI/I2C\n\n");
    
    context.uart.puts("Use 'network help' for command options.\n");
}

/// Show network status
fn show_network_status(context: &mut ShellContext) {
    context.uart.puts("=== Network Status ===\n");
    
    if let Some(controller) = get_network_controller() {
        let ethernet_status = controller.get_interface_status(NetworkInterface::GigabitEthernet);
        let wifi_status = controller.get_interface_status(NetworkInterface::WiFi6);
        
        write_bool_with_text(context, "Ethernet: ", ethernet_status, "\n");
        write_bool_with_text(context, "WiFi: ", wifi_status, "\n");
        
        let metrics = controller.get_total_metrics();
        write_number_with_text(context, "Packets TX: ", metrics.packets_transmitted, "\n");
        write_number_with_text(context, "Packets RX: ", metrics.packets_received, "\n");
        write_number_with_text(context, "Bytes TX: ", metrics.bytes_transmitted, "\n");
        write_number_with_text(context, "Bytes RX: ", metrics.bytes_received, "\n");
    } else {
        context.uart.puts("Network controller not initialized\n");
    }
}

/// Show network help
fn show_network_help(context: &mut ShellContext) {
    context.uart.puts("=== Network Commands ===\n");
    context.uart.puts("  network overview     - Show network overview\n");
    context.uart.puts("  network status       - Show network status\n");
    context.uart.puts("  network ethernet     - Ethernet management\n");
    context.uart.puts("  network wifi         - WiFi management\n");
    context.uart.puts("  network protocols    - Protocol management\n");
    context.uart.puts("  network diagnostics  - Run diagnostics\n");
    context.uart.puts("  network help         - Show this help\n");
}