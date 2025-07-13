//! ESR Information structures and types
//!
//! Defines the core data structures for holding decoded ESR information
//! including EsrInfo struct and EsrDetails enum.

use super::{data_fault_status::DataFaultStatus, exception_class::ExceptionClass};

/// Decoded ESR information
#[derive(Debug, Clone)]
pub struct EsrInfo {
    /// Exception class from ESR[31:26]
    pub exception_class: ExceptionClass,
    /// Instruction length bit from ESR[25]
    pub instruction_length: bool,
    /// Instruction Specific Syndrome from ESR[24:0]
    pub iss: u32,
    /// Raw ESR value
    pub raw_esr: u32,
    /// Detailed syndrome information based on exception class
    pub details: EsrDetails,
}

/// Detailed exception syndrome information based on exception class
#[derive(Debug, Clone)]
pub enum EsrDetails {
    /// Unknown exception or unsupported class
    Unknown,

    /// Data abort syndrome
    DataAbort {
        /// Data Fault Status Code
        dfsc: DataFaultStatus,
        /// Write not Read bit (1 = write, 0 = read)
        wnr: bool,
        /// Set/Way bit for cache maintenance operations
        s1ptw: bool,
        /// Cache maintenance bit
        cm: bool,
        /// External abort type
        ea: bool,
        /// FAR (Fault Address Register) not valid
        fnv: bool,
        /// Set when fault address is valid
        set: bool,
        /// Acquire/Release bit
        ar: bool,
        /// Synchronous fault bit
        sf: bool,
    },

    /// Instruction abort syndrome
    InstructionAbort {
        /// Instruction Fault Status Code
        ifsc: u32,
        /// Set/Way bit for cache maintenance operations
        s1ptw: bool,
        /// External abort type
        ea: bool,
        /// FAR not valid
        fnv: bool,
        /// Set when fault address is valid
        set: bool,
    },

    /// System call syndrome
    SystemCall {
        /// SVC immediate value (for SVC instructions)
        imm16: u16,
    },

    /// System register access syndrome
    SystemRegister {
        /// Direction (1 = read, 0 = write)
        direction: bool,
        /// Target register
        rt: u8,
        /// CRn field
        crn: u8,
        /// CRM field
        crm: u8,
        /// Op0 field
        op0: u8,
        /// Op1 field
        op1: u8,
        /// Op2 field
        op2: u8,
    },

    /// Breakpoint syndrome
    Breakpoint {
        /// Breakpoint comment field
        comment: u16,
    },

    /// Watchpoint syndrome
    Watchpoint {
        /// Debug state change
        dsc: bool,
        /// Watchpoint data address register
        wpt: u8,
        /// Watchpoint register number
        wrn: u8,
    },

    /// Software step syndrome
    SoftwareStep {
        /// Exception state
        ex: bool,
        /// Step state
        ss: bool,
    },
}

impl EsrInfo {
    /// Create a new EsrInfo structure from raw ESR value
    pub fn new(esr: u32) -> Self {
        let exception_class = ExceptionClass::from(esr);
        let instruction_length = (esr & (1 << 25)) != 0;
        let iss = esr & 0x1FFFFFF;
        let details = Self::decode_details(&exception_class, iss);

        EsrInfo {
            exception_class,
            instruction_length,
            iss,
            raw_esr: esr,
            details,
        }
    }

