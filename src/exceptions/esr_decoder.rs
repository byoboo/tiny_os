//! ESR_EL1 (Exception Syndrome Register) Decoding
//!
//! This module provides comprehensive decoding of the ESR_EL1 register to
//! identify specific exception causes and extract detailed fault information.
//!
//! ESR_EL1 format:
//! - Bits [31:26]: Exception Class (EC)
//! - Bits [25:25]: Instruction Length (IL)
//! - Bits [24:0]:  Instruction Specific Syndrome (ISS)

/// Exception Class values from ESR_EL1[31:26]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ExceptionClass {
    /// Unknown exception
    Unknown = 0x00,
    /// Trapped WFI or WFE instruction
    WfiWfe = 0x01,
    /// Trapped MCR/MRC access (coproc 15)
    McrMrc15 = 0x03,
    /// Trapped MCRR/MRRC access (coproc 15)
    McrrMrrc15 = 0x04,
    /// Trapped MCR/MRC access (coproc 14)
    McrMrc14 = 0x05,
    /// Trapped LDC/STC access
    LdcStc = 0x06,
    /// Trapped access to SVE/SIMD/FP
    SveSmdFp = 0x07,
    /// Trapped VMRS access
    Vmrs = 0x08,
    /// Trapped pointer authentication instruction
    PAuth = 0x09,
    /// Trapped MCR/MRC access (coproc 14)
    McrMrc14_2 = 0x0C,
    /// Branch Target Exception
    BranchTarget = 0x0D,
    /// Illegal Execution state
    IllegalExecution = 0x0E,
    /// SVC instruction execution in AArch32 state
    Svc32 = 0x11,
    /// HVC instruction execution in AArch32 state
    Hvc32 = 0x12,
    /// SMC instruction execution in AArch32 state
    Smc32 = 0x13,
    /// SVC instruction execution in AArch64 state
    Svc64 = 0x15,
    /// HVC instruction execution in AArch64 state
    Hvc64 = 0x16,
    /// SMC instruction execution in AArch64 state
    Smc64 = 0x17,
    /// Trapped MSR/MRS or system instruction execution
    MsrMrs = 0x18,
    /// Trapped access to SVE functionality
    Sve = 0x19,
    /// Trapped ERET/ERETAA/ERETAB instruction
    Eret = 0x1A,
    /// Exception from Pointer Authentication failure
    PAuthFail = 0x1C,
    /// Instruction Abort from lower Exception Level
    InstructionAbortLower = 0x20,
    /// Instruction Abort from same Exception Level
    InstructionAbortSame = 0x21,
    /// PC alignment fault
    PcAlignment = 0x22,
    /// Data Abort from lower Exception Level
    DataAbortLower = 0x24,
    /// Data Abort from same Exception Level
    DataAbortSame = 0x25,
    /// SP alignment fault
    SpAlignment = 0x26,
    /// Trapped floating-point exception (AArch32)
    FpException32 = 0x28,
    /// Trapped floating-point exception (AArch64)
    FpException64 = 0x2C,
    /// SError interrupt
    SError = 0x2F,
    /// Breakpoint exception from lower Exception Level
    BreakpointLower = 0x30,
    /// Breakpoint exception from same Exception Level
    BreakpointSame = 0x31,
    /// Software Step exception from lower Exception Level
    SoftwareStepLower = 0x32,
    /// Software Step exception from same Exception Level
    SoftwareStepSame = 0x33,
    /// Watchpoint exception from lower Exception Level
    WatchpointLower = 0x34,
    /// Watchpoint exception from same Exception Level
    WatchpointSame = 0x35,
    /// BKPT instruction execution (AArch32)
    Bkpt32 = 0x38,
    /// Vector Catch exception (AArch32)
    VectorCatch32 = 0x3A,
    /// BRK instruction execution (AArch64)
    Brk64 = 0x3C,
}

