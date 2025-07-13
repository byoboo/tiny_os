//! Advanced Memory Protection Manager
//!
//! This module provides the central coordinator for all advanced memory
//! protection features including page permissions, ASLR, stack protection, and CFI.

use core::{
    mem::MaybeUninit,
    ptr::addr_of_mut,
    sync::atomic::{AtomicBool, Ordering},
};

use crate::{
    memory::{MemoryManager, PAGE_SIZE},
    process::scheduler::get_current_task_id,
};

use super::{
    aslr::AslrManager,
    cfi::CfiManager,
    permissions::{AdvancedProtectionStats, PagePermissions, PermissionFaultType, PermissionFaultResult, ProtectedPage},
    stack::AdvancedStackProtection,
};

/// Maximum number of pages that can be tracked for permissions
const MAX_PROTECTED_PAGES: usize = 1024;

/// Maximum number of processes for protection tracking
const MAX_PROTECTED_PROCESSES: usize = 32;

/// Advanced memory protection manager that coordinates all protection features
#[derive(Debug)]
pub struct AdvancedMemoryProtection {
    /// Protected pages array
    protected_pages: [ProtectedPage; MAX_PROTECTED_PAGES],
    /// Number of protected pages
    protected_page_count: usize,
    /// Memory manager reference
    memory_manager: Option<*mut MemoryManager>,
    /// ASLR manager
    aslr_manager: AslrManager,
    /// Advanced stack protection
    stack_protection: AdvancedStackProtection,
    /// CFI manager
    cfi_manager: CfiManager,
    /// Protection statistics
    stats: AdvancedProtectionStats,
}

impl AdvancedMemoryProtection {
    /// Create a new advanced memory protection manager
    pub const fn new() -> Self {
        Self {
            protected_pages: [ProtectedPage::new(0, 0, PagePermissions::none(), 0); MAX_PROTECTED_PAGES],
            protected_page_count: 0,
            memory_manager: None,
            aslr_manager: AslrManager::new(),
            stack_protection: AdvancedStackProtection::new(),
            cfi_manager: CfiManager::new(),
            stats: AdvancedProtectionStats::new(),
        }
    }

    /// Initialize the advanced memory protection manager
    pub fn init(&mut self, memory_manager: *mut MemoryManager) {
        self.memory_manager = Some(memory_manager);
        self.aslr_manager.set_enabled(true);
        self.cfi_manager.set_enabled(true);
        self.stats.aslr_enabled = true;
        self.stats.cfi_enabled = true;
    }

    /// Set page permissions for a virtual address
    pub fn set_page_permissions(
        &mut self,
        virtual_addr: u64,
        permissions: PagePermissions,
    ) -> Result<(), &'static str> {
        if self.protected_page_count >= MAX_PROTECTED_PAGES {
            return Err("Too many protected pages");
        }

        // Find existing page or create new one
        let mut page_index = None;
        for i in 0..self.protected_page_count {
            if self.protected_pages[i].virtual_addr == virtual_addr {
                page_index = Some(i);
                break;
            }
        }

        let index = if let Some(idx) = page_index {
            idx
        } else {
            let idx = self.protected_page_count;
            self.protected_page_count += 1;
            idx
        };

        // Update page permissions
        self.protected_pages[index] = ProtectedPage::new(
            virtual_addr,
            virtual_addr, // Simplified - in real implementation would translate to physical
            permissions,
            get_current_task_id().unwrap_or(0) as usize,
        );

        // Apply permissions to hardware page table (simulated)
        self.apply_permissions_to_hardware(virtual_addr, permissions)?;

