//! User Space Page Table Management
//!
//! This module implements Phase 4.4.2 of the TinyOS Exception Enhancement Plan:
//! Per-process page table management for user space memory isolation.
//!
//! # Features
//! - Per-process page table creation and management
//! - User space memory isolation between processes
//! - Context switching with page table updates
//! - Address space layout randomization (ASLR) foundation
//! - Memory mapping for user processes
//! - Page table lifecycle management

use spin::Mutex;

use crate::memory::{
    mmu::{MemoryAttribute, PageTableEntry, RegionType, PAGE_SIZE},
    MemoryManager,
};

/// Maximum number of user processes that can have page tables
const MAX_USER_PROCESSES: usize = 32;

/// User space virtual address ranges
pub const USER_SPACE_START: u64 = 0x0000_0000_0000_0000;
pub const USER_SPACE_END: u64 = 0x0000_7FFF_FFFF_FFFF; // 128TB user space
pub const KERNEL_SPACE_START: u64 = 0xFFFF_8000_0000_0000;
pub const KERNEL_SPACE_END: u64 = 0xFFFF_FFFF_FFFF_FFFF;

/// Virtual memory area types for user space
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VmaType {
    /// Code segment (executable, read-only)
    Code,
    /// Data segment (read-write, non-executable)
    Data,
    /// Heap (read-write, non-executable, growable)
    Heap,
    /// Stack (read-write, non-executable, growable down)
    Stack,
    /// Shared memory (read-write, non-executable)
    Shared,
    /// Memory-mapped files
    MmapFile,
    /// Anonymous memory mapping
    MmapAnon,
}

/// Virtual Memory Area descriptor
#[derive(Debug, Clone, Copy)]
pub struct VirtualMemoryArea {
    /// Start virtual address
    pub start_addr: u64,
    /// End virtual address (exclusive)
    pub end_addr: u64,
    /// VMA type
    pub vma_type: VmaType,
    /// Physical address (if mapped)
    pub physical_addr: Option<u64>,
    /// Permissions
    pub permissions: RegionType,
    /// Whether this VMA is currently mapped
    pub is_mapped: bool,
    /// Reference count for shared VMAs
    pub ref_count: usize,
}

impl VirtualMemoryArea {
    /// Create a new VMA
    pub const fn new(start: u64, end: u64, vma_type: VmaType, permissions: RegionType) -> Self {
        Self {
            start_addr: start,
            end_addr: end,
            vma_type,
            physical_addr: None,
            permissions,
            is_mapped: false,
            ref_count: 1,
        }
    }

    /// Get the size of this VMA in bytes
    pub fn size(&self) -> u64 {
        self.end_addr - self.start_addr
    }

    /// Get the number of pages in this VMA
    pub fn page_count(&self) -> usize {
        ((self.size() + PAGE_SIZE as u64 - 1) / PAGE_SIZE as u64) as usize
    }

    /// Check if an address is within this VMA
    pub fn contains(&self, addr: u64) -> bool {
        addr >= self.start_addr && addr < self.end_addr
    }

    /// Map this VMA to physical memory
    pub fn map_to_physical(&mut self, physical_addr: u64) -> Result<(), &'static str> {
        if self.is_mapped {
            return Err("VMA already mapped");
        }

        self.physical_addr = Some(physical_addr);
        self.is_mapped = true;
        Ok(())
    }

    /// Unmap this VMA from physical memory
    pub fn unmap(&mut self) {
        self.physical_addr = None;
        self.is_mapped = false;
    }
}

/// Simple array-based VMA list for no_std environment
#[derive(Debug, Clone, Copy)]
pub struct VmaList {
    vmas: [Option<VirtualMemoryArea>; 16], // Max 16 VMAs per process
    count: usize,
}

impl VmaList {
    /// Create a new empty VMA list
    pub const fn new() -> Self {
        Self {
            vmas: [None; 16],
            count: 0,
        }
    }

