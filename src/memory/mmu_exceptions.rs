//! MMU Exception Handling for TinyOS
//!
//! This module provides comprehensive MMU exception handling including:
//! - Page fault analysis and handling
//! - TLB miss management
//! - Memory access violation processing
//! - Integration with memory protection system
//!
//! # Architecture
//!
//! The MMU exception system works by:
//! 1. Intercepting memory management unit exceptions
//! 2. Analyzing fault addresses and access types
//! 3. Determining appropriate recovery actions
//! 4. Integrating with the broader memory management system

use crate::memory::{MemoryManager, BLOCK_SIZE};

/// MMU exception types as defined by ARM64 architecture
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MmuExceptionType {
    /// Address size fault (translation table level)
    AddressSizeFault { level: u8 },
    /// Translation fault (page not mapped)
    TranslationFault { level: u8 },
    /// Access flag fault (page not accessed)
    AccessFlagFault { level: u8 },
    /// Permission fault (insufficient permissions)
    PermissionFault { level: u8 },
    /// Alignment fault (misaligned access)
    AlignmentFault,
    /// TLB conflict abort
    TlbConflictAbort,
    /// Unsupported atomic update
    UnsupportedAtomicUpdate,
    /// Implementation defined exception
    ImplementationDefined { fault_code: u8 },
}

/// Memory access type that caused the fault
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccessType {
    /// Read access
    Read,
    /// Write access
    Write,
    /// Instruction fetch
    InstructionFetch,
}

/// Information about an MMU fault
#[derive(Debug, Clone, Copy)]
pub struct MmuFaultInfo {
    /// Virtual address that caused the fault
    pub fault_address: u64,
    /// Type of access that failed
    pub access_type: AccessType,
    /// Specific MMU exception type
    pub exception_type: MmuExceptionType,
    /// Instruction syndrome register value
    pub iss: u32,
    /// Whether the fault occurred in user or kernel mode
    pub user_mode: bool,
    /// Exception link register (return address)
    pub exception_lr: u64,
}

/// Recovery action for MMU faults
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MmuRecoveryAction {
    /// Continue execution (fault handled successfully)
    Continue,
    /// Terminate the current process
    TerminateProcess,
    /// Panic the system (unrecoverable fault)
    SystemPanic,
    /// Retry the operation (after fixing the issue)
    Retry,
}

/// Statistics for MMU exception handling
#[derive(Debug, Clone, Copy)]
pub struct MmuExceptionStats {
    /// Total number of MMU exceptions handled
    pub total_exceptions: u64,
    /// Number of page faults
    pub page_faults: u64,
    /// Number of permission faults
    pub permission_faults: u64,
    /// Number of alignment faults
    pub alignment_faults: u64,
    /// Number of TLB misses
    pub tlb_misses: u64,
    /// Number of successfully recovered faults
    pub recovered_faults: u64,
    /// Number of faults that caused process termination
    pub fatal_faults: u64,
}

/// MMU Exception Handler
pub struct MmuExceptionHandler {
    /// Exception statistics
    stats: MmuExceptionStats,
    /// Whether MMU exception handling is enabled
    enabled: bool,
}

impl MmuExceptionHandler {
    /// Create a new MMU exception handler
    pub const fn new() -> Self {
        Self {
            stats: MmuExceptionStats {
                total_exceptions: 0,
                page_faults: 0,
                permission_faults: 0,
                alignment_faults: 0,
                tlb_misses: 0,
                recovered_faults: 0,
                fatal_faults: 0,
            },
            enabled: false,
        }
    }

    /// Initialize the MMU exception handler
    pub fn init(&mut self) {
        self.enabled = true;
        self.stats = MmuExceptionStats {
            total_exceptions: 0,
            page_faults: 0,
            permission_faults: 0,
            alignment_faults: 0,
            tlb_misses: 0,
            recovered_faults: 0,
            fatal_faults: 0,
        };
    }

