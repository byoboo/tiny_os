//! Security Driver Module
//! 
//! Consolidated security functionality from Week 6 implementation
//! Provides TrustZone, real-time scheduling, and system hardening

pub mod trustzone;
pub mod realtime;
pub mod hardening;
pub mod controller;

#[cfg(test)]
mod tests;

pub use controller::SecurityController;
pub use trustzone::{TrustZoneController, SecurityLevel};
pub use realtime::{RealTimeScheduler, RtPriority, RtTask};
pub use hardening::{HardeningController, HardeningLevel};

/// Security-related errors
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecurityError {
    NotInitialized,
    HardwareError,
    InvalidLevel,
    PermissionDenied,
    TrustZoneNotAvailable,
    ConfigurationError,
}

/// Security metrics for system monitoring
#[derive(Debug, Default)]
pub struct SecurityMetrics {
    pub threat_detections: u32,
    pub security_violations: u32,
    pub trustzone_switches: u64,
    pub failed_authentications: u32,
    pub security_score: u8, // 0-100
}

/// Real-time performance metrics
#[derive(Debug, Default)]
pub struct RealTimeMetrics {
    pub task_switches: u64,
    pub missed_deadlines: u32,
    pub average_latency_us: u64,
    pub max_latency_us: u64,
    pub scheduler_overhead_us: u64,
}