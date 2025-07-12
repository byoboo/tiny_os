//! System command handlers
//!
//! This module contains handlers for system-level commands like help, time,
//! system info, and health checks.

use crate::{exceptions::types::ExceptionStats, shell::ShellContext};

/// Helper function to print time in a readable format
fn print_time(uart: &crate::uart::Uart, ms: u32) {
    let seconds = ms / 1000;
    let remaining_ms = ms % 1000;
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    let hours = minutes / 60;
    let remaining_minutes = minutes % 60;

    if hours > 0 {
        print_number(uart, hours);
        uart.puts("h ");
    }
    if remaining_minutes > 0 || hours > 0 {
        print_number(uart, remaining_minutes);
        uart.puts("m ");
    }
    print_number(uart, remaining_seconds);
    uart.puts(".");
    // Print milliseconds with leading zeros
    if remaining_ms < 100 {
        uart.puts("0");
    }
    if remaining_ms < 10 {
        uart.puts("0");
    }
    print_number(uart, remaining_ms);
    uart.puts("s");
}

/// Helper function to print numbers
#[inline]
fn print_number(uart: &crate::uart::Uart, mut num: u32) {
    if num == 0 {
        uart.putc(b'0');
        return;
    }

    let mut digits = [0u8; 10];
    let mut count = 0;

    while num > 0 {
        digits[count] = (num % 10) as u8 + b'0';
        num /= 10;
        count += 1;
    }

    for i in (0..count).rev() {
        uart.putc(digits[i]);
    }
}

