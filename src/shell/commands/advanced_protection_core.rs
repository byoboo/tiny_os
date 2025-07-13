use crate::shell::core::ShellContext;

/// Main advanced memory protection command handler
pub fn cmd_advanced_protection(args: &[&str], context: &mut ShellContext) {
    if args.is_empty() {
        show_advanced_protection_help(context);
        return;
    }

    match args[0] {
        "status" => super::advanced_protection_status::cmd_advanced_protection_status(args, context),
        "permissions" => super::advanced_protection_permissions::cmd_advanced_protection_permissions(args, context),
        "aslr" => super::advanced_protection_aslr::cmd_advanced_protection_aslr(args, context),
        "stack" => super::advanced_protection_stack::cmd_advanced_protection_stack(args, context),
        "test" => super::advanced_protection_testing::cmd_advanced_protection_test(args, context),
        "stats" => super::advanced_protection_stats::cmd_advanced_protection_stats(args, context),
        "help" => show_advanced_protection_help(context),
        _ => {
            context.uart.puts(
                "Unknown advanced protection command. Use 'help' for available commands.\r\n",
            );
        }
    }
}

/// Show advanced protection help
fn show_advanced_protection_help(context: &mut ShellContext) {
    context
        .uart
        .puts("Advanced Memory Protection Commands:\r\n");
    context
        .uart
        .puts("  status          - Show protection system status\r\n");
    context
        .uart
        .puts("  permissions     - Manage page permissions\r\n");
    context
        .uart
        .puts("  aslr            - Address Space Layout Randomization\r\n");
    context
        .uart
        .puts("  stack           - Stack protection features\r\n");
    context
        .uart
        .puts("  test            - Run protection system tests\r\n");
    context
        .uart
        .puts("  stats           - Show protection statistics\r\n");
    context
        .uart
        .puts("  help            - Show this help message\r\n");
}
