//! COW Fault Module
//!
//! This module handles Copy-on-Write fault processing, including fault types,
//! fault information structures, and statistics tracking.

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