impl From<u32> for ExceptionClass {
    fn from(value: u32) -> Self {
        match value {
            0x00 => ExceptionClass::Unknown,
            0x01 => ExceptionClass::WfiWfe,
            0x03 => ExceptionClass::McrMrc15,
            0x04 => ExceptionClass::McrrMrrc15,
            0x05 => ExceptionClass::McrMrc14,
            0x06 => ExceptionClass::LdcStc,
            0x07 => ExceptionClass::SveSmdFp,
            0x08 => ExceptionClass::Vmrs,
            0x09 => ExceptionClass::PAuth,
            0x0C => ExceptionClass::McrMrc14_2,
            0x0D => ExceptionClass::BranchTarget,
            0x0E => ExceptionClass::IllegalExecution,
            0x11 => ExceptionClass::Svc32,
            0x12 => ExceptionClass::Hvc32,
            0x13 => ExceptionClass::Smc32,
            0x15 => ExceptionClass::Svc64,
            0x16 => ExceptionClass::Hvc64,
            0x17 => ExceptionClass::Smc64,
            0x18 => ExceptionClass::MsrMrs,
            0x19 => ExceptionClass::Sve,
            0x1A => ExceptionClass::Eret,
            0x1C => ExceptionClass::PAuthFail,
            0x20 => ExceptionClass::InstructionAbortLower,
            0x21 => ExceptionClass::InstructionAbortSame,
            0x22 => ExceptionClass::PcAlignment,
            0x24 => ExceptionClass::DataAbortLower,
            0x25 => ExceptionClass::DataAbortSame,
            0x26 => ExceptionClass::SpAlignment,
            0x28 => ExceptionClass::FpException32,
            0x2C => ExceptionClass::FpException64,
            0x2F => ExceptionClass::SError,
            0x30 => ExceptionClass::BreakpointLower,
            0x31 => ExceptionClass::BreakpointSame,
            0x32 => ExceptionClass::SoftwareStepLower,
            0x33 => ExceptionClass::SoftwareStepSame,
            0x34 => ExceptionClass::WatchpointLower,
            0x35 => ExceptionClass::WatchpointSame,
            0x38 => ExceptionClass::Bkpt32,
            0x3A => ExceptionClass::VectorCatch32,
            0x3C => ExceptionClass::Brk64,
            _ => ExceptionClass::Unknown,
        }
    }
}

impl ExceptionClass {
    /// Get a human-readable description of the exception class
    pub fn description(&self) -> &'static str {
        match self {
            ExceptionClass::Unknown => "Unknown exception",
            ExceptionClass::WfiWfe => "WFI/WFE instruction trapped",
            ExceptionClass::McrMrc15 => "MCR/MRC access to coprocessor 15",
            ExceptionClass::McrrMrrc15 => "MCRR/MRRC access to coprocessor 15",
            ExceptionClass::McrMrc14 => "MCR/MRC access to coprocessor 14",
            ExceptionClass::LdcStc => "LDC/STC access",
            ExceptionClass::SveSmdFp => "SVE/SIMD/FP access trapped",
            ExceptionClass::Vmrs => "VMRS access",
            ExceptionClass::PAuth => "Pointer authentication instruction",
            ExceptionClass::McrMrc14_2 => "MCR/MRC access to coprocessor 14 (2)",
            ExceptionClass::BranchTarget => "Branch Target Exception",
            ExceptionClass::IllegalExecution => "Illegal Execution state",
            ExceptionClass::Svc32 => "SVC instruction (AArch32)",
            ExceptionClass::Hvc32 => "HVC instruction (AArch32)",
            ExceptionClass::Smc32 => "SMC instruction (AArch32)",
            ExceptionClass::Svc64 => "SVC instruction (AArch64)",
            ExceptionClass::Hvc64 => "HVC instruction (AArch64)",
            ExceptionClass::Smc64 => "SMC instruction (AArch64)",
            ExceptionClass::MsrMrs => "MSR/MRS/system instruction",
            ExceptionClass::Sve => "SVE access trapped",
            ExceptionClass::Eret => "ERET/ERETAA/ERETAB instruction",
            ExceptionClass::PAuthFail => "Pointer Authentication failure",
            ExceptionClass::InstructionAbortLower => "Instruction Abort from lower EL",
            ExceptionClass::InstructionAbortSame => "Instruction Abort from same EL",
            ExceptionClass::PcAlignment => "PC alignment fault",
            ExceptionClass::DataAbortLower => "Data Abort from lower EL",
            ExceptionClass::DataAbortSame => "Data Abort from same EL",
            ExceptionClass::SpAlignment => "SP alignment fault",
            ExceptionClass::FpException32 => "Floating-point exception (AArch32)",
            ExceptionClass::FpException64 => "Floating-point exception (AArch64)",
            ExceptionClass::SError => "SError interrupt",
            ExceptionClass::BreakpointLower => "Breakpoint from lower EL",
            ExceptionClass::BreakpointSame => "Breakpoint from same EL",
            ExceptionClass::SoftwareStepLower => "Software Step from lower EL",
            ExceptionClass::SoftwareStepSame => "Software Step from same EL",
            ExceptionClass::WatchpointLower => "Watchpoint from lower EL",
            ExceptionClass::WatchpointSame => "Watchpoint from same EL",
            ExceptionClass::Bkpt32 => "BKPT instruction (AArch32)",
            ExceptionClass::VectorCatch32 => "Vector Catch exception (AArch32)",
            ExceptionClass::Brk64 => "BRK instruction (AArch64)",
        }
    }
}

/// Data Fault Status Code for data aborts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DataFaultStatus {
    /// Address size fault, level 0 of translation or translation table base
    /// register
    AddressSizeFaultLevel0 = 0b000000,
    /// Address size fault, level 1
    AddressSizeFaultLevel1 = 0b000001,
    /// Address size fault, level 2
    AddressSizeFaultLevel2 = 0b000010,
    /// Address size fault, level 3
    AddressSizeFaultLevel3 = 0b000011,
    /// Translation fault, level 0
    TranslationFaultLevel0 = 0b000100,
    /// Translation fault, level 1
    TranslationFaultLevel1 = 0b000101,
    /// Translation fault, level 2
    TranslationFaultLevel2 = 0b000110,
    /// Translation fault, level 3
    TranslationFaultLevel3 = 0b000111,
    /// Access flag fault, level 1
    AccessFaultLevel1 = 0b001001,
    /// Access flag fault, level 2
    AccessFaultLevel2 = 0b001010,
    /// Access flag fault, level 3
    AccessFaultLevel3 = 0b001011,
    /// Permission fault, level 1
    PermissionFaultLevel1 = 0b001101,
    /// Permission fault, level 2
    PermissionFaultLevel2 = 0b001110,
    /// Permission fault, level 3
    PermissionFaultLevel3 = 0b001111,
    /// Synchronous External abort, not on translation table walk
    SynchronousExternalAbort = 0b010000,
    /// Synchronous Tag Check Fault
    SynchronousTagCheck = 0b010001,
    /// Asynchronous External abort
    AsynchronousExternalAbort = 0b010110,
    /// Asynchronous Tag Check Fault
    AsynchronousTagCheck = 0b010111,
    /// Synchronous External abort on translation table walk, level 0
    SynchronousExternalAbortTtwLevel0 = 0b011100,
    /// Synchronous External abort on translation table walk, level 1
    SynchronousExternalAbortTtwLevel1 = 0b011101,
    /// Synchronous External abort on translation table walk, level 2
    SynchronousExternalAbortTtwLevel2 = 0b011110,
    /// Synchronous External abort on translation table walk, level 3
    SynchronousExternalAbortTtwLevel3 = 0b011111,
    /// Alignment fault
    AlignmentFault = 0b100001,
    /// Debug exception
    DebugException = 0b100010,
    /// TLB conflict abort
    TlbConflictAbort = 0b110000,
    /// Unsupported atomic hardware update fault
    UnsupportedAtomicHardwareUpdate = 0b110001,
    /// Implementation defined fault
    ImplementationDefined = 0b111101,
    /// Lockdown
    Lockdown = 0b111110,
    /// Exclusive access fault
    ExclusiveAccessFault = 0b111111,
}

impl From<u32> for DataFaultStatus {
    fn from(value: u32) -> Self {
        match value & 0b111111 {
            0b000000 => DataFaultStatus::AddressSizeFaultLevel0,
            0b000001 => DataFaultStatus::AddressSizeFaultLevel1,
            0b000010 => DataFaultStatus::AddressSizeFaultLevel2,
            0b000011 => DataFaultStatus::AddressSizeFaultLevel3,
            0b000100 => DataFaultStatus::TranslationFaultLevel0,
            0b000101 => DataFaultStatus::TranslationFaultLevel1,
            0b000110 => DataFaultStatus::TranslationFaultLevel2,
            0b000111 => DataFaultStatus::TranslationFaultLevel3,
            0b001001 => DataFaultStatus::AccessFaultLevel1,
            0b001010 => DataFaultStatus::AccessFaultLevel2,
            0b001011 => DataFaultStatus::AccessFaultLevel3,
            0b001101 => DataFaultStatus::PermissionFaultLevel1,
            0b001110 => DataFaultStatus::PermissionFaultLevel2,
            0b001111 => DataFaultStatus::PermissionFaultLevel3,
            0b010000 => DataFaultStatus::SynchronousExternalAbort,
            0b010001 => DataFaultStatus::SynchronousTagCheck,
            0b010110 => DataFaultStatus::AsynchronousExternalAbort,
            0b010111 => DataFaultStatus::AsynchronousTagCheck,
            0b011100 => DataFaultStatus::SynchronousExternalAbortTtwLevel0,
            0b011101 => DataFaultStatus::SynchronousExternalAbortTtwLevel1,
            0b011110 => DataFaultStatus::SynchronousExternalAbortTtwLevel2,
            0b011111 => DataFaultStatus::SynchronousExternalAbortTtwLevel3,
            0b100001 => DataFaultStatus::AlignmentFault,
            0b100010 => DataFaultStatus::DebugException,
            0b110000 => DataFaultStatus::TlbConflictAbort,
            0b110001 => DataFaultStatus::UnsupportedAtomicHardwareUpdate,
            0b111101 => DataFaultStatus::ImplementationDefined,
            0b111110 => DataFaultStatus::Lockdown,
            0b111111 => DataFaultStatus::ExclusiveAccessFault,
            _ => DataFaultStatus::ImplementationDefined,
        }
    }
}

impl DataFaultStatus {
    /// Get a human-readable description of the data fault status
    pub fn description(&self) -> &'static str {
        match self {
            DataFaultStatus::AddressSizeFaultLevel0 => "Address size fault, level 0",
            DataFaultStatus::AddressSizeFaultLevel1 => "Address size fault, level 1",
            DataFaultStatus::AddressSizeFaultLevel2 => "Address size fault, level 2",
            DataFaultStatus::AddressSizeFaultLevel3 => "Address size fault, level 3",
            DataFaultStatus::TranslationFaultLevel0 => "Translation fault, level 0",
            DataFaultStatus::TranslationFaultLevel1 => "Translation fault, level 1",
            DataFaultStatus::TranslationFaultLevel2 => "Translation fault, level 2",
            DataFaultStatus::TranslationFaultLevel3 => "Translation fault, level 3",
            DataFaultStatus::AccessFaultLevel1 => "Access flag fault, level 1",
            DataFaultStatus::AccessFaultLevel2 => "Access flag fault, level 2",
            DataFaultStatus::AccessFaultLevel3 => "Access flag fault, level 3",
            DataFaultStatus::PermissionFaultLevel1 => "Permission fault, level 1",
            DataFaultStatus::PermissionFaultLevel2 => "Permission fault, level 2",
            DataFaultStatus::PermissionFaultLevel3 => "Permission fault, level 3",
            DataFaultStatus::SynchronousExternalAbort => "Synchronous External abort",
            DataFaultStatus::SynchronousTagCheck => "Synchronous Tag Check fault",
            DataFaultStatus::AsynchronousExternalAbort => "Asynchronous External abort",
            DataFaultStatus::AsynchronousTagCheck => "Asynchronous Tag Check fault",
            DataFaultStatus::SynchronousExternalAbortTtwLevel0 => {
                "Synchronous External abort on TTW, level 0"
            }
            DataFaultStatus::SynchronousExternalAbortTtwLevel1 => {
                "Synchronous External abort on TTW, level 1"
            }
            DataFaultStatus::SynchronousExternalAbortTtwLevel2 => {
                "Synchronous External abort on TTW, level 2"
            }
            DataFaultStatus::SynchronousExternalAbortTtwLevel3 => {
                "Synchronous External abort on TTW, level 3"
            }
            DataFaultStatus::AlignmentFault => "Alignment fault",
            DataFaultStatus::DebugException => "Debug exception",
            DataFaultStatus::TlbConflictAbort => "TLB conflict abort",
            DataFaultStatus::UnsupportedAtomicHardwareUpdate => {
                "Unsupported atomic hardware update fault"
            }
            DataFaultStatus::ImplementationDefined => "Implementation defined fault",
            DataFaultStatus::Lockdown => "Lockdown",
            DataFaultStatus::ExclusiveAccessFault => "Exclusive access fault",
        }
    }
}

/// Decoded ESR_EL1 information
#[derive(Debug, Clone)]
pub struct EsrInfo {
    /// Raw ESR_EL1 value
    pub raw_esr: u32,
    /// Exception class
    pub exception_class: ExceptionClass,
    /// Instruction length (0 = 16-bit, 1 = 32-bit)
    pub instruction_length: bool,
    /// Instruction Specific Syndrome
    pub iss: u32,
    /// Additional decoded information based on exception class
    pub details: EsrDetails,
}

/// Exception-specific details
#[derive(Debug, Clone)]
pub enum EsrDetails {
    /// Unknown or unsupported exception
    Unknown,
    /// System call (SVC) information
    SystemCall {
        /// SVC immediate value
        immediate: u16,
    },
    /// Data abort information
    DataAbort {
        /// Whether the fault address is valid
        fault_address_valid: bool,
        /// Write not Read (true = write, false = read)
        write_not_read: bool,
        /// Sign extend (for loads)
        sign_extend: bool,
        /// Access size (0=byte, 1=halfword, 2=word, 3=doubleword)
        access_size: u8,
        /// Data fault status
        fault_status: DataFaultStatus,
        /// Cache maintenance operation
        cache_maintenance: bool,
    },
    /// Instruction abort information
    InstructionAbort {
        /// Instruction fault status
        fault_status: DataFaultStatus,
    },
    /// Alignment fault information
    AlignmentFault {
        /// Write not Read
        write_not_read: bool,
        /// Access size
        access_size: u8,
    },
    /// PC alignment fault
    PcAlignmentFault,
    /// SP alignment fault
    SpAlignmentFault,
    /// Breakpoint information
    Breakpoint {
        /// Breakpoint immediate value
        immediate: u16,
    },
    /// Software step exception
    SoftwareStep,
    /// Watchpoint exception
    Watchpoint,
}

/// ESR_EL1 decoder
pub struct EsrDecoder;

impl EsrDecoder {
    /// Create a new ESR decoder
    pub fn new() -> Self {
        EsrDecoder
    }

    /// Decode ESR_EL1 register value
    pub fn decode_esr(&self, esr: u32) -> EsrInfo {
        Self::decode(esr)
    }