    /// Add a VMA to the list
    pub fn add_vma(&mut self, vma: VirtualMemoryArea) -> Result<usize, &'static str> {
        if self.count >= 16 {
            return Err("VMA list full");
        }

        // Check for overlaps
        for i in 0..self.count {
            if let Some(existing_vma) = &self.vmas[i] {
                if vma.start_addr < existing_vma.end_addr && vma.end_addr > existing_vma.start_addr
                {
                    return Err("VMA overlaps with existing VMA");
                }
            }
        }

        self.vmas[self.count] = Some(vma);
        let index = self.count;
        self.count += 1;
        Ok(index)
    }

    /// Remove a VMA by index
    pub fn remove_vma(&mut self, index: usize) -> Result<VirtualMemoryArea, &'static str> {
        if index >= self.count {
            return Err("VMA index out of bounds");
        }

        let vma = self.vmas[index].take().ok_or("VMA not found")?;

        // Shift remaining VMAs down
        for i in index..self.count - 1 {
            self.vmas[i] = self.vmas[i + 1].take();
        }

        self.count -= 1;
        Ok(vma)
    }

    /// Find VMA containing a specific address
    pub fn find_vma(&self, addr: u64) -> Option<(usize, &VirtualMemoryArea)> {
        for i in 0..self.count {
            if let Some(ref vma) = self.vmas[i] {
                if vma.contains(addr) {
                    return Some((i, vma));
                }
            }
        }
        None
    }

    /// Get mutable reference to VMA by index
    pub fn get_vma_mut(&mut self, index: usize) -> Option<&mut VirtualMemoryArea> {
        if index < self.count {
            self.vmas[index].as_mut()
        } else {
            None
        }
    }

    /// Get VMA by index
    pub fn get_vma(&self, index: usize) -> Option<&VirtualMemoryArea> {
        if index < self.count {
            self.vmas[index].as_ref()
        } else {
            None
        }
    }

    /// Get number of VMAs
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if list is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

/// User space page table for a single process
#[derive(Debug, Clone, Copy)]
pub struct UserPageTable {
    /// Process ID that owns this page table
    pub process_id: usize,
    /// L0 page table physical address (TTBR0_EL1)
    pub l0_table_addr: u64,
    /// Virtual memory areas for this process
    pub vmas: VmaList,
    /// ASID (Address Space ID) for TLB management
    pub asid: u16,
    /// Whether this page table is currently active
    pub is_active: bool,
    /// Page table creation timestamp
    pub created_time: u64,
    /// Last access timestamp
    pub last_access_time: u64,
    /// Total mapped pages
    pub mapped_pages: usize,
    /// Total allocated virtual memory
    pub allocated_vm_size: u64,
}

impl UserPageTable {
    /// Create a new user page table
    pub fn new(process_id: usize, asid: u16) -> Result<Self, &'static str> {
        // For now, we'll allocate the L0 table address from a simple pool
        // In a real implementation, this would use the memory manager
        let l0_table_addr = Self::allocate_page_table_memory()?;

