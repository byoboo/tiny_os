//! Core Memory Allocator
//!
//! This module contains the core block-based memory allocation algorithm
//! using a bitmap to track allocated and free blocks.

use super::{hardware::MemoryHardware, layout::MemoryHardwareConfig};

/// Core block allocator using bitmap-based allocation
pub struct BlockAllocator {
    config: MemoryHardwareConfig,
    next_free_block: u32,
    allocated_blocks: u32,
}

impl BlockAllocator {
    /// Create a new block allocator with default configuration
    pub fn new() -> Self {
        Self {
            config: MemoryHardwareConfig::default(),
            next_free_block: 0,
            allocated_blocks: 0,
        }
    }

    /// Create a new block allocator with custom configuration
    pub fn with_config(config: MemoryHardwareConfig) -> Self {
        Self {
            config,
            next_free_block: 0,
            allocated_blocks: 0,
        }
    }

    /// Initialize the allocator
    ///
    /// This clears the bitmap and marks the bitmap blocks themselves as used.
    pub fn init(&mut self) {
        // Clear the bitmap (all blocks free)
        unsafe {
            MemoryHardware::clear_memory_range(self.config.heap_start, self.config.bitmap_size);
        }

        // Mark bitmap blocks as used (they contain the bitmap itself)
        let bitmap_blocks = self.config.bitmap_blocks();
        for i in 0..bitmap_blocks {
            self.set_block_used(i);
        }

        self.allocated_blocks = bitmap_blocks;
        self.next_free_block = bitmap_blocks;
    }

    /// Allocate a single block of memory
    ///
    /// Returns the address of the allocated block, or None if allocation fails.
    #[inline]
    pub fn allocate_block(&mut self) -> Option<u32> {
        self.allocate_blocks(1)
    }

    /// Allocate multiple contiguous blocks of memory
    ///
    /// Returns the address of the first allocated block, or None if allocation
    /// fails.
    pub fn allocate_blocks(&mut self, num_blocks: u32) -> Option<u32> {
        if num_blocks == 0 || num_blocks > self.config.total_blocks {
            return None;
        }

        // Find contiguous free blocks
        for start_block in 0..self.config.total_blocks {
            let mut all_free = true;

            // Check if we have enough contiguous blocks
            if start_block + num_blocks > self.config.total_blocks {
                break;
            }

            // Check if all required blocks are free
            for i in 0..num_blocks {
                if self.is_block_used(start_block + i) {
                    all_free = false;
                    break;
                }
            }

            if all_free {
                // Mark all blocks as used
                for i in 0..num_blocks {
                    self.set_block_used(start_block + i);
                }

                self.allocated_blocks += num_blocks;
                self.next_free_block = (start_block + num_blocks) % self.config.total_blocks;

                // Calculate the actual memory address
                let block_address =
                    self.config.usable_heap_start() + (start_block * self.config.block_size);

                return Some(block_address);
            }
        }

        None // No contiguous blocks available
    }

    /// Free a block of memory at the given address
    ///
    /// Returns true if the block was successfully freed, false otherwise.
    pub fn free_block(&mut self, address: u32) -> bool {
        // Check if address is within heap bounds
        let usable_start = self.config.usable_heap_start();
        if address < usable_start || address >= self.config.heap_end() {
            return false;
        }

        // Check if address is block-aligned
        if (address - usable_start) % self.config.block_size != 0 {
            return false;
        }

        // Calculate block number
        let block_number = (address - usable_start) / self.config.block_size;

        // Check if block is actually allocated
        if !self.is_block_used(block_number) {
            return false;
        }

        // Mark block as free
        self.set_block_free(block_number);
        self.allocated_blocks -= 1;

        // Update free block hint
        if block_number < self.next_free_block {
            self.next_free_block = block_number;
        }

        true
    }

    /// Allocate memory with specific alignment
    pub fn allocate_aligned(&mut self, size_bytes: u32, alignment: u32) -> Option<u32> {
        // Calculate blocks needed
        #[allow(clippy::manual_div_ceil)]
        let blocks_needed = (size_bytes + self.config.block_size - 1) / self.config.block_size;

        // For simplicity, our blocks are already 64-byte aligned
        // In a real implementation, we'd handle arbitrary alignment
        if alignment <= self.config.block_size {
            return self.allocate_blocks(blocks_needed);
        }

        None // Unsupported alignment for now
    }

    /// Check if a block is marked as used
    #[inline]
    fn is_block_used(&self, block_number: u32) -> bool {
        if block_number >= self.config.total_blocks {
            return true; // Out of bounds blocks are considered "used"
        }

        let byte_index = block_number / 8;
        let bit_index = block_number % 8;
        let bitmap_address = self.config.heap_start + byte_index;

        unsafe {
            let byte_value = MemoryHardware::read_u8(bitmap_address);
            (byte_value & (1 << bit_index)) != 0
        }
    }

    /// Mark a block as used
    #[inline]
    fn set_block_used(&mut self, block_number: u32) {
        if block_number >= self.config.total_blocks {
            return; // Out of bounds
        }

        let byte_index = block_number / 8;
        let bit_index = block_number % 8;
        let bitmap_address = self.config.heap_start + byte_index;

        unsafe {
            let mut byte_value = MemoryHardware::read_u8(bitmap_address);
            byte_value |= 1 << bit_index;
            MemoryHardware::write_u8(bitmap_address, byte_value);
        }
    }

    /// Mark a block as free
    #[inline]
    fn set_block_free(&mut self, block_number: u32) {
        if block_number >= self.config.total_blocks {
            return; // Out of bounds
        }

        let byte_index = block_number / 8;
        let bit_index = block_number % 8;
        let bitmap_address = self.config.heap_start + byte_index;

        unsafe {
            let mut byte_value = MemoryHardware::read_u8(bitmap_address);
            byte_value &= !(1 << bit_index);
            MemoryHardware::write_u8(bitmap_address, byte_value);
        }
    }

    /// Get the number of allocated blocks
    #[inline]
    pub fn allocated_blocks(&self) -> u32 {
        self.allocated_blocks
    }

    /// Get the total number of blocks
    #[inline]
    pub fn total_blocks(&self) -> u32 {
        self.config.total_blocks
    }

    /// Get the number of free blocks
    #[inline]
    pub fn free_blocks(&self) -> u32 {
        self.config.total_blocks - self.allocated_blocks
    }

    /// Get the heap configuration
    #[inline]
    pub fn config(&self) -> &MemoryHardwareConfig {
        &self.config
    }

    /// Get heap start address
    #[inline]
    pub fn heap_start(&self) -> u32 {
        self.config.heap_start
    }

    /// Get heap end address
    #[inline]
    pub fn heap_end(&self) -> u32 {
        self.config.heap_end()
    }
}

impl Default for BlockAllocator {
    fn default() -> Self {
        Self::new()
    }
}
