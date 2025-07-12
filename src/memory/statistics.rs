//! Memory Usage Statistics and Analysis
//!
//! This module provides comprehensive memory usage statistics, fragmentation
//! analysis, and performance metrics for the memory management system.

use super::{allocator::BlockAllocator, layout::BLOCK_SIZE, protection::CorruptionDetection};

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_blocks: u32,
    pub allocated_blocks: u32,
    pub free_blocks: u32,
    pub block_size: u32,
    pub total_heap_size: u32,
    pub used_heap_size: u32,
    pub free_heap_size: u32,
    pub heap_start: u32,
    pub heap_end: u32,
    pub largest_free_block: u32,
    pub fragmentation_percent: u32,
    pub corruption_detected: bool,
}

/// Memory statistics collector and analyzer
pub struct MemoryStatistics<'a> {
    allocator: &'a BlockAllocator,
}

impl<'a> MemoryStatistics<'a> {
    /// Create a new statistics collector for the given allocator
    pub fn new(allocator: &'a BlockAllocator) -> Self {
        Self { allocator }
    }

    /// Get comprehensive memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        let config = self.allocator.config();

        MemoryStats {
            total_blocks: self.allocator.total_blocks(),
            allocated_blocks: self.allocator.allocated_blocks(),
            free_blocks: self.allocator.free_blocks(),
            block_size: config.block_size,
            total_heap_size: config.heap_size,
            used_heap_size: self.allocator.allocated_blocks() * config.block_size,
            free_heap_size: self.allocator.free_blocks() * config.block_size,
            heap_start: self.allocator.heap_start(),
            heap_end: self.allocator.heap_end(),
            largest_free_block: self.get_largest_free_block(),
            fragmentation_percent: self.get_fragmentation(),
            corruption_detected: !self.check_integrity(),
        }
    }

    /// Find the largest contiguous free block
    pub fn get_largest_free_block(&self) -> u32 {
        let mut max_consecutive = 0;
        let mut current_consecutive = 0;

        for i in 0..self.allocator.total_blocks() {
            if !self.allocator.is_block_used(i) {
                current_consecutive += 1;
                if current_consecutive > max_consecutive {
                    max_consecutive = current_consecutive;
                }
            } else {
                current_consecutive = 0;
            }
        }

        max_consecutive * BLOCK_SIZE
    }

    /// Calculate memory fragmentation percentage (0-100)
    ///
    /// Fragmentation is calculated as the difference between the total free
    /// space and the largest contiguous free block, expressed as a
    /// percentage.
    pub fn get_fragmentation(&self) -> u32 {
        let largest_free_blocks = self.get_largest_free_block() / BLOCK_SIZE;
        let total_free_blocks = self.allocator.free_blocks();

        if total_free_blocks == 0 {
            return 0;
        }

        // Fragmentation = (1 - largest_free/total_free) * 100
        let efficiency = (largest_free_blocks * 100) / total_free_blocks;
        100u32.saturating_sub(efficiency)
    }

    /// Check allocator integrity
    fn check_integrity(&self) -> bool {
        use super::protection::MemoryProtection;
        MemoryProtection::check_corruption(self.allocator)
    }

    /// Get detailed fragmentation analysis
    pub fn get_fragmentation_analysis(&self) -> FragmentationAnalysis {
        let mut free_segments = Vec::new();
        let mut current_segment_start = None;
        let mut current_segment_size = 0;

        // Scan for free segments
        for i in 0..self.allocator.total_blocks() {
            if !self.allocator.is_block_used(i) {
                if current_segment_start.is_none() {
                    current_segment_start = Some(i);
                    current_segment_size = 1;
                } else {
                    current_segment_size += 1;
                }
            } else {
                if let Some(_start) = current_segment_start {
                    free_segments.push(current_segment_size);
                    current_segment_start = None;
                    current_segment_size = 0;
                }
            }
        }

        // Handle final segment
        if current_segment_start.is_some() {
            free_segments.push(current_segment_size);
        }

        // Calculate statistics
        let total_segments = free_segments.len() as u32;
        let largest_segment = free_segments.iter().max().copied().unwrap_or(0);
        let smallest_segment = free_segments.iter().min().copied().unwrap_or(0);

        let average_segment = if total_segments > 0 {
            free_segments.iter().sum::<u32>() / total_segments
        } else {
            0
        };

        FragmentationAnalysis {
            total_free_segments: total_segments,
            largest_free_segment: largest_segment * BLOCK_SIZE,
            smallest_free_segment: smallest_segment * BLOCK_SIZE,
            average_segment_size: average_segment * BLOCK_SIZE,
            fragmentation_ratio: self.get_fragmentation(),
        }
    }

    /// Get memory usage by regions
    pub fn get_usage_by_region(&self) -> Vec<MemoryRegion> {
        let config = self.allocator.config();
        let mut regions = Vec::new();

        // Bitmap region
        regions.push(MemoryRegion {
            name: "Bitmap",
            start: config.heap_start,
            size: config.bitmap_size,
            blocks_used: config.bitmap_blocks(),
            usage_type: RegionUsage::System,
        });

        // Scan allocated regions in the heap
        let mut current_region_start = None;
        let mut current_region_blocks = 0;
        let bitmap_blocks = config.bitmap_blocks();

        for i in bitmap_blocks..self.allocator.total_blocks() {
            if self.allocator.is_block_used(i) {
                if current_region_start.is_none() {
                    current_region_start = Some(i);
                    current_region_blocks = 1;
                } else {
                    current_region_blocks += 1;
                }
            } else {
                if let Some(start) = current_region_start {
                    let region_start = config.usable_heap_start() + (start * config.block_size);
                    regions.push(MemoryRegion {
                        name: "Allocated",
                        start: region_start,
                        size: current_region_blocks * config.block_size,
                        blocks_used: current_region_blocks,
                        usage_type: RegionUsage::Allocated,
                    });
                    current_region_start = None;
                    current_region_blocks = 0;
                }
            }
        }

        // Handle final region
        if let Some(start) = current_region_start {
            let region_start = config.usable_heap_start() + (start * config.block_size);
            regions.push(MemoryRegion {
                name: "Allocated",
                start: region_start,
                size: current_region_blocks * config.block_size,
                blocks_used: current_region_blocks,
                usage_type: RegionUsage::Allocated,
            });
        }

        regions
    }
}