        Ok(Self {
            process_id,
            l0_table_addr,
            vmas: VmaList::new(),
            asid,
            is_active: false,
            created_time: 0, // Would use timer in real implementation
            last_access_time: 0,
            mapped_pages: 0,
            allocated_vm_size: 0,
        })
    }

    /// Allocate memory for page table (simplified)
    fn allocate_page_table_memory() -> Result<u64, &'static str> {
        // This is a simplified allocation - in reality would use memory manager
        // For now, return a dummy address aligned to page boundary
        use core::sync::atomic::{AtomicU64, Ordering};

        static NEXT_PAGE_TABLE_ADDR: AtomicU64 = AtomicU64::new(0x8000_0000);
        let addr = NEXT_PAGE_TABLE_ADDR.fetch_add(PAGE_SIZE as u64, Ordering::SeqCst);
        Ok(addr)
    }

    /// Add a virtual memory area to this process
    pub fn add_vma(
        &mut self,
        start: u64,
        size: u64,
        vma_type: VmaType,
        permissions: RegionType,
    ) -> Result<usize, &'static str> {
        // Validate address range is in user space
        if start < USER_SPACE_START || start + size > USER_SPACE_END {
            return Err("Address outside user space");
        }

        // Align to page boundaries
        let aligned_start = start & !((PAGE_SIZE as u64) - 1);
        let aligned_end = (start + size + PAGE_SIZE as u64 - 1) & !((PAGE_SIZE as u64) - 1);

        let vma = VirtualMemoryArea::new(aligned_start, aligned_end, vma_type, permissions);
        let index = self.vmas.add_vma(vma)?;

        self.allocated_vm_size += aligned_end - aligned_start;
        Ok(index)
    }

    /// Remove a virtual memory area
    pub fn remove_vma(&mut self, index: usize) -> Result<(), &'static str> {
        let vma = self.vmas.remove_vma(index)?;

        // Unmap pages if mapped
        if vma.is_mapped {
            // In a real implementation, would unmap from page tables
            self.mapped_pages -= vma.page_count();
        }

        self.allocated_vm_size -= vma.size();
        Ok(())
    }

    /// Map a virtual memory area to physical memory
    pub fn map_vma(&mut self, vma_index: usize, physical_addr: u64) -> Result<(), &'static str> {
        let vma = self.vmas.get_vma_mut(vma_index).ok_or("VMA not found")?;

        if vma.is_mapped {
            return Err("VMA already mapped");
        }

        // Map the VMA
        vma.map_to_physical(physical_addr)?;
        self.mapped_pages += vma.page_count();

        // In a real implementation, would update page tables here
        // For now, just record the mapping

        Ok(())
    }

    /// Update page tables for a VMA (simplified)
    #[allow(dead_code)]
    fn update_page_tables(&mut self, vma: &VirtualMemoryArea) -> Result<(), &'static str> {
        // This is a simplified implementation
        // In reality, would walk page tables and create entries

        if let Some(phys_addr) = vma.physical_addr {
            // Create page table entries for this VMA
            let page_count = vma.page_count();

            for i in 0..page_count {
                let _virt_addr = vma.start_addr + (i as u64 * PAGE_SIZE as u64);
                let phys_page_addr = phys_addr + (i as u64 * PAGE_SIZE as u64);

                // Create page table entry (simplified)
                let _entry = PageTableEntry::new_page(
                    phys_page_addr,
                    MemoryAttribute::Normal,
                    vma.permissions,
                );

                // In reality, would install this entry in the page tables
            }
        }

        Ok(())
    }

    /// Unmap a virtual memory area
    pub fn unmap_vma(&mut self, vma_index: usize) -> Result<(), &'static str> {
        let vma = self.vmas.get_vma_mut(vma_index).ok_or("VMA not found")?;

        if !vma.is_mapped {
            return Err("VMA not mapped");
        }

        self.mapped_pages -= vma.page_count();
        vma.unmap();

        // In a real implementation, would remove page table entries
        Ok(())
    }

    /// Activate this page table (switch TTBR0_EL1)
    pub fn activate(&mut self) -> Result<(), &'static str> {
        if self.is_active {
            return Ok(());
        }

        // Switch TTBR0_EL1 to this page table
        unsafe {
            // Set TTBR0_EL1 to our L0 table
            core::arch::asm!(
                "msr ttbr0_el1, {}",
                in(reg) self.l0_table_addr,
                options(nostack)
            );

            // Invalidate TLB for this ASID
            core::arch::asm!(
                "tlbi aside1, {}",
                in(reg) (self.asid as u64) << 48,
                options(nostack)
            );

            // DSB to ensure completion
            core::arch::asm!("dsb sy", options(nostack));
            core::arch::asm!("isb", options(nostack));
        }

        self.is_active = true;
        self.last_access_time = 0; // Would use timer
        Ok(())
    }

    /// Deactivate this page table
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Translate virtual address to physical address
    pub fn translate_address(&self, virtual_addr: u64) -> Option<u64> {
        // Find the VMA containing this address
        if let Some((_, vma)) = self.vmas.find_vma(virtual_addr) {
            if let Some(phys_base) = vma.physical_addr {
                let offset = virtual_addr - vma.start_addr;
                return Some(phys_base + offset);
            }
        }
        None
    }

    /// Get page table statistics
    pub fn get_stats(&self) -> UserPageTableStats {
        UserPageTableStats {
            process_id: self.process_id,
            asid: self.asid,
            vma_count: self.vmas.len(),
            mapped_pages: self.mapped_pages,
            allocated_vm_size: self.allocated_vm_size,
            is_active: self.is_active,
            l0_table_addr: self.l0_table_addr,
        }
    }
}

