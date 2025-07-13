//! ESR Utility functions
//!
//! Helper functions for ESR manipulation and analysis

use super::esr_info::EsrInfo;
use super::exception_class::ExceptionClass;

/// Extract exception class from raw ESR value
pub fn extract_exception_class(esr: u32) -> ExceptionClass {
    ExceptionClass::from(esr)
}

/// Extract instruction length bit from ESR
pub fn extract_instruction_length(esr: u32) -> bool {
    (esr & (1 << 25)) != 0
}

/// Extract ISS (Instruction Specific Syndrome) from ESR
pub fn extract_iss(esr: u32) -> u32 {
    esr & 0x1FFFFFF
}

/// Check if ESR indicates a 32-bit instruction
pub fn is_32bit_instruction(esr: u32) -> bool {
    extract_instruction_length(esr)
}

/// Check if ESR indicates a 16-bit instruction
pub fn is_16bit_instruction(esr: u32) -> bool {
    !extract_instruction_length(esr)
}

/// Validate ESR value structure
pub fn validate_esr(esr: u32) -> bool {
    let ec = (esr >> 26) & 0x3F;
    // Check if exception class is within valid range
    ec <= 0x3F
}

/// Get ESR field breakdown as tuple (EC, IL, ISS)
pub fn breakdown_esr(esr: u32) -> (u8, bool, u32) {
    let ec = ((esr >> 26) & 0x3F) as u8;
    let il = (esr & (1 << 25)) != 0;
    let iss = esr & 0x1FFFFFF;
    (ec, il, iss)
}

/// Reconstruct ESR from components
pub fn reconstruct_esr(ec: u8, il: bool, iss: u32) -> u32 {
    let mut esr = (ec as u32 & 0x3F) << 26;
    if il {
        esr |= 1 << 25;
    }
    esr |= iss & 0x1FFFFFF;
    esr
}

/// Compare two ESR values ignoring certain fields
pub fn esr_equals_ignore_iss(esr1: u32, esr2: u32) -> bool {
    let ec1 = (esr1 >> 26) & 0x3F;
    let ec2 = (esr2 >> 26) & 0x3F;
    let il1 = (esr1 >> 25) & 1;
    let il2 = (esr2 >> 25) & 1;
    
    ec1 == ec2 && il1 == il2
}

/// Get a summary of ESR statistics for a batch of ESRs
pub fn analyze_esr_batch(esrs: &[u32]) -> EsrBatchAnalysis {
    let mut analysis = EsrBatchAnalysis::new();
    
    for &esr in esrs {
        let info = EsrInfo::new(esr);
        analysis.total_count += 1;
        
        match info.exception_class {
            ExceptionClass::DataAbortLower | ExceptionClass::DataAbortSame => {
                analysis.data_abort_count += 1;
            },
            ExceptionClass::InstructionAbortLower | ExceptionClass::InstructionAbortSame => {
                analysis.instruction_abort_count += 1;
            },
            ExceptionClass::Svc32 | ExceptionClass::Svc64 => {
                analysis.system_call_count += 1;
            },
            _ => {
                analysis.other_exception_count += 1;
            },
        }
        
        if info.is_translation_fault() {
            analysis.translation_fault_count += 1;
        }
        
        if info.is_permission_fault() {
            analysis.permission_fault_count += 1;
        }
    }
    
    analysis
}

/// ESR batch analysis results
#[derive(Debug, Clone, Copy)]
pub struct EsrBatchAnalysis {
    pub total_count: usize,
    pub data_abort_count: usize,
    pub instruction_abort_count: usize,
    pub system_call_count: usize,
    pub other_exception_count: usize,
    pub translation_fault_count: usize,
    pub permission_fault_count: usize,
}

impl EsrBatchAnalysis {
    pub fn new() -> Self {
        Self {
            total_count: 0,
            data_abort_count: 0,
            instruction_abort_count: 0,
            system_call_count: 0,
            other_exception_count: 0,
            translation_fault_count: 0,
            permission_fault_count: 0,
        }
    }
    
    /// Get the percentage of data aborts
    pub fn data_abort_percentage(&self) -> f32 {
        if self.total_count == 0 {
            0.0
        } else {
            (self.data_abort_count as f32 / self.total_count as f32) * 100.0
        }
    }
    
    /// Get the percentage of instruction aborts
    pub fn instruction_abort_percentage(&self) -> f32 {
        if self.total_count == 0 {
            0.0
        } else {
            (self.instruction_abort_count as f32 / self.total_count as f32) * 100.0
        }
    }
    
    /// Get the percentage of system calls
    pub fn system_call_percentage(&self) -> f32 {
        if self.total_count == 0 {
            0.0
        } else {
            (self.system_call_count as f32 / self.total_count as f32) * 100.0
        }
    }
}
