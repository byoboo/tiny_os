//! Stack management command handlers
//!
//! This module contains handlers for stack allocation, deallocation, switching,
//! status monitoring, and testing functionality.

use super::utils::{parse_number, print_hex, print_number};
use crate::shell::ShellContext;

/// Stack status command
pub fn cmd_stack_status(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("=== Stack Management Status ===\r\n");

    use crate::memory::get_stack_manager;

    let stack_manager = get_stack_manager();
    let stats = stack_manager.get_statistics();

    context.uart.puts("Stack Allocation:\r\n");
    context.uart.puts("  - Allocated stacks: ");
    print_number(&context.uart, stats.allocated_stacks as u32);
    context.uart.puts(" / ");
    print_number(&context.uart, stats.total_stacks as u32);
    context.uart.puts("\r\n");

    context.uart.puts("  - Total allocations: ");
    print_number(&context.uart, stats.allocation_count as u32);
    context.uart.puts("\r\n");

    context.uart.puts("  - Stack overflows: ");
    print_number(&context.uart, stats.overflow_count as u32);
    context.uart.puts("\r\n");

    context.uart.puts("Usage Statistics:\r\n");
    context.uart.puts("  - Total usage: ");
    print_number(&context.uart, stats.total_usage as u32);
    context.uart.puts(" bytes\r\n");

    context.uart.puts("  - Maximum usage: ");
    print_number(&context.uart, stats.max_usage as u32);
    context.uart.puts(" bytes\r\n");

    // Current stack info
    if let Some(current_stack) = stack_manager.get_current_stack() {
        context.uart.puts("Current Stack:\r\n");
        context.uart.puts("  - Stack ID: ");
        print_number(&context.uart, current_stack.stack_id as u32);
        context.uart.puts("\r\n");

        context.uart.puts("  - Base address: 0x");
        print_hex(&context.uart, current_stack.base_address);
        context.uart.puts("\r\n");

        context.uart.puts("  - Size: ");
        print_number(&context.uart, current_stack.size as u32);
        context.uart.puts(" bytes\r\n");

        context.uart.puts("  - Current SP: 0x");
        print_hex(&context.uart, current_stack.current_sp);
        context.uart.puts("\r\n");

        context.uart.puts("  - Max usage: ");
        print_number(&context.uart, current_stack.max_usage as u32);
        context.uart.puts(" bytes\r\n");

        context.uart.puts("  - Overflows: ");
        print_number(&context.uart, current_stack.overflow_count as u32);
        context.uart.puts("\r\n");

        context.uart.puts("  - Protection: ");
        if current_stack.protection.user_accessible {
            context.uart.puts("User");
        } else {
            context.uart.puts("Kernel");
        }
        context.uart.puts("\r\n");
    } else {
        context
            .uart
            .puts("No current stack information available\r\n");
    }

    context.uart.puts("===============================\r\n");
}

/// Stack allocation command
pub fn cmd_stack_alloc(args: &[&str], context: &mut ShellContext) {
    use crate::memory::{get_stack_manager, StackProtection};

    let protection = if args.len() > 1 && args[1] == "user" {
        StackProtection::USER_STACK
    } else {
        StackProtection::KERNEL_STACK
    };

    let stack_manager = get_stack_manager();

    // We need to get the VMM to allocate a stack
    let vmm = crate::memory::get_virtual_memory_manager();

    match stack_manager.allocate_stack(protection, vmm) {
        Ok(stack_id) => {
            context.uart.puts("Stack allocated successfully\r\n");
            context.uart.puts("Stack ID: ");
            print_number(&context.uart, stack_id as u32);
            context.uart.puts("\r\n");

            if let Some(stack_info) = stack_manager.get_stack_info(stack_id) {
                context.uart.puts("Base address: 0x");
                print_hex(&context.uart, stack_info.base_address);
                context.uart.puts("\r\n");

                context.uart.puts("Size: ");
                print_number(&context.uart, stack_info.size as u32);
                context.uart.puts(" bytes\r\n");
            }
        }
        Err(e) => {
            context.uart.puts("Stack allocation failed: ");
            match e {
                crate::memory::StackError::OutOfMemory => context.uart.puts("Out of memory"),
                crate::memory::StackError::AllocationFailed => {
                    context.uart.puts("Allocation failed")
                }
                _ => context.uart.puts("Unknown error"),
            }
            context.uart.puts("\r\n");
        }
    }
}

