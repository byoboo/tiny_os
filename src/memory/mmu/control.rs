//! MMU Control Module
//!
//! This module provides global MMU control functions including initialization,
//! enable/disable operations, and address translation services.

use crate::memory::{
    layout::HEAP_END,
    mmu::vmm::{VirtualMemoryManager, VirtualMemoryStats},
};

/// Global virtual memory manager instance
static mut VIRTUAL_MEMORY_MANAGER: Option<VirtualMemoryManager> = None;

/// Initialize virtual memory management
pub fn init_virtual_memory() -> Result<(), &'static str> {
    unsafe {
        // Allocate page tables at end of heap
        let page_table_base = (HEAP_END - 0x10000) as u64; // Reserve 64KB for page tables

        let mut vmm = VirtualMemoryManager::new(page_table_base);
        vmm.init()?;

        VIRTUAL_MEMORY_MANAGER = Some(vmm);
    }

    Ok(())
}

/// Enable MMU globally
pub fn enable_mmu_global() -> Result<(), &'static str> {
    unsafe {
        if let Some(ref mut vmm) = VIRTUAL_MEMORY_MANAGER {
            vmm.enable_mmu()
        } else {
            Err("Virtual memory manager not initialized")
        }
    }
}

/// Disable MMU globally
pub fn disable_mmu_global() -> Result<(), &'static str> {
    unsafe {
        if let Some(ref mut vmm) = VIRTUAL_MEMORY_MANAGER {
            vmm.disable_mmu()
        } else {
            Err("Virtual memory manager not initialized")
        }
    }
}

/// Get virtual memory statistics
pub fn get_virtual_memory_stats() -> Option<VirtualMemoryStats> {
    unsafe {
        if let Some(ref vmm) = VIRTUAL_MEMORY_MANAGER {
            Some(vmm.get_stats())
        } else {
            None
        }
    }
}

/// Check if MMU is enabled globally
pub fn is_mmu_enabled_global() -> bool {
    unsafe {
        if let Some(ref vmm) = VIRTUAL_MEMORY_MANAGER {
            vmm.is_mmu_enabled()
        } else {
            false
        }
    }
}

/// Translate virtual address to physical address globally
pub fn translate_address_global(virt_addr: u64) -> Result<u64, &'static str> {
    unsafe {
        if let Some(ref vmm) = VIRTUAL_MEMORY_MANAGER {
            vmm.translate_address(virt_addr)
        } else {
            Err("Virtual memory manager not initialized")
        }
    }
}

/// Invalidate TLB globally
pub fn invalidate_tlb_global() {
    unsafe {
        if let Some(ref vmm) = VIRTUAL_MEMORY_MANAGER {
            vmm.invalidate_tlb();
        }
    }
}

/// Get mutable reference to virtual memory manager (for internal use)
pub fn get_virtual_memory_manager() -> &'static mut VirtualMemoryManager {
    unsafe {
        if let Some(ref mut vmm) = VIRTUAL_MEMORY_MANAGER {
            vmm
        } else {
            panic!("Virtual memory manager not initialized")
        }
    }
}
