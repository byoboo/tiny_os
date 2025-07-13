//! User Space Memory Management Module
//!
//! This module implements user space page table management for user space
//! memory isolation.
//!
//! # Features
//! - Per-process page table creation and management
//! - User space memory isolation between processes
//! - Context switching with page table updates
//! - Address space layout randomization (ASLR) foundation
//! - Memory mapping for user processes
//! - Page table lifecycle management

use spin::Mutex;

use crate::memory::MemoryManager;

// Re-export module components
pub mod layout;
pub mod manager;
pub mod mapping;
pub mod page_table;
pub mod vma;

// Re-export key types and functions for backward compatibility
pub use layout::{
    create_standard_vmas, is_kernel_space_address, is_user_space_address, KERNEL_SPACE_END,
    KERNEL_SPACE_START, USER_SPACE_END, USER_SPACE_START,
};
pub use manager::{UserSpaceManager, UserSpaceStats, MAX_USER_PROCESSES};
pub use mapping::MemoryMapper;
pub use page_table::{UserPageTable, UserPageTableStats};
pub use vma::{VirtualMemoryArea, VmaList, VmaType};

/// Global user space manager instance
static USER_SPACE_MANAGER: Mutex<Option<UserSpaceManager>> = Mutex::new(None);

/// Initialize global user space manager
pub fn init_user_space_manager(memory_manager: *mut MemoryManager) {
    let mut manager = UserSpaceManager::new();
    manager.init(memory_manager);
    *USER_SPACE_MANAGER.lock() = Some(manager);
}

/// Execute operation with user space manager if available
pub fn with_user_space_manager<F, R>(f: F) -> Result<R, &'static str>
where
    F: FnOnce(&mut UserSpaceManager) -> R,
{
    let mut manager = USER_SPACE_MANAGER.lock();
    match manager.as_mut() {
        Some(m) => Ok(f(m)),
        None => Err("User space manager not initialized"),
    }
}

/// Helper function to create standard user process memory layout
pub fn create_standard_user_layout(process_id: usize) -> Result<usize, &'static str> {
    with_user_space_manager(|manager| {
        // Create page table
        let slot = manager.create_page_table(process_id)?;

        // Add standard VMAs using the layout module
        let standard_vmas = layout::create_standard_vmas();

        if let Some(page_table) = manager.get_page_table_mut(slot) {
            for vma in standard_vmas.iter() {
                let size = vma.end_addr - vma.start_addr;
                page_table.add_vma(vma.start_addr, size, vma.vma_type, vma.permissions)?;
            }
        }

        Ok(slot)
    })?
}
