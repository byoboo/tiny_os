//! MMU Types Module
//!
//! This module defines the core types, constants, and enums used throughout
//! the Memory Management Unit implementation, including memory attributes,
//! page types, and region definitions.

/// Number of entries in a translation table (512 entries x 8 bytes = 4KB)
pub const TTBR_ENTRIES: usize = 512;

/// ARM64 page sizes and constants
pub const PAGE_SIZE: u32 = 4096; // 4KB pages
pub const PAGE_SHIFT: u32 = 12;
pub const PAGE_MASK: u32 = PAGE_SIZE - 1;

/// Translation table constants for 4KB granule
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