        self.stats.protected_pages = self.protected_page_count;
        Ok(())
    }

    /// Get page permissions for a virtual address
    pub fn get_page_permissions(&self, virtual_addr: u64) -> Option<PagePermissions> {
        for i in 0..self.protected_page_count {
            if self.protected_pages[i].virtual_addr == virtual_addr {
                return Some(self.protected_pages[i].permissions);
            }
        }
        None
    }

    /// Handle permission fault
    pub fn handle_permission_fault(
        &mut self,
        virtual_addr: u64,
        fault_type: PermissionFaultType,
    ) -> PermissionFaultResult {
        self.stats.permission_faults += 1;

        // Check if this is a legitimate access
        if let Some(permissions) = self.get_page_permissions(virtual_addr) {
            match fault_type {
                PermissionFaultType::Read | PermissionFaultType::ReadViolation if permissions.is_readable() => {
                    return PermissionFaultResult::Continue;
                }
                PermissionFaultType::Write | PermissionFaultType::WriteViolation if permissions.is_writable() => {
                    return PermissionFaultResult::Continue;
                }
                PermissionFaultType::Execute | PermissionFaultType::ExecuteViolation if permissions.is_executable() => {
                    return PermissionFaultResult::Continue;
                }
                PermissionFaultType::UserAccess | PermissionFaultType::UserAccessViolation if permissions.user_accessible => {
                    return PermissionFaultResult::Continue;
                }
                _ => {
                    // Permission violation
                    return PermissionFaultResult::Terminate;
                }
            }
        }

        // Unknown page or access violation
        PermissionFaultResult::Terminate
    }

    /// Get ASLR offset
    pub fn get_aslr_offset(&mut self) -> u64 {
        self.aslr_manager.get_random_offset()
    }

    /// Setup stack protection for a process
    pub fn setup_stack_protection(
        &mut self,
        process_id: usize,
        stack_start: u64,
        stack_size: u64,
    ) -> Result<u64, &'static str> {
        if process_id >= MAX_PROTECTED_PROCESSES {
            return Err("Invalid process ID");
        }

        let stack_end = stack_start + stack_size;
        self.stack_protection
            .set_stack_boundaries(process_id, stack_start, stack_end);

        // Generate stack canary
        let canary = self.stack_protection.generate_canary(process_id);

        // Set stack pages as non-executable
        let mut addr = stack_start;
        while addr < stack_end {
            self.set_page_permissions(addr, PagePermissions::stack_page())?;
            addr += PAGE_SIZE as u64;
        }

        self.stats.stack_protections += 1;
        Ok(canary)
    }

    /// Verify stack canary
    pub fn verify_stack_canary(&mut self, process_id: usize, canary: u64) -> bool {
        let result = self.stack_protection.verify_canary(process_id, canary);
        if !result {
            self.stats.canary_violations += 1;
        }
        result
    }

    /// Push return address for CFI
    pub fn push_return_address(&mut self, process_id: usize, address: u64) -> bool {
        self.cfi_manager.push_return_address(process_id, address)
    }

    /// Pop and validate return address for CFI
    pub fn pop_return_address(&mut self, process_id: usize, expected_address: u64) -> bool {
        let result = self.cfi_manager
            .pop_return_address(process_id, expected_address);
        if !result {
            self.stats.cfi_violations += 1;
            self.stats.return_address_mismatches += 1;
        }
        result
    }

    /// Get protection statistics
    pub fn get_advanced_stats(&self) -> AdvancedProtectionStats {
        let mut stats = self.stats;
        let (canary_checks, stack_overflows) = self.stack_protection.get_stats();
        let (return_validations, cfi_violations) = self.cfi_manager.get_stats();
        
        // Update core stats
        stats.stack_protections = canary_checks;
        stats.canary_violations = stack_overflows;
        stats.cfi_violations = cfi_violations;
        stats.return_address_mismatches = return_validations;
        stats.aslr_randomizations = self.aslr_manager.get_randomizations();
        
        // Update backward compatibility fields
        stats.total_protected_pages = stats.protected_pages;
        stats.protected_stacks = stats.stack_protections;
        stats.stack_canaries_active = canary_checks;
        stats.stack_violations = stats.canary_violations;
        stats.faults_handled = stats.permission_faults;
        stats.faults_terminated = stats.permission_faults / 2; // Simplified estimate
        stats.rop_attacks_blocked = stats.cfi_violations;
        
        stats
    }

    /// Enable/disable ASLR
    pub fn set_aslr_enabled(&mut self, enabled: bool) {
        self.aslr_manager.set_enabled(enabled);
        self.stats.aslr_enabled = enabled;
    }

    /// Enable/disable CFI
    pub fn set_cfi_enabled(&mut self, enabled: bool) {
        self.cfi_manager.set_enabled(enabled);
        self.stats.cfi_enabled = enabled;
    }

    /// Get protected page count
    pub fn get_protected_page_count(&self) -> usize {
        self.protected_page_count
    }

    /// Apply permissions to hardware page table (simulated)
    fn apply_permissions_to_hardware(
        &mut self,
        _virtual_addr: u64,
        _permissions: PagePermissions,
    ) -> Result<(), &'static str> {
        // In a real implementation, this would:
        // 1. Translate virtual address to page table entry
        // 2. Update hardware page table with permission bits
        // 3. Flush TLB if necessary
        // 4. Handle any hardware-specific permission enforcement
        
        // For now, this is a no-op simulation
        Ok(())
    }
}

