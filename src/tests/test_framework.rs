//! Test Framework for TinyOS
//! 
//! Provides testing utilities, assertions, and result tracking.

use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};

/// Test result for individual test cases
#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub duration: Duration,
    pub category: TestCategory,
}

/// Categories of tests
#[derive(Debug, Clone, PartialEq)]
pub enum TestCategory {
    Unit,
    Integration,
    Performance,
    Memory,
    Hardware,
    System,
}

impl fmt::Display for TestCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TestCategory::Unit => write!(f, "Unit"),
            TestCategory::Integration => write!(f, "Integration"),
            TestCategory::Performance => write!(f, "Performance"),
            TestCategory::Memory => write!(f, "Memory"),
            TestCategory::Hardware => write!(f, "Hardware"),
            TestCategory::System => write!(f, "System"),
        }
    }
}

/// Collection of test results
#[derive(Debug)]
pub struct TestResults {
    pub results: Vec<TestResult>,
    pub start_time: Instant,
}

impl TestResults {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            start_time: Instant::now(),
        }
    }

    pub fn add_result(&mut self, result: TestResult) {
        self.results.push(result);
    }

    pub fn add_pass(&mut self, name: &str, category: TestCategory, duration: Duration) {
        self.add_result(TestResult {
            name: name.to_string(),
            passed: true,
            message: "PASS".to_string(),
            duration,
            category,
        });
    }

    pub fn add_fail(&mut self, name: &str, category: TestCategory, duration: Duration, message: &str) {
        self.add_result(TestResult {
            name: name.to_string(),
            passed: false,
            message: message.to_string(),
            duration,
            category,
        });
    }

    pub fn merge(&mut self, other: TestResults) {
        self.results.extend(other.results);
    }

    pub fn passed_count(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    pub fn failed_count(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    pub fn total_count(&self) -> usize {
        self.results.len()
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_count() == 0 {
            0.0
        } else {
            self.passed_count() as f64 / self.total_count() as f64 * 100.0
        }
    }

    pub fn total_duration(&self) -> Duration {
        self.results.iter().map(|r| r.duration).sum()
    }

    pub fn category_summary(&self) -> HashMap<TestCategory, (usize, usize)> {
        let mut summary = HashMap::new();
        
        for result in &self.results {
            let entry = summary.entry(result.category.clone()).or_insert((0, 0));
            if result.passed {
                entry.0 += 1;
            } else {
                entry.1 += 1;
            }
        }
        
        summary
    }

    pub fn print_summary(&self) {
        let total_duration = self.start_time.elapsed();
        
        println!("\n🎯 Test Results Summary");
        println!("=====================");
        println!("Total Tests: {}", self.total_count());
        println!("Passed: {} ✅", self.passed_count());
        println!("Failed: {} ❌", self.failed_count());
        println!("Success Rate: {:.1}%", self.success_rate());
        println!("Test Duration: {:.2}s", self.total_duration().as_secs_f64());
        println!("Total Duration: {:.2}s", total_duration.as_secs_f64());
        
        // Category breakdown
        println!("\n📊 Results by Category:");
        let summary = self.category_summary();
        for (category, (passed, failed)) in summary {
            println!("  {}: {} ✅ {} ❌", category, passed, failed);
        }
        
        // Failed tests details
        if self.failed_count() > 0 {
            println!("\n❌ Failed Tests:");
            for result in &self.results {
                if !result.passed {
                    println!("  {} [{}]: {}", result.name, result.category, result.message);
                }
            }
        }
        
        println!("\n{}", if self.failed_count() == 0 { "🎉 All tests passed!" } else { "⚠️  Some tests failed" });
    }
}

/// Test assertion macros and utilities
pub struct TestAssert;

impl TestAssert {
    pub fn assert_eq<T: PartialEq + fmt::Debug>(expected: T, actual: T, message: &str) -> Result<(), String> {
        if expected == actual {
            Ok(())
        } else {
            Err(format!("{}: expected {:?}, got {:?}", message, expected, actual))
        }
    }

    pub fn assert_true(condition: bool, message: &str) -> Result<(), String> {
        if condition {
            Ok(())
        } else {
            Err(format!("{}: condition was false", message))
        }
    }

    pub fn assert_false(condition: bool, message: &str) -> Result<(), String> {
        if !condition {
            Ok(())
        } else {
            Err(format!("{}: condition was true", message))
        }
    }

    pub fn assert_ne<T: PartialEq + fmt::Debug>(not_expected: T, actual: T, message: &str) -> Result<(), String> {
        if not_expected != actual {
            Ok(())
        } else {
            Err(format!("{}: values should not be equal: {:?}", message, actual))
        }
    }

    pub fn assert_range<T: PartialOrd + fmt::Debug>(value: T, min: T, max: T, message: &str) -> Result<(), String> {
        if value >= min && value <= max {
            Ok(())
        } else {
            Err(format!("{}: {:?} not in range [{:?}, {:?}]", message, value, min, max))
        }
    }
}

/// Utility for running timed tests
pub fn run_test<F>(name: &str, category: TestCategory, test_fn: F) -> TestResult
where
    F: FnOnce() -> Result<(), String>,
{
    let start = Instant::now();
    
    match test_fn() {
        Ok(()) => TestResult {
            name: name.to_string(),
            passed: true,
            message: "PASS".to_string(),
            duration: start.elapsed(),
            category,
        },
        Err(msg) => TestResult {
            name: name.to_string(),
            passed: false,
            message: msg,
            duration: start.elapsed(),
            category,
        },
    }
}

/// Memory test utilities
pub struct MemoryTestUtils;

impl MemoryTestUtils {
    pub fn simulate_allocation(size: usize) -> Result<usize, String> {
        // Simulate memory allocation
        if size == 0 {
            Err("Cannot allocate zero bytes".to_string())
        } else if size > 4 * 1024 * 1024 {
            Err("Allocation too large".to_string())
        } else {
            Ok(0x100000) // Simulated address
        }
    }

    pub fn simulate_free(address: usize) -> Result<(), String> {
        if address == 0 {
            Err("Cannot free null pointer".to_string())
        } else {
            Ok(())
        }
    }

    pub fn check_memory_pattern(address: usize, size: usize, pattern: u8) -> Result<(), String> {
        // Simulate memory pattern checking
        if address == 0 {
            Err("Invalid address".to_string())
        } else if size == 0 {
            Err("Invalid size".to_string())
        } else {
            Ok(()) // Simulate successful pattern check
        }
    }
}

/// Hardware test utilities
pub struct HardwareTestUtils;

impl HardwareTestUtils {
    pub fn simulate_gpio_set(pin: u32, state: bool) -> Result<(), String> {
        if pin > 53 {
            Err(format!("Invalid GPIO pin: {}", pin))
        } else {
            Ok(())
        }
    }

    pub fn simulate_uart_write(data: &[u8]) -> Result<usize, String> {
        if data.is_empty() {
            Err("No data to write".to_string())
        } else {
            Ok(data.len())
        }
    }

    pub fn simulate_timer_read() -> Result<u64, String> {
        Ok(12345678) // Simulated timer value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_utilities() {
        assert!(TestAssert::assert_eq(5, 5, "Numbers should be equal").is_ok());
        assert!(TestAssert::assert_eq(5, 6, "Numbers should be equal").is_err());
        
        assert!(TestAssert::assert_true(true, "Should be true").is_ok());
        assert!(TestAssert::assert_false(false, "Should be false").is_ok());
        
        assert!(TestAssert::assert_range(5, 1, 10, "Should be in range").is_ok());
        assert!(TestAssert::assert_range(15, 1, 10, "Should be in range").is_err());
    }

    #[test]
    fn test_results_tracking() {
        let mut results = TestResults::new();
        
        results.add_pass("test1", TestCategory::Unit, Duration::from_millis(10));
        results.add_fail("test2", TestCategory::Integration, Duration::from_millis(20), "Failed");
        
        assert_eq!(results.total_count(), 2);
        assert_eq!(results.passed_count(), 1);
        assert_eq!(results.failed_count(), 1);
        assert_eq!(results.success_rate(), 50.0);
    }

    #[test]
    fn test_memory_utils() {
        assert!(MemoryTestUtils::simulate_allocation(64).is_ok());
        assert!(MemoryTestUtils::simulate_allocation(0).is_err());
        assert!(MemoryTestUtils::simulate_free(0x100000).is_ok());
        assert!(MemoryTestUtils::simulate_free(0).is_err());
    }
}
