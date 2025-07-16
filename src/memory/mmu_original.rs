//! ARM64 MMU and Virtual Memory Management
//!
//! This module implements Phase 4.2 of the TinyOS Exception Enhancement Plan:
//! Virtual Memory Support with page table management and address translation.
//!
//! # Features
//! - ARM64 page table structures and management
//! - Virtual-to-physical address translation
//! - Memory mapping system for kernel and user space
//! - TLB management integration
//! - Copy-on-write preparation

use core::ptr::{read_volatile, write_volatile};

use crate::memory::layout::{HEAP_END, HEAP_START, KERNEL_END, KERNEL_START};

/// ARM64 page sizes and constants
pub const PAGE_SIZE: u32 = 4096; // 4KB pages
pub const PAGE_SHIFT: u32 = 12;
pub const PAGE_MASK: u32 = PAGE_SIZE - 1;

/// Translation table constants for 4KB granule
pub const TTBR_ENTRIES: usize = 512; // 512 entries per table (9 bits)
pub const L1_TABLE_SIZE: usize = TTBR_ENTRIES * 8; // 8 bytes per entry
pub const L2_TABLE_SIZE: usize = TTBR_ENTRIES * 8;
pub const L3_TABLE_SIZE: usize = TTBR_ENTRIES * 8;

/// ARM64 Memory Attributes
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryAttribute {
    /// Normal memory, write-back cacheable
    Normal = 0b11111111,
    /// Device memory, non-cacheable
    Device = 0b00000000,
    /// Normal memory, non-cacheable
    NormalNC = 0b01000100,
}

/// Page table entry types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PageType {
    /// Invalid entry
    Invalid = 0b00,
    /// Block entry (1GB or 2MB)
    Block = 0b01,
    /// Table entry (points to next level) / Page entry (4KB)
    TableOrPage = 0b11,
}

/// Virtual memory region types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegionType {
    /// Kernel code (read-only, executable)
    KernelCode,
    /// Kernel data (read-write, non-executable)
    KernelData,
    /// User code (read-only, executable, user accessible)
    UserCode,
    /// User data (read-write, non-executable, user accessible)
    UserData,
    /// Device memory (non-cacheable, non-executable)
    Device,
    /// Shared memory between kernel and user
    Shared,
}

/// Page table entry structure
#[derive(Debug, Clone, Copy)]
pub struct PageTableEntry {
    pub raw: u64,
}

impl PageTableEntry {
    /// Create a new invalid page table entry
    pub const fn new() -> Self {
        Self { raw: 0 }
    }

    /// Create a block entry
    pub fn new_block(phys_addr: u64, attr: MemoryAttribute, region_type: RegionType) -> Self {
        let mut entry = phys_addr & 0xFFFFFFFFF000; // Clear lower 12 bits
        entry |= PageType::Block as u64;
        entry |= Self::get_access_permissions(region_type);
        entry |= Self::get_memory_attributes(attr);
        entry |= 1 << 10; // Access flag
        Self { raw: entry }
    }

    /// Create a table entry pointing to next level
    pub fn new_table(next_table_addr: u64) -> Self {
        let mut entry = next_table_addr & 0xFFFFFFFFF000; // Clear lower 12 bits
        entry |= PageType::TableOrPage as u64;
        entry |= 1 << 10; // Access flag
        Self { raw: entry }
    }

    /// Create a page entry (4KB)
    pub fn new_page(phys_addr: u64, attr: MemoryAttribute, region_type: RegionType) -> Self {
        let mut entry = phys_addr & 0xFFFFFFFFF000; // Clear lower 12 bits
        entry |= PageType::TableOrPage as u64;
        entry |= Self::get_access_permissions(region_type);
        entry |= Self::get_memory_attributes(attr);
        entry |= 1 << 10; // Access flag
        Self { raw: entry }
    }

    /// Check if entry is valid
    pub fn is_valid(&self) -> bool {
        (self.raw & 0x1) != 0
    }

    /// Get the type of this entry
    pub fn get_type(&self) -> PageType {
        match self.raw & 0x3 {
            0b00 => PageType::Invalid,
            0b01 => PageType::Block,
            0b11 => {
                // Could be Table or Page - check context
                // For now, assume TableOrPage at levels 0-2, Page at level 3
                PageType::TableOrPage
            }
            _ => PageType::Invalid,
        }
    }

