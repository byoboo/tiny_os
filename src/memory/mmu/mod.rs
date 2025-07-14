//! MMU (Memory Management Unit) Module
//!
//! This module provides comprehensive Memory Management Unit functionality including:
//! - Page table entry management and translation tables
//! - Virtual memory manager with mapping/unmapping capabilities  
//! - MMU control operations and global functions
//! - ARM64-specific memory management features
//!
//! The module is organized into focused submodules:
//! - `types`: Core MMU types, enums, and constants
//! - `tables`: Page table entries and translation table management
//! - `vmm`: Virtual Memory Manager implementation
//! - `control`: Global MMU control and initialization functions

pub mod types;
pub mod tables;
pub mod vmm;
pub mod control;

// Re-export commonly used types and functions
pub use types::{MemoryAttribute, PageType, RegionType, TTBR_ENTRIES, PAGE_SIZE, PAGE_SHIFT, PAGE_MASK};
pub use tables::{PageTableEntry, TranslationTable};
pub use vmm::{VirtualMemoryManager, VirtualMemoryStats};
pub use control::{
    init_virtual_memory,
    enable_mmu_global,
    disable_mmu_global,
    get_virtual_memory_stats,
    is_mmu_enabled_global,
    translate_address_global,
    invalidate_tlb_global,
    get_virtual_memory_manager,
};
