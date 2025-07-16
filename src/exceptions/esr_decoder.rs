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
#[path = "data_fault_status.rs"]
mod data_fault_status;
#[path = "esr_decoder_core.rs"]
mod esr_decoder_core;
#[path = "esr_description.rs"]
mod esr_description;
#[path = "esr_info.rs"]
mod esr_info;
#[path = "esr_utils.rs"]
mod esr_utils;
#[path = "exception_class.rs"]
mod exception_class;

pub use data_fault_status::DataFaultStatus;
pub use esr_decoder_core::EsrDecoder;
pub use esr_description::{
    format_data_abort_info, format_debug_info, format_instruction_abort_info, get_description,
    get_detailed_description, get_fault_severity, is_recoverable_fault,
};
pub use esr_info::{EsrDetails, EsrInfo};
pub use esr_utils::{
    analyze_esr_batch, breakdown_esr, esr_equals_ignore_iss, extract_exception_class,
    extract_instruction_length, extract_iss, is_16bit_instruction, is_32bit_instruction,
    reconstruct_esr, validate_esr, EsrBatchAnalysis,
};
pub use exception_class::ExceptionClass;