    /// Get physical address from entry
    pub fn get_phys_addr(&self) -> u64 {
        self.raw & 0xFFFFFFFFF000
    }

    /// Get access permissions for region type
    fn get_access_permissions(region_type: RegionType) -> u64 {
        match region_type {
            RegionType::KernelCode => {
                // EL1 read-only, not accessible to EL0
                0 << 6 // AP[2:1] = 00 (read-write EL1)
            }
            RegionType::KernelData => {
                // EL1 read-write, not accessible to EL0
                0 << 6 // AP[2:1] = 00 (read-write EL1)
            }
            RegionType::UserCode => {
                // EL0/EL1 read-only
                (0b10 << 6) | (1 << 54) // AP[2:1] = 10 (read-only), UXN=0
            }
            RegionType::UserData => {
                // EL0/EL1 read-write, not executable
                (0b01 << 6) | (1 << 54) // AP[2:1] = 01 (read-write), UXN=1
            }
            RegionType::Device => {
                // EL1 read-write, not executable, not accessible to EL0
                (1 << 53) | (1 << 54) // PXN=1, UXN=1
            }
            RegionType::Shared => {
                // EL0/EL1 read-write, not executable
                (0b01 << 6) | (1 << 54) // AP[2:1] = 01 (read-write), UXN=1
            }
        }
    }

    /// Get memory attributes
    fn get_memory_attributes(attr: MemoryAttribute) -> u64 {
        // MAIR index in bits [4:2]
        match attr {
            MemoryAttribute::Normal => 0 << 2,   // MAIR index 0
            MemoryAttribute::Device => 1 << 2,   // MAIR index 1
            MemoryAttribute::NormalNC => 2 << 2, // MAIR index 2
        }
    }
}

/// ARM64 Translation Table
pub struct TranslationTable {
    /// Table entries (512 x 8 bytes = 4KB)
    entries: [PageTableEntry; TTBR_ENTRIES],
    /// Physical address of this table
    phys_addr: u64,
    /// Level of this table (0, 1, 2, or 3)
    #[allow(dead_code)]
    level: u8,
}

impl TranslationTable {
    /// Create a new translation table
    pub fn new(phys_addr: u64, level: u8) -> Self {
        Self {
            entries: [PageTableEntry::new(); TTBR_ENTRIES],
            phys_addr,
            level,
        }
    }

    /// Get entry at index
    pub fn get_entry(&self, index: usize) -> Option<&PageTableEntry> {
        self.entries.get(index)
    }

    /// Set entry at index
    pub fn set_entry(&mut self, index: usize, entry: PageTableEntry) -> Result<(), &'static str> {
        if index >= TTBR_ENTRIES {
            return Err("Invalid table index");
        }
        self.entries[index] = entry;
        Ok(())
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        for entry in &mut self.entries {
            *entry = PageTableEntry::new();
        }
    }

    /// Write table to memory
    pub fn write_to_memory(&self) -> Result<(), &'static str> {
        unsafe {
            let table_ptr = self.phys_addr as *mut u64;
            for (i, entry) in self.entries.iter().enumerate() {
                write_volatile(table_ptr.add(i), entry.raw);
            }
        }
        Ok(())
    }

    /// Read table from memory
    pub fn read_from_memory(&mut self) -> Result<(), &'static str> {
        unsafe {
            let table_ptr = self.phys_addr as *const u64;
            for (i, entry) in self.entries.iter_mut().enumerate() {
                entry.raw = read_volatile(table_ptr.add(i));
            }
        }
        Ok(())
    }
}

/// Virtual Memory Manager
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
                in(reg) self.l1_kernel_table.phys_addr
            );

            // Set TTBR0_EL1 (user space)
            core::arch::asm!(
                "msr ttbr0_el1, {}",
                in(reg) self.l1_user_table.phys_addr
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
            kernel_table_addr: self.l1_kernel_table.phys_addr,
            user_table_addr: self.l1_user_table.phys_addr,
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
