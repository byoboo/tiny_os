//! Virtual Memory Manager Module
//!
//! This module implements the core Virtual Memory Manager (VMM) that handles
//! memory mapping, unmapping, address translation, and MMU control operations.

use crate::memory::layout::{HEAP_END, HEAP_START, KERNEL_END, KERNEL_START};
use crate::memory::mmu::types::{MemoryAttribute, PageType, RegionType};
use crate::memory::mmu::tables::{PageTableEntry, TranslationTable};

/// Size of L1 translation table (4KB)
const L1_TABLE_SIZE: usize = 4096;

/// Virtual Memory Manager structure
pub struct VirtualMemoryManager {
    /// Level 1 translation table (TTBR1_EL1 - kernel space)
    l1_kernel_table: TranslationTable,
    /// Level 1 translation table (TTBR0_EL1 - user space)  
    l1_user_table: TranslationTable,
    /// Base address for page table allocations
    #[allow(dead_code)]
    page_table_base: u64,
    /// Next available page table address
    next_table_addr: u64,
    /// MMU enabled state
    mmu_enabled: bool,
}

impl VirtualMemoryManager {
    /// Create a new virtual memory manager
    pub fn new(page_table_base: u64) -> Self {
        Self {
            l1_kernel_table: TranslationTable::new(page_table_base, 1),
            l1_user_table: TranslationTable::new(page_table_base + L1_TABLE_SIZE as u64, 1),
            page_table_base,
            next_table_addr: page_table_base + (L1_TABLE_SIZE * 2) as u64,
            mmu_enabled: false,
        }
    }

    /// Initialize the virtual memory system
    pub fn init(&mut self) -> Result<(), &'static str> {
        // Clear both L1 tables
        self.l1_kernel_table.clear();
        self.l1_user_table.clear();

        // Write tables to memory
        self.l1_kernel_table.write_to_memory()?;
        self.l1_user_table.write_to_memory()?;

        // Set up initial kernel mappings
        self.setup_kernel_mappings()?;

