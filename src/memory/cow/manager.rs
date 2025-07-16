//! COW Manager Module
//!
//! This module provides the main Copy-on-Write management functionality,
//! including page registration, fault handling, and memory allocation.

use core::ptr::{read_volatile, write_volatile};

use spin::Mutex;

use crate::memory::{
    cow::{
        fault::{CowFault, CowFaultType, CowStatistics},
        page::CowPage,
    },
    mmu::{RegionType, PAGE_SIZE},
    MemoryManager,
};

/// Maximum number of COW pages that can be tracked
const MAX_COW_PAGES: usize = 64;

/// COW Manager - handles all COW operations
pub struct CowManager {
    /// Array of COW pages (replacing HashMap)
    cow_pages: [Option<CowPage>; MAX_COW_PAGES],
    /// Number of active COW pages
    active_pages: usize,
    /// COW statistics
    statistics: CowStatistics,
    /// Memory manager for page allocation
    memory_manager: Option<*mut MemoryManager>,
    /// Next process ID for tracking
    next_process_id: usize,
}

// SAFETY: In a bare-metal environment, we don't have actual threads,
// so the raw pointer is safe to share across "threads" (interrupt contexts)
unsafe impl Send for CowManager {}
unsafe impl Sync for CowManager {}

impl CowManager {
    /// Create a new COW manager
    pub fn new() -> Self {
        Self {
            cow_pages: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
            ],
            active_pages: 0,
            statistics: CowStatistics::default(),
            memory_manager: None,
            next_process_id: 1,
        }
    }

    /// Initialize COW manager with memory manager reference
    pub fn init(&mut self, memory_manager: *mut MemoryManager) {
        self.memory_manager = Some(memory_manager);
    }

    /// Find a COW page by physical address
    fn find_cow_page(&self, physical_addr: u64) -> Option<usize> {
        let aligned_phys = physical_addr & !((PAGE_SIZE as u64) - 1);
        for i in 0..MAX_COW_PAGES {
            if let Some(ref page) = self.cow_pages[i] {
                if page.physical_addr == aligned_phys {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Find an empty slot for a new COW page
    fn find_empty_slot(&self) -> Option<usize> {
        for i in 0..MAX_COW_PAGES {
            if self.cow_pages[i].is_none() {
                return Some(i);
            }
        }
        None
    }

    /// Register a page for COW tracking
    pub fn register_page(
        &mut self,
        physical_addr: u64,
        virtual_addr: u64,
        permissions: RegionType,
        process_id: usize,
    ) -> Result<(), &'static str> {
        // Align addresses to page boundaries
        let aligned_phys = physical_addr & !((PAGE_SIZE as u64) - 1);
        let aligned_virt = virtual_addr & !((PAGE_SIZE as u64) - 1);

        if let Some(index) = self.find_cow_page(aligned_phys) {
            // Page already exists, add reference
            if let Some(ref mut cow_page) = self.cow_pages[index] {
                cow_page.add_reference(aligned_virt, process_id)?;

                // Enable COW protection if multiple references
                if cow_page.should_be_cow() {
                    cow_page.enable_cow();
                }
            }
        } else {
            // Create new COW page entry
            if let Some(slot) = self.find_empty_slot() {
                let mut cow_page = CowPage::new(aligned_phys, permissions);
                cow_page.add_reference(aligned_virt, process_id)?;
                self.cow_pages[slot] = Some(cow_page);
                self.active_pages += 1;
            } else {
                return Err("COW page table full");
            }
        }

        // Update statistics
        self.statistics.cow_pages_count = self.active_pages;
        self.statistics.update_peak_pages(self.active_pages);

        Ok(())
    }

    /// Unregister a page from COW tracking
    pub fn unregister_page(
        &mut self,
        physical_addr: u64,
        virtual_addr: u64,
        process_id: usize,
    ) -> Result<bool, &'static str> {
        let aligned_phys = physical_addr & !((PAGE_SIZE as u64) - 1);
        let aligned_virt = virtual_addr & !((PAGE_SIZE as u64) - 1);

        if let Some(index) = self.find_cow_page(aligned_phys) {
            if let Some(ref mut cow_page) = self.cow_pages[index] {
                let should_remove = cow_page.remove_reference(aligned_virt, process_id);

                if should_remove {
                    self.cow_pages[index] = None;
                    self.active_pages -= 1;
                    self.statistics.cow_pages_count = self.active_pages;
                    return Ok(true);
                } else if !cow_page.should_be_cow() {
                    // Disable COW protection if only one reference left
                    cow_page.disable_cow();
                }
            }
        }

        Ok(false)
    }

    /// Handle COW fault - duplicate page and update mappings
    pub fn handle_cow_fault(&mut self, fault: CowFault) -> Result<u64, &'static str> {
        let aligned_phys = fault.physical_address & !((PAGE_SIZE as u64) - 1);

        // Get COW page metadata
        let cow_page_index = self
            .find_cow_page(aligned_phys)
            .ok_or("COW page not found")?;

        // Extract original permissions before any mutations
        let original_permissions = {
            let cow_page = self.cow_pages[cow_page_index]
                .as_ref()
                .ok_or("COW page not found")?;

            // Verify this is a COW fault
            if !cow_page.is_cow {
                return Err("Page is not COW-protected");
            }

            // Only handle write faults for COW
            if fault.fault_type != CowFaultType::WriteAccess {
                return Err("COW fault on non-write access");
            }

            cow_page.original_permissions
        };

        // Allocate new page for the writing process
        let new_page = self.allocate_new_page()?;

        // Copy page content
        self.copy_page_content(aligned_phys, new_page)?;

        // Update COW tracking
        self.unregister_page(aligned_phys, fault.virtual_address, fault.process_id)?;
        self.register_page(
            new_page,
            fault.virtual_address,
            original_permissions,
            fault.process_id,
        )?;

        // Update statistics
        self.statistics.record_cow_fault();
        self.statistics.record_page_duplication();

        Ok(new_page)
    }

    /// Check if a page is COW-protected
    pub fn is_cow_protected(&self, physical_addr: u64) -> bool {
        let aligned_phys = physical_addr & !((PAGE_SIZE as u64) - 1);
        if let Some(index) = self.find_cow_page(aligned_phys) {
            if let Some(ref page) = self.cow_pages[index] {
                return page.is_cow;
            }
        }
        false
    }

    /// Get COW page information
    pub fn get_cow_page_info(&self, physical_addr: u64) -> Option<&CowPage> {
        let aligned_phys = physical_addr & !((PAGE_SIZE as u64) - 1);
        if let Some(index) = self.find_cow_page(aligned_phys) {
            if let Some(ref page) = self.cow_pages[index] {
                return Some(page);
            }
        }
        None
    }

    /// Get COW statistics
    pub fn get_statistics(&self) -> &CowStatistics {
        &self.statistics
    }

    /// Get next process ID
    pub fn get_next_process_id(&mut self) -> usize {
        let id = self.next_process_id;
        self.next_process_id += 1;
        id
    }

    /// Allocate a new page for COW duplication
    fn allocate_new_page(&mut self) -> Result<u64, &'static str> {
        if let Some(memory_manager) = self.memory_manager {
            unsafe {
                if let Some(block_addr) = (*memory_manager).allocate_block() {
                    Ok(block_addr as u64)
                } else {
                    Err("Failed to allocate new page for COW")
                }
            }
        } else {
            Err("Memory manager not initialized")
        }
    }

    /// Copy content from one page to another
    fn copy_page_content(&self, source_phys: u64, dest_phys: u64) -> Result<(), &'static str> {
        // Safety: We're copying between valid physical addresses
        unsafe {
            let source_ptr = source_phys as *const u8;
            let dest_ptr = dest_phys as *mut u8;

            // Copy page content in chunks for better performance
            for i in 0..(PAGE_SIZE / 8) {
                let offset = i * 8;
                let src_u64 = read_volatile(source_ptr.add(offset as usize) as *const u64);
                write_volatile(dest_ptr.add(offset as usize) as *mut u64, src_u64);
            }
        }

        Ok(())
    }

    /// Create a COW mapping between two virtual addresses
    pub fn create_cow_mapping(
        &mut self,
        source_virt: u64,
        dest_virt: u64,
        source_process: usize,
        dest_process: usize,
        physical_addr: u64,
        permissions: RegionType,
    ) -> Result<(), &'static str> {
        // Register both virtual addresses to the same physical page
        self.register_page(physical_addr, source_virt, permissions, source_process)?;
        self.register_page(physical_addr, dest_virt, permissions, dest_process)?;

        Ok(())
    }

    /// Get all COW pages for debugging
    pub fn get_all_cow_pages(&self) -> [(u64, Option<&CowPage>); MAX_COW_PAGES] {
        let mut result = [(0u64, None); MAX_COW_PAGES];
        for i in 0..MAX_COW_PAGES {
            if let Some(ref page) = self.cow_pages[i] {
                result[i] = (page.physical_addr, Some(page));
            }
        }
        result
    }

    /// Force COW protection on a page
    pub fn force_cow_protection(&mut self, physical_addr: u64) -> Result<(), &'static str> {
        let aligned_phys = physical_addr & !((PAGE_SIZE as u64) - 1);

        if let Some(index) = self.find_cow_page(aligned_phys) {
            if let Some(ref mut cow_page) = self.cow_pages[index] {
                cow_page.enable_cow();
                return Ok(());
            }
        }
        Err("Page not found in COW tracking")
    }

    /// Remove COW protection from a page
    pub fn remove_cow_protection(&mut self, physical_addr: u64) -> Result<(), &'static str> {
        let aligned_phys = physical_addr & !((PAGE_SIZE as u64) - 1);

        if let Some(index) = self.find_cow_page(aligned_phys) {
            if let Some(ref mut cow_page) = self.cow_pages[index] {
                cow_page.disable_cow();
                return Ok(());
            }
        }
        Err("Page not found in COW tracking")
    }
}

/// Global COW manager instance
static COW_MANAGER: Mutex<Option<CowManager>> = Mutex::new(None);

/// Initialize global COW manager
pub fn init_cow_manager(memory_manager: *mut MemoryManager) {
    let mut cow_manager = CowManager::new();
    cow_manager.init(memory_manager);
    *COW_MANAGER.lock() = Some(cow_manager);
}

/// Execute operation with COW manager if available
pub fn with_cow_manager<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut CowManager) -> R,
{
    if let Some(ref mut manager) = *COW_MANAGER.lock() {
        Some(f(manager))
    } else {
        None
    }
}
