//! Data Fault Status enumeration and utilities
//!
//! Defines the DataFaultStatus enum for decoding data abort fault information
//! from the ISS field of ESR_EL1 register.

/// Data Fault Status Code (DFSC) values for data abort exceptions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DataFaultStatus {
    /// Address size fault in level 0 of translation or translation table base register
    AddressSizeLevel0 = 0x00,
    /// Address size fault in level 1 of translation
    AddressSizeLevel1 = 0x01,
    /// Address size fault in level 2 of translation
    AddressSizeLevel2 = 0x02,
    /// Address size fault in level 3 of translation
    AddressSizeLevel3 = 0x03,
    /// Translation fault in level 0 of translation
    TranslationLevel0 = 0x04,
    /// Translation fault in level 1 of translation
    TranslationLevel1 = 0x05,
    /// Translation fault in level 2 of translation
    TranslationLevel2 = 0x06,
    /// Translation fault in level 3 of translation
    TranslationLevel3 = 0x07,
    /// Access flag fault in level 1 of translation
    AccessFlagLevel1 = 0x09,
    /// Access flag fault in level 2 of translation
    AccessFlagLevel2 = 0x0A,
    /// Access flag fault in level 3 of translation
    AccessFlagLevel3 = 0x0B,
    /// Permission fault in level 1 of translation
    PermissionLevel1 = 0x0D,
    /// Permission fault in level 2 of translation
    PermissionLevel2 = 0x0E,
    /// Permission fault in level 3 of translation
    PermissionLevel3 = 0x0F,
    /// Synchronous External abort, not on translation table walk
    SynchronousExternal = 0x10,
    /// Synchronous Tag Check Fault
    TagCheckFault = 0x11,
    /// Asynchronous External abort
    AsynchronousExternal = 0x17,
    /// Synchronous External abort on translation table walk, level 0
    ExternalWalkLevel0 = 0x1C,
    /// Synchronous External abort on translation table walk, level 1
    ExternalWalkLevel1 = 0x1D,
    /// Synchronous External abort on translation table walk, level 2
    ExternalWalkLevel2 = 0x1E,
    /// Synchronous External abort on translation table walk, level 3
    ExternalWalkLevel3 = 0x1F,
    /// Synchronous parity or ECC error on memory access, not on translation table walk
    ParityErrorMemory = 0x18,
    /// Synchronous parity or ECC error on translation table walk, level 0
    ParityErrorWalkLevel0 = 0x1A,
    /// Synchronous parity or ECC error on translation table walk, level 1
    ParityErrorWalkLevel1 = 0x1B,
    /// Synchronous parity or ECC error on translation table walk, level 2
    ParityErrorWalkLevel2 = 0x19,
    /// Synchronous parity or ECC error on translation table walk, level 3
    ParityErrorWalkLevel3 = 0x20,
    /// Alignment fault
    AlignmentFault = 0x21,
    /// Debug exception
    DebugException = 0x22,
    /// TLB conflict abort
    TlbConflict = 0x30,
    /// Unsupported atomic hardware update fault
    UnsupportedAtomic = 0x31,
    /// Implementation defined fault (Lockdown)
    ImplementationDefined = 0x34,
    /// Implementation defined fault (Unsupported Exclusive or Atomic access)
    ImplementationDefinedExclusive = 0x35,
}

