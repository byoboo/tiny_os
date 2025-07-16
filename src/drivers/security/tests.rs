//! No-std Tests for Security Module
//!
//! Tests that work in the embedded no_std environment

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drivers::security::{
        hardening::{HardeningController, HardeningLevel},
        realtime::{RealTimeScheduler, RtPriority, RtTask},
        trustzone::{SecurityLevel, TrustZoneController},
        RealTimeMetrics, SecurityError, SecurityMetrics,
    };

    #[test]
    fn test_trustzone_controller_creation() {
        let controller = TrustZoneController::new();
        assert_eq!(controller.get_security_level(), SecurityLevel::Production);
    }

    #[test]
    fn test_security_level_transitions() {
        let mut controller = TrustZoneController::new();

        // Test upgrading from Development
        controller
            .set_security_level(SecurityLevel::Development)
            .unwrap();
        assert!(controller
            .set_security_level(SecurityLevel::Production)
            .is_ok());
        assert!(controller
            .set_security_level(SecurityLevel::Critical)
            .is_ok());
        assert!(controller
            .set_security_level(SecurityLevel::Maximum)
            .is_ok());
    }

    #[test]
    fn test_realtime_scheduler_creation() {
        let scheduler = RealTimeScheduler::new();
        assert!(scheduler.get_current_task().is_none());
    }

    #[test]
    fn test_realtime_task_creation() {
        let task = RtTask::new(1, RtPriority::RealTime, 1000, 2000);
        assert_eq!(task.id, 1);
        assert_eq!(task.priority, RtPriority::RealTime);
        assert_eq!(task.deadline_us, 1000);
        assert_eq!(task.period_us, 2000);
        assert_eq!(task.missed_deadlines, 0);
    }

    #[test]
    fn test_realtime_priority_ordering() {
        assert!(RtPriority::SystemCritical < RtPriority::HardwareInterrupt);
        assert!(RtPriority::HardwareInterrupt < RtPriority::RealTime);
        assert!(RtPriority::RealTime < RtPriority::HighPriority);
        assert!(RtPriority::HighPriority < RtPriority::Normal);
        assert!(RtPriority::Normal < RtPriority::Background);
    }

    #[test]
    fn test_hardening_controller_creation() {
        let controller = HardeningController::new();
        assert_eq!(controller.get_hardening_level(), HardeningLevel::Standard);
    }

    #[test]
    fn test_hardening_level_settings() {
        let mut controller = HardeningController::new();

        // Test setting different hardening levels
        assert!(controller
            .set_hardening_level(HardeningLevel::Basic)
            .is_ok());
        assert_eq!(controller.get_hardening_level(), HardeningLevel::Basic);

        assert!(controller
            .set_hardening_level(HardeningLevel::Enhanced)
            .is_ok());
        assert_eq!(controller.get_hardening_level(), HardeningLevel::Enhanced);

        assert!(controller
            .set_hardening_level(HardeningLevel::Maximum)
            .is_ok());
        assert_eq!(controller.get_hardening_level(), HardeningLevel::Maximum);
    }

    #[test]
    fn test_hardening_mitigations() {
        let mut controller = HardeningController::new();

        // Test default mitigations
        let mitigations = controller.get_mitigations();
        assert!(mitigations.stack_protection);
        assert!(mitigations.aslr_enabled);
        assert!(mitigations.nx_bit_enabled);
        assert!(mitigations.stack_canaries);
        assert!(mitigations.fortify_source);
    }

    #[test]
    fn test_stack_overflow_detection() {
        let mut controller = HardeningController::new();

        // Test normal stack usage
        let stack_base = 0x1000;
        let stack_size = 0x1000;
        let normal_sp = stack_base + 0x500;
        assert!(!controller.check_stack_overflow(normal_sp, stack_base, stack_size));

        // Test stack overflow
        let overflow_sp = stack_base - 1;
        assert!(controller.check_stack_overflow(overflow_sp, stack_base, stack_size));

        // Test stack underflow
        let underflow_sp = stack_base + stack_size + 1;
        assert!(controller.check_stack_overflow(underflow_sp, stack_base, stack_size));
    }

    #[test]
    fn test_control_flow_integrity() {
        let mut controller = HardeningController::new();

        // Test valid control flow
        let return_addr = 0x8000;
        let expected_addr = 0x8000;
        assert!(controller.validate_control_flow(return_addr, expected_addr));

        // Test invalid control flow
        let invalid_addr = 0x9000;
        assert!(!controller.validate_control_flow(invalid_addr, expected_addr));
    }

    #[test]
    fn test_stack_canary_validation() {
        let mut controller = HardeningController::new();

        // Test valid canary
        let canary = 0xDEADBEEF;
        assert!(controller.check_stack_canary(canary, canary));

        // Test corrupted canary
        let corrupted = 0xCAFEBABE;
        assert!(!controller.check_stack_canary(corrupted, canary));
    }

    #[test]
    fn test_security_metrics_default() {
        let metrics = SecurityMetrics::default();
        assert_eq!(metrics.threat_detections, 0);
        assert_eq!(metrics.security_violations, 0);
        assert_eq!(metrics.trustzone_switches, 0);
        assert_eq!(metrics.failed_authentications, 0);
        assert_eq!(metrics.security_score, 0);
    }

    #[test]
    fn test_realtime_metrics_default() {
        let metrics = RealTimeMetrics::default();
        assert_eq!(metrics.task_switches, 0);
        assert_eq!(metrics.missed_deadlines, 0);
        assert_eq!(metrics.average_latency_us, 0);
        assert_eq!(metrics.max_latency_us, 0);
        assert_eq!(metrics.scheduler_overhead_us, 0);
    }

    #[test]
    fn test_hardening_assessment() {
        let mut controller = HardeningController::new();

        // Test assessment scoring
        let score = controller.assess_hardening().unwrap();
        assert!(score > 0);
        assert!(score <= 100);
    }

    #[test]
    fn test_realtime_task_management() {
        let mut scheduler = RealTimeScheduler::new();

        // Test adding tasks
        let task1 = RtTask::new(1, RtPriority::RealTime, 1000, 2000);
        let task2 = RtTask::new(2, RtPriority::HighPriority, 2000, 4000);

        assert!(scheduler.add_task(task1).is_ok());
        assert!(scheduler.add_task(task2).is_ok());

        // Test task retrieval
        assert!(scheduler.get_task(1).is_some());
        assert!(scheduler.get_task(2).is_some());
        assert!(scheduler.get_task(999).is_none());

        // Test task removal
        assert!(scheduler.remove_task(1).is_ok());
        assert!(scheduler.get_task(1).is_none());
    }

    #[test]
    fn test_schedulability_analysis() {
        let mut scheduler = RealTimeScheduler::new();

        // Add schedulable tasks (total utilization < 1.0)
        let task1 = RtTask::new(1, RtPriority::RealTime, 100, 1000); // 10% utilization
        let task2 = RtTask::new(2, RtPriority::HighPriority, 200, 2000); // 10% utilization

        scheduler.add_task(task1).unwrap();
        scheduler.add_task(task2).unwrap();

        // System should be schedulable
        assert!(scheduler.analyze_schedulability().unwrap());
    }
}
