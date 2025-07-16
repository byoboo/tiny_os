//! COW Page Module  
//!
//! This module manages individual Copy-on-Write pages, including reference
//! counting, virtual address mapping, and COW protection state management.

use crate::memory::{
    cow::utils::{ProcessIdArray, SimpleVec},
    mmu::RegionType,
};

/// COW page metadata and management
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
                if let Some(&addr) = self.virtual_addresses.get(i) {
                    if addr == virtual_addr {
                        let _ = self.virtual_addresses.remove(i);
                        break;
                    }
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
                    if let Some(&pid) = self.process_ids.get(i) {
                        if pid == process_id {
                            let _ = self.process_ids.remove(i);
                            break;
                        }
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
