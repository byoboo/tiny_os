// TinyOS Shell Privilege Management Commands
// Focused module for privilege level management testing

use crate::{process, shell::ShellContext};

/// Handle privilege level management test
pub fn handle_privilege_test(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== Privilege Level Management Test ===\r\n");

    // Test current privilege level
    context
        .uart
        .puts("1. Testing Current Privilege Level...\r\n");

    let current_level = process::privilege::get_current_privilege_level();
    context.uart.puts("   Current Privilege Level: ");
    match current_level {
        process::privilege::PrivilegeLevel::EL0 => context.uart.puts("EL0 (User Mode)"),
        process::privilege::PrivilegeLevel::EL1 => context.uart.puts("EL1 (Kernel Mode)"),
        process::privilege::PrivilegeLevel::EL2 => context.uart.puts("EL2 (Hypervisor Mode)"),
        process::privilege::PrivilegeLevel::EL3 => context.uart.puts("EL3 (Secure Mode)"),
    }
    context.uart.puts("\r\n");

    // Test privilege level checks
    context
        .uart
        .puts("\r\n2. Testing Privilege Level Checks...\r\n");

    context.uart.puts("   Is User Mode: ");
    if process::privilege::is_user_mode() {
        context.uart.puts("Yes");
    } else {
        context.uart.puts("No");
    }
    context.uart.puts("\r\n");

    context.uart.puts("   Is Kernel Mode: ");
    if process::privilege::is_kernel_mode() {
        context.uart.puts("Yes");
    } else {
        context.uart.puts("No");
    }
    context.uart.puts("\r\n");

    // Test privilege validation
    context
        .uart
        .puts("\r\n3. Testing Privilege Validation...\r\n");

    let validation_result =
        process::privilege::validate_privilege(process::privilege::PrivilegeLevel::EL1);
    context.uart.puts("   EL1 Privilege Validation: ");
    match validation_result {
        Ok(()) => context.uart.puts("Passed"),
        Err(msg) => {
            context.uart.puts("Failed - ");
            context.uart.puts(msg);
        }
    }
    context.uart.puts("\r\n");

    // Test SPSR conversion
    context.uart.puts("\r\n4. Testing SPSR Conversion...\r\n");

    let el0_spsr = process::privilege::PrivilegeLevel::EL0.to_spsr_bits();
    let el1_spsr = process::privilege::PrivilegeLevel::EL1.to_spsr_bits();

    context.uart.puts("   EL0 SPSR bits: 0x");
    context.uart.put_hex(el0_spsr);
    context.uart.puts("\r\n");

    context.uart.puts("   EL1 SPSR bits: 0x");
    context.uart.put_hex(el1_spsr);
    context.uart.puts("\r\n");

    // Test privilege statistics
    context.uart.puts("\r\n5. Privilege Statistics:\r\n");
    let (el0_to_el1, el1_to_el0, violations, syscalls) = process::privilege::get_privilege_stats();
    context.uart.puts("   EL0 to EL1 transitions: ");
    context.uart.put_hex(el0_to_el1);
    context.uart.puts("\r\n");
    context.uart.puts("   EL1 to EL0 transitions: ");
    context.uart.put_hex(el1_to_el0);
    context.uart.puts("\r\n");
    context.uart.puts("   Privilege violations: ");
    context.uart.put_hex(violations);
    context.uart.puts("\r\n");
    context.uart.puts("   System calls: ");
    context.uart.put_hex(syscalls);
    context.uart.puts("\r\n");

    context
        .uart
        .puts("\r\n✅ Privilege Level Management Test Complete\r\n");
}

/// Handle privilege statistics display
pub fn handle_privilege_stats(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== Privilege Management Statistics ===\r\n");

    let (el0_to_el1, el1_to_el0, violations, syscalls) = process::privilege::get_privilege_stats();

    context.uart.puts("EL0 to EL1 Transitions: ");
    context.uart.put_hex(el0_to_el1);
    context.uart.puts("\r\n");

    context.uart.puts("EL1 to EL0 Transitions: ");
    context.uart.put_hex(el1_to_el0);
    context.uart.puts("\r\n");

    context.uart.puts("Privilege Violations: ");
    context.uart.put_hex(violations);
    context.uart.puts("\r\n");

    context.uart.puts("System Calls: ");
    context.uart.put_hex(syscalls);
    context.uart.puts("\r\n");

    context.uart.puts("Current Privilege Level: ");
    let current_level = process::privilege::get_current_privilege_level();
    match current_level {
        process::privilege::PrivilegeLevel::EL0 => context.uart.puts("EL0 (User Mode)"),
        process::privilege::PrivilegeLevel::EL1 => context.uart.puts("EL1 (Kernel Mode)"),
        process::privilege::PrivilegeLevel::EL2 => context.uart.puts("EL2 (Hypervisor Mode)"),
        process::privilege::PrivilegeLevel::EL3 => context.uart.puts("EL3 (Secure Mode)"),
    }
    context.uart.puts("\r\n");
}
