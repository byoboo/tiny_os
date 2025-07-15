//! Ethernet Shell Commands
//! 
//! Ethernet-specific command implementations

use crate::shell::ShellContext;
use crate::drivers::network::{get_network_controller, NetworkInterface};
use crate::utils::formatting::{write_number_with_text, write_bool_with_text};

/// Ethernet command handler
pub fn cmd_ethernet(args: &[&str], context: &mut ShellContext) {
    if args.is_empty() {
        show_ethernet_status(context);
        return;
    }

    match args[0] {
        "status" => show_ethernet_status(context),
        "link" => check_ethernet_link(context),
        "stats" => show_ethernet_stats(context),
        "help" => show_ethernet_help(context),
        _ => {
            context.uart.puts("Unknown ethernet command. Use 'network ethernet help' for options.\n");
        }
    }
}

/// Show ethernet status
fn show_ethernet_status(context: &mut ShellContext) {
    context.uart.puts("=== Ethernet Status ===\n");
    
    if let Some(controller) = get_network_controller() {
        let ethernet = controller.get_ethernet();
        let status = ethernet.get_status();
        
        context.uart.puts("Status: ");
        context.uart.puts(status.as_str());
        context.uart.puts("\n");
        
        let metrics = ethernet.get_metrics();
        write_number_with_text(context, "Link Speed: ", metrics.link_speed_mbps as u64, " Mbps\n");
        write_number_with_text(context, "Packets TX: ", metrics.packets_transmitted, "\n");
        write_number_with_text(context, "Packets RX: ", metrics.packets_received, "\n");
    } else {
        context.uart.puts("Network controller not initialized\n");
    }
}

/// Check ethernet link
fn check_ethernet_link(context: &mut ShellContext) {
    context.uart.puts("=== Ethernet Link Check ===\n");
    
    if let Some(controller) = get_network_controller() {
        let ethernet = controller.get_ethernet();
        let link_up = ethernet.check_link();
        
        context.uart.puts("Link Status: ");
        context.uart.puts(if link_up { "Up" } else { "Down" });
        context.uart.puts("\n");
    } else {
        context.uart.puts("Network controller not initialized\n");
    }
}

/// Show ethernet statistics
fn show_ethernet_stats(context: &mut ShellContext) {
    context.uart.puts("=== Ethernet Statistics ===\n");
    
    if let Some(controller) = get_network_controller() {
        let ethernet = controller.get_ethernet();
        let metrics = ethernet.get_metrics();
        
        write_number_with_text(context, "Bytes TX: ", metrics.bytes_transmitted, "\n");
        write_number_with_text(context, "Bytes RX: ", metrics.bytes_received, "\n");
        write_number_with_text(context, "Packets TX: ", metrics.packets_transmitted, "\n");
        write_number_with_text(context, "Packets RX: ", metrics.packets_received, "\n");
        write_number_with_text(context, "Errors: ", metrics.errors as u64, "\n");
    } else {
        context.uart.puts("Network controller not initialized\n");
    }
}

/// Show ethernet help
fn show_ethernet_help(context: &mut ShellContext) {
    context.uart.puts("=== Ethernet Commands ===\n");
    context.uart.puts("  network ethernet status  - Show ethernet status\n");
    context.uart.puts("  network ethernet link    - Check link status\n");
    context.uart.puts("  network ethernet stats   - Show statistics\n");
    context.uart.puts("  network ethernet help    - Show this help\n");
}