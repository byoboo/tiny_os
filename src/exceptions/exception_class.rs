//! Exception Class enumeration and utilities
//!
//! Defines the ExceptionClass enum and its associated implementations
//! for decoding ESR_EL1[31:26] Exception Class field.

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
    /// Trapped MRS/MSR, MRC/MCR access (AArch64)
    SystemRegister = 0x18,
    /// Trapped access to SVE functionality
    SveTrapped = 0x19,
    /// Exception from pointer authentication failure
    PAuthFailure = 0x1C,
    /// Instruction Abort from lower Exception level
    InstructionAbortLower = 0x20,
    /// Instruction Abort from same Exception level
    InstructionAbortSame = 0x21,
    /// PC Alignment fault
    PcAlignment = 0x22,
    /// Data Abort from lower Exception level
    DataAbortLower = 0x24,
    /// Data Abort from same Exception level
    DataAbortSame = 0x25,
    /// SP Alignment fault
    SpAlignment = 0x26,
    /// Trapped floating point exception (AArch32)
    FpException32 = 0x28,
    /// Trapped floating point exception (AArch64)
    FpException64 = 0x2C,
    /// SError interrupt
    SError = 0x2F,
    /// Breakpoint from lower Exception level
    BreakpointLower = 0x30,
    /// Breakpoint from same Exception level
    BreakpointSame = 0x31,
    /// Software Step from lower Exception level
    SoftwareStepLower = 0x32,
    /// Software Step from same Exception level
    SoftwareStepSame = 0x33,
    /// Watchpoint from lower Exception level
    WatchpointLower = 0x34,
    /// Watchpoint from same Exception level
    WatchpointSame = 0x35,
    /// BKPT instruction execution in AArch32 state
    Bkpt32 = 0x38,
    /// Vector Catch exception in AArch32 state
    VectorCatch32 = 0x3A,
    /// BRK instruction execution in AArch64 state
    Brk64 = 0x3C,
}

impl From<u32> for ExceptionClass {
    fn from(value: u32) -> Self {
        match (value >> 26) & 0x3F {
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
            0x18 => ExceptionClass::SystemRegister,
            0x19 => ExceptionClass::SveTrapped,
            0x1C => ExceptionClass::PAuthFailure,
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
    /// Get human-readable description of the exception class
    pub fn description(&self) -> &'static str {
        match self {
            ExceptionClass::Unknown => "Unknown exception",
            ExceptionClass::WfiWfe => "Trapped WFI or WFE instruction",
            ExceptionClass::McrMrc15 => "Trapped MCR/MRC access to CP15",
            ExceptionClass::McrrMrrc15 => "Trapped MCRR/MRRC access to CP15",
            ExceptionClass::McrMrc14 => "Trapped MCR/MRC access to CP14",
            ExceptionClass::LdcStc => "Trapped LDC/STC access",
            ExceptionClass::SveSmdFp => "Trapped access to SVE/SIMD/FP",
            ExceptionClass::Vmrs => "Trapped VMRS access",
            ExceptionClass::PAuth => "Trapped pointer authentication instruction",
            ExceptionClass::McrMrc14_2 => "Trapped MCR/MRC access to CP14 (2)",
            ExceptionClass::BranchTarget => "Branch Target Exception",
            ExceptionClass::IllegalExecution => "Illegal Execution state",
            ExceptionClass::Svc32 => "SVC instruction execution in AArch32 state",
            ExceptionClass::Hvc32 => "HVC instruction execution in AArch32 state",
            ExceptionClass::Smc32 => "SMC instruction execution in AArch32 state",
            ExceptionClass::Svc64 => "SVC instruction execution in AArch64 state",
            ExceptionClass::Hvc64 => "HVC instruction execution in AArch64 state",
            ExceptionClass::Smc64 => "SMC instruction execution in AArch64 state",
            ExceptionClass::SystemRegister => "Trapped MRS/MSR, MRC/MCR access",
            ExceptionClass::SveTrapped => "Trapped access to SVE functionality",
            ExceptionClass::PAuthFailure => "Exception from pointer authentication failure",
            ExceptionClass::InstructionAbortLower => "Instruction Abort from lower Exception level",
            ExceptionClass::InstructionAbortSame => "Instruction Abort from same Exception level",
            ExceptionClass::PcAlignment => "PC Alignment fault",
            ExceptionClass::DataAbortLower => "Data Abort from lower Exception level",
            ExceptionClass::DataAbortSame => "Data Abort from same Exception level",
            ExceptionClass::SpAlignment => "SP Alignment fault",
            ExceptionClass::FpException32 => "Trapped floating point exception (AArch32)",
            ExceptionClass::FpException64 => "Trapped floating point exception (AArch64)",
            ExceptionClass::SError => "SError interrupt",
            ExceptionClass::BreakpointLower => "Breakpoint from lower Exception level",
            ExceptionClass::BreakpointSame => "Breakpoint from same Exception level",
            ExceptionClass::SoftwareStepLower => "Software Step from lower Exception level",
            ExceptionClass::SoftwareStepSame => "Software Step from same Exception level",
            ExceptionClass::WatchpointLower => "Watchpoint from lower Exception level",
            ExceptionClass::WatchpointSame => "Watchpoint from same Exception level",
            ExceptionClass::Bkpt32 => "BKPT instruction execution in AArch32 state",
            ExceptionClass::VectorCatch32 => "Vector Catch exception in AArch32 state",
            ExceptionClass::Brk64 => "BRK instruction execution in AArch64 state",
        }
    }

    /// Check if this is a data abort exception
    pub fn is_data_abort(&self) -> bool {
        matches!(self, ExceptionClass::DataAbortLower | ExceptionClass::DataAbortSame)
    }

    /// Check if this is an instruction abort exception
    pub fn is_instruction_abort(&self) -> bool {
        matches!(self, ExceptionClass::InstructionAbortLower | ExceptionClass::InstructionAbortSame)
    }

    /// Check if this is a system call exception
    pub fn is_system_call(&self) -> bool {
        matches!(self, ExceptionClass::Svc32 | ExceptionClass::Svc64)
    }

    /// Check if this is a hypervisor call exception
    pub fn is_hypervisor_call(&self) -> bool {
        matches!(self, ExceptionClass::Hvc32 | ExceptionClass::Hvc64)
    }

    /// Check if this is a secure monitor call exception
    pub fn is_secure_monitor_call(&self) -> bool {
        matches!(self, ExceptionClass::Smc32 | ExceptionClass::Smc64)
    }

    /// Check if this is a debug-related exception
    pub fn is_debug_exception(&self) -> bool {
        matches!(
            self,
            ExceptionClass::BreakpointLower
                | ExceptionClass::BreakpointSame
                | ExceptionClass::SoftwareStepLower
                | ExceptionClass::SoftwareStepSame
                | ExceptionClass::WatchpointLower
                | ExceptionClass::WatchpointSame
                | ExceptionClass::Bkpt32
                | ExceptionClass::Brk64
        )
    }
}