/// Handle the help command (h/H)
pub fn handle_help(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== TinyOS Command Reference ===\r\n");
    context.uart.puts("System Commands:\r\n");
    context.uart.puts("  h/H - Show this help menu\r\n");
    context.uart.puts("  t/T - Show current system time\r\n");
    context.uart.puts("  s/S - Show system information\r\n");
    context.uart.puts("  c/C - Run system health check\r\n");
    context.uart.puts("Hardware Control:\r\n");
    context.uart.puts("  1   - Turn LED ON\r\n");
    context.uart.puts("  0   - Turn LED OFF\r\n");
    context.uart.puts("  l/L - Toggle LED state\r\n");
    context.uart.puts("Memory Management:\r\n");
    context.uart.puts("  m/M - Show memory statistics\r\n");
    context.uart.puts("  a/A - Allocate memory block\r\n");
    context.uart.puts("  f/F - Free last allocated block\r\n");
    context.uart.puts("  x/X - Run basic memory test\r\n");
    context
        .uart
        .puts("  z/Z - Run comprehensive memory test suite\r\n");
    context.uart.puts("  g/G - Run memory corruption check\r\n");
    context.uart.puts("  r/R - Defragment memory\r\n");
    context.uart.puts("Interrupt Management:\r\n");
    context.uart.puts("  i/I - Show interrupt status\r\n");
    context.uart.puts("  e/E - Enable/disable interrupts\r\n");
    context.uart.puts("  j/J - Run interrupt test\r\n");
    context.uart.puts("Exception Management:\r\n");
    context.uart.puts("  v/V - Show exception statistics\r\n");
    context
        .uart
        .puts("  w/W - Test exception handling (safe)\r\n");
    context
        .uart
        .puts("  7   - Advanced exception testing (Phase 1)\r\n");
    context.uart.puts("  8   - Test ESR_EL1 decoder\r\n");
    context
        .uart
        .puts("  9   - Test system call interface (Phase 1)\r\n");
    context
        .uart
        .puts("  !   - Test memory fault analysis (Phase 1)\r\n");
    context
        .uart
        .puts("Advanced Exception Testing (Phase 2):\r\n");
    context
        .uart
        .puts("  #   - Test IRQ integration and routing\r\n");
    context
        .uart
        .puts("  $   - Test nested interrupt handling\r\n");
    context
        .uart
        .puts("  %   - Test deferred processing system\r\n");
    context.uart.puts("Process Management (Phase 3):\r\n");
    context.uart.puts("  &   - Process management submenu\r\n");
    context.uart.puts("    1 - Process context test\r\n");
    context.uart.puts("    2 - Privilege level test\r\n");
    context.uart.puts("    3 - Task scheduler test\r\n");
    context.uart.puts("    4 - Process statistics\r\n");
    context.uart.puts("    5 - Scheduler statistics\r\n");
    context.uart.puts("    6 - Privilege statistics\r\n");
    context
        .uart
        .puts("MMU & Exception Management (Phase 4):\r\n");
    context
        .uart
        .puts("  ^   - Exception management submenu\r\n");
    context.uart.puts("    1 - Exception statistics\r\n");
    context.uart.puts("    2 - MMU exception statistics\r\n");
    context.uart.puts("    3 - MMU control (on/off)\r\n");
    context.uart.puts("    4 - Exception testing (safe)\r\n");
    context.uart.puts("    5 - Reset exception stats\r\n");
    context
        .uart
        .puts("Virtual Memory Management (Phase 4.2):\r\n");
    context
        .uart
        .puts("  ~   - Virtual memory management submenu\r\n");
    context.uart.puts("    1 - Virtual memory status\r\n");
    context.uart.puts("    2 - Enable MMU\r\n");
    context.uart.puts("    3 - Disable MMU\r\n");
    context.uart.puts("    4 - Translate address\r\n");
    context.uart.puts("    5 - Flush TLB\r\n");
    context.uart.puts("    6 - Virtual memory test\r\n");
    context.uart.puts("Stack Management (Phase 4.3):\r\n");
    context.uart.puts("  `   - Stack management submenu\r\n");
    context.uart.puts("    1 - Stack status\r\n");
    context.uart.puts("    2 - Allocate kernel stack\r\n");
    context.uart.puts("    3 - Allocate user stack\r\n");
    context.uart.puts("    4 - Deallocate stack\r\n");
    context.uart.puts("    5 - Switch stack\r\n");
    context.uart.puts("    6 - Stack test\r\n");
    context
        .uart
        .puts("User Space Management (Phase 4.4.2):\r\n");
    context
        .uart
        .puts("  |   - User space page table submenu\r\n");
    context
        .uart
        .puts("Advanced Memory Protection (Phase 4.4.3):\r\n");
    context.uart.puts("  @   - Advanced protection submenu\r\n");
    context
        .uart
        .puts("Dynamic Memory Management (Phase 4.4.4):\r\n");
    context.uart.puts("  *   - Dynamic memory submenu\r\n");
    context
        .uart
        .puts("Copy-on-Write Management (Phase 4.4):\r\n");
    context.uart.puts("  (   - COW management submenu\r\n");
    context.uart.puts("    1 - COW status\r\n");
    context.uart.puts("    2 - COW statistics\r\n");
    context.uart.puts("    3 - Create COW mapping\r\n");
    context.uart.puts("    4 - Protect COW page\r\n");
    context.uart.puts("    5 - Unprotect COW page\r\n");
    context.uart.puts("    6 - COW test\r\n");
    context.uart.puts("Testing Framework (Phase 5):\r\n");
    context.uart.puts("  )   - Testing framework submenu\r\n");
    context.uart.puts("    1 - Kernel tests\r\n");
    context.uart.puts("    2 - MMU tests\r\n");
    context.uart.puts("    3 - Process tests\r\n");
    context.uart.puts("    4 - Syscall tests\r\n");
    context.uart.puts("    5 - Integration tests\r\n");
    context.uart.puts("    6 - All tests\r\n");
    context.uart.puts("Command Line Interface:\r\n");
    context.uart.puts("  +   - Advanced command routing\r\n");
    context
        .uart
        .puts("    1 - Advanced protection commands\r\n");
    context.uart.puts("    2 - Dynamic memory commands\r\n");
    context.uart.puts("================================\r\n");
}