/// Detailed fragmentation analysis
#[derive(Debug)]
pub struct FragmentationAnalysis {
    pub total_free_segments: u32,
    pub largest_free_segment: u32,
    pub smallest_free_segment: u32,
    pub average_segment_size: u32,
    pub fragmentation_ratio: u32,
}

/// Memory region information
#[derive(Debug)]
pub struct MemoryRegion {
    pub name: &'static str,
    pub start: u32,
    pub size: u32,
    pub blocks_used: u32,
    pub usage_type: RegionUsage,
}

#[derive(Debug)]
pub enum RegionUsage {
    System,
    Allocated,
    Free,
}

/// Memory defragmentation utilities
pub struct MemoryDefragmenter<'a> {
    allocator: &'a mut BlockAllocator,
}

impl<'a> MemoryDefragmenter<'a> {
    /// Create a new defragmenter for the given allocator
    pub fn new(allocator: &'a mut BlockAllocator) -> Self {
        Self { allocator }
    }

    /// Perform simple defragmentation
    ///
    /// This is a simplified defragmentation that just updates the free block
    /// hint. In a more advanced system, we'd actually move allocated
    /// blocks.
    pub fn defragment(&mut self) -> DefragmentationResult {
        let stats_before = MemoryStatistics::new(self.allocator).get_stats();

        // Find the first free block for our next_free_block hint
        for i in 0..self.allocator.total_blocks() {
            if !self.allocator.is_block_used(i) {
                // Update the internal next_free_block hint (if we had access)
                // For now, this is a placeholder for more advanced defragmentation
                break;
            }
        }

        let stats_after = MemoryStatistics::new(self.allocator).get_stats();

        DefragmentationResult {
            blocks_moved: 0, // Placeholder - would be actual moved blocks
            fragmentation_before: stats_before.fragmentation_percent,
            fragmentation_after: stats_after.fragmentation_percent,
            largest_free_before: stats_before.largest_free_block,
            largest_free_after: stats_after.largest_free_block,
        }
    }
}

/// Defragmentation operation result
#[derive(Debug)]
pub struct DefragmentationResult {
    pub blocks_moved: u32,
    pub fragmentation_before: u32,
    pub fragmentation_after: u32,
    pub largest_free_before: u32,
    pub largest_free_after: u32,
}

// We need a minimal Vec implementation for no_std
// This is a simplified version for our use case
pub struct Vec<T> {
    data: [Option<T>; 32], // Fixed size for simplicity
    len: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Self {
            data: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None,
            ],
            len: 0,
        }
    }

    fn push(&mut self, item: T) {
        if self.len < 32 {
            self.data[self.len] = Some(item);
            self.len += 1;
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.data[..self.len].iter().filter_map(|x| x.as_ref())
    }
}
