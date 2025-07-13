// TinyOS Shell Process Context Commands
// Focused module for process context management testing

use crate::{process, shell::ShellContext};

/// Handle process context management test
pub fn handle_process_context_test(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== Process Context Management Test ===\r\n");

    // Test process context creation
    context
        .uart
        .puts("1. Testing Process Context Creation...\r\n");

    let test_context = process::context::ProcessContext::new(
        1,
        0x1000_0000, // user stack
        0x2000_0000, // kernel stack
        0x3000_0000, // entry point
    );

    context
        .uart
        .puts("   ✓ Process context created successfully\r\n");
    context.uart.puts("   Process ID: ");
    context.uart.put_hex(test_context.pid as u64);
    context.uart.puts("\r\n");

    context.uart.puts("   User Stack: 0x");
    context.uart.put_hex(test_context.user_stack_pointer);
    context.uart.puts("\r\n");

    context.uart.puts("   Kernel Stack: 0x");
    context.uart.put_hex(test_context.kernel_stack_pointer);
    context.uart.puts("\r\n");

    context.uart.puts("   Entry Point: 0x");
    context.uart.put_hex(test_context.program_counter);
    context.uart.puts("\r\n");

    // Test process state management
    context
        .uart
        .puts("\r\n2. Testing Process State Management...\r\n");

    let mut test_context = test_context;

    context.uart.puts("   Initial State: ");
    match test_context.get_state() {
        process::context::ProcessState::Ready => context.uart.puts("Ready"),
        process::context::ProcessState::Running => context.uart.puts("Running"),
        process::context::ProcessState::Blocked => context.uart.puts("Blocked"),
        process::context::ProcessState::Terminated => context.uart.puts("Terminated"),
    }
    context.uart.puts("\r\n");

    // Test state transitions
    test_context.set_state(process::context::ProcessState::Running);
    context.uart.puts("   After transition: Running\r\n");

    test_context.set_state(process::context::ProcessState::Blocked);
    context.uart.puts("   After transition: Blocked\r\n");

    test_context.set_state(process::context::ProcessState::Terminated);
    context.uart.puts("   After transition: Terminated\r\n");

    // Test context switching
    context
        .uart
        .puts("\r\n3. Testing Context Switch Operations...\r\n");

    let mut test_context =
        process::context::ProcessContext::new(2, 0x4000_0000, 0x5000_0000, 0x6000_0000);

    // Test context save
    let save_result = test_context.save_context();
    context.uart.puts("   Context Save Result: ");
    match save_result {
        process::context::ContextSwitchResult::Success => context.uart.puts("Success"),
        process::context::ContextSwitchResult::InvalidState => context.uart.puts("Invalid State"),
        process::context::ContextSwitchResult::HardwareError => context.uart.puts("Hardware Error"),
        process::context::ContextSwitchResult::MemoryError => context.uart.puts("Memory Error"),
    }
    context.uart.puts("\r\n");

    // Test context restore
    let restore_result = test_context.restore_context();
    context.uart.puts("   Context Restore Result: ");
    match restore_result {
        process::context::ContextSwitchResult::Success => context.uart.puts("Success"),
        process::context::ContextSwitchResult::InvalidState => context.uart.puts("Invalid State"),
        process::context::ContextSwitchResult::HardwareError => context.uart.puts("Hardware Error"),
        process::context::ContextSwitchResult::MemoryError => context.uart.puts("Memory Error"),
    }
    context.uart.puts("\r\n");

    // Test time slice management
    context
        .uart
        .puts("\r\n4. Testing Time Slice Management...\r\n");

    test_context.set_time_slice(5);
    context.uart.puts("   Time slice set to 5\r\n");

    for i in 1..=6 {
        let expired = test_context.decrement_time_slice();
        context.uart.puts("   Tick ");
        context.uart.put_hex(i);
        context.uart.puts(": ");
        if expired {
            context.uart.puts("Time slice expired\r\n");
        } else {
            context.uart.puts("Time slice remaining\r\n");
        }
    }

    // Show context statistics
    context.uart.puts("\r\n5. Context Statistics:\r\n");
    let (switches, failures) = process::context::get_context_stats();
    context.uart.puts("   Context switches: ");
    context.uart.put_hex(switches);
    context.uart.puts("\r\n");
    context.uart.puts("   Switch failures: ");
    context.uart.put_hex(failures);
    context.uart.puts("\r\n");

    context
        .uart
        .puts("\r\n✅ Process Context Management Test Complete\r\n");
}