impl From<u32> for DataFaultStatus {
    fn from(iss: u32) -> Self {
        let dfsc = iss & 0x3F;
        match dfsc {
            0x00 => DataFaultStatus::AddressSizeLevel0,
            0x01 => DataFaultStatus::AddressSizeLevel1,
            0x02 => DataFaultStatus::AddressSizeLevel2,
            0x03 => DataFaultStatus::AddressSizeLevel3,
            0x04 => DataFaultStatus::TranslationLevel0,
            0x05 => DataFaultStatus::TranslationLevel1,
            0x06 => DataFaultStatus::TranslationLevel2,
            0x07 => DataFaultStatus::TranslationLevel3,
            0x09 => DataFaultStatus::AccessFlagLevel1,
            0x0A => DataFaultStatus::AccessFlagLevel2,
            0x0B => DataFaultStatus::AccessFlagLevel3,
            0x0D => DataFaultStatus::PermissionLevel1,
            0x0E => DataFaultStatus::PermissionLevel2,
            0x0F => DataFaultStatus::PermissionLevel3,
            0x10 => DataFaultStatus::SynchronousExternal,
            0x11 => DataFaultStatus::TagCheckFault,
            0x17 => DataFaultStatus::AsynchronousExternal,
            0x18 => DataFaultStatus::ParityErrorMemory,
            0x19 => DataFaultStatus::ParityErrorWalkLevel2,
            0x1A => DataFaultStatus::ParityErrorWalkLevel0,
            0x1B => DataFaultStatus::ParityErrorWalkLevel1,
            0x1C => DataFaultStatus::ExternalWalkLevel0,
            0x1D => DataFaultStatus::ExternalWalkLevel1,
            0x1E => DataFaultStatus::ExternalWalkLevel2,
            0x1F => DataFaultStatus::ExternalWalkLevel3,
            0x20 => DataFaultStatus::ParityErrorWalkLevel3,
            0x21 => DataFaultStatus::AlignmentFault,
            0x22 => DataFaultStatus::DebugException,
            0x30 => DataFaultStatus::TlbConflict,
            0x31 => DataFaultStatus::UnsupportedAtomic,
            0x34 => DataFaultStatus::ImplementationDefined,
            0x35 => DataFaultStatus::ImplementationDefinedExclusive,
            _ => DataFaultStatus::ImplementationDefined, // Default for unknown codes
        }
    }
}

