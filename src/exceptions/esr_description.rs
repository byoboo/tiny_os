//! ESR Description and formatting utilities
//!
//! Functions for generating human-readable descriptions of ESR information

use super::esr_info::{EsrDetails, EsrInfo};

/// Get a comprehensive description of ESR information
pub fn get_description(esr_info: &EsrInfo) -> &'static str {
    esr_info.exception_class.description()
}

/// Check if ESR indicates a recoverable fault
pub fn is_recoverable_fault(esr_info: &EsrInfo) -> bool {
    match &esr_info.details {
        EsrDetails::DataAbort { dfsc, .. } => {
            // Translation and permission faults are typically recoverable
            dfsc.is_translation_fault() || dfsc.is_permission_fault()
        }
        EsrDetails::InstructionAbort { ifsc, .. } => {
            // Translation and permission faults for instruction aborts
            let fault_type = *ifsc & 0x3C;
            matches!(fault_type, 0x04..=0x0F)
        }
        _ => false,
    }
}

/// Get fault severity level (0 = info, 1 = warning, 2 = error, 3 = critical)
pub fn get_fault_severity(esr_info: &EsrInfo) -> u8 {
    match &esr_info.details {
        EsrDetails::DataAbort { dfsc, .. } => {
            if dfsc.is_external_abort() {
                3 // Critical - hardware issue
            } else if dfsc.is_permission_fault() {
                2 // Error - access violation
            } else if dfsc.is_translation_fault() {
                1 // Warning - page fault
            } else {
                2 // Error - other data fault
            }
        }
        EsrDetails::InstructionAbort { .. } => 2, // Error - instruction fetch issue
        EsrDetails::SystemCall { .. } => 0,       // Info - normal system call
        EsrDetails::Breakpoint { .. } => 0,       // Info - debug breakpoint
        EsrDetails::Watchpoint { .. } => 0,       // Info - debug watchpoint
        EsrDetails::SoftwareStep { .. } => 0,     // Info - debug step
        _ => 1,                                   // Warning - unknown exception
    }
}

// Simplified functions for no_std environment - removed format! macro usage
/// Get detailed description including syndrome information (simplified)
pub fn get_detailed_description(_esr_info: &EsrInfo) -> &'static str {
    "Detailed description not available in no_std environment"
}

/// Format ESR information for debugging output (simplified)
pub fn format_debug_info(_esr_info: &EsrInfo) -> &'static str {
    "Debug info formatting not available in no_std environment"
}

/// Format data abort specific information (simplified)
pub fn format_data_abort_info(_esr_info: &EsrInfo) -> Option<&'static str> {
    Some("Data abort formatting not available in no_std environment")
}

/// Format instruction abort specific information (simplified)
pub fn format_instruction_abort_info(_esr_info: &EsrInfo) -> Option<&'static str> {
    Some("Instruction abort formatting not available in no_std environment")
}