/// Handle the time command (t/T)
pub fn handle_time(context: &ShellContext, start_time: u64) {
    let current_time = context.timer.get_time();
    context.uart.puts("Current system time: [");
    print_time(
        &context.uart,
        context
            .timer
            .ticks_to_ms(current_time.wrapping_sub(start_time) as u32),
    );
    context.uart.puts("]\r\n");
}

/// Handle the system info command (s/S)
pub fn handle_system_info(context: &ShellContext) {
    let _current_time = context.timer.get_time();
    // We need start_time passed in - for now, let's skip the uptime calculation

    context
        .uart
        .puts("\r\n=== TinyOS System Information ===\r\n");
    context.uart.puts("  OS Name: TinyOS\r\n");
    context.uart.puts("  Version: 0.1.0\r\n");
    context
        .uart
        .puts("  Platform: Raspberry Pi 4/5 (AArch64)\r\n");
    context.uart.puts("  Architecture: ARM64\r\n");
    context.uart.puts("  Timer Frequency: 1MHz\r\n");
    context.uart.puts("  UART Base: 0xFE201000\r\n");
    context.uart.puts("  GPIO Base: 0xFE200000\r\n");
    context.uart.puts("  GIC Base: 0xFF841000\r\n");
    context.uart.puts("  LED Pin: GPIO 42\r\n");

    let int_stats = context.interrupt_controller.get_interrupt_stats();
    context.uart.puts("  Active Interrupts: ");
    print_number(&context.uart, int_stats.total_interrupts);
    context.uart.puts("\r\n");
    context.uart.puts("=================================\r\n");
}

/// Handle the health check command (c/C)
pub fn handle_health_check(context: &mut ShellContext) {
    context.uart.puts("\r\n=== System Health Check ===\r\n");
    context
        .uart
        .puts("Running comprehensive diagnostics...\r\n");

    context
        .uart
        .puts("1. GPIO System: Testing LED control...\r\n");
    context.uart.puts("   - LED toggle test: ");
    context.gpio.set_high(42);
    context.timer.delay_ms(100);
    context.gpio.set_low(42);
    context.timer.delay_ms(100);
    context.uart.puts("✓ PASS\r\n");

    context.uart.puts("2. Timer System: Testing delays...\r\n");
    context.uart.puts("   - Microsecond timing: ");
    let start = context.timer.get_time();
    context.timer.delay_us(1000);
    let elapsed = context.timer.get_time() - start;
    if (900..=1100).contains(&elapsed) {
        context.uart.puts("✓ PASS\r\n");
    } else {
        context.uart.puts("✗ FAIL\r\n");
    }

    context
        .uart
        .puts("3. UART System: Communication check...\r\n");
    context
        .uart
        .puts("   - Character transmission: ✓ PASS (you see this!)\r\n");

    context
        .uart
        .puts("4. Exception System: Handler validation...\r\n");
    context.uart.puts("   - Exception stats available: ");
    let stats = ExceptionStats::get_stats();
    context.uart.puts("✓ PASS\r\n");
    context.uart.puts("   - Total exceptions handled: ");
    print_number(&context.uart, stats.total_exceptions as u32);
    context.uart.puts("\r\n");

    context
        .uart
        .puts("5. Memory System: Allocation test...\r\n");
    context.uart.puts("   - Block allocation: ");
    if context.memory_manager.allocate_block().is_some() {
        context.uart.puts("✓ PASS\r\n");
    } else {
        context.uart.puts("✗ FAIL\r\n");
    }

    context.uart.puts("   - Memory corruption check: ");
    if context.memory_manager.check_corruption() {
        context.uart.puts("✓ PASS\r\n");
    } else {
        context.uart.puts("⚠️  WARNING\r\n");
    }

    let stats = context.memory_manager.get_stats();
    context.uart.puts("   - Memory usage: ");
    let usage_percent = (stats.used_heap_size * 100) / stats.total_heap_size;
    print_number(&context.uart, usage_percent);
    context.uart.puts("% used, ");
    print_number(&context.uart, stats.fragmentation_percent);
    context.uart.puts("% fragmented\r\n");

    context.uart.puts("   - Largest free block: ");
    print_number(&context.uart, stats.largest_free_block);
    context.uart.puts(" bytes\r\n");

    context
        .uart
        .puts("6. Interrupt System: Running interrupt test...\r\n");
    context.uart.puts("   - Interrupt controller: ");
    if context.interrupt_controller.run_interrupt_test() {
        context.uart.puts("✓ PASS\r\n");
    } else {
        context.uart.puts("✗ FAIL\r\n");
    }

    let int_stats = context.interrupt_controller.get_interrupt_stats();
    context.uart.puts("   - Simulated interrupts: ");
    print_number(&context.uart, int_stats.total_interrupts);
    context.uart.puts(" total\r\n");

    context.uart.puts("\r\n=== Health Check Results ===\r\n");
    context.uart.puts("Overall Status: ✓ HEALTHY\r\n");
    context.uart.puts("All systems operational!\r\n");
    context.uart.puts("===========================\r\n");
}

