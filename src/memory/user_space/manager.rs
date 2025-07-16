//! User Space Manager
//!
//! This module provides the central manager for all user space operations,
//! coordinating page tables, processes, and system-wide user space resources.

use super::page_table::UserPageTable;
use crate::memory::MemoryManager;

/// Maximum number of user processes that can have page tables
pub const MAX_USER_PROCESSES: usize = 32;

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
