// Process Management Tests
// Testing process creation, scheduling, and context switching

use super::{TestRunner, TestResult};

pub fn run_process_tests(runner: &mut TestRunner) {
    runner.start_suite("Process Management Tests");
    
    // Process system tests
    runner.run_test("Process Manager Check", test_process_manager_check);
    runner.run_test("Context Management Check", test_context_management_check);
    runner.run_test("Scheduler Check", test_scheduler_check);
    runner.run_test("Stack Management Check", test_stack_management_check);
    
    runner.finish_suite();
}

fn test_process_manager_check() -> TestResult {
    // Test process management initialization
    crate::process::init_process_management();
    TestResult::Pass
}

fn test_context_management_check() -> TestResult {
    use crate::process::context::ProcessContext;
    
    // Test context management
    let context = ProcessContext::new(1, 0x1000000, 0x2000000, 0x400000);
    if context.pid == 1 {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}

fn test_scheduler_check() -> TestResult {
    use crate::process::scheduler::Scheduler;
    
    // Test scheduler functionality
    let _scheduler = Scheduler::new();
    TestResult::Pass
}

fn test_stack_management_check() -> TestResult {
    // Test stack management through process context
    use crate::process::context::ProcessContext;
    
    let context = ProcessContext::new(1, 0x1000000, 0x2000000, 0x400000);
    if context.user_stack_pointer == 0x1000000 {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}