impl DataFaultStatus {
    /// Get human-readable description of the data fault status
    pub fn description(&self) -> &'static str {
        match self {
            DataFaultStatus::AddressSizeLevel0 => "Address size fault in level 0 of translation",
            DataFaultStatus::AddressSizeLevel1 => "Address size fault in level 1 of translation",
            DataFaultStatus::AddressSizeLevel2 => "Address size fault in level 2 of translation",
            DataFaultStatus::AddressSizeLevel3 => "Address size fault in level 3 of translation",
            DataFaultStatus::TranslationLevel0 => "Translation fault in level 0 of translation",
            DataFaultStatus::TranslationLevel1 => "Translation fault in level 1 of translation",
            DataFaultStatus::TranslationLevel2 => "Translation fault in level 2 of translation",
            DataFaultStatus::TranslationLevel3 => "Translation fault in level 3 of translation",
            DataFaultStatus::AccessFlagLevel1 => "Access flag fault in level 1 of translation",
            DataFaultStatus::AccessFlagLevel2 => "Access flag fault in level 2 of translation",
            DataFaultStatus::AccessFlagLevel3 => "Access flag fault in level 3 of translation",
            DataFaultStatus::PermissionLevel1 => "Permission fault in level 1 of translation",
            DataFaultStatus::PermissionLevel2 => "Permission fault in level 2 of translation",
            DataFaultStatus::PermissionLevel3 => "Permission fault in level 3 of translation",
            DataFaultStatus::SynchronousExternal => "Synchronous External abort",
            DataFaultStatus::TagCheckFault => "Synchronous Tag Check Fault",
            DataFaultStatus::AsynchronousExternal => "Asynchronous External abort",
            DataFaultStatus::ParityErrorMemory => "Synchronous parity/ECC error on memory access",
            DataFaultStatus::ExternalWalkLevel0 => "Synchronous External abort on translation table walk, level 0",
            DataFaultStatus::ExternalWalkLevel1 => "Synchronous External abort on translation table walk, level 1",
            DataFaultStatus::ExternalWalkLevel2 => "Synchronous External abort on translation table walk, level 2",
            DataFaultStatus::ExternalWalkLevel3 => "Synchronous External abort on translation table walk, level 3",
            DataFaultStatus::ParityErrorWalkLevel0 => "Synchronous parity/ECC error on translation table walk, level 0",
            DataFaultStatus::ParityErrorWalkLevel1 => "Synchronous parity/ECC error on translation table walk, level 1",
            DataFaultStatus::ParityErrorWalkLevel2 => "Synchronous parity/ECC error on translation table walk, level 2",
            DataFaultStatus::ParityErrorWalkLevel3 => "Synchronous parity/ECC error on translation table walk, level 3",
            DataFaultStatus::AlignmentFault => "Alignment fault",
            DataFaultStatus::DebugException => "Debug exception",
            DataFaultStatus::TlbConflict => "TLB conflict abort",
            DataFaultStatus::UnsupportedAtomic => "Unsupported atomic hardware update fault",
            DataFaultStatus::ImplementationDefined => "Implementation defined fault",
            DataFaultStatus::ImplementationDefinedExclusive => "Implementation defined fault (Exclusive/Atomic)",
        }
    }

    /// Check if this is a translation fault
    pub fn is_translation_fault(&self) -> bool {
        matches!(
            self,
            DataFaultStatus::TranslationLevel0
                | DataFaultStatus::TranslationLevel1
                | DataFaultStatus::TranslationLevel2
                | DataFaultStatus::TranslationLevel3
        )
    }

    /// Check if this is a permission fault
    pub fn is_permission_fault(&self) -> bool {
        matches!(
            self,
            DataFaultStatus::PermissionLevel1
                | DataFaultStatus::PermissionLevel2
                | DataFaultStatus::PermissionLevel3
        )
    }

    /// Check if this is an access flag fault
    pub fn is_access_flag_fault(&self) -> bool {
        matches!(
            self,
            DataFaultStatus::AccessFlagLevel1
                | DataFaultStatus::AccessFlagLevel2
                | DataFaultStatus::AccessFlagLevel3
        )
    }

    /// Check if this is an external abort
    pub fn is_external_abort(&self) -> bool {
        matches!(
            self,
            DataFaultStatus::SynchronousExternal
                | DataFaultStatus::AsynchronousExternal
                | DataFaultStatus::ExternalWalkLevel0
                | DataFaultStatus::ExternalWalkLevel1
                | DataFaultStatus::ExternalWalkLevel2
                | DataFaultStatus::ExternalWalkLevel3
        )
    }

    /// Get the translation table level involved in the fault
    pub fn translation_level(&self) -> Option<u8> {
        match self {
            DataFaultStatus::AddressSizeLevel0 | DataFaultStatus::TranslationLevel0 | DataFaultStatus::ExternalWalkLevel0 | DataFaultStatus::ParityErrorWalkLevel0 => Some(0),
            DataFaultStatus::AddressSizeLevel1 | DataFaultStatus::TranslationLevel1 | DataFaultStatus::AccessFlagLevel1 | DataFaultStatus::PermissionLevel1 | DataFaultStatus::ExternalWalkLevel1 | DataFaultStatus::ParityErrorWalkLevel1 => Some(1),
            DataFaultStatus::AddressSizeLevel2 | DataFaultStatus::TranslationLevel2 | DataFaultStatus::AccessFlagLevel2 | DataFaultStatus::PermissionLevel2 | DataFaultStatus::ExternalWalkLevel2 | DataFaultStatus::ParityErrorWalkLevel2 => Some(2),
            DataFaultStatus::AddressSizeLevel3 | DataFaultStatus::TranslationLevel3 | DataFaultStatus::AccessFlagLevel3 | DataFaultStatus::PermissionLevel3 | DataFaultStatus::ExternalWalkLevel3 | DataFaultStatus::ParityErrorWalkLevel3 => Some(3),
            _ => None,
        }
    }
}