    /// Handle an MMU exception
    pub fn handle_mmu_exception(
        &mut self,
        fault_info: MmuFaultInfo,
        memory_manager: &mut MemoryManager,
    ) -> MmuRecoveryAction {
        if !self.enabled {
            return MmuRecoveryAction::SystemPanic;
        }

        self.stats.total_exceptions += 1;

        // Analyze the fault type and determine recovery action
        let recovery_action = match fault_info.exception_type {
            MmuExceptionType::TranslationFault { level: _ } => {
                self.stats.page_faults += 1;
                self.handle_page_fault(fault_info, memory_manager)
            }
            MmuExceptionType::PermissionFault { level: _ } => {
                self.stats.permission_faults += 1;
                self.handle_permission_fault(fault_info)
            }
            MmuExceptionType::AlignmentFault => {
                self.stats.alignment_faults += 1;
                self.handle_alignment_fault(fault_info)
            }
            MmuExceptionType::AccessFlagFault { level: _ } => {
                self.stats.tlb_misses += 1;
                self.handle_access_flag_fault(fault_info)
            }
            _ => {
                // Other fault types are generally fatal
                MmuRecoveryAction::SystemPanic
            }
        };

        // Update statistics based on recovery action
        match recovery_action {
            MmuRecoveryAction::Continue | MmuRecoveryAction::Retry => {
                self.stats.recovered_faults += 1;
            }
            MmuRecoveryAction::TerminateProcess | MmuRecoveryAction::SystemPanic => {
                self.stats.fatal_faults += 1;
            }
        }

        recovery_action
    }

    /// Handle a page fault (translation fault)
    fn handle_page_fault(
        &mut self,
        fault_info: MmuFaultInfo,
        memory_manager: &mut MemoryManager,
    ) -> MmuRecoveryAction {
        // For now, treat page faults as fatal in this simple OS
        // In a full OS, this would:
        // 1. Check if the address is in a valid VMA
        // 2. Allocate physical pages if needed
        // 3. Update page tables
        // 4. Handle copy-on-write, swap, etc.

        // Check if the fault address is in our managed memory range
        let heap_start = crate::memory::HEAP_START as u64;
        let heap_end = heap_start + (crate::memory::HEAP_SIZE as u64);

        if fault_info.fault_address >= heap_start && fault_info.fault_address < heap_end {
            // This is within our heap range - could be a lazy allocation
            MmuRecoveryAction::TerminateProcess
        } else {
            // Outside our managed range - definitely invalid
            if fault_info.user_mode {
                MmuRecoveryAction::TerminateProcess
            } else {
                MmuRecoveryAction::SystemPanic
            }
        }
    }

    /// Handle a permission fault
    fn handle_permission_fault(&mut self, fault_info: MmuFaultInfo) -> MmuRecoveryAction {
        // Check if this is a COW fault first
        if fault_info.access_type == AccessType::Write {
            // This could be a COW fault - check with COW manager
            if let Some(cow_manager) = crate::memory::get_cow_manager() {
                // Try to resolve the virtual address to physical address
                // For now, we'll use a simple approach
                let physical_addr = self.resolve_virtual_to_physical(fault_info.fault_address);

                if let Some(phys_addr) = physical_addr {
                    if cow_manager.is_cow_protected(phys_addr) {
                        // This is a COW fault - create fault info and handle it
                        let cow_fault = crate::memory::create_cow_fault_from_exception(
                            fault_info.fault_address,
                            phys_addr,
                            true, // is_write
                            0,    // process_id (simplified for now)
                        );

                        // Try to handle the COW fault
                        match cow_manager.handle_cow_fault(cow_fault) {
                            Ok(_new_page_addr) => {
                                // COW fault handled successfully
                                // Update page table mapping to point to new page
                                // This would require MMU integration
                                return MmuRecoveryAction::Retry;
                            }
                            Err(_) => {
                                // COW fault handling failed
                                // Fall through to standard permission fault handling
                            }
                        }
                    }
                }
            }
        }

        // Standard permission fault handling
        // Permission faults are generally security violations
        // In user mode, terminate the process
        // In kernel mode, this is likely a bug - panic
        if fault_info.user_mode {
            MmuRecoveryAction::TerminateProcess
        } else {
            MmuRecoveryAction::SystemPanic
        }
    }

    /// Resolve virtual address to physical address (simplified)
    fn resolve_virtual_to_physical(&self, virtual_addr: u64) -> Option<u64> {
        // This is a simplified implementation
        // In a real system, this would walk the page tables
        // For now, we'll use the VMM if available
        crate::memory::translate_address_global(virtual_addr).ok()
    }

