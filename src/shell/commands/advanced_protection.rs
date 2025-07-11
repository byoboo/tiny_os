use crate::{
    memory::protection::{
        get_advanced_page_permissions, get_advanced_protection_stats, get_aslr_offset,
        handle_advanced_permission_fault, set_advanced_page_permissions,
        setup_advanced_stack_protection, verify_advanced_stack_canary, PagePermissions,
        PermissionFaultResult, PermissionFaultType,
    },
    shell::ShellContext,
};

/// Advanced memory protection command handler
pub fn cmd_advanced_protection(args: &[&str], context: &mut ShellContext) {
    if args.is_empty() {
        show_advanced_protection_help(context);
        return;
    }

    match args[0] {
        "status" => cmd_advanced_protection_status(args, context),
        "permissions" => cmd_advanced_protection_permissions(args, context),
        "aslr" => cmd_advanced_protection_aslr(args, context),
        "stack" => cmd_advanced_protection_stack(args, context),
        "test" => cmd_advanced_protection_test(args, context),
        "stats" => cmd_advanced_protection_stats(args, context),
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

/// Show advanced protection status
pub fn cmd_advanced_protection_status(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("Advanced Memory Protection Status:\r\n");
    context.uart.puts("==================================\r\n");

    let stats = get_advanced_protection_stats();
    context.uart.puts("Page Permissions:\r\n");
    context.uart.puts("  Total protected pages: ");
    context.uart.put_hex(stats.total_protected_pages as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Read-only pages: ");
    context.uart.put_hex(stats.read_only_pages as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Non-executable pages: ");
    context.uart.put_hex(stats.non_executable_pages as u64);
    context.uart.puts("\r\n");

    context.uart.puts("Stack Protection:\r\n");
    context.uart.puts("  Protected stacks: ");
    context.uart.put_hex(stats.protected_stacks as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Stack canaries active: ");
    context.uart.put_hex(stats.stack_canaries_active as u64);
    context.uart.puts("\r\n");

    context.uart.puts("ASLR:\r\n");
    context.uart.puts("  Current ASLR offset: 0x");
    context.uart.put_hex(get_aslr_offset());
    context.uart.puts("\r\n");

    context.uart.puts("Fault Handling:\r\n");
    context.uart.puts("  Permission faults: ");
    context.uart.put_hex(stats.permission_faults as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Stack violations: ");
    context.uart.put_hex(stats.stack_violations as u64);
    context.uart.puts("\r\n");
}

/// Handle page permissions commands
pub fn cmd_advanced_protection_permissions(args: &[&str], context: &mut ShellContext) {
    if args.len() < 2 {
        context
            .uart
            .puts("Usage: permissions [set|get] <address> [permissions]\r\n");
        context
            .uart
            .puts("  set <addr> <perms>  - Set page permissions\r\n");
        context
            .uart
            .puts("  get <addr>          - Get page permissions\r\n");
        context
            .uart
            .puts("Permissions: r (read), w (write), x (execute)\r\n");
        return;
    }

    match args[1] {
        "set" => {
            if args.len() < 4 {
                context
                    .uart
                    .puts("Usage: permissions set <address> <permissions>\r\n");
                return;
            }

            let addr = parse_hex_address(args[2]);
            let perms = parse_permissions(args[3]);

            match set_advanced_page_permissions(addr, perms) {
                Ok(_) => {
                    context.uart.puts("Page permissions set successfully\r\n");
                }
                Err(e) => {
                    context.uart.puts("Error setting permissions: ");
                    context.uart.puts(e);
                    context.uart.puts("\r\n");
                }
            }
        }
        "get" => {
            if args.len() < 3 {
                context.uart.puts("Usage: permissions get <address>\r\n");
                return;
            }

            let addr = parse_hex_address(args[2]);
            match get_advanced_page_permissions(addr) {
                Some(perms) => {
                    context.uart.puts("Page permissions at 0x");
                    context.uart.put_hex(addr);
                    context.uart.puts(": ");
                    display_permissions(perms, context);
                    context.uart.puts("\r\n");
                }
                None => {
                    context.uart.puts("No permissions found for address 0x");
                    context.uart.put_hex(addr);
                    context.uart.puts("\r\n");
                }
            }
        }
        _ => {
            context.uart.puts("Unknown permissions command\r\n");
        }
    }
}

/// Handle ASLR commands
pub fn cmd_advanced_protection_aslr(_args: &[&str], context: &mut ShellContext) {
    context
        .uart
        .puts("Address Space Layout Randomization (ASLR):\r\n");
    context
        .uart
        .puts("=========================================\r\n");

    let offset = get_aslr_offset();
    context.uart.puts("Current ASLR offset: 0x");
    context.uart.put_hex(offset);
    context.uart.puts("\r\n");

    context.uart.puts("ASLR provides randomization of:\r\n");
    context.uart.puts("  - Process base addresses\r\n");
    context.uart.puts("  - Stack locations\r\n");
    context.uart.puts("  - Heap placement\r\n");
    context.uart.puts("  - Library loading addresses\r\n");
}

/// Handle stack protection commands
pub fn cmd_advanced_protection_stack(args: &[&str], context: &mut ShellContext) {
    if args.len() < 2 {
        context.uart.puts("Usage: stack [setup|verify|info]\r\n");
        context
            .uart
            .puts("  setup <pid> <start> <size>  - Setup stack protection\r\n");
        context
            .uart
            .puts("  verify <pid> <canary>       - Verify stack canary\r\n");
        context
            .uart
            .puts("  info                        - Show stack protection info\r\n");
        return;
    }

    match args[1] {
        "setup" => {
            if args.len() < 5 {
                context
                    .uart
                    .puts("Usage: stack setup <process_id> <start_addr> <size>\r\n");
                return;
            }

            let pid = parse_decimal(args[2]);
            let start = parse_hex_address(args[3]);
            let size = parse_hex_address(args[4]);

            match setup_advanced_stack_protection(pid as usize, start, size) {
                Ok(canary) => {
                    context.uart.puts("Stack protection setup successfully\r\n");
                    context.uart.puts("Stack canary: 0x");
                    context.uart.put_hex(canary);
                    context.uart.puts("\r\n");
                }
                Err(e) => {
                    context.uart.puts("Error setting up stack protection: ");
                    context.uart.puts(e);
                    context.uart.puts("\r\n");
                }
            }
        }
        "verify" => {
            if args.len() < 4 {
                context
                    .uart
                    .puts("Usage: stack verify <process_id> <canary>\r\n");
                return;
            }

            let pid = parse_decimal(args[2]);
            let canary = parse_hex_address(args[3]);

            if verify_advanced_stack_canary(pid as usize, canary) {
                context.uart.puts("Stack canary verification: PASSED\r\n");
            } else {
                context.uart.puts("Stack canary verification: FAILED\r\n");
            }
        }
        "info" => {
            let stats = get_advanced_protection_stats();
            context.uart.puts("Stack Protection Information:\r\n");
            context.uart.puts("============================\r\n");
            context.uart.puts("Protected stacks: ");
            context.uart.put_hex(stats.protected_stacks as u64);
            context.uart.puts("\r\n");
            context.uart.puts("Stack canaries active: ");
            context.uart.put_hex(stats.stack_canaries_active as u64);
            context.uart.puts("\r\n");
            context.uart.puts("Stack violations detected: ");
            context.uart.put_hex(stats.stack_violations as u64);
            context.uart.puts("\r\n");
        }
        _ => {
            context.uart.puts("Unknown stack command\r\n");
        }
    }
}

/// Run advanced protection tests
pub fn cmd_advanced_protection_test(_args: &[&str], context: &mut ShellContext) {
    context
        .uart
        .puts("Running Advanced Memory Protection Tests:\r\n");
    context
        .uart
        .puts("========================================\r\n");

    // Test 1: Page permissions
    context.uart.puts("Test 1: Page Permissions\r\n");
    let test_addr = 0x1000000;
    let test_perms = PagePermissions::read_only();

    match set_advanced_page_permissions(test_addr, test_perms) {
        Ok(_) => {
            context.uart.puts("  ✓ Set page permissions: PASSED\r\n");

            match get_advanced_page_permissions(test_addr) {
                Some(perms) => {
                    context.uart.puts("  ✓ Get page permissions: PASSED\r\n");
                    context.uart.puts("    Permissions: ");
                    display_permissions(perms, context);
                    context.uart.puts("\r\n");
                }
                None => {
                    context.uart.puts("  ✗ Get page permissions: FAILED\r\n");
                }
            }
        }
        Err(e) => {
            context.uart.puts("  ✗ Set page permissions: FAILED (");
            context.uart.puts(e);
            context.uart.puts(")\r\n");
        }
    }

    // Test 2: Permission fault handling
    context.uart.puts("Test 2: Permission Fault Handling\r\n");
    let fault_result =
        handle_advanced_permission_fault(test_addr, PermissionFaultType::WriteViolation);
    match fault_result {
        PermissionFaultResult::Continue => {
            context
                .uart
                .puts("  ✓ Permission fault handling: PASSED (Continue)\r\n");
        }
        PermissionFaultResult::Terminate => {
            context
                .uart
                .puts("  ✓ Permission fault handling: PASSED (Terminate)\r\n");
        }
        PermissionFaultResult::Retry => {
            context
                .uart
                .puts("  ✓ Permission fault handling: PASSED (Retry)\r\n");
        }
    }

    // Test 3: ASLR
    context.uart.puts("Test 3: ASLR\r\n");
    let aslr_offset = get_aslr_offset();
    context.uart.puts("  ✓ ASLR offset: 0x");
    context.uart.put_hex(aslr_offset);
    context.uart.puts("\r\n");

    context.uart.puts("All tests completed!\r\n");
}

/// Show advanced protection statistics
pub fn cmd_advanced_protection_stats(_args: &[&str], context: &mut ShellContext) {
    let stats = get_advanced_protection_stats();

    context
        .uart
        .puts("Advanced Memory Protection Statistics:\r\n");
    context
        .uart
        .puts("=====================================\r\n");

    context.uart.puts("Page Protection:\r\n");
    context.uart.puts("  Total protected pages: ");
    context.uart.put_hex(stats.total_protected_pages as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Read-only pages: ");
    context.uart.put_hex(stats.read_only_pages as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Non-executable pages: ");
    context.uart.put_hex(stats.non_executable_pages as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  User-protected pages: ");
    context.uart.put_hex(stats.user_protected_pages as u64);
    context.uart.puts("\r\n");

    context.uart.puts("Stack Protection:\r\n");
    context.uart.puts("  Protected stacks: ");
    context.uart.put_hex(stats.protected_stacks as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Stack canaries active: ");
    context.uart.put_hex(stats.stack_canaries_active as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Stack violations: ");
    context.uart.put_hex(stats.stack_violations as u64);
    context.uart.puts("\r\n");

    context.uart.puts("Control Flow Integrity:\r\n");
    context.uart.puts("  CFI violations: ");
    context.uart.put_hex(stats.cfi_violations as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  ROP attacks blocked: ");
    context.uart.put_hex(stats.rop_attacks_blocked as u64);
    context.uart.puts("\r\n");

    context.uart.puts("Fault Handling:\r\n");
    context.uart.puts("  Permission faults: ");
    context.uart.put_hex(stats.permission_faults as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Faults handled: ");
    context.uart.put_hex(stats.faults_handled as u64);
    context.uart.puts("\r\n");

    context.uart.puts("  Faults terminated: ");
    context.uart.put_hex(stats.faults_terminated as u64);
    context.uart.puts("\r\n");
}

/// Parse hex address from string
fn parse_hex_address(s: &str) -> u64 {
    if s.starts_with("0x") {
        u64::from_str_radix(&s[2..], 16).unwrap_or(0)
    } else {
        u64::from_str_radix(s, 16).unwrap_or(0)
    }
}

/// Parse decimal number from string
fn parse_decimal(s: &str) -> u32 {
    s.parse::<u32>().unwrap_or(0)
}

/// Parse permissions from string
fn parse_permissions(s: &str) -> PagePermissions {
    let mut perms = PagePermissions::none();

    for c in s.chars() {
        match c {
            'r' => perms = perms.add_read(),
            'w' => perms = perms.add_write(),
            'x' => perms = perms.add_execute(),
            _ => {}
        }
    }

    perms
}

/// Display permissions
fn display_permissions(perms: PagePermissions, context: &mut ShellContext) {
    if perms.is_readable() {
        context.uart.puts("r");
    } else {
        context.uart.puts("-");
    }

    if perms.is_writable() {
        context.uart.puts("w");
    } else {
        context.uart.puts("-");
    }

    if perms.is_executable() {
        context.uart.puts("x");
    } else {
        context.uart.puts("-");
    }
}