/// Stack deallocation command
pub fn cmd_stack_dealloc(args: &[&str], context: &mut ShellContext) {
    if args.len() < 2 {
        context.uart.puts("Usage: stack_dealloc <stack_id>\r\n");
        return;
    }

    // Parse stack ID
    let stack_id = match parse_number(args[1]) {
        Some(id) => id as usize,
        None => {
            context.uart.puts("Invalid stack ID\r\n");
            return;
        }
    };

    use crate::memory::get_stack_manager;

    let stack_manager = get_stack_manager();
    let vmm = crate::memory::get_virtual_memory_manager();

    match stack_manager.deallocate_stack(stack_id, vmm) {
        Ok(()) => {
            context.uart.puts("Stack deallocated successfully\r\n");
        }
        Err(e) => {
            context.uart.puts("Stack deallocation failed: ");
            match e {
                crate::memory::StackError::InvalidStackId => context.uart.puts("Invalid stack ID"),
                crate::memory::StackError::AllocationFailed => {
                    context.uart.puts("Deallocation failed")
                }
                _ => context.uart.puts("Unknown error"),
            }
            context.uart.puts("\r\n");
        }
    }
}

/// Stack switching command
pub fn cmd_stack_switch(args: &[&str], context: &mut ShellContext) {
    if args.len() < 2 {
        context.uart.puts("Usage: stack_switch <stack_id>\r\n");
        return;
    }

    // Parse stack ID
    let stack_id = match parse_number(args[1]) {
        Some(id) => id as usize,
        None => {
            context.uart.puts("Invalid stack ID\r\n");
            return;
        }
    };

    use crate::memory::get_stack_manager;

    let stack_manager = get_stack_manager();

    match stack_manager.switch_stack(stack_id) {
        Ok(new_sp) => {
            context.uart.puts("Stack switched successfully\r\n");
            context.uart.puts("New stack pointer: 0x");
            print_hex(&context.uart, new_sp);
            context.uart.puts("\r\n");

            // Note: In a real implementation, we would need to actually switch
            // the stack pointer using assembly, but for now we just report success
            context
                .uart
                .puts("(Note: Stack pointer update requires assembly integration)\r\n");
        }
        Err(e) => {
            context.uart.puts("Stack switching failed: ");
            match e {
                crate::memory::StackError::InvalidStackId => context.uart.puts("Invalid stack ID"),
                _ => context.uart.puts("Unknown error"),
            }
            context.uart.puts("\r\n");
        }
    }
}

/// Stack test command
pub fn cmd_stack_test(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("=== Stack Management Test ===\r\n");

    use crate::memory::{get_stack_manager, StackProtection};

    let stack_manager = get_stack_manager();
    let vmm = crate::memory::get_virtual_memory_manager();

    // Test 1: Allocate a kernel stack
    context.uart.puts("Test 1: Allocating kernel stack... ");
    match stack_manager.allocate_stack(StackProtection::KERNEL_STACK, vmm) {
        Ok(stack_id) => {
            context.uart.puts("✓ PASS (ID: ");
            print_number(&context.uart, stack_id as u32);
            context.uart.puts(")\r\n");

            // Test 2: Get stack info
            context.uart.puts("Test 2: Getting stack info... ");
            if let Some(stack_info) = stack_manager.get_stack_info(stack_id) {
                context.uart.puts("✓ PASS\r\n");
                context.uart.puts("   Base: 0x");
                print_hex(&context.uart, stack_info.base_address);
                context.uart.puts(", Size: ");
                print_number(&context.uart, stack_info.size as u32);
                context.uart.puts(" bytes\r\n");
            } else {
                context.uart.puts("✗ FAIL\r\n");
            }

            // Test 3: Deallocate stack
            context.uart.puts("Test 3: Deallocating stack... ");
            match stack_manager.deallocate_stack(stack_id, vmm) {
                Ok(()) => context.uart.puts("✓ PASS\r\n"),
                Err(_) => context.uart.puts("✗ FAIL\r\n"),
            }
        }
        Err(_) => {
            context.uart.puts("✗ FAIL\r\n");
        }
    }

    // Test 4: Allocate user stack
    context.uart.puts("Test 4: Allocating user stack... ");
    match stack_manager.allocate_stack(StackProtection::USER_STACK, vmm) {
        Ok(stack_id) => {
            context.uart.puts("✓ PASS (ID: ");
            print_number(&context.uart, stack_id as u32);
            context.uart.puts(")\r\n");

            // Clean up
            let _ = stack_manager.deallocate_stack(stack_id, vmm);
        }
        Err(_) => {
            context.uart.puts("✗ FAIL\r\n");
        }
    }

    context.uart.puts("=============================\r\n");
}
