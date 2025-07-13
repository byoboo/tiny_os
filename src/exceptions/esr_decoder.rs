//! ESR_EL1 (Exception Syndrome Register) Decoding
//!
//! This module provides comprehensive decoding of the ESR_EL1 register to
//! identify specific exception causes and extract detailed fault information.
//!
//! ESR_EL1 format:
//! - Bits [31:26]: Exception Class (EC)
//! - Bits [25:25]: Instruction Length (IL)
//! - Bits [24:0]:  Instruction Specific Syndrome (ISS)

// Re-export modular components
#[path = "exception_class.rs"]
mod exception_class;
#[path = "data_fault_status.rs"]
mod data_fault_status;
#[path = "esr_info.rs"]
mod esr_info;
#[path = "esr_decoder_core.rs"]
mod esr_decoder_core;
#[path = "esr_description.rs"]
mod esr_description;
#[path = "esr_utils.rs"]
mod esr_utils;

pub use exception_class::ExceptionClass;
pub use data_fault_status::DataFaultStatus;
pub use esr_info::{EsrInfo, EsrDetails};
pub use esr_decoder_core::EsrDecoder;
pub use esr_description::{
    get_description, get_detailed_description, format_debug_info,
    format_data_abort_info, format_instruction_abort_info,
    is_recoverable_fault, get_fault_severity,
};
pub use esr_utils::{
    extract_exception_class, extract_instruction_length, extract_iss,
    is_32bit_instruction, is_16bit_instruction, validate_esr,
    breakdown_esr, reconstruct_esr, esr_equals_ignore_iss,
    analyze_esr_batch, EsrBatchAnalysis,
};
