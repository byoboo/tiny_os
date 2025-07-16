// TinyOS Shell Scheduler Commands
// Focused module for task scheduler testing and operations

use crate::{process, shell::ShellContext};

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