    /// Decode ESR_EL1 register value
    pub fn decode(esr: u32) -> EsrInfo {
        let exception_class = ExceptionClass::from((esr >> 26) & 0x3F);
        let instruction_length = (esr >> 25) & 1 == 1;
        let iss = esr & 0x1FFFFFF;

        let details = Self::decode_details(&exception_class, iss);

        EsrInfo {
            raw_esr: esr,
            exception_class,
            instruction_length,
            iss,
            details,
        }
    }

    /// Decode exception-specific details
    fn decode_details(exception_class: &ExceptionClass, iss: u32) -> EsrDetails {
        match exception_class {
            ExceptionClass::Svc64 | ExceptionClass::Svc32 => EsrDetails::SystemCall {
                immediate: (iss & 0xFFFF) as u16,
            },
            ExceptionClass::DataAbortLower | ExceptionClass::DataAbortSame => {
                EsrDetails::DataAbort {
                    fault_address_valid: (iss >> 10) & 1 == 1,
                    write_not_read: (iss >> 6) & 1 == 1,
                    sign_extend: (iss >> 21) & 1 == 1,
                    access_size: ((iss >> 22) & 3) as u8,
                    fault_status: DataFaultStatus::from(iss & 0x3F),
                    cache_maintenance: (iss >> 8) & 1 == 1,
                }
            }
            ExceptionClass::InstructionAbortLower | ExceptionClass::InstructionAbortSame => {
                EsrDetails::InstructionAbort {
                    fault_status: DataFaultStatus::from(iss & 0x3F),
                }
            }
            ExceptionClass::PcAlignment => EsrDetails::PcAlignmentFault,
            ExceptionClass::SpAlignment => EsrDetails::SpAlignmentFault,
            ExceptionClass::BreakpointLower | ExceptionClass::BreakpointSame => {
                EsrDetails::Breakpoint {
                    immediate: (iss & 0xFFFF) as u16,
                }
            }
            ExceptionClass::SoftwareStepLower | ExceptionClass::SoftwareStepSame => {
                EsrDetails::SoftwareStep
            }
            ExceptionClass::WatchpointLower | ExceptionClass::WatchpointSame => {
                EsrDetails::Watchpoint
            }
            _ => EsrDetails::Unknown,
        }
    }

    /// Get human-readable description of exception
    pub fn get_description(esr_info: &EsrInfo) -> &'static str {
        match esr_info.exception_class {
            ExceptionClass::Unknown => "Unknown exception",
            ExceptionClass::Svc64 => "System call (SVC) from AArch64",
            ExceptionClass::Svc32 => "System call (SVC) from AArch32",
            ExceptionClass::DataAbortLower => "Data abort from lower exception level",
            ExceptionClass::DataAbortSame => "Data abort from same exception level",
            ExceptionClass::InstructionAbortLower => "Instruction abort from lower exception level",
            ExceptionClass::InstructionAbortSame => "Instruction abort from same exception level",
            ExceptionClass::PcAlignment => "PC alignment fault",
            ExceptionClass::SpAlignment => "SP alignment fault",
            ExceptionClass::BreakpointLower => "Breakpoint from lower exception level",
            ExceptionClass::BreakpointSame => "Breakpoint from same exception level",
            ExceptionClass::SoftwareStepLower => "Software step from lower exception level",
            ExceptionClass::SoftwareStepSame => "Software step from same exception level",
            ExceptionClass::WatchpointLower => "Watchpoint from lower exception level",
            ExceptionClass::WatchpointSame => "Watchpoint from same exception level",
            ExceptionClass::WfiWfe => "Trapped WFI/WFE instruction",
            ExceptionClass::MsrMrs => "Trapped MSR/MRS or system instruction",
            ExceptionClass::IllegalExecution => "Illegal execution state",
            ExceptionClass::BranchTarget => "Branch target exception",
            ExceptionClass::Brk64 => "BRK instruction (AArch64)",
            ExceptionClass::Bkpt32 => "BKPT instruction (AArch32)",
            ExceptionClass::SError => "SError interrupt",
            _ => "Other exception type",
        }
    }
}
