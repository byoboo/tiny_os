//! TinyOS Comprehensive Test Suite
//! 
//! This module provides a complete testing framework for TinyOS including:
//! - Unit tests for individual components
//! - Integration tests for system-wide functionality
//! - Performance benchmarks
//! - Hardware simulation and mocking

#[cfg(test)]
pub mod test_framework;

#[cfg(test)]
pub mod mocks;

#[cfg(test)]
pub mod unit_tests;

#[cfg(test)]
pub mod integration_tests;

#[cfg(test)]
pub mod performance_tests;

#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use std::sync::{Arc, Mutex};

/// Test configuration and shared state
#[cfg(test)]
pub struct TestConfig {
    pub verbose: bool,
    pub timeout_ms: u64,
    pub hardware_simulation: bool,
}

#[cfg(test)]
impl Default for TestConfig {
    fn default() -> Self {
        Self {
            verbose: true,
            timeout_ms: 5000,
            hardware_simulation: true,
        }
    }
}

/// Shared test state for coordination between tests
#[cfg(test)]
pub struct TestState {
    pub test_count: Arc<Mutex<u32>>,
    pub passed_tests: Arc<Mutex<u32>>,
    pub failed_tests: Arc<Mutex<u32>>,
    pub test_results: Arc<Mutex<HashMap<String, TestResult>>>,
}

#[cfg(test)]
impl TestState {
    pub fn new() -> Self {
        Self {
            test_count: Arc::new(Mutex::new(0)),
            passed_tests: Arc::new(Mutex::new(0)),
            failed_tests: Arc::new(Mutex::new(0)),
            test_results: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn record_test(&self, name: String, result: TestResult) {
        if let Ok(mut results) = self.test_results.lock() {
            results.insert(name, result.clone());
        }
        
        if let Ok(mut count) = self.test_count.lock() {
            *count += 1;
        }
        
        match result.status {
            TestStatus::Passed => {
                if let Ok(mut passed) = self.passed_tests.lock() {
                    *passed += 1;
                }
            }
            TestStatus::Failed => {
                if let Ok(mut failed) = self.failed_tests.lock() {
                    *failed += 1;
                }
            }
            _ => {}
        }
    }

    pub fn get_summary(&self) -> TestSummary {
        let test_count = self.test_count.lock().unwrap_or_else(|_| std::sync::MutexGuard::leak(Mutex::new(0).lock().unwrap()));
        let passed = self.passed_tests.lock().unwrap_or_else(|_| std::sync::MutexGuard::leak(Mutex::new(0).lock().unwrap()));
        let failed = self.failed_tests.lock().unwrap_or_else(|_| std::sync::MutexGuard::leak(Mutex::new(0).lock().unwrap()));
        
        TestSummary {
            total_tests: *test_count,
            passed: *passed,
            failed: *failed,
            success_rate: if *test_count > 0 { (*passed as f32 / *test_count as f32) * 100.0 } else { 0.0 },
        }
    }
}

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct TestResult {
    pub status: TestStatus,
    pub message: String,
    pub duration_ms: u64,
    pub details: Option<String>,
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
    Timeout,
}

#[cfg(test)]
#[derive(Debug)]
pub struct TestSummary {
    pub total_tests: u32,
    pub passed: u32,
    pub failed: u32,
    pub success_rate: f32,
}

/// Main test runner function
#[cfg(test)]
pub fn run_all_tests() -> TestSummary {
    println!("🚀 Starting TinyOS Comprehensive Test Suite");
    println!("==========================================");
    
    let test_state = TestState::new();
    let config = TestConfig::default();
    
    // Run unit tests
    println!("\n📋 Running Unit Tests...");
    unit_tests::run_unit_tests(&test_state, &config);
    
    // Run integration tests
    println!("\n🔗 Running Integration Tests...");
    integration_tests::run_integration_tests(&test_state, &config);
    
    // Run performance tests
    println!("\n⚡ Running Performance Tests...");
    performance_tests::run_performance_tests(&test_state, &config);
    
    let summary = test_state.get_summary();
    
    println!("\n📊 Test Summary");
    println!("================");
    println!("Total Tests: {}", summary.total_tests);
    println!("Passed: {} ✅", summary.passed);
    println!("Failed: {} ❌", summary.failed);
    println!("Success Rate: {:.2}%", summary.success_rate);
    
    if summary.failed == 0 {
        println!("\n🎉 All tests passed! TinyOS is ready for deployment.");
    } else {
        println!("\n⚠️  Some tests failed. Please review the results above.");
    }
    
    summary
}

/// Macro for creating test cases with automatic registration
#[cfg(test)]
#[macro_export]
macro_rules! test_case {
    ($name:expr, $test_state:expr, $test_fn:expr) => {{
        let start_time = std::time::Instant::now();
        print!("  {} ... ", $name);
        
        let result = std::panic::catch_unwind(|| {
            $test_fn()
        });
        
        let duration = start_time.elapsed().as_millis() as u64;
        
        let test_result = match result {
            Ok(Ok(_)) => {
                println!("✅ PASS ({} ms)", duration);
                TestResult {
                    status: TestStatus::Passed,
                    message: "Test passed".to_string(),
                    duration_ms: duration,
                    details: None,
                }
            }
            Ok(Err(e)) => {
                println!("❌ FAIL ({} ms) - {}", duration, e);
                TestResult {
                    status: TestStatus::Failed,
                    message: e.to_string(),
                    duration_ms: duration,
                    details: None,
                }
            }
            Err(_) => {
                println!("❌ PANIC ({} ms)", duration);
                TestResult {
                    status: TestStatus::Failed,
                    message: "Test panicked".to_string(),
                    duration_ms: duration,
                    details: None,
                }
            }
        };
        
        $test_state.record_test($name.to_string(), test_result);
    }};
}
