//! Performance Driver Module
//!
//! Consolidated performance functionality from Week 4-6 implementations
//! Provides benchmarking, monitoring, and optimization features

pub mod benchmarks;
pub mod metrics;
pub mod power;
pub mod thermal;

pub use benchmarks::{BenchmarkResult, BenchmarkSuite};
pub use metrics::{PerformanceMetrics, SystemMetrics};
pub use power::{PowerController, PowerManagement};
pub use thermal::{ThermalController, ThermalStatus};

/// Performance-related errors
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PerformanceError {
    NotInitialized,
    HardwareError,
    InvalidConfiguration,
    ThermalThrottling,
    PowerLimitExceeded,
    BenchmarkFailed,
}

/// Performance optimization levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptimizationLevel {
    PowerSaving,
    Balanced,
    Performance,
    Maximum,
}

/// Performance monitoring configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub enable_benchmarking: bool,
    pub enable_power_monitoring: bool,
    pub enable_thermal_monitoring: bool,
    pub optimization_level: OptimizationLevel,
    pub thermal_threshold_celsius: u8,
}