        Ok(())
    }

    /// Set up initial kernel space mappings
    fn setup_kernel_mappings(&mut self) -> Result<(), &'static str> {
        // Map kernel code (identity mapping)
        self.map_region(
            KERNEL_START as u64,
            KERNEL_START as u64,
            (KERNEL_END - KERNEL_START) as u64,
            MemoryAttribute::Normal,
            RegionType::KernelCode,
            true, // kernel space
        )?;

        // Map kernel heap
        self.map_region(
            HEAP_START as u64,
            HEAP_START as u64,
            (HEAP_END - HEAP_START) as u64,
            MemoryAttribute::Normal,
            RegionType::KernelData,
            true, // kernel space
        )?;

        // Map peripheral space (for UART, GPIO, etc.)
        self.map_region(
            0xFE000000, // BCM2835 peripheral base
            0xFE000000,
            0x01000000, // 16MB peripheral space
            MemoryAttribute::Device,
            RegionType::Device,
            true, // kernel space
        )?;

        Ok(())
    }

    /// Map a virtual memory region
    pub fn map_region(
        &mut self,
        virt_addr: u64,
        phys_addr: u64,
        size: u64,
        attr: MemoryAttribute,
        region_type: RegionType,
        is_kernel: bool,
    ) -> Result<(), &'static str> {
        // For now, implement simple 2MB block mappings
        // This can be enhanced later for 4KB page granularity

        let table = if is_kernel {
            &mut self.l1_kernel_table
        } else {
            &mut self.l1_user_table
        };

        // Calculate number of 2MB blocks needed
        let block_size = 2 * 1024 * 1024; // 2MB
        let blocks_needed = (size + block_size - 1) / block_size;

        for i in 0..blocks_needed {
            let va = virt_addr + (i * block_size);
            let pa = phys_addr + (i * block_size);

            // Calculate L1 index (bits [30:21] for 2MB blocks)
            let l1_index = ((va >> 21) & 0x1FF) as usize;

            // Create block entry
            let entry = PageTableEntry::new_block(pa, attr, region_type);
            table.set_entry(l1_index, entry)?;
        }

        // Write updated table to memory
        table.write_to_memory()?;
        Ok(())
    }

    /// Unmap a virtual memory region
    pub fn unmap_region(&mut self, virt_addr: u64, size: u64) -> Result<(), &'static str> {
        // For 2MB blocks, calculate number of blocks needed
        let block_size = 2 * 1024 * 1024; // 2MB
        let blocks_needed = (size + block_size - 1) / block_size;

        for i in 0..blocks_needed {
            let va = virt_addr + (i * block_size);

            // Calculate L1 index (bits [30:21] for 2MB blocks)
            let l1_index = ((va >> 21) & 0x1FF) as usize;

            // Determine which table to use
            let is_kernel_addr = (va & (1u64 << 63)) != 0;
            let table = if is_kernel_addr {
                &mut self.l1_kernel_table
            } else {
                &mut self.l1_user_table
            };

            // Invalidate entry
            let invalid_entry = PageTableEntry::new();
            table.set_entry(l1_index, invalid_entry)?;
        }

        // Write updated tables to memory
        self.l1_kernel_table.write_to_memory()?;
        self.l1_user_table.write_to_memory()?;

        // Invalidate TLB
        self.invalidate_tlb();

        Ok(())
    }

    /// Enable the MMU
    pub fn enable_mmu(&mut self) -> Result<(), &'static str> {
        if self.mmu_enabled {
            return Ok(());
        }

        unsafe {
            // Set up MAIR_EL1 (Memory Attribute Indirection Register)
            let mair_value = (MemoryAttribute::Normal as u64) |          // Index 0: Normal memory
                ((MemoryAttribute::Device as u64) << 8) |   // Index 1: Device memory
                ((MemoryAttribute::NormalNC as u64) << 16); // Index 2: Normal non-cacheable

            core::arch::asm!(
                "msr mair_el1, {}",
                in(reg) mair_value
            );

            // Set up TCR_EL1 (Translation Control Register)
            let tcr_value = (16_u64 << 0) |  // T0SZ: 48-bit virtual address space (64-16=48)
                (16_u64 << 16) | // T1SZ: 48-bit virtual address space  
                (2_u64 << 30) |  // TG1: 4KB granule for TTBR1_EL1
                (2_u64 << 14) |  // TG0: 4KB granule for TTBR0_EL1
                (3_u64 << 32) |  // IPS: 48-bit physical address space
                (1_u64 << 23) |  // EPD1: Enable TTBR1_EL1 walks
                (0_u64 << 7); // EPD0: Enable TTBR0_EL1 walks

            core::arch::asm!(
                "msr tcr_el1, {}",
                in(reg) tcr_value
            );

            // Set TTBR1_EL1 (kernel space)
            core::arch::asm!(
                "msr ttbr1_el1, {}",
                in(reg) self.l1_kernel_table.phys_addr()
            );

            // Set TTBR0_EL1 (user space)
            core::arch::asm!(
                "msr ttbr0_el1, {}",
                in(reg) self.l1_user_table.phys_addr()
            );

            // Ensure all instructions are completed
            core::arch::asm!("dsb sy");
            core::arch::asm!("isb");

            // Enable MMU in SCTLR_EL1
            let mut sctlr: u64;
            core::arch::asm!(
                "mrs {}, sctlr_el1",
                out(reg) sctlr
            );
            sctlr |= 1; // Set M bit (MMU enable)
            core::arch::asm!(
                "msr sctlr_el1, {}",
                in(reg) sctlr
            );

            // Ensure MMU is enabled
            core::arch::asm!("dsb sy");
            core::arch::asm!("isb");
        }

        self.mmu_enabled = true;
        Ok(())
    }

    /// Disable the MMU
    pub fn disable_mmu(&mut self) -> Result<(), &'static str> {
        if !self.mmu_enabled {
            return Ok(());
        }

        unsafe {
            // Disable MMU in SCTLR_EL1
            let mut sctlr: u64;
            core::arch::asm!(
                "mrs {}, sctlr_el1",
                out(reg) sctlr
            );
            sctlr &= !1; // Clear M bit (MMU disable)
            core::arch::asm!(
                "msr sctlr_el1, {}",
                in(reg) sctlr
            );

            // Ensure MMU is disabled
            core::arch::asm!("dsb sy");
            core::arch::asm!("isb");
        }

        self.mmu_enabled = false;
        Ok(())
    }

    /// Invalidate TLB
    pub fn invalidate_tlb(&self) {
        unsafe {
            // Invalidate all TLB entries
            core::arch::asm!("tlbi alle1");
            core::arch::asm!("dsb sy");
            core::arch::asm!("isb");
        }
    }

    /// Translate virtual address to physical address
    pub fn translate_address(&self, virt_addr: u64) -> Result<u64, &'static str> {
        if !self.mmu_enabled {
            // If MMU is disabled, virtual == physical
            return Ok(virt_addr);
        }

        // Determine which table to use based on address
        let is_kernel_addr = (virt_addr & (1u64 << 63)) != 0;
        let table = if is_kernel_addr {
            &self.l1_kernel_table
        } else {
            &self.l1_user_table
        };

        // For 2MB blocks, extract L1 index
        let l1_index = ((virt_addr >> 21) & 0x1FF) as usize;

        if let Some(entry) = table.get_entry(l1_index) {
            if entry.is_valid() && entry.get_type() == PageType::Block {
                // Extract physical base address and add offset
                let block_base = entry.get_phys_addr();
                let offset = virt_addr & 0x1FFFFF; // Lower 21 bits
                return Ok(block_base + offset);
            }
        }

        Err("Address translation failed")
    }

    /// Check if MMU is enabled
    pub fn is_mmu_enabled(&self) -> bool {
        self.mmu_enabled
    }

    /// Get MMU statistics
    pub fn get_stats(&self) -> VirtualMemoryStats {
        VirtualMemoryStats {
            mmu_enabled: self.mmu_enabled,
            kernel_table_addr: self.l1_kernel_table.phys_addr(),
            user_table_addr: self.l1_user_table.phys_addr(),
            next_table_addr: self.next_table_addr,
        }
    }
}

/// Virtual memory statistics
#[derive(Debug, Clone)]
pub struct VirtualMemoryStats {
    pub mmu_enabled: bool,
    pub kernel_table_addr: u64,
    pub user_table_addr: u64,
    pub next_table_addr: u64,
}
