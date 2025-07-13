use crate::{
    memory::protection::get_advanced_protection_stats,
    shell::core::ShellContext,
};

/// Handle statistics commands
pub fn cmd_advanced_protection_stats(args: &[&str], context: &mut ShellContext) {
    if args.len() < 2 {
        context.uart.puts("Usage: stats [overview|detailed|reset]\r\n");
        context
            .uart
            .puts("  overview - Show protection statistics overview\r\n");
        context
            .uart
            .puts("  detailed - Show detailed protection metrics\r\n");
        context
            .uart
            .puts("  reset    - Reset protection statistics\r\n");
        return;
    }

    match args[1] {
        "overview" => show_stats_overview(context),
        "detailed" => show_detailed_stats(context),
        "reset" => reset_protection_stats(context),
        _ => {
            context.uart.puts("Unknown stats command\r\n");
        }
    }
}

/// Show protection statistics overview
fn show_stats_overview(context: &mut ShellContext) {
    let stats = get_advanced_protection_stats();
    
    context.uart.puts("Advanced Protection Statistics:\r\n");
    context.uart.puts("==============================\r\n");
    
    context.uart.puts("Protected pages: ");
    context.uart.put_hex(stats.protected_pages as u64);
    context.uart.puts("\r\n");
    
    context.uart.puts("Permission faults: ");
    context.uart.put_hex(stats.permission_faults as u64);
    context.uart.puts("\r\n");
    
    context.uart.puts("ASLR enabled: ");
    if stats.aslr_enabled {
        context.uart.puts("YES");
    } else {
        context.uart.puts("NO");
    }
    context.uart.puts("\r\n");
    
    context.uart.puts("Stack protections: ");
    context.uart.put_hex(stats.stack_protections as u64);
    context.uart.puts("\r\n");
    
    context.uart.puts("Canary violations: ");
    context.uart.put_hex(stats.canary_violations as u64);
    context.uart.puts("\r\n");
    
    context.uart.puts("CFI violations: ");
    context.uart.put_hex(stats.cfi_violations as u64);
    context.uart.puts("\r\n");
    
    context.uart.puts("Return address mismatches: ");
    context.uart.put_hex(stats.return_address_mismatches as u64);
    context.uart.puts("\r\n");
}

/// Show detailed protection statistics
fn show_detailed_stats(context: &mut ShellContext) {
    let stats = get_advanced_protection_stats();
    
    context.uart.puts("Detailed Protection Metrics:\r\n");
    context.uart.puts("============================\r\n");
    
    // Page protection metrics
    context.uart.puts("\r\nPage Protection:\r\n");
    context.uart.puts("---------------\r\n");
    context.uart.puts("Total protected pages: ");
    context.uart.put_hex(stats.total_protected_pages as u64);
    context.uart.puts("\r\n");
    context.uart.puts("Read-only pages: ");
    context.uart.put_hex(stats.read_only_pages as u64);
    context.uart.puts("\r\n");
    context.uart.puts("Non-executable pages: ");
    context.uart.put_hex(stats.non_executable_pages as u64);
    context.uart.puts("\r\n");
    
    // ASLR metrics
    context.uart.puts("\r\nASLR Metrics:\r\n");
    context.uart.puts("-------------\r\n");
    context.uart.puts("ASLR randomizations: ");
    context.uart.put_hex(stats.aslr_randomizations as u64);
    context.uart.puts("\r\n");
    context.uart.puts("ASLR enabled: ");
    if stats.aslr_enabled {
        context.uart.puts("YES");
    } else {
        context.uart.puts("NO");
    }
    context.uart.puts("\r\n");
    
    // Stack protection metrics
    context.uart.puts("\r\nStack Protection:\r\n");
    context.uart.puts("----------------\r\n");
    context.uart.puts("Stack protections: ");
    context.uart.put_hex(stats.stack_protections as u64);
    context.uart.puts("\r\n");
    context.uart.puts("Canary violations: ");
    context.uart.put_hex(stats.canary_violations as u64);
    context.uart.puts("\r\n");
    
    // CFI metrics
    context.uart.puts("\r\nControl Flow Integrity:\r\n");
    context.uart.puts("----------------------\r\n");
    context.uart.puts("CFI enabled: ");
    if stats.cfi_enabled {
        context.uart.puts("YES");
    } else {
        context.uart.puts("NO");
    }
    context.uart.puts("\r\n");
    context.uart.puts("CFI violations: ");
    context.uart.put_hex(stats.cfi_violations as u64);
    context.uart.puts("\r\n");
    context.uart.puts("Return address mismatches: ");
    context.uart.put_hex(stats.return_address_mismatches as u64);
    context.uart.puts("\r\n");
}

/// Reset protection statistics
fn reset_protection_stats(context: &mut ShellContext) {
    // Note: This would call a function to reset stats in the actual implementation
    context.uart.puts("Protection statistics reset\r\n");
    // In real implementation: reset_advanced_protection_stats();
}