/// Global advanced memory protection manager
static mut ADVANCED_MEMORY_PROTECTION: MaybeUninit<AdvancedMemoryProtection> =
    MaybeUninit::uninit();
static ADVANCED_MEMORY_PROTECTION_INIT: AtomicBool = AtomicBool::new(false);

/// Initialize advanced memory protection manager
pub fn init_advanced_memory_protection(memory_manager: *mut MemoryManager) {
    unsafe {
        let mut manager = AdvancedMemoryProtection::new();
        manager.init(memory_manager);
        ADVANCED_MEMORY_PROTECTION = MaybeUninit::new(manager);
        ADVANCED_MEMORY_PROTECTION_INIT.store(true, Ordering::SeqCst);
    }
}

/// Execute a function with the global advanced memory protection manager
pub fn with_advanced_memory_protection<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut AdvancedMemoryProtection) -> R,
{
    unsafe {
        if !ADVANCED_MEMORY_PROTECTION_INIT.load(Ordering::SeqCst) {
            return None;
        }
        Some(f(
            &mut *addr_of_mut!(ADVANCED_MEMORY_PROTECTION).cast::<AdvancedMemoryProtection>()
        ))
    }
}

/// Set page permissions (global function)
pub fn set_advanced_page_permissions(
    virtual_addr: u64,
    permissions: PagePermissions,
) -> Result<(), &'static str> {
    with_advanced_memory_protection(|manager| {
        manager.set_page_permissions(virtual_addr, permissions)
    })
    .unwrap_or(Err("Advanced memory protection manager not initialized"))
}

/// Get page permissions (global function)
pub fn get_advanced_page_permissions(virtual_addr: u64) -> Option<PagePermissions> {
    with_advanced_memory_protection(|manager| manager.get_page_permissions(virtual_addr))
        .unwrap_or(None)
}

/// Handle permission fault (global function)
pub fn handle_advanced_permission_fault(
    virtual_addr: u64,
    fault_type: PermissionFaultType,
) -> PermissionFaultResult {
    with_advanced_memory_protection(|manager| {
        manager.handle_permission_fault(virtual_addr, fault_type)
    })
    .unwrap_or(PermissionFaultResult::Continue)
}

/// Get ASLR offset (global function)
pub fn get_aslr_offset() -> u64 {
    with_advanced_memory_protection(|manager| manager.get_aslr_offset()).unwrap_or(0)
}

/// Setup stack protection (global function)
pub fn setup_advanced_stack_protection(
    process_id: usize,
    stack_start: u64,
    stack_size: u64,
) -> Result<u64, &'static str> {
    with_advanced_memory_protection(|manager| {
        manager.setup_stack_protection(process_id, stack_start, stack_size)
    })
    .unwrap_or(Err("Advanced memory protection manager not initialized"))
}

/// Verify stack canary (global function)
pub fn verify_advanced_stack_canary(process_id: usize, canary: u64) -> bool {
    with_advanced_memory_protection(|manager| manager.verify_stack_canary(process_id, canary))
        .unwrap_or(false)
}

/// Get advanced protection statistics (global function)
pub fn get_advanced_protection_stats() -> AdvancedProtectionStats {
    with_advanced_memory_protection(|manager| manager.get_advanced_stats())
        .unwrap_or_else(AdvancedProtectionStats::new)
}
