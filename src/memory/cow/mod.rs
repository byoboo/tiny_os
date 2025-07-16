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

// Modular components
pub mod fault;
pub mod manager;
pub mod page;
pub mod utils;

// Re-export key types for compatibility
pub use fault::{CowFault, CowFaultType, CowStatistics};
pub use manager::{init_cow_manager, with_cow_manager, CowManager};
pub use page::CowPage;
pub use utils::{
    ProcessIdArray, SimpleVec, SimpleVecIter, MAX_PROCESS_IDS_PER_PAGE, MAX_VIRT_ADDRS_PER_PAGE,
};

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
