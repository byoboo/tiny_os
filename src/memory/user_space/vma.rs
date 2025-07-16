//! Virtual Memory Area Management
//!
//! This module handles Virtual Memory Areas (VMAs) for user processes,
//! including creation, management, and operations on virtual memory regions.

use crate::memory::{mmu::RegionType, PAGE_SIZE};

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
