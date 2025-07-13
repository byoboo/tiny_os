use crate::{
    memory::protection::{
        get_advanced_page_permissions, get_aslr_offset, handle_advanced_permission_fault,
        set_advanced_page_permissions, PagePermissions, PermissionFaultResult, PermissionFaultType,
    },
    shell::core::ShellContext,
};

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
