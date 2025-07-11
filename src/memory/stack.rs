//! Stack Management and Protection
//!
//! This module implements advanced stack management features for TinyOS,
//! including stack allocation, guard pages, overflow protection, and
//! privilege level stack switching.

use crate::memory::{
    mmu::{VirtualMemoryManager, RegionType, MemoryAttribute},
};

/// Stack size constants
pub const STACK_SIZE: usize = 0x4000;  // 16KB stack
pub const GUARD_PAGE_SIZE: usize = 0x1000;  // 4KB guard page
pub const MAX_STACKS: usize = 16;  // Maximum number of stacks

/// Stack allocation base address (start at 0x8000_0000)
pub const STACK_BASE: u64 = 0x8000_0000;

/// Stack allocation and management errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackError {
    OutOfMemory,
    InvalidStackId,
    StackOverflow,
    StackUnderflow,
    GuardPageViolation,
    AllocationFailed,
}

/// Stack protection and access flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackProtection {
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
    pub user_accessible: bool,
}

impl StackProtection {
    pub const KERNEL_STACK: Self = Self {
        readable: true,
        writable: true,
        executable: false,
        user_accessible: false,
    };

    pub const USER_STACK: Self = Self {
        readable: true,
        writable: true,
        executable: false,
        user_accessible: true,
    };

    pub const GUARD_PAGE: Self = Self {
        readable: false,
        writable: false,
        executable: false,
        user_accessible: false,
    };
}

/// Stack information and metadata
#[derive(Debug, Clone, Copy)]
pub struct StackInfo {
    pub stack_id: usize,
    pub base_address: u64,
    pub top_address: u64,
    pub current_sp: u64,
    pub size: u64,
    pub protection: StackProtection,
    pub guard_bottom: u64,
    pub guard_top: u64,
    pub allocated: bool,
    pub overflow_count: usize,
    pub max_usage: usize,
}

/// Stack manager for system-wide stack allocation and protection
pub struct StackManager {
    stacks: [Option<StackInfo>; MAX_STACKS],
    kernel_stack_id: Option<usize>,
    current_stack_id: Option<usize>,
    allocation_count: usize,
    overflow_count: usize,
    next_stack_addr: u64,
}

impl StackManager {
    /// Create a new stack manager
    pub const fn new() -> Self {
        Self {
            stacks: [None; MAX_STACKS],
            kernel_stack_id: None,
            current_stack_id: None,
            allocation_count: 0,
            overflow_count: 0,
            next_stack_addr: STACK_BASE,
        }
    }

    /// Initialize the stack manager with kernel stack
    pub fn init(&mut self, vmm: &mut VirtualMemoryManager) -> Result<(), StackError> {
        // Allocate kernel stack
        let kernel_stack_id = self.allocate_stack(StackProtection::KERNEL_STACK, vmm)?;
        self.kernel_stack_id = Some(kernel_stack_id);
        self.current_stack_id = Some(kernel_stack_id);

        // Set up stack protection in MMU
        if let Some(stack_info) = &self.stacks[kernel_stack_id] {
            self.setup_stack_protection(stack_info, vmm)?;
        }

        Ok(())
    }

    /// Allocate a new stack with specified protection
    pub fn allocate_stack(&mut self, protection: StackProtection, vmm: &mut VirtualMemoryManager) -> Result<usize, StackError> {
        // Find free stack slot
        let stack_id = self.find_free_stack_slot()?;

        // Calculate total size needed (guard + stack + guard)
        let total_size = (GUARD_PAGE_SIZE + STACK_SIZE + GUARD_PAGE_SIZE) as u64;

        // Allocate memory for stack + guard pages
        let base_address = self.next_stack_addr;
        self.next_stack_addr += total_size;

        // Calculate addresses
        let guard_bottom = base_address;
        let stack_base = base_address + GUARD_PAGE_SIZE as u64;
        let stack_top = stack_base + STACK_SIZE as u64;
        let guard_top = stack_top;

        // Create stack info
        let stack_info = StackInfo {
            stack_id,
            base_address: stack_base,
            top_address: stack_top,
            current_sp: stack_top,  // Stack grows downward
            size: STACK_SIZE as u64,
            protection,
            guard_bottom,
            guard_top,
            allocated: true,
            overflow_count: 0,
            max_usage: 0,
        };

        // Store stack info
        self.stacks[stack_id] = Some(stack_info);
        self.allocation_count += 1;

        // Setup memory protection
        self.setup_stack_protection(&stack_info, vmm)?;

        Ok(stack_id)
    }

