//! Core ESR Decoder implementation
//!
//! Main decoder functionality and static decode function

use super::esr_info::EsrInfo;

/// ESR (Exception Syndrome Register) decoder
pub struct EsrDecoder;

impl EsrDecoder {
    /// Create a new ESR decoder instance
    pub fn new() -> Self {
        EsrDecoder
    }

    /// Decode an ESR value into structured information
    pub fn decode_esr(&self, esr: u32) -> EsrInfo {
        EsrInfo::new(esr)
    }

    /// Static decode function for convenience
    pub fn decode(esr: u32) -> EsrInfo {
        EsrInfo::new(esr)
    }
}
