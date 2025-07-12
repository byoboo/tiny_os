// Testing Framework Shell Commands
// Shell command handlers for the testing framework

use crate::shell::ShellContext;
use crate::testing;

/// Handle kernel testing command
pub fn handle_kernel_tests(context: &ShellContext) {
    context.uart.puts("\r\n=== Running Kernel Tests ===\r\n");
    testing::run_kernel_tests();
}

/// Handle MMU testing command
pub fn handle_mmu_tests(context: &ShellContext) {
    context.uart.puts("\r\n=== Running MMU Tests ===\r\n");
    testing::run_mmu_tests();
}

/// Handle process testing command
pub fn handle_process_tests(context: &ShellContext) {
    context.uart.puts("\r\n=== Running Process Tests ===\r\n");
    testing::run_process_tests();
}

/// Handle system call testing command
pub fn handle_syscall_tests(context: &ShellContext) {
    context.uart.puts("\r\n=== Running System Call Tests ===\r\n");
    testing::run_syscall_tests();
}

/// Handle integration testing command
pub fn handle_integration_tests(context: &ShellContext) {
    context.uart.puts("\r\n=== Running Integration Tests ===\r\n");
    testing::run_integration_tests();
}

/// Handle run all tests command
pub fn handle_all_tests(context: &ShellContext) {
    context.uart.puts("\r\n=== Running All Tests ===\r\n");
    testing::run_all_tests();
}

/// Handle testing help command
pub fn handle_testing_help(context: &ShellContext) {
    context.uart.puts("\r\n=== Testing Framework Commands ===\r\n");
    context.uart.puts("TK - Run kernel unit tests\r\n");
    context.uart.puts("TM - Run MMU and virtual memory tests\r\n");
    context.uart.puts("TP - Run process management tests\r\n");
    context.uart.puts("TS - Run system call tests\r\n");
    context.uart.puts("TI - Run integration tests\r\n");
    context.uart.puts("TA - Run all tests\r\n");
    context.uart.puts("=====================================\r\n");
}
