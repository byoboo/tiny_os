//! Test Reporter and Result Tracking
//! 
//! Comprehensive test result tracking and reporting system

use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestStatus {
    Pass,
    Fail(String),
    Skip(String),
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub name: String,
    pub status: TestStatus,
    pub duration: Duration,
    pub category: String,
    pub description: String,
    pub assertions: u32,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestSuite {
    pub name: String,
    pub results: Vec<TestResult>,
    pub total_duration: Duration,
    pub summary: TestSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestSummary {
    pub total: u32,
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
    pub errors: u32,
    pub pass_rate: f64,
}

pub struct TestReporter {
    results: Vec<TestResult>,
    current_test: Option<String>,
    start_time: Option<Instant>,
    suite_start: Instant,
    categories: HashMap<String, Vec<TestResult>>,
}

impl TestReporter {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            current_test: None,
            start_time: None,
            suite_start: Instant::now(),
            categories: HashMap::new(),
        }
    }

    pub fn start_test(&mut self, name: &str, category: &str, description: &str) {
        self.current_test = Some(name.to_string());
        self.start_time = Some(Instant::now());
        println!("🧪 Starting test: {} ({})", name, category);
    }

    pub fn end_test(&mut self, status: TestStatus, assertions: u32) {
        if let (Some(name), Some(start)) = (self.current_test.take(), self.start_time.take()) {
            let duration = start.elapsed();
            let result = TestResult {
                name: name.clone(),
                status: status.clone(),
                duration,
                category: "unknown".to_string(), // Will be updated if category tracking is added
                description: "".to_string(),
                assertions,
                timestamp: chrono::Utc::now().to_rfc3339(),
            };

            match &status {
                TestStatus::Pass => println!("✅ {} passed ({:?}, {} assertions)", name, duration, assertions),
                TestStatus::Fail(msg) => println!("❌ {} failed: {} ({:?})", name, msg, duration),
                TestStatus::Skip(msg) => println!("⏭️  {} skipped: {} ({:?})", name, msg, duration),
                TestStatus::Error(msg) => println!("💥 {} error: {} ({:?})", name, msg, duration),
            }

            self.results.push(result);
        }
    }

    pub fn pass(&mut self, message: &str) {
        self.end_test(TestStatus::Pass, 1);
    }

    pub fn fail(&mut self, message: &str) {
        self.end_test(TestStatus::Fail(message.to_string()), 0);
    }

    pub fn skip(&mut self, message: &str) {
        self.end_test(TestStatus::Skip(message.to_string()), 0);
    }

    pub fn error(&mut self, message: &str) {
        self.end_test(TestStatus::Error(message.to_string()), 0);
    }

    pub fn assert_true(&mut self, condition: bool, message: &str) -> bool {
        if condition {
            println!("  ✓ {}", message);
            true
        } else {
            println!("  ✗ {}", message);
            self.fail(&format!("Assertion failed: {}", message));
            false
        }
    }

    pub fn assert_eq<T: PartialEq + std::fmt::Debug>(&mut self, actual: T, expected: T, message: &str) -> bool {
        if actual == expected {
            println!("  ✓ {}", message);
            true
        } else {
            let error_msg = format!("{}: expected {:?}, got {:?}", message, expected, actual);
            println!("  ✗ {}", error_msg);
            self.fail(&error_msg);
            false
        }
    }

    pub fn assert_ne<T: PartialEq + std::fmt::Debug>(&mut self, actual: T, expected: T, message: &str) -> bool {
        if actual != expected {
            println!("  ✓ {}", message);
            true
        } else {
            let error_msg = format!("{}: values should not be equal: {:?}", message, actual);
            println!("  ✗ {}", error_msg);
            self.fail(&error_msg);
            false
        }
    }

    pub fn generate_summary(&self) -> TestSummary {
        let total = self.results.len() as u32;
        let passed = self.results.iter().filter(|r| matches!(r.status, TestStatus::Pass)).count() as u32;
        let failed = self.results.iter().filter(|r| matches!(r.status, TestStatus::Fail(_))).count() as u32;
        let skipped = self.results.iter().filter(|r| matches!(r.status, TestStatus::Skip(_))).count() as u32;
        let errors = self.results.iter().filter(|r| matches!(r.status, TestStatus::Error(_))).count() as u32;
        
        let pass_rate = if total > 0 { (passed as f64 / total as f64) * 100.0 } else { 0.0 };

        TestSummary {
            total,
            passed,
            failed,
            skipped,
            errors,
            pass_rate,
        }
    }

    pub fn print_summary(&self) {
        let summary = self.generate_summary();
        let total_duration = self.suite_start.elapsed();

        println!("\n" + "═".repeat(80).as_str());
        println!("🏁 TEST SUITE SUMMARY");
        println!("═".repeat(80));
        println!("📊 Results: {} total, {} passed, {} failed, {} skipped, {} errors", 
                 summary.total, summary.passed, summary.failed, summary.skipped, summary.errors);
        println!("⏱️  Total Duration: {:?}", total_duration);
        println!("📈 Pass Rate: {:.1}%", summary.pass_rate);

        if summary.failed > 0 || summary.errors > 0 {
            println!("\n❌ FAILED TESTS:");
            for result in &self.results {
                match &result.status {
                    TestStatus::Fail(msg) => println!("  • {}: {}", result.name, msg),
                    TestStatus::Error(msg) => println!("  • {}: {}", result.name, msg),
                    _ => {}
                }
            }
        }

        if summary.pass_rate >= 100.0 {
            println!("\n🎉 ALL TESTS PASSED! 🎉");
        } else if summary.pass_rate >= 90.0 {
            println!("\n✅ Most tests passed - {} failures to investigate", summary.failed + summary.errors);
        } else {
            println!("\n⚠️  Significant test failures - {} issues need attention", summary.failed + summary.errors);
        }
        println!("═".repeat(80));
    }

    pub fn export_json(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let suite = TestSuite {
            name: "TinyOS Test Suite".to_string(),
            results: self.results.clone(),
            total_duration: self.suite_start.elapsed(),
            summary: self.generate_summary(),
        };

        let json = serde_json::to_string_pretty(&suite)?;
        std::fs::write(filename, json)?;
        println!("📋 Test results exported to {}", filename);
        Ok(())
    }

    pub fn get_results(&self) -> &[TestResult] {
        &self.results
    }
}

// Add chrono dependency for timestamps
extern crate chrono;