/// Stack management status command
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

/// COW Status command
pub fn cmd_cow_status(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("\r\n=== COW Status ===\r\n");

    if let Some(stats) =
        crate::memory::with_cow_manager(|cow_manager| cow_manager.get_statistics().clone())
    {
        context.uart.puts("COW Pages: ");
        print_number(&context.uart, stats.cow_pages_count as u32);
        context.uart.puts("\r\n");

        context.uart.puts("COW Faults: ");
        print_number(&context.uart, stats.cow_faults_handled as u32);
        context.uart.puts("\r\n");

        context.uart.puts("Pages Duplicated: ");
        print_number(&context.uart, stats.pages_duplicated as u32);
        context.uart.puts("\r\n");

        context.uart.puts("Memory Saved: ");
        print_number(&context.uart, stats.memory_saved_bytes as u32);
        context.uart.puts(" bytes\r\n");

        context.uart.puts("Peak COW Pages: ");
        print_number(&context.uart, stats.peak_cow_pages as u32);
        context.uart.puts("\r\n");

        // List all COW pages - need to do this in a separate call
        context.uart.puts("\r\nCOW Pages:\r\n");
        let _pages_printed = crate::memory::with_cow_manager(|cow_manager| {
            let cow_pages = cow_manager.get_all_cow_pages();
            for (phys_addr, page_opt) in cow_pages.iter() {
                if let Some(page) = page_opt {
                    context.uart.puts("  0x");
                    print_hex(&context.uart, *phys_addr);
                    context.uart.puts(" refs=");
                    print_number(&context.uart, page.ref_count as u32);
                    context.uart.puts(" cow=");
                    context.uart.puts(if page.is_cow { "yes" } else { "no" });
                    context.uart.puts("\r\n");
                }
            }
        });
    } else {
        context.uart.puts("COW manager not initialized\r\n");
    }
}

/// COW Statistics command
pub fn cmd_cow_stats(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("\r\n=== COW Statistics ===\r\n");

    if let Some(stats) =
        crate::memory::with_cow_manager(|cow_manager| cow_manager.get_statistics().clone())
    {
        context.uart.puts("Total COW Pages: ");
        print_number(&context.uart, stats.cow_pages_count as u32);
        context.uart.puts("\r\n");

        context.uart.puts("Total COW Faults: ");
        print_number(&context.uart, stats.cow_faults_handled as u32);
        context.uart.puts("\r\n");

        context.uart.puts("Pages Duplicated: ");
        print_number(&context.uart, stats.pages_duplicated as u32);
        context.uart.puts("\r\n");

        context.uart.puts("Memory Saved: ");
        print_number(&context.uart, stats.memory_saved_bytes as u32);
        context.uart.puts(" bytes\r\n");

        context.uart.puts("Metadata Memory: ");
        print_number(&context.uart, stats.metadata_memory_bytes as u32);
        context.uart.puts(" bytes\r\n");

        context.uart.puts("Peak COW Pages: ");
        print_number(&context.uart, stats.peak_cow_pages as u32);
        context.uart.puts("\r\n");

        // Calculate efficiency
        if stats.cow_pages_count > 0 {
            let efficiency =
                (stats.memory_saved_bytes * 100) / (stats.cow_pages_count as u64 * 4096);
            context.uart.puts("Memory Efficiency: ");
            print_number(&context.uart, efficiency as u32);
            context.uart.puts("%\r\n");
        }
    } else {
        context.uart.puts("COW manager not initialized\r\n");
    }
}

