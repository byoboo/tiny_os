//! MMU Tables Module
//!
//! This module implements page table entry structures and translation table
//! management for ARM64 MMU, including entry creation, validation, and
//! memory synchronization.

use core::ptr::{read_volatile, write_volatile};

use crate::memory::mmu::types::{MemoryAttribute, PageType, RegionType, TTBR_ENTRIES};

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

    /// Get the physical address of this table
    pub fn phys_addr(&self) -> u64 {
        self.phys_addr
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
