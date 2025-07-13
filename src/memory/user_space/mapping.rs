//! Memory Mapping and Address Translation
//!
//! This module handles memory mapping operations and virtual-to-physical
//! address translation for user space processes.

use super::{
    layout::is_user_space_address,
    vma::{VirtualMemoryArea, VmaType},
};
use crate::memory::{
    mmu::{MemoryAttribute, PageTableEntry},
    PAGE_SIZE,
};

/// Memory mapping operations
pub struct MemoryMapper;

impl MemoryMapper {
    /// Update page tables for a VMA (simplified implementation)
    pub fn update_page_tables_for_vma(vma: &VirtualMemoryArea) -> Result<(), &'static str> {
        if let Some(phys_addr) = vma.physical_addr {
            let page_count = vma.page_count();

            for i in 0..page_count {
                let _virt_addr = vma.start_addr + (i as u64 * PAGE_SIZE as u64);
                let phys_page_addr = phys_addr + (i as u64 * PAGE_SIZE as u64);

                // Create page table entry (simplified for no_std)
                let _entry = PageTableEntry::new_page(
                    phys_page_addr,
                    MemoryAttribute::Normal,
                    vma.permissions,
                );

                // In reality, would install this entry in the page tables
            }

            Ok(())
        } else {
            Err("VMA not mapped to physical memory")
        }
    }

    /// Validate memory mapping request
    pub fn validate_mapping(start: u64, size: u64, vma_type: VmaType) -> Result<(), &'static str> {
        // Check address is in user space
        if !is_user_space_address(start, size) {
            return Err("Address outside user space");
        }

        // Check alignment
        if start % PAGE_SIZE as u64 != 0 {
            return Err("Start address not page-aligned");
        }

        // Check size
        if size == 0 {
            return Err("Invalid size");
        }

        // Type-specific validation
        match vma_type {
            VmaType::Code => {
                // Code should be executable but not writable
            }
            VmaType::Data | VmaType::Heap => {
                // Data and heap should be writable but not executable
            }
            VmaType::Stack => {
                // Stack should be writable, not executable, and grow down
            }
            VmaType::Shared | VmaType::MmapFile | VmaType::MmapAnon => {
                // Shared and mmap regions have flexible permissions
            }
        }

        Ok(())
    }

    /// Calculate physical address from virtual address within a VMA
    pub fn translate_address_in_vma(vma: &VirtualMemoryArea, virtual_addr: u64) -> Option<u64> {
        if !vma.contains(virtual_addr) {
            return None;
        }

        if let Some(phys_base) = vma.physical_addr {
            let offset = virtual_addr - vma.start_addr;
            Some(phys_base + offset)
        } else {
            None
        }
    }

    /// Align address to page boundary
    pub fn align_to_page(addr: u64) -> u64 {
        addr & !((PAGE_SIZE as u64) - 1)
    }

    /// Align size to page boundary (round up)
    pub fn align_size_to_page(size: u64) -> u64 {
        (size + PAGE_SIZE as u64 - 1) & !((PAGE_SIZE as u64) - 1)
    }
}

// Note: In a no_std environment, we avoid Vec and use fixed-size arrays or
// direct page table manipulation for memory efficiency