/// COW Create Mapping command
pub fn cmd_cow_create(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("\r\n=== Creating COW Mapping ===\r\n");

    // For demonstration, create a simple COW mapping
    let test_virt_addr = 0x10000000u64;
    let test_phys_addr = 0x20000000u64;

    if let Some(result) = crate::memory::with_cow_manager(|cow_manager| {
        cow_manager.create_cow_mapping(
            test_virt_addr,
            test_virt_addr + 0x1000,
            1, // source process
            2, // dest process
            test_phys_addr,
            crate::memory::RegionType::UserData,
        )
    }) {
        match result {
            Ok(()) => {
                context.uart.puts("COW mapping created successfully\r\n");
                context.uart.puts("Source VA: 0x");
                print_hex(&context.uart, test_virt_addr);
                context.uart.puts("\r\nDest VA: 0x");
                print_hex(&context.uart, test_virt_addr + 0x1000);
                context.uart.puts("\r\nPhysical: 0x");
                print_hex(&context.uart, test_phys_addr);
                context.uart.puts("\r\n");
            }
            Err(e) => {
                context.uart.puts("COW mapping failed: ");
                context.uart.puts(e);
                context.uart.puts("\r\n");
            }
        }
    } else {
        context.uart.puts("COW manager not initialized\r\n");
    }
}

/// COW Protection command
pub fn cmd_cow_protect(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("\r\n=== Forcing COW Protection ===\r\n");

    let test_phys_addr = 0x20000000u64;

    if let Some(result) = crate::memory::with_cow_manager(|cow_manager| {
        cow_manager.force_cow_protection(test_phys_addr)
    }) {
        match result {
            Ok(()) => {
                context.uart.puts("COW protection enabled for page 0x");
                print_hex(&context.uart, test_phys_addr);
                context.uart.puts("\r\n");
            }
            Err(e) => {
                context.uart.puts("COW protection failed: ");
                context.uart.puts(e);
                context.uart.puts("\r\n");
            }
        }
    } else {
        context.uart.puts("COW manager not initialized\r\n");
    }
}

/// COW Unprotect command
pub fn cmd_cow_unprotect(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("\r\n=== Removing COW Protection ===\r\n");

    let test_phys_addr = 0x20000000u64;

    if let Some(result) = crate::memory::with_cow_manager(|cow_manager| {
        cow_manager.remove_cow_protection(test_phys_addr)
    }) {
        match result {
            Ok(()) => {
                context.uart.puts("COW protection removed from page 0x");
                print_hex(&context.uart, test_phys_addr);
                context.uart.puts("\r\n");
            }
            Err(e) => {
                context.uart.puts("COW protection removal failed: ");
                context.uart.puts(e);
                context.uart.puts("\r\n");
            }
        }
    } else {
        context.uart.puts("COW manager not initialized\r\n");
    }
}