    /// Decode detailed syndrome information based on exception class
    fn decode_details(exception_class: &ExceptionClass, iss: u32) -> EsrDetails {
        match exception_class {
            ExceptionClass::DataAbortLower | ExceptionClass::DataAbortSame => {
                EsrDetails::DataAbort {
                    dfsc: DataFaultStatus::from(iss),
                    wnr: (iss & (1 << 6)) != 0,
                    s1ptw: (iss & (1 << 7)) != 0,
                    cm: (iss & (1 << 8)) != 0,
                    ea: (iss & (1 << 9)) != 0,
                    fnv: (iss & (1 << 10)) != 0,
                    set: (iss & (1 << 11)) != 0,
                    ar: (iss & (1 << 14)) != 0,
                    sf: (iss & (1 << 16)) != 0,
                }
            }

            ExceptionClass::InstructionAbortLower | ExceptionClass::InstructionAbortSame => {
                EsrDetails::InstructionAbort {
                    ifsc: iss & 0x3F,
                    s1ptw: (iss & (1 << 7)) != 0,
                    ea: (iss & (1 << 9)) != 0,
                    fnv: (iss & (1 << 10)) != 0,
                    set: (iss & (1 << 11)) != 0,
                }
            }

            ExceptionClass::Svc32 | ExceptionClass::Svc64 => EsrDetails::SystemCall {
                imm16: (iss & 0xFFFF) as u16,
            },

            ExceptionClass::SystemRegister => EsrDetails::SystemRegister {
                direction: (iss & (1 << 0)) != 0,
                rt: ((iss >> 5) & 0x1F) as u8,
                crn: ((iss >> 10) & 0xF) as u8,
                crm: ((iss >> 1) & 0xF) as u8,
                op0: ((iss >> 20) & 0x3) as u8,
                op1: ((iss >> 14) & 0x7) as u8,
                op2: ((iss >> 17) & 0x7) as u8,
            },

            ExceptionClass::BreakpointLower
            | ExceptionClass::BreakpointSame
            | ExceptionClass::Bkpt32 => EsrDetails::Breakpoint {
                comment: (iss & 0xFFFF) as u16,
            },

            ExceptionClass::WatchpointLower | ExceptionClass::WatchpointSame => {
                EsrDetails::Watchpoint {
                    dsc: (iss & (1 << 14)) != 0,
                    wpt: ((iss >> 5) & 0x1F) as u8,
                    wrn: (iss & 0xF) as u8,
                }
            }

            ExceptionClass::SoftwareStepLower | ExceptionClass::SoftwareStepSame => {
                EsrDetails::SoftwareStep {
                    ex: (iss & (1 << 6)) != 0,
                    ss: (iss & (1 << 21)) != 0,
                }
            }

            _ => EsrDetails::Unknown,
        }
    }

    /// Check if this exception indicates a write operation
    pub fn is_write_operation(&self) -> bool {
        match &self.details {
            EsrDetails::DataAbort { wnr, .. } => *wnr,
            _ => false,
        }
    }

    /// Check if this exception has a valid fault address
    pub fn has_valid_fault_address(&self) -> bool {
        match &self.details {
            EsrDetails::DataAbort { fnv, .. } => !fnv,
            EsrDetails::InstructionAbort { fnv, .. } => !fnv,
            _ => false,
        }
    }

    /// Get the fault status code if this is a memory fault
    pub fn fault_status_code(&self) -> Option<u32> {
        match &self.details {
            EsrDetails::DataAbort { dfsc, .. } => Some(*dfsc as u32),
            EsrDetails::InstructionAbort { ifsc, .. } => Some(*ifsc),
            _ => None,
        }
    }

    /// Check if this is a translation fault
    pub fn is_translation_fault(&self) -> bool {
        match &self.details {
            EsrDetails::DataAbort { dfsc, .. } => dfsc.is_translation_fault(),
            EsrDetails::InstructionAbort { ifsc, .. } => {
                matches!(*ifsc & 0x3C, 0x04..=0x07) // Translation faults levels
                                                    // 0-3
            }
            _ => false,
        }
    }

    /// Check if this is a permission fault
    pub fn is_permission_fault(&self) -> bool {
        match &self.details {
            EsrDetails::DataAbort { dfsc, .. } => dfsc.is_permission_fault(),
            EsrDetails::InstructionAbort { ifsc, .. } => {
                matches!(*ifsc & 0x3C, 0x0C..=0x0F) // Permission faults levels
                                                    // 1-3
            }
            _ => false,
        }
    }
}