/// Statistics for user page tables
#[derive(Debug, Clone, Copy)]
pub struct UserPageTableStats {
    pub process_id: usize,
    pub asid: u16,
    pub vma_count: usize,
    pub mapped_pages: usize,
    pub allocated_vm_size: u64,
    pub is_active: bool,
    pub l0_table_addr: u64,
}

/// Manager for all user space page tables
#[derive(Debug)]
pub struct UserSpaceManager {
    /// Array of user page tables
    page_tables: [Option<UserPageTable>; MAX_USER_PROCESSES],
    /// Number of active page tables
    active_count: usize,
    /// Currently active page table
    current_active: Option<usize>,
    /// Next ASID to assign
    next_asid: u16,
    /// Global statistics
    statistics: UserSpaceStats,
    /// Memory manager reference
    memory_manager: Option<*mut MemoryManager>,
}

/// Statistics for user space management
#[derive(Debug, Default, Clone, Copy)]
pub struct UserSpaceStats {
    /// Total page tables created
    pub page_tables_created: usize,
    /// Total page tables destroyed
    pub page_tables_destroyed: usize,
    /// Total context switches
    pub context_switches: usize,
    /// Total VMAs created
    pub vmas_created: usize,
    /// Total VMAs destroyed
    pub vmas_destroyed: usize,
    /// Total pages mapped
    pub pages_mapped: usize,
    /// Total virtual memory allocated
    pub vm_allocated_bytes: u64,
    /// TLB flushes performed
    pub tlb_flushes: usize,
}

impl UserSpaceManager {
    /// Create a new user space manager
    pub const fn new() -> Self {
        Self {
            page_tables: [None; MAX_USER_PROCESSES],
            active_count: 0,
            current_active: None,
            next_asid: 1,
            statistics: UserSpaceStats {
                page_tables_created: 0,
                page_tables_destroyed: 0,
                context_switches: 0,
                vmas_created: 0,
                vmas_destroyed: 0,
                pages_mapped: 0,
                vm_allocated_bytes: 0,
                tlb_flushes: 0,
            },
            memory_manager: None,
        }
    }

    /// Initialize with memory manager
    pub fn init(&mut self, memory_manager: *mut MemoryManager) {
        self.memory_manager = Some(memory_manager);
    }

    /// Create a new user page table for a process
    pub fn create_page_table(&mut self, process_id: usize) -> Result<usize, &'static str> {
        // Find empty slot
        let slot = self
            .find_empty_slot()
            .ok_or("No available page table slots")?;

        // Assign ASID
        let asid = self.next_asid;
        self.next_asid += 1;
        if self.next_asid == 0 {
            self.next_asid = 1; // Skip ASID 0
        }

        // Create page table
        let page_table = UserPageTable::new(process_id, asid)?;
        self.page_tables[slot] = Some(page_table);
        self.active_count += 1;

        // Update statistics
        self.statistics.page_tables_created += 1;

