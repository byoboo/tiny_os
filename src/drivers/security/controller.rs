//! Security Controller Integration
//! 
//! Main security controller that manages all security subsystems
//! Refactored from week6_security.rs

use super::{SecurityError, SecurityMetrics, RealTimeMetrics};
use super::trustzone::{TrustZoneController, SecurityLevel};
use super::realtime::{RealTimeScheduler, RtTask};
use super::hardening::{HardeningController, HardeningLevel};

/// Main security controller for Pi 4/5
pub struct SecurityController {
    trustzone: TrustZoneController,
    realtime: RealTimeScheduler,
    hardening: HardeningController,
    initialized: bool,
}

impl SecurityController {
    pub fn new() -> Self {
        Self {
            trustzone: TrustZoneController::new(),
            realtime: RealTimeScheduler::new(),
            hardening: HardeningController::new(),
            initialized: false,
        }
    }

    /// Initialize security subsystem
    pub fn init(&mut self) -> Result<(), SecurityError> {
        // Initialize TrustZone if available
        match self.trustzone.init() {
            Ok(()) => {
                // TrustZone available
            }
            Err(SecurityError::TrustZoneNotAvailable) => {
                // Continue without TrustZone
            }
            Err(e) => return Err(e),
        }
        
        // Initialize real-time scheduler
        self.realtime.init()?;
        
        // Initialize system hardening
        self.hardening.init()?;
        
        self.initialized = true;
        Ok(())
    }

    /// Get TrustZone controller
    pub fn get_trustzone(&mut self) -> &mut TrustZoneController {
        &mut self.trustzone
    }

    /// Get real-time scheduler
    pub fn get_realtime(&mut self) -> &mut RealTimeScheduler {
        &mut self.realtime
    }

    /// Get hardening controller
    pub fn get_hardening(&mut self) -> &mut HardeningController {
        &mut self.hardening
    }

    /// Set overall security level
    pub fn set_security_level(&mut self, level: SecurityLevel) -> Result<(), SecurityError> {
        // Update TrustZone security level
        self.trustzone.set_security_level(level)?;
        
        // Update hardening level based on security level
        let hardening_level = match level {
            SecurityLevel::Development => HardeningLevel::Basic,
            SecurityLevel::Production => HardeningLevel::Standard,
            SecurityLevel::Critical => HardeningLevel::Enhanced,
            SecurityLevel::Maximum => HardeningLevel::Maximum,
        };
        
        self.hardening.set_hardening_level(hardening_level)?;
        Ok(())
    }

    /// Get combined security metrics
    pub fn get_security_metrics(&self) -> SecurityMetrics {
        let trustzone_metrics = self.trustzone.get_metrics();
        let hardening_metrics = self.hardening.get_metrics();
        
        SecurityMetrics {
            threat_detections: trustzone_metrics.threat_detections + hardening_metrics.threat_detections,
            security_violations: trustzone_metrics.security_violations + hardening_metrics.security_violations,
            trustzone_switches: trustzone_metrics.trustzone_switches,
            failed_authentications: trustzone_metrics.failed_authentications,
            security_score: (trustzone_metrics.security_score + hardening_metrics.security_score) / 2,
        }
    }

    /// Get real-time metrics
    pub fn get_realtime_metrics(&self) -> &RealTimeMetrics {
        self.realtime.get_metrics()
    }

    /// Run comprehensive security scan
    pub fn run_security_scan(&mut self) -> Result<u8, SecurityError> {
        if !self.initialized {
            return Err(SecurityError::NotInitialized);
        }
        
        let trustzone_score = self.trustzone.run_security_scan()?;
        let hardening_score = self.hardening.assess_hardening()?;
        
        // Calculate overall security score
        let overall_score = (trustzone_score + hardening_score) / 2;
        Ok(overall_score)
    }

    /// Add real-time task
    pub fn add_realtime_task(&mut self, task: RtTask) -> Result<(), SecurityError> {
        self.realtime.add_task(task)
    }

    /// Schedule next real-time task
    pub fn schedule_next_task(&mut self) -> Option<u32> {
        self.realtime.schedule_next()
    }

    /// Check system schedulability
    pub fn check_schedulability(&self) -> Result<bool, SecurityError> {
        self.realtime.analyze_schedulability()
    }

    /// Is system initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

/// Global security controller instance
static mut SECURITY_CONTROLLER: Option<SecurityController> = None;
static mut SECURITY_CONTROLLER_INIT: bool = false;

/// Initialize global security controller
pub fn init_security_controller() -> Result<(), SecurityError> {
    unsafe {
        if !SECURITY_CONTROLLER_INIT {
            let mut controller = SecurityController::new();
            controller.init()?;
            SECURITY_CONTROLLER = Some(controller);
            SECURITY_CONTROLLER_INIT = true;
        }
    }
    Ok(())
}

/// Get global security controller
pub fn get_security_controller() -> Option<&'static mut SecurityController> {
    unsafe {
        if SECURITY_CONTROLLER_INIT {
            SECURITY_CONTROLLER.as_mut()
        } else {
            None
        }
    }
}

/// Get global real-time scheduler
pub fn get_rt_scheduler() -> Option<&'static mut RealTimeScheduler> {
    unsafe {
        if SECURITY_CONTROLLER_INIT {
            SECURITY_CONTROLLER.as_mut().map(|c| c.get_realtime())
        } else {
            None
        }
    }
}

/// Get global hardening controller
pub fn get_hardening_controller() -> Option<&'static mut HardeningController> {
    unsafe {
        if SECURITY_CONTROLLER_INIT {
            SECURITY_CONTROLLER.as_mut().map(|c| c.get_hardening())
        } else {
            None
        }
    }
}

/// Show Week 6 security capabilities
pub fn show_week6_capabilities() -> &'static str {
    "Week 6 Security Capabilities:\n\
     • ARM TrustZone (Secure/Non-secure worlds)\n\
     • Real-time Scheduling (Microsecond precision)\n\
     • System Hardening (Stack protection, ASLR, NX)\n\
     • Security Monitoring (Threat detection, metrics)\n\
     • Exploit Mitigation (CFI, Stack canaries)"
}