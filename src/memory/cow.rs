//! Copy-on-Write (COW) Memory Management
//!
//! This module implements Phase 4.4.1 of the TinyOS Exception Enhancement Plan:
//! Copy-on-Write page tracking and management for memory sharing and
//! efficiency.
//!
//! # Features
//! - COW page metadata tracking
//! - Reference counting for shared pages
//! - COW fault handling integration
//! - Page duplication on write access
//! - COW statistics and monitoring
//! - Memory sharing between processes/stacks

use core::ptr::{read_volatile, write_volatile};

use spin::Mutex;

use crate::memory::{
    mmu::{RegionType, PAGE_SIZE},
    MemoryManager,
};

/// Maximum number of COW pages that can be tracked
const MAX_COW_PAGES: usize = 64;

/// Maximum number of virtual addresses per COW page
const MAX_VIRT_ADDRS_PER_PAGE: usize = 8;

/// Maximum number of process IDs per COW page
#[allow(dead_code)]
const MAX_PROCESS_IDS_PER_PAGE: usize = 4;

/// Simple array-based vector for no_std environment
#[derive(Debug)]
pub struct SimpleVec<T> {
    data: [T; MAX_VIRT_ADDRS_PER_PAGE],
    len: usize,
}

impl<T: Copy + Default> SimpleVec<T> {
    pub fn new() -> Self {
        Self {
            data: [T::default(); MAX_VIRT_ADDRS_PER_PAGE],
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.len < MAX_VIRT_ADDRS_PER_PAGE {
            self.data[self.len] = item;
            self.len += 1;
            Ok(())
        } else {
            Err("SimpleVec is full")
        }
    }

    pub fn remove(&mut self, index: usize) -> Result<T, &'static str> {
        if index >= self.len {
            return Err("Index out of bounds");
        }

        let item = self.data[index];
        for i in index..self.len - 1 {
            self.data[i] = self.data[i + 1];
        }
        self.len -= 1;
        Ok(item)
    }

    pub fn contains(&self, item: &T) -> bool
    where
        T: PartialEq,
    {
        for i in 0..self.len {
            if &self.data[i] == item {
                return true;
            }
        }
        false
    }

    pub fn iter(&self) -> SimpleVecIter<'_, T> {
        SimpleVecIter {
            data: &self.data,
            len: self.len,
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

/// Iterator for SimpleVec
pub struct SimpleVecIter<'a, T> {
    data: &'a [T; MAX_VIRT_ADDRS_PER_PAGE],
    len: usize,
    index: usize,
}

impl<'a, T> Iterator for SimpleVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = &self.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

/// Process ID array for COW pages
type ProcessIdArray = SimpleVec<usize>;

/// COW page metadata structure
#[derive(Debug)]
pub struct CowPage {
    /// Physical address of the page
    pub physical_addr: u64,
    /// Reference count for shared access
    pub ref_count: usize,
    /// Whether this page is currently COW-protected
    pub is_cow: bool,
    /// Original permissions before COW protection
    pub original_permissions: RegionType,
    /// Virtual addresses that map to this page
    pub virtual_addresses: SimpleVec<u64>,
    /// Process IDs that share this page
    pub process_ids: ProcessIdArray,
}

impl CowPage {
    /// Create a new COW page entry
    pub fn new(physical_addr: u64, permissions: RegionType) -> Self {
        Self {
            physical_addr,
            ref_count: 1,
            is_cow: false,
            original_permissions: permissions,
            virtual_addresses: SimpleVec::new(),
            process_ids: SimpleVec::new(),
        }
    }

    /// Increment reference count
    pub fn add_reference(
        &mut self,
        virtual_addr: u64,
        process_id: usize,
    ) -> Result<(), &'static str> {
        self.ref_count += 1;
        self.virtual_addresses.push(virtual_addr)?;
        if !self.process_ids.contains(&process_id) {
            self.process_ids.push(process_id)?;
        }
        Ok(())
    }