    /// Deallocate a stack
    pub fn deallocate_stack(&mut self, stack_id: usize, vmm: &mut VirtualMemoryManager) -> Result<(), StackError> {
        if stack_id >= MAX_STACKS {
            return Err(StackError::InvalidStackId);
        }

        if let Some(stack_info) = &self.stacks[stack_id] {
            // Remove memory protection
            self.remove_stack_protection(stack_info, vmm)?;

            // Clear stack info
            self.stacks[stack_id] = None;
            self.allocation_count = self.allocation_count.saturating_sub(1);
        }

        Ok(())
    }

    /// Switch to a different stack
    pub fn switch_stack(&mut self, stack_id: usize) -> Result<u64, StackError> {
        if stack_id >= MAX_STACKS {
            return Err(StackError::InvalidStackId);
        }

        let stack_info = self.stacks[stack_id]
            .as_ref()
            .ok_or(StackError::InvalidStackId)?;

        let _old_stack_id = self.current_stack_id;
        self.current_stack_id = Some(stack_id);

        // Return new stack pointer
        Ok(stack_info.current_sp)
    }

    /// Get current stack information
    pub fn get_current_stack(&self) -> Option<&StackInfo> {
        self.current_stack_id
            .and_then(|id| self.stacks[id].as_ref())
    }

    /// Get stack information by ID
    pub fn get_stack_info(&self, stack_id: usize) -> Option<&StackInfo> {
        if stack_id >= MAX_STACKS {
            return None;
        }
        self.stacks[stack_id].as_ref()
    }

    /// Check if address is within stack bounds
    pub fn is_valid_stack_address(&self, address: u64, stack_id: usize) -> bool {
        if let Some(stack_info) = self.get_stack_info(stack_id) {
            address >= stack_info.base_address && address < stack_info.top_address
        } else {
            false
        }
    }

    /// Handle stack overflow detection
    pub fn handle_stack_overflow(&mut self, stack_id: usize, fault_address: u64) -> Result<(), StackError> {
        if let Some(stack_info) = &mut self.stacks[stack_id] {
            // Check if fault is in guard page
            if fault_address >= stack_info.guard_bottom && fault_address < stack_info.guard_bottom + GUARD_PAGE_SIZE as u64 {
                stack_info.overflow_count += 1;
                self.overflow_count += 1;
                return Err(StackError::StackOverflow);
            }

            // Check if fault is in top guard page
            if fault_address >= stack_info.guard_top && fault_address < stack_info.guard_top + GUARD_PAGE_SIZE as u64 {
                stack_info.overflow_count += 1;
                self.overflow_count += 1;
                return Err(StackError::StackUnderflow);
            }
        }

        Err(StackError::InvalidStackId)
    }

    /// Update stack usage statistics
    pub fn update_stack_usage(&mut self, stack_id: usize, current_sp: u64) {
        if let Some(stack_info) = &mut self.stacks[stack_id] {
            stack_info.current_sp = current_sp;
            
            // Calculate usage (stack grows downward)
            let usage = stack_info.top_address - current_sp;
            if usage > stack_info.max_usage as u64 {
                stack_info.max_usage = usage as usize;
            }
        }
    }