    /// Handle an alignment fault
    fn handle_alignment_fault(&mut self, fault_info: MmuFaultInfo) -> MmuRecoveryAction {
        // Alignment faults can sometimes be fixed by emulation
        // For simplicity, we'll terminate/panic
        if fault_info.user_mode {
            MmuRecoveryAction::TerminateProcess
        } else {
            MmuRecoveryAction::SystemPanic
        }
    }

    /// Handle an access flag fault (TLB miss)
    fn handle_access_flag_fault(&mut self, _fault_info: MmuFaultInfo) -> MmuRecoveryAction {
        // Access flag faults just mean the TLB entry needs to be updated
        // The hardware will retry the access automatically
        MmuRecoveryAction::Retry
    }

    /// Get current statistics
    pub fn get_stats(&self) -> MmuExceptionStats {
        self.stats
    }

    /// Check if MMU exception handling is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable or disable MMU exception handling
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Parse MMU exception information from ESR_EL1
pub fn parse_mmu_exception(
    esr_el1: u32,
    far_el1: u64,
    user_mode: bool,
    exception_lr: u64,
) -> MmuFaultInfo {
    // Extract Exception Class (bits 31:26)
    let ec = (esr_el1 >> 26) & 0x3F;

    // Extract Instruction Syndrome (bits 24:0)
    let iss = esr_el1 & 0x1FFFFFF;

    // For data/instruction aborts, extract fault status (bits 5:0 of ISS)
    let fault_status = iss & 0x3F;

    // Extract Write/not Read (bit 6) for data aborts
    let wnr = (iss >> 6) & 1;

    // Determine access type based on exception class
    let access_type = match ec {
        0x20 | 0x21 => AccessType::InstructionFetch, // Instruction abort
        0x24 | 0x25 => {
            // Data abort
            if wnr == 1 {
                AccessType::Write
            } else {
                AccessType::Read
            }
        }
        _ => AccessType::Read, // Default
    };

    // Parse fault status into exception type
    let exception_type = match fault_status & 0x3C {
        0x00 => MmuExceptionType::AddressSizeFault {
            level: (fault_status & 3) as u8,
        },
        0x04 => MmuExceptionType::TranslationFault {
            level: (fault_status & 3) as u8,
        },
        0x08 => MmuExceptionType::AccessFlagFault {
            level: (fault_status & 3) as u8,
        },
        0x0C => MmuExceptionType::PermissionFault {
            level: (fault_status & 3) as u8,
        },
        0x21 => MmuExceptionType::AlignmentFault,
        0x30 => MmuExceptionType::TlbConflictAbort,
        0x31 => MmuExceptionType::UnsupportedAtomicUpdate,
        _ => MmuExceptionType::ImplementationDefined {
            fault_code: (fault_status & 0x3F) as u8,
        },
    };

    MmuFaultInfo {
        fault_address: far_el1,
        access_type,
        exception_type,
        iss,
        user_mode,
        exception_lr,
    }
}

/// Global MMU exception handler instance
static mut MMU_EXCEPTION_HANDLER: MmuExceptionHandler = MmuExceptionHandler::new();

/// Initialize MMU exception handling
pub fn init_mmu_exceptions() {
    unsafe {
        MMU_EXCEPTION_HANDLER.init();
    }
}

/// Handle MMU exception (called from exception vectors)
pub fn handle_mmu_exception_global(
    esr_el1: u32,
    far_el1: u64,
    user_mode: bool,
    exception_lr: u64,
    memory_manager: &mut MemoryManager,
) -> MmuRecoveryAction {
    let fault_info = parse_mmu_exception(esr_el1, far_el1, user_mode, exception_lr);

    unsafe { MMU_EXCEPTION_HANDLER.handle_mmu_exception(fault_info, memory_manager) }
}

/// Get MMU exception statistics
pub fn get_mmu_exception_stats() -> MmuExceptionStats {
    unsafe { MMU_EXCEPTION_HANDLER.get_stats() }
}

/// Check if MMU exception handling is enabled
pub fn is_mmu_exception_handling_enabled() -> bool {
    unsafe { MMU_EXCEPTION_HANDLER.is_enabled() }
}

/// Enable or disable MMU exception handling
pub fn set_mmu_exception_handling_enabled(enabled: bool) {
    unsafe {
        MMU_EXCEPTION_HANDLER.set_enabled(enabled);
    }
}