    /// Decrement reference count
    pub fn remove_reference(&mut self, virtual_addr: u64, process_id: usize) -> bool {
        if self.ref_count > 0 {
            self.ref_count -= 1;

            // Remove virtual address
            for i in 0..self.virtual_addresses.len() {
                if self.virtual_addresses.data[i] == virtual_addr {
                    let _ = self.virtual_addresses.remove(i);
                    break;
                }
            }

            // Remove process ID if no more virtual addresses for this process
            let process_has_mappings = self.virtual_addresses.iter().any(|&_addr| {
                // This would need process-specific address space checking
                // For now, assume simple removal
                false
            });

            if !process_has_mappings {
                for i in 0..self.process_ids.len() {
                    if self.process_ids.data[i] == process_id {
                        let _ = self.process_ids.remove(i);
                        break;
                    }
                }
            }
        }

        self.ref_count == 0
    }

    /// Check if page should be COW-protected
    pub fn should_be_cow(&self) -> bool {
        self.ref_count > 1
    }

    /// Convert to COW protection
    pub fn enable_cow(&mut self) {
        self.is_cow = true;
    }

    /// Disable COW protection
    pub fn disable_cow(&mut self) {
        self.is_cow = false;
    }
}

/// COW statistics for monitoring
#[derive(Debug, Default, Clone)]
pub struct CowStatistics {
    /// Total COW pages tracked
    pub cow_pages_count: usize,
    /// Total COW faults handled
    pub cow_faults_handled: usize,
    /// Total pages duplicated
    pub pages_duplicated: usize,
    /// Total memory saved through sharing
    pub memory_saved_bytes: u64,
    /// Total memory used for COW metadata
    pub metadata_memory_bytes: u64,
    /// Peak number of COW pages
    pub peak_cow_pages: usize,
}

impl CowStatistics {
    /// Update statistics after COW operation
    pub fn record_cow_fault(&mut self) {
        self.cow_faults_handled += 1;
    }

    /// Record page duplication
    pub fn record_page_duplication(&mut self) {
        self.pages_duplicated += 1;
    }

    /// Calculate memory savings
    pub fn calculate_memory_saved(&self) -> u64 {
        // Memory saved = (total references - actual pages) * page size
        // This is a simplified calculation
        self.memory_saved_bytes
    }

    /// Update peak COW pages
    pub fn update_peak_pages(&mut self, current_count: usize) {
        if current_count > self.peak_cow_pages {
            self.peak_cow_pages = current_count;
        }
    }
}

/// COW fault types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CowFaultType {
    /// Write access to COW-protected page
    WriteAccess,
    /// Read access to COW-protected page (should not happen)
    ReadAccess,
    /// Execute access to COW-protected page
    ExecuteAccess,
}

/// COW fault information
#[derive(Debug)]
pub struct CowFault {
    /// Virtual address that caused the fault
    pub virtual_address: u64,
    /// Physical address of the COW page
    pub physical_address: u64,
    /// Type of access that caused the fault
    pub fault_type: CowFaultType,
    /// Process ID that caused the fault
    pub process_id: usize,
    /// Current reference count of the page
    pub ref_count: usize,
}

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
    let mut guard = COW_MANAGER.lock();
    if let Some(ref mut manager) = *guard {
        Some(f(manager))
    } else {
        None
    }
}

/// Helper function to create COW fault from exception information
pub fn create_cow_fault_from_exception(
    virtual_addr: u64,
    physical_addr: u64,
    is_write: bool,
    process_id: usize,
) -> CowFault {
    let fault_type = if is_write {
        CowFaultType::WriteAccess
    } else {
        CowFaultType::ReadAccess
    };

    // Get reference count from COW manager
    let ref_count = with_cow_manager(|manager| {
        manager
            .get_cow_page_info(physical_addr)
            .map(|page| page.ref_count)
            .unwrap_or(0)
    })
    .unwrap_or(0);

    CowFault {
        virtual_address: virtual_addr,
        physical_address: physical_addr,
        fault_type,
        process_id,
        ref_count,
    }
}