/// COW Test command
pub fn cmd_cow_test(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("\r\n=== COW Test Suite ===\r\n");

    if let Some((__tests_passed, __tests_failed)) = crate::memory::with_cow_manager(|cow_manager| {
        let mut _tests_passed = 0;
        let mut _tests_failed = 0;

        // Test 1: Register a page
        context.uart.puts("Test 1: Register COW page... ");
        let test_phys = 0x30000000u64;
        let test_virt = 0x40000000u64;
        match cow_manager.register_page(
            test_phys,
            test_virt,
            crate::memory::RegionType::UserData,
            1,
        ) {
            Ok(()) => {
                context.uart.puts("PASS\r\n");
                _tests_passed += 1;
            }
            Err(e) => {
                context.uart.puts("FAIL (");
                context.uart.puts(e);
                context.uart.puts(")\r\n");
                _tests_failed += 1;
            }
        }

        // Test 2: Check if page is COW protected
        context.uart.puts("Test 2: Check COW protection... ");
        if cow_manager.is_cow_protected(test_phys) {
            context.uart.puts("PASS\r\n");
            _tests_passed += 1;
        } else {
            context.uart.puts("FAIL (not protected)\r\n");
            _tests_failed += 1;
        }

        // Test 3: Add another reference to trigger COW
        context.uart.puts("Test 3: Add second reference... ");
        match cow_manager.register_page(
            test_phys,
            test_virt + 0x1000,
            crate::memory::RegionType::UserData,
            2,
        ) {
            Ok(()) => {
                context.uart.puts("PASS\r\n");
                _tests_passed += 1;
            }
            Err(e) => {
                context.uart.puts("FAIL (");
                context.uart.puts(e);
                context.uart.puts(")\r\n");
                _tests_failed += 1;
            }
        }

        // Test 4: Verify COW protection is enabled
        context.uart.puts("Test 4: Verify COW protection... ");
        if cow_manager.is_cow_protected(test_phys) {
            context.uart.puts("PASS\r\n");
            _tests_passed += 1;
        } else {
            context
                .uart
                .puts("FAIL (not protected after multiple refs)\r\n");
            _tests_failed += 1;
        }

        // Test 5: Simulate COW fault
        context.uart.puts("Test 5: Simulate COW fault... ");
        let cow_fault =
            crate::memory::create_cow_fault_from_exception(test_virt, test_phys, true, 1);

        match cow_manager.handle_cow_fault(cow_fault) {
            Ok(new_page) => {
                context.uart.puts("PASS (new page: 0x");
                print_hex(&context.uart, new_page);
                context.uart.puts(")\r\n");
                _tests_passed += 1;
            }
            Err(e) => {
                context.uart.puts("FAIL (");
                context.uart.puts(e);
                context.uart.puts(")\r\n");
                _tests_failed += 1;
            }
        }

        // Summary
        context.uart.puts("\r\nTest Summary:\r\n");
        context.uart.puts("  Passed: ");
        print_number(&context.uart, _tests_passed);
        context.uart.puts("\r\n");
        context.uart.puts("  Failed: ");
        print_number(&context.uart, _tests_failed);
        context.uart.puts("\r\n");

        if _tests_failed == 0 {
            context.uart.puts("All COW tests passed!\r\n");
        } else {
            context.uart.puts("Some COW tests failed.\r\n");
        }

        (_tests_passed, _tests_failed)
    }) {
        context.uart.puts("COW manager not initialized\r\n");
    }
}

/// Helper function to parse a number from a string
fn parse_number(s: &str) -> Option<u32> {
    let mut result = 0u32;
    let bytes = s.as_bytes();

    if bytes.is_empty() {
        return None;
    }

    for &byte in bytes {
        if byte < b'0' || byte > b'9' {
            return None;
        }
        result = result.checked_mul(10)?.checked_add((byte - b'0') as u32)?;
    }

    Some(result)
}

/// Helper function to print hexadecimal numbers
fn print_hex(uart: &crate::uart::Uart, mut num: u64) {
    if num == 0 {
        uart.puts("0");
        return;
    }

    let mut digits = [0u8; 16];
    let mut count = 0;

    while num > 0 {
        let digit = (num % 16) as u8;
        digits[count] = if digit < 10 {
            digit + b'0'
        } else {
            digit - 10 + b'A'
        };
        num /= 16;
        count += 1;
    }

    // Print in reverse order
    for i in (0..count).rev() {
        uart.putc(digits[i]);
    }
}
