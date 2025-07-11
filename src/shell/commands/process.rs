// TinyOS Shell Process Management Commands
// Phase 3: Process Management Foundation Commands

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

/// Handle task scheduler test
pub fn handle_scheduler_test(context: &ShellContext) {
    context.uart.puts("\r\n=== Task Scheduler Test ===\r\n");

    // Test scheduler initialization
    context
        .uart
        .puts("1. Testing Scheduler Initialization...\r\n");

    if process::scheduler::is_scheduler_enabled() {
        context.uart.puts("   ✓ Scheduler is enabled\r\n");
    } else {
        context.uart.puts("   ⚠ Scheduler is disabled\r\n");
    }

    // Test task creation
    context.uart.puts("\r\n2. Testing Task Creation...\r\n");

    let task1_id = process::scheduler::create_task(
        "test_task_1",
        process::scheduler::TaskPriority::Normal,
        0x1000_0000,
        0x2000_0000,
        0x1000,
    );

    context.uart.puts("   Task 1 created with ID: ");
    context.uart.put_hex(task1_id as u64);
    context.uart.puts("\r\n");

    let task2_id = process::scheduler::create_task(
        "test_task_2",
        process::scheduler::TaskPriority::High,
        0x1000_1000,
        0x2001_0000,
        0x1000,
    );

    context.uart.puts("   Task 2 created with ID: ");
    context.uart.put_hex(task2_id as u64);
    context.uart.puts("\r\n");

    let task3_id = process::scheduler::create_task(
        "test_task_3",
        process::scheduler::TaskPriority::Low,
        0x1000_2000,
        0x2002_0000,
        0x1000,
    );

    context.uart.puts("   Task 3 created with ID: ");
    context.uart.put_hex(task3_id as u64);
    context.uart.puts("\r\n");

    // Test task count
    context.uart.puts("\r\n3. Testing Task Count...\r\n");

    let task_count = process::scheduler::get_task_count();
    context.uart.puts("   Total tasks: ");
    context.uart.put_hex(task_count as u64);
    context.uart.puts("\r\n");

    // Test scheduling
    context
        .uart
        .puts("\r\n4. Testing Round-Robin Scheduling...\r\n");

    for i in 1..=5 {
        context.uart.puts("   Schedule round ");
        context.uart.put_hex(i);
        context.uart.puts(": ");

        if let Some(scheduled_task_id) = process::scheduler::schedule() {
            context.uart.puts("Task ");
            context.uart.put_hex(scheduled_task_id as u64);
            context.uart.puts(" scheduled\r\n");
        } else {
            context.uart.puts("No task scheduled (idle)\r\n");
        }
    }

    // Test priority scheduling
    context
        .uart
        .puts("\r\n5. Testing Priority-Based Scheduling...\r\n");
    context
        .uart
        .puts("   (High priority task should be scheduled first)\r\n");

    // Test preemption
    context.uart.puts("\r\n6. Testing Preemption...\r\n");

    if process::scheduler::handle_timer_preemption() {
        context.uart.puts("   ✓ Timer preemption triggered\r\n");
    } else {
        context.uart.puts("   ⚠ No preemption needed\r\n");
    }

    // Test task destruction
    context.uart.puts("\r\n7. Testing Task Destruction...\r\n");

    match process::scheduler::destroy_task(task1_id) {
        Ok(()) => {
            context.uart.puts("   ✓ Task 1 destroyed successfully\r\n");
        }
        Err(msg) => {
            context.uart.puts("   ✗ Task 1 destruction failed: ");
            context.uart.puts(msg);
            context.uart.puts("\r\n");
        }
    }

    // Show final task count
    let final_task_count = process::scheduler::get_task_count();
    context.uart.puts("   Final task count: ");
    context.uart.put_hex(final_task_count as u64);
    context.uart.puts("\r\n");

    context.uart.puts("\r\n✅ Task Scheduler Test Complete\r\n");
}

/// Handle process statistics display
pub fn handle_process_stats(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== Process Management Statistics ===\r\n");

    let stats = process::get_process_stats();

    context.uart.puts("Context Switches: ");
    context.uart.put_hex(stats.context_switches);
    context.uart.puts("\r\n");

    context.uart.puts("Privilege Escalations: ");
    context.uart.put_hex(stats.privilege_escalations);
    context.uart.puts("\r\n");

    context.uart.puts("Privilege Violations: ");
    context.uart.put_hex(stats.privilege_violations);
    context.uart.puts("\r\n");

    context.uart.puts("Tasks Created: ");
    context.uart.put_hex(stats.tasks_created);
    context.uart.puts("\r\n");

    context.uart.puts("Tasks Destroyed: ");
    context.uart.put_hex(stats.tasks_destroyed);
    context.uart.puts("\r\n");

    context.uart.puts("Scheduler Preemptions: ");
    context.uart.put_hex(stats.scheduler_preemptions);
    context.uart.puts("\r\n");
}

/// Handle scheduler statistics display
pub fn handle_scheduler_stats(context: &ShellContext) {
    context.uart.puts("\r\n=== Scheduler Statistics ===\r\n");

    let stats = process::scheduler::get_scheduler_stats();

    context.uart.puts("Context Switches: ");
    context.uart.put_hex(stats.context_switches);
    context.uart.puts("\r\n");

    context.uart.puts("Preemptions: ");
    context.uart.put_hex(stats.preemptions);
    context.uart.puts("\r\n");

    context.uart.puts("Tasks Created: ");
    context.uart.put_hex(stats.tasks_created);
    context.uart.puts("\r\n");

    context.uart.puts("Tasks Destroyed: ");
    context.uart.put_hex(stats.tasks_destroyed);
    context.uart.puts("\r\n");

    context.uart.puts("Scheduler Calls: ");
    context.uart.put_hex(stats.scheduler_calls);
    context.uart.puts("\r\n");

    context.uart.puts("Idle Time: ");
    context.uart.put_hex(stats.idle_time);
    context.uart.puts("\r\n");

    context.uart.puts("Total Run Time: ");
    context.uart.put_hex(stats.total_run_time);
    context.uart.puts("\r\n");

    context.uart.puts("Current Task Count: ");
    context
        .uart
        .put_hex(process::scheduler::get_task_count() as u64);
    context.uart.puts("\r\n");

    context.uart.puts("Scheduler Enabled: ");
    if process::scheduler::is_scheduler_enabled() {
        context.uart.puts("Yes");
    } else {
        context.uart.puts("No");
    }
    context.uart.puts("\r\n");
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
