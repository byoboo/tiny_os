//! Dynamic Memory Management Module
//!
//! This module provides advanced dynamic memory management features including:
//! - Dynamic stack growth and shrinking
//! - Lazy page allocation
//! - Memory pressure handling
//! - Memory optimization and defragmentation
//! - Hardware-assisted context switching

use spin::Mutex;

use crate::memory::{mmu_exceptions::MmuFaultInfo, MemoryManager};

// Re-export module components
pub mod context;
pub mod lazy;
pub mod manager;
pub mod pressure;
pub mod stack;

// Re-export key types and functions for backward compatibility
pub use context::HardwareContextSwitcher;
pub use lazy::{LazyAllocationPolicy, LazyPage, LazyPageAllocator};
pub use manager::{DynamicMemoryManager, DynamicMemoryStats};
pub use pressure::{MemoryPressureHandler, OptimizationStrategy};
pub use stack::{DynamicStack, DynamicStackManager, PressureLevel, StackGrowthPolicy};

/// Global dynamic memory manager instance
static DYNAMIC_MEMORY_MANAGER: Mutex<Option<DynamicMemoryManager>> = Mutex::new(None);

/// Initialize the dynamic memory manager
pub fn init_dynamic_memory_manager() -> Result<(), &'static str> {
    *DYNAMIC_MEMORY_MANAGER.lock() = Some(DynamicMemoryManager::new());
    if let Some(manager) = DYNAMIC_MEMORY_MANAGER.lock().as_mut() {
        manager.init()?;
    }
    Ok(())
}

/// Execute a closure with the dynamic memory manager
pub fn with_dynamic_memory_manager<F, R>(f: F) -> Result<R, &'static str>
where
    F: FnOnce(&mut DynamicMemoryManager) -> R,
{
    let mut manager = DYNAMIC_MEMORY_MANAGER.lock();
    match manager.as_mut() {
        Some(m) => Ok(f(m)),
        None => Err("Dynamic memory manager not initialized"),
    }
}

/// Handle dynamic memory fault
pub fn handle_dynamic_memory_fault(
    fault_info: &MmuFaultInfo,
    memory_manager: &mut MemoryManager,
) -> Result<(), &'static str> {
    with_dynamic_memory_manager(|manager| manager.handle_dynamic_fault(fault_info, memory_manager))?
}

/// Check memory pressure
pub fn check_dynamic_memory_pressure(
    available_memory: usize,
) -> Result<PressureLevel, &'static str> {
    with_dynamic_memory_manager(|manager| manager.check_memory_pressure(available_memory))
}

/// Create a dynamic stack
pub fn create_dynamic_stack(
    base_address: u64,
    initial_size: usize,
    max_size: usize,
) -> Result<u32, &'static str> {
    with_dynamic_memory_manager(|manager| {
        manager.create_dynamic_stack(base_address, initial_size, max_size)
    })?
}

/// Add a lazy page
pub fn add_lazy_page(virtual_address: u64) -> Result<usize, &'static str> {
    with_dynamic_memory_manager(|manager| manager.add_lazy_page(virtual_address))?
}

/// Perform fast context switch
pub fn fast_context_switch(from_asid: u16, to_asid: u16) -> Result<(), &'static str> {
    with_dynamic_memory_manager(|manager| manager.fast_context_switch(from_asid, to_asid))?
}

/// Get dynamic memory statistics
pub fn get_dynamic_memory_stats() -> Result<DynamicMemoryStats, &'static str> {
    with_dynamic_memory_manager(|manager| manager.get_statistics().clone())
}

/// Check if dynamic memory management is enabled
pub fn is_dynamic_memory_enabled() -> bool {
    DYNAMIC_MEMORY_MANAGER
        .lock()
        .as_ref()
        .map_or(false, |m| m.is_enabled())
}

/// Enable dynamic memory management
pub fn enable_dynamic_memory() -> Result<(), &'static str> {
    with_dynamic_memory_manager(|manager| {
        manager.set_enabled(true);
    })
}

/// Disable dynamic memory management
pub fn disable_dynamic_memory() -> Result<(), &'static str> {
    with_dynamic_memory_manager(|manager| {
        manager.set_enabled(false);
    })
}