    /// Get stack manager statistics
    pub fn get_statistics(&self) -> StackManagerStats {
        let mut allocated_count = 0;
        let mut total_usage = 0;
        let mut max_usage = 0;

        for stack_info in self.stacks.iter().flatten() {
            allocated_count += 1;
            total_usage += stack_info.max_usage;
            if stack_info.max_usage > max_usage {
                max_usage = stack_info.max_usage;
            }
        }

        StackManagerStats {
            allocated_stacks: allocated_count,
            total_stacks: MAX_STACKS,
            allocation_count: self.allocation_count,
            overflow_count: self.overflow_count,
            total_usage,
            max_usage,
        }
    }

    /// Find free stack slot
    fn find_free_stack_slot(&self) -> Result<usize, StackError> {
        for (i, stack) in self.stacks.iter().enumerate() {
            if stack.is_none() {
                return Ok(i);
            }
        }
        Err(StackError::OutOfMemory)
    }

    /// Setup stack protection in MMU
    fn setup_stack_protection(&self, stack_info: &StackInfo, vmm: &mut VirtualMemoryManager) -> Result<(), StackError> {
        // Map bottom guard page (no access)
        vmm.map_region(
            stack_info.guard_bottom,
            stack_info.guard_bottom,
            GUARD_PAGE_SIZE as u64,
            MemoryAttribute::Normal,
            RegionType::KernelData,
            true,
        ).map_err(|_| StackError::AllocationFailed)?;

        // Map top guard page (no access)
        vmm.map_region(
            stack_info.guard_top,
            stack_info.guard_top,
            GUARD_PAGE_SIZE as u64,
            MemoryAttribute::Normal,
            RegionType::KernelData,
            true,
        ).map_err(|_| StackError::AllocationFailed)?;

        // Map stack with appropriate protection
        let region_type = if stack_info.protection.user_accessible { 
            RegionType::UserData 
        } else { 
            RegionType::KernelData 
        };
        
        vmm.map_region(
            stack_info.base_address,
            stack_info.base_address,
            STACK_SIZE as u64,
            MemoryAttribute::Normal,
            region_type,
            !stack_info.protection.user_accessible,
        ).map_err(|_| StackError::AllocationFailed)?;

        Ok(())
    }

    /// Remove stack protection from MMU
    fn remove_stack_protection(&self, stack_info: &StackInfo, vmm: &mut VirtualMemoryManager) -> Result<(), StackError> {
        // Unmap guard pages and stack
        let total_size = (GUARD_PAGE_SIZE + STACK_SIZE + GUARD_PAGE_SIZE) as u64;
        vmm.unmap_region(stack_info.guard_bottom, total_size)
            .map_err(|_| StackError::AllocationFailed)?;

        Ok(())
    }
}

/// Stack manager statistics
#[derive(Debug, Clone)]
pub struct StackManagerStats {
    pub allocated_stacks: usize,
    pub total_stacks: usize,
    pub allocation_count: usize,
    pub overflow_count: usize,
    pub total_usage: usize,
    pub max_usage: usize,
}

/// Global stack manager instance
static mut STACK_MANAGER: StackManager = StackManager::new();

/// Initialize the global stack manager
pub fn init_stack_manager() -> Result<(), StackError> {
    // Get the global VMM instance
    let vmm = crate::memory::mmu::get_virtual_memory_manager();
    
    unsafe {
        core::ptr::addr_of_mut!(STACK_MANAGER).as_mut().unwrap().init(vmm)
    }
}

/// Get reference to global stack manager
pub fn get_stack_manager() -> &'static mut StackManager {
    unsafe { core::ptr::addr_of_mut!(STACK_MANAGER).as_mut().unwrap() }
}

// Stack switching assembly functions (to be implemented in assembly)
extern "C" {
    pub fn switch_to_stack(new_sp: u64) -> u64;
    pub fn get_current_sp() -> u64;
    pub fn setup_el0_stack(stack_pointer: u64);
    pub fn setup_el1_stack(stack_pointer: u64);
}
