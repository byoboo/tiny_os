//! User Page Table Management
//!
//! This module handles individual user page tables for processes, including
//! activation, deactivation, and hardware-level page table operations.

use core::sync::atomic::{AtomicU64, Ordering};

use crate::memory::{mmu::RegionType, PAGE_SIZE};
use super::vma::{VmaList, VmaType, VirtualMemoryArea};
use super::mapping::MemoryMapper;
use super::layout::is_user_space_address;

/// User space page table for a single process
#[derive(Debug, Clone, Copy)]
pub struct UserPageTable {
    /// Process ID that owns this page table
    pub process_id: usize,
    /// L0 page table physical address (TTBR0_EL1)
    pub l0_table_addr: u64,
    /// Virtual memory areas for this process
    pub vmas: VmaList,
    /// ASID (Address Space ID) for TLB management
    pub asid: u16,
    /// Whether this page table is currently active
    pub is_active: bool,
    /// Page table creation timestamp
    pub created_time: u64,
    /// Last access timestamp
    pub last_access_time: u64,
    /// Total mapped pages
    pub mapped_pages: usize,
    /// Total allocated virtual memory
    pub allocated_vm_size: u64,
}

impl UserPageTable {
    /// Create a new user page table
    pub fn new(process_id: usize, asid: u16) -> Result<Self, &'static str> {
        // Allocate L0 page table memory
        let l0_table_addr = Self::allocate_page_table_memory()?;

        Ok(Self {
            process_id,
            l0_table_addr,
            vmas: VmaList::new(),
            asid,
            is_active: false,
            created_time: 0, // Would use timer in real implementation
            last_access_time: 0,
            mapped_pages: 0,
            allocated_vm_size: 0,
        })
    }

    /// Allocate memory for page table (simplified)
    fn allocate_page_table_memory() -> Result<u64, &'static str> {
        // This is a simplified allocation - in reality would use memory manager
        // For now, return a dummy address aligned to page boundary
        static NEXT_PAGE_TABLE_ADDR: AtomicU64 = AtomicU64::new(0x8000_0000);
        let addr = NEXT_PAGE_TABLE_ADDR.fetch_add(PAGE_SIZE as u64, Ordering::SeqCst);
        Ok(addr)
    }

    /// Add a virtual memory area to this process
    pub fn add_vma(
        &mut self,
        start: u64,
        size: u64,
        vma_type: VmaType,
        permissions: RegionType,
    ) -> Result<usize, &'static str> {
        // Validate address range is in user space
        if !is_user_space_address(start, size) {
            return Err("Address outside user space");
        }

        // Align to page boundaries
        let aligned_start = MemoryMapper::align_to_page(start);
        let aligned_end = aligned_start + MemoryMapper::align_size_to_page(size);

        let vma = VirtualMemoryArea::new(aligned_start, aligned_end, vma_type, permissions);
        let index = self.vmas.add_vma(vma)?;

        self.allocated_vm_size += aligned_end - aligned_start;
        Ok(index)
    }

    /// Remove a virtual memory area
    pub fn remove_vma(&mut self, index: usize) -> Result<(), &'static str> {
        let vma = self.vmas.remove_vma(index)?;

        // Unmap pages if mapped
        if vma.is_mapped {
            self.mapped_pages -= vma.page_count();
        }

        self.allocated_vm_size -= vma.size();
        Ok(())
    }

    /// Map a virtual memory area to physical memory
    pub fn map_vma(&mut self, vma_index: usize, physical_addr: u64) -> Result<(), &'static str> {
        let vma = self.vmas.get_vma_mut(vma_index).ok_or("VMA not found")?;

        if vma.is_mapped {
            return Err("VMA already mapped");
        }

        // Map the VMA
        vma.map_to_physical(physical_addr)?;
        self.mapped_pages += vma.page_count();

        // Update page tables
        MemoryMapper::update_page_tables_for_vma(vma)?;

        Ok(())
    }

    /// Unmap a virtual memory area
    pub fn unmap_vma(&mut self, vma_index: usize) -> Result<(), &'static str> {
        let vma = self.vmas.get_vma_mut(vma_index).ok_or("VMA not found")?;

        if !vma.is_mapped {
            return Err("VMA not mapped");
        }

        self.mapped_pages -= vma.page_count();
        vma.unmap();

        // In a real implementation, would remove page table entries
        Ok(())
    }

    /// Activate this page table (switch TTBR0_EL1)
    pub fn activate(&mut self) -> Result<(), &'static str> {
        if self.is_active {
            return Ok(());
        }

        // Switch TTBR0_EL1 to this page table
        unsafe {
            // Set TTBR0_EL1 to our L0 table
            core::arch::asm!(
                "msr ttbr0_el1, {}",
                in(reg) self.l0_table_addr,
                options(nostack)
            );

            // Invalidate TLB for this ASID
            core::arch::asm!(
                "tlbi aside1, {}",
                in(reg) (self.asid as u64) << 48,
                options(nostack)
            );

            // DSB to ensure completion
            core::arch::asm!("dsb sy", options(nostack));
            core::arch::asm!("isb", options(nostack));
        }

        self.is_active = true;
        self.last_access_time = 0; // Would use timer
        Ok(())
    }

    /// Deactivate this page table
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Translate virtual address to physical address
    pub fn translate_address(&self, virtual_addr: u64) -> Option<u64> {
        // Find the VMA containing this address
        if let Some((_, vma)) = self.vmas.find_vma(virtual_addr) {
            MemoryMapper::translate_address_in_vma(vma, virtual_addr)
        } else {
            None
        }
    }

    /// Get page table statistics
    pub fn get_stats(&self) -> UserPageTableStats {
        UserPageTableStats {
            process_id: self.process_id,
            asid: self.asid,
            vma_count: self.vmas.len(),
            mapped_pages: self.mapped_pages,
            allocated_vm_size: self.allocated_vm_size,
            is_active: self.is_active,
            l0_table_addr: self.l0_table_addr,
        }
    }
}

/// Statistics for user page tables
#[derive(Debug, Clone, Copy)]
pub struct UserPageTableStats {
    pub process_id: usize,
    pub asid: u16,
    pub vma_count: usize,
    pub mapped_pages: usize,
    pub allocated_vm_size: u64,
    pub is_active: bool,
    pub l0_table_addr: u64,
}
