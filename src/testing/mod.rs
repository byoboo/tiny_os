// TinyOS Testing Framework
// Advanced no_std testing infrastructure for kernel development

use crate::uart::Uart;

pub mod kernel_tests;
pub mod mmu_tests;
pub mod process_tests;
pub mod syscall_tests;
pub mod integration_tests;

// Test result tracking
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TestResult {
    Pass,
    Fail,
    Skip,
}

// Test runner for kernel-level testing
pub struct TestRunner {
    uart: Uart,
    test_passed: usize,
    test_failed: usize,
    test_skipped: usize,
    current_suite: &'static str,
}

impl TestRunner {
    pub fn new(uart: Uart) -> Self {
        Self {
            uart,
            test_passed: 0,
            test_failed: 0,
            test_skipped: 0,
            current_suite: "Unknown",
        }
    }

    pub fn start_suite(&mut self, suite_name: &'static str) {
        self.current_suite = suite_name;
        self.test_passed = 0;
        self.test_failed = 0;
        self.test_skipped = 0;
        
        self.uart.puts("\r\n=== ");
        self.uart.puts(suite_name);
        self.uart.puts(" Test Suite ===\r\n");
    }

    pub fn run_test<F>(&mut self, test_name: &str, test_func: F) -> TestResult
    where
        F: FnOnce() -> TestResult,
    {
        self.uart.puts("Running: ");
        self.uart.puts(test_name);
        self.uart.puts("... ");
        
        let result = test_func();
        
        match result {
            TestResult::Pass => {
                self.uart.puts("PASS\r\n");
                self.test_passed += 1;
            }
            TestResult::Fail => {
                self.uart.puts("FAIL\r\n");
                self.test_failed += 1;
            }
            TestResult::Skip => {
                self.uart.puts("SKIP\r\n");
                self.test_skipped += 1;
            }
        }
        
        result
    }

    pub fn assert_eq<T>(&mut self, left: T, right: T, msg: &str) -> TestResult
    where
        T: PartialEq,
    {
        if left == right {
            TestResult::Pass
        } else {
            self.uart.puts("ASSERTION FAILED: ");
            self.uart.puts(msg);
            self.uart.puts("\r\n");
            TestResult::Fail
        }
    }

    pub fn assert_true(&mut self, condition: bool, msg: &str) -> TestResult {
        if condition {
            TestResult::Pass
        } else {
            self.uart.puts("ASSERTION FAILED: ");
            self.uart.puts(msg);
            self.uart.puts("\r\n");
            TestResult::Fail
        }
    }

    pub fn assert_false(&mut self, condition: bool, msg: &str) -> TestResult {
        if !condition {
            TestResult::Pass
        } else {
            self.uart.puts("ASSERTION FAILED: ");
            self.uart.puts(msg);
            self.uart.puts(" (expected false)\r\n");
            TestResult::Fail
        }
    }

    pub fn assert_not_null<T>(&mut self, ptr: *const T, msg: &str) -> TestResult {
        if !ptr.is_null() {
            TestResult::Pass
        } else {
            self.uart.puts("ASSERTION FAILED: ");
            self.uart.puts(msg);
            self.uart.puts(" (expected non-null pointer)\r\n");
            TestResult::Fail
        }
    }

    pub fn finish_suite(&mut self) -> bool {
        self.uart.puts("\r\n=== ");
        self.uart.puts(self.current_suite);
        self.uart.puts(" Results ===\r\n");
        
        self.uart.puts("Passed: ");
        self.print_number(self.test_passed as u32);
        self.uart.puts(", Failed: ");
        self.print_number(self.test_failed as u32);
        self.uart.puts(", Skipped: ");
        self.print_number(self.test_skipped as u32);
        self.uart.puts("\r\n");
        
        let total = self.test_passed + self.test_failed + self.test_skipped;
        if total > 0 {
            let pass_rate = (self.test_passed * 100) / total;
            self.uart.puts("Pass Rate: ");
            self.print_number(pass_rate as u32);
            self.uart.puts("%\r\n");
        }
        
        self.uart.puts("\r\n");
        self.test_failed == 0
    }

    fn print_number(&mut self, mut num: u32) {
        if num == 0 {
            self.uart.puts("0");
            return;
        }
        
        let mut buffer = [0u8; 10];
        let mut index = 0;
        
        while num > 0 {
            buffer[index] = (num % 10) as u8 + b'0';
            num /= 10;
            index += 1;
        }
        
        for i in (0..index).rev() {
            self.uart.putc(buffer[i]);
        }
    }
}

// Test management functions
pub fn run_all_tests() {
    let uart = Uart::new();
    let mut runner = TestRunner::new(uart);
    
    // Run all test suites
    kernel_tests::run_kernel_tests(&mut runner);
    mmu_tests::run_mmu_tests(&mut runner);
    process_tests::run_process_tests(&mut runner);
    syscall_tests::run_syscall_tests(&mut runner);
    integration_tests::run_integration_tests(&mut runner);
    
    runner.uart.puts("=== ALL TESTS COMPLETE ===\r\n");
}

pub fn run_kernel_tests() {
    let uart = Uart::new();
    let mut runner = TestRunner::new(uart);
    kernel_tests::run_kernel_tests(&mut runner);
}

pub fn run_mmu_tests() {
    let uart = Uart::new();
    let mut runner = TestRunner::new(uart);
    mmu_tests::run_mmu_tests(&mut runner);
}

pub fn run_process_tests() {
    let uart = Uart::new();
    let mut runner = TestRunner::new(uart);
    process_tests::run_process_tests(&mut runner);
}

pub fn run_syscall_tests() {
    let uart = Uart::new();
    let mut runner = TestRunner::new(uart);
    syscall_tests::run_syscall_tests(&mut runner);
}

pub fn run_integration_tests() {
    let uart = Uart::new();
    let mut runner = TestRunner::new(uart);
    integration_tests::run_integration_tests(&mut runner);
}