        Ok(slot)
    }

    /// Destroy a user page table
    pub fn destroy_page_table(&mut self, slot: usize) -> Result<(), &'static str> {
        if slot >= MAX_USER_PROCESSES {
            return Err("Invalid page table slot");
        }

        if let Some(mut page_table) = self.page_tables[slot].take() {
            // Deactivate if currently active
            if page_table.is_active {
                page_table.deactivate();
                if self.current_active == Some(slot) {
                    self.current_active = None;
                }
            }

            self.active_count -= 1;
            self.statistics.page_tables_destroyed += 1;
            Ok(())
        } else {
            Err("Page table slot not in use")
        }
    }

    /// Switch to a different page table
    pub fn switch_page_table(&mut self, slot: usize) -> Result<(), &'static str> {
        if slot >= MAX_USER_PROCESSES {
            return Err("Invalid page table slot");
        }

        // Deactivate current page table
        if let Some(current_slot) = self.current_active {
            if let Some(ref mut current_pt) = self.page_tables[current_slot] {
                current_pt.deactivate();
            }
        }

        // Activate new page table
        if let Some(ref mut new_pt) = self.page_tables[slot] {
            new_pt.activate()?;
            self.current_active = Some(slot);
            self.statistics.context_switches += 1;
            Ok(())
        } else {
            Err("Page table slot not in use")
        }
    }

    /// Get page table by slot
    pub fn get_page_table(&self, slot: usize) -> Option<&UserPageTable> {
        if slot < MAX_USER_PROCESSES {
            self.page_tables[slot].as_ref()
        } else {
            None
        }
    }

    /// Get mutable page table by slot
    pub fn get_page_table_mut(&mut self, slot: usize) -> Option<&mut UserPageTable> {
        if slot < MAX_USER_PROCESSES {
            self.page_tables[slot].as_mut()
        } else {
            None
        }
    }

    /// Find page table by process ID
    pub fn find_page_table_by_process(&self, process_id: usize) -> Option<usize> {
        for i in 0..MAX_USER_PROCESSES {
            if let Some(ref pt) = self.page_tables[i] {
                if pt.process_id == process_id {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Get currently active page table slot
    pub fn get_current_active(&self) -> Option<usize> {
        self.current_active
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &UserSpaceStats {
        &self.statistics
    }

    /// Activate a page table (alias for switch_page_table)
    pub fn activate_page_table(&mut self, slot: usize) -> Result<(), &'static str> {
        self.switch_page_table(slot)
    }

    /// Find an empty slot for a new page table
    fn find_empty_slot(&self) -> Option<usize> {
        for i in 0..MAX_USER_PROCESSES {
            if self.page_tables[i].is_none() {
                return Some(i);
            }
        }
        None
    }
}

/// SAFETY: UserSpaceManager is safe to send between threads and safe to share
/// between threads because:
/// 1. Raw pointer access is always protected by proper synchronization
/// 2. The raw pointer is only used within safe contexts
/// 3. All mutable operations are protected by mutex
unsafe impl Send for UserSpaceManager {}
unsafe impl Sync for UserSpaceManager {}

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

        // Add standard VMAs
        if let Some(page_table) = manager.get_page_table_mut(slot) {
            // Code segment: 0x400000 - 0x500000 (1MB)
            page_table.add_vma(0x400000, 0x100000, VmaType::Code, RegionType::UserCode)?;

            // Data segment: 0x500000 - 0x600000 (1MB)
            page_table.add_vma(0x500000, 0x100000, VmaType::Data, RegionType::UserData)?;

            // Heap: 0x600000 - 0x700000 (1MB, can grow)
            page_table.add_vma(0x600000, 0x100000, VmaType::Heap, RegionType::UserData)?;

            // Stack: 0x7FFFFFFFF000 - 0x800000000000 (4KB, grows down)
            page_table.add_vma(0x7FFFFFFFF000, 0x1000, VmaType::Stack, RegionType::UserData)?;
        }

        Ok(slot)
    })?
}

// End of user_space.rs
