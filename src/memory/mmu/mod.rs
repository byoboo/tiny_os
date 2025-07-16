//! MMU (Memory Management Unit) Module
//!
//! This module provides comprehensive Memory Management Unit functionality
//! including:
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

pub mod control;
pub mod tables;
pub mod types;
pub mod vmm;

// Re-export commonly used types and functions
pub use control::{
    disable_mmu_global, enable_mmu_global, get_virtual_memory_manager, get_virtual_memory_stats,
    init_virtual_memory, invalidate_tlb_global, is_mmu_enabled_global, translate_address_global,
};
pub use tables::{PageTableEntry, TranslationTable};
pub use types::{
    MemoryAttribute, PageType, RegionType, PAGE_MASK, PAGE_SHIFT, PAGE_SIZE, TTBR_ENTRIES,
};
pub use vmm::{VirtualMemoryManager, VirtualMemoryStats};
