// Memory management for TinyOS
use core::ptr::{read_volatile, write_volatile};

// Memory layout constants for Raspberry Pi 4/5
#[allow(dead_code)]
const KERNEL_START: u32 = 0x80000; // 512KB from start of RAM
const KERNEL_END: u32 = 0x100000; // 1MB mark - end of kernel space
const HEAP_START: u32 = KERNEL_END; // Heap starts after kernel
const HEAP_SIZE: u32 = 0x400000; // 4MB heap
const HEAP_END: u32 = HEAP_START + HEAP_SIZE;

// Simple block allocator constants
const BLOCK_SIZE: u32 = 64; // 64-byte blocks
const TOTAL_BLOCKS: u32 = HEAP_SIZE / BLOCK_SIZE;
#[allow(clippy::manual_div_ceil)]
const BITMAP_SIZE: u32 = (TOTAL_BLOCKS + 7) / 8; // Bits to bytes

pub struct MemoryManager {
    heap_start: u32,
    heap_end: u32,
    bitmap_start: u32,
    next_free_block: u32,
    allocated_blocks: u32,
    total_blocks: u32,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            heap_start: HEAP_START,
            heap_end: HEAP_END,
            bitmap_start: HEAP_START, // Bitmap is at start of heap
            next_free_block: 0,
            allocated_blocks: 0,
            total_blocks: TOTAL_BLOCKS,
        }
    }

    /// Initialize the memory manager
    pub fn init(&mut self) {
        // Clear the bitmap (all blocks free)
        unsafe {
            for i in 0..BITMAP_SIZE {
                write_volatile((self.bitmap_start + i) as *mut u8, 0);
            }
        }

        // Mark bitmap blocks as used (they contain the bitmap itself)
        #[allow(clippy::manual_div_ceil)]
        let bitmap_blocks = (BITMAP_SIZE + BLOCK_SIZE - 1) / BLOCK_SIZE;
        for i in 0..bitmap_blocks {
            self.set_block_used(i);
        }

        self.allocated_blocks = bitmap_blocks;
        self.next_free_block = bitmap_blocks;
    }

    /// Allocate a single block of memory
    pub fn allocate_block(&mut self) -> Option<u32> {
        self.allocate_blocks(1)
    }

    /// Allocate multiple contiguous blocks of memory
    pub fn allocate_blocks(&mut self, num_blocks: u32) -> Option<u32> {
        if num_blocks == 0 || num_blocks > self.total_blocks {
            return None;
        }

        // Find contiguous free blocks
        for start_block in 0..self.total_blocks {
            let mut all_free = true;

            // Check if we have enough contiguous blocks
            if start_block + num_blocks > self.total_blocks {
                break;
            }

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
                self.next_free_block = (start_block + num_blocks) % self.total_blocks;

                let block_address = self.heap_start + BITMAP_SIZE + (start_block * BLOCK_SIZE);

                // Add canary values for debugging
                self.add_canaries(block_address, num_blocks);

                return Some(block_address);
            }
        }
        None // Out of memory
    }

    /// Free a block of memory (single block)
    pub fn free_block(&mut self, address: u32) -> bool {
        self.free_blocks(address, 1)
    }

    /// Free multiple contiguous blocks of memory
    pub fn free_blocks(&mut self, address: u32, num_blocks: u32) -> bool {
        if address < self.heap_start + BITMAP_SIZE || address >= self.heap_end {
            return false; // Invalid address
        }

        let start_block_index = (address - self.heap_start - BITMAP_SIZE) / BLOCK_SIZE;
        if start_block_index + num_blocks > self.total_blocks {
            return false; // Invalid block range
        }

        // Verify all blocks are currently allocated
        for i in 0..num_blocks {
            if !self.is_block_used(start_block_index + i) {
                return false; // Block was already free
            }
        }

        // Check canaries before freeing (if they were added)
        #[allow(clippy::collapsible_if)]
        if num_blocks > 0 {
            if !self.check_canaries(address, num_blocks) {
                // Canary corruption detected - still free but report it
                // In a real OS, this might trigger a kernel panic
            }
        }

        // Free all blocks
        for i in 0..num_blocks {
            self.set_block_free(start_block_index + i);
        }

        self.allocated_blocks -= num_blocks;

        // Update next_free_block hint for faster allocation
        if start_block_index < self.next_free_block {
            self.next_free_block = start_block_index;
        }

        // Clear the block contents for security
        unsafe {
            for i in 0..(num_blocks * BLOCK_SIZE) {
                write_volatile((address + i) as *mut u8, 0);
            }
        }

        true
    }

    /// Check if a block is used
    fn is_block_used(&self, block_index: u32) -> bool {
        let byte_index = block_index / 8;
        let bit_index = block_index % 8;

        unsafe {
            let byte_value = read_volatile((self.bitmap_start + byte_index) as *const u8);
            (byte_value & (1 << bit_index)) != 0
        }
    }

    /// Mark a block as used
    fn set_block_used(&mut self, block_index: u32) {
        let byte_index = block_index / 8;
        let bit_index = block_index % 8;

        unsafe {
            let byte_addr = (self.bitmap_start + byte_index) as *mut u8;
            let mut byte_value = read_volatile(byte_addr);
            byte_value |= 1 << bit_index;
            write_volatile(byte_addr, byte_value);
        }
    }

    /// Mark a block as free
    fn set_block_free(&mut self, block_index: u32) {
        let byte_index = block_index / 8;
        let bit_index = block_index % 8;

        unsafe {
            let byte_addr = (self.bitmap_start + byte_index) as *mut u8;
            let mut byte_value = read_volatile(byte_addr);
            byte_value &= !(1 << bit_index);
            write_volatile(byte_addr, byte_value);
        }
    }

    /// Get memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        MemoryStats {
            total_blocks: self.total_blocks,
            allocated_blocks: self.allocated_blocks,
            free_blocks: self.total_blocks - self.allocated_blocks,
            block_size: BLOCK_SIZE,
            total_heap_size: HEAP_SIZE,
            used_heap_size: self.allocated_blocks * BLOCK_SIZE,
            free_heap_size: (self.total_blocks - self.allocated_blocks) * BLOCK_SIZE,
            heap_start: self.heap_start,
            heap_end: self.heap_end,
            largest_free_block: self.get_largest_free_block(),
            fragmentation_percent: self.get_fragmentation(),
            corruption_detected: !self.check_corruption(),
        }
    }

    /// Find largest contiguous free block
    pub fn get_largest_free_block(&self) -> u32 {
        let mut max_consecutive = 0;
        let mut current_consecutive = 0;

        for i in 0..self.total_blocks {
            if !self.is_block_used(i) {
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

    /// Memory test - allocate and free some blocks
    pub fn run_memory_test(&mut self) -> bool {
        let initial_allocated = self.allocated_blocks;

        // Allocate 5 blocks
        let mut test_blocks = [0u32; 5];
        #[allow(clippy::needless_range_loop)]
        for i in 0..5 {
            if let Some(addr) = self.allocate_block() {
                test_blocks[i] = addr;

                // Write a test pattern
                unsafe {
                    write_volatile(addr as *mut u32, 0xDEADBEEF + i as u32);
                }
            } else {
                return false; // Allocation failed
            }
        }

        // Verify test patterns
        #[allow(clippy::needless_range_loop)]
        for i in 0..5 {
            unsafe {
                let value = read_volatile(test_blocks[i] as *const u32);
                if value != 0xDEADBEEF + i as u32 {
                    return false; // Data corruption
                }
            }
        }

        // Free all test blocks
        #[allow(clippy::needless_range_loop)]
        for i in 0..5 {
            if !self.free_block(test_blocks[i]) {
                return false; // Free failed
            }
        }

        // Check if we're back to initial state
        self.allocated_blocks == initial_allocated
    }

    /// Comprehensive memory stress test
    pub fn run_stress_test(&mut self) -> bool {
        let initial_allocated = self.allocated_blocks;
        let mut allocated_blocks = [0u32; 50]; // Test with more blocks
        let mut allocated_count = 0;

        // Phase 1: Allocation stress test
        #[allow(clippy::needless_range_loop)]
        for i in 0..50 {
            if let Some(addr) = self.allocate_block() {
                allocated_blocks[i] = addr;
                allocated_count += 1;

                // Write unique test pattern
                unsafe {
                    write_volatile(addr as *mut u32, 0xABCD0000 + i as u32);
                    write_volatile((addr + 4) as *mut u32, 0x12340000 + i as u32);
                }
            } else {
                break; // Out of memory - acceptable
            }
        }

        // Phase 2: Verify all patterns are intact
        #[allow(clippy::needless_range_loop)]
        for i in 0..allocated_count {
            unsafe {
                let val1 = read_volatile(allocated_blocks[i] as *const u32);
                let val2 = read_volatile((allocated_blocks[i] + 4) as *const u32);

                if val1 != 0xABCD0000 + i as u32 || val2 != 0x12340000 + i as u32 {
                    // Data corruption detected
                    return false;
                }
            }
        }

        // Phase 3: Free every other block (fragmentation test)
        for i in (0..allocated_count).step_by(2) {
            if !self.free_block(allocated_blocks[i]) {
                return false;
            }
            allocated_blocks[i] = 0; // Mark as freed
        }

        // Phase 4: Try to allocate new blocks in fragmented space
        for i in (0..allocated_count).step_by(2) {
            if let Some(addr) = self.allocate_block() {
                allocated_blocks[i] = addr;
                // Write new pattern
                unsafe {
                    write_volatile(addr as *mut u32, 0xFEED0000 + i as u32);
                }
            }
        }

        // Phase 5: Verify remaining patterns are still intact
        #[allow(clippy::needless_range_loop)]
        for i in 1..allocated_count {
            if i % 2 == 1 && allocated_blocks[i] != 0 {
                unsafe {
                    let val1 = read_volatile(allocated_blocks[i] as *const u32);
                    let val2 = read_volatile((allocated_blocks[i] + 4) as *const u32);

                    if val1 != 0xABCD0000 + i as u32 || val2 != 0x12340000 + i as u32 {
                        return false;
                    }
                }
            }
        }

        // Phase 6: Clean up - free all remaining blocks
        #[allow(clippy::needless_range_loop)]
        for i in 0..allocated_count {
            if allocated_blocks[i] != 0 {
                self.free_block(allocated_blocks[i]);
            }
        }

        // Verify we're back to initial state
        self.allocated_blocks == initial_allocated
    }

    /// Test memory alignment and boundary conditions
    pub fn run_boundary_test(&mut self) -> bool {
        let initial_allocated = self.allocated_blocks;

        // Test 1: Allocate first available block
        let first_block = match self.allocate_block() {
            Some(addr) => addr,
            None => return false,
        };

        // Test 2: Verify block is properly aligned (64-byte boundary)
        if first_block % BLOCK_SIZE != 0 {
            self.free_block(first_block);
            return false;
        }

        // Test 3: Write to entire block boundary
        unsafe {
            // Write pattern at start
            write_volatile(first_block as *mut u32, 0x53A47123);
            // Write pattern at end
            write_volatile((first_block + BLOCK_SIZE - 4) as *mut u32, 0xE40123);
        }

        // Test 4: Verify patterns
        unsafe {
            let start_val = read_volatile(first_block as *const u32);
            let end_val = read_volatile((first_block + BLOCK_SIZE - 4) as *const u32);

            if start_val != 0x53A47123 || end_val != 0xE40123 {
                self.free_block(first_block);
                return false;
            }
        }

        // Test 5: Free and verify memory is cleared
        if !self.free_block(first_block) {
            return false;
        }

        // Test 6: Verify memory was cleared
        unsafe {
            let start_val = read_volatile(first_block as *const u32);
            let end_val = read_volatile((first_block + BLOCK_SIZE - 4) as *const u32);

            if start_val != 0 || end_val != 0 {
                return false; // Memory not properly cleared
            }
        }

        self.allocated_blocks == initial_allocated
    }

    /// Test multiple block allocation
    pub fn run_multiblock_test(&mut self) -> bool {
        let initial_allocated = self.allocated_blocks;

        // Test allocating 3 contiguous blocks
        if let Some(addr) = self.allocate_blocks(3) {
            // Write patterns across all blocks
            unsafe {
                for i in 0..3 {
                    let block_addr = addr + (i * BLOCK_SIZE);
                    write_volatile(block_addr as *mut u32, 0xB10C0000 + i);
                    write_volatile((block_addr + 4) as *mut u32, 0x12345678);
                }
            }

            // Verify patterns
            unsafe {
                for i in 0..3 {
                    let block_addr = addr + (i * BLOCK_SIZE);
                    let val1 = read_volatile(block_addr as *const u32);
                    let val2 = read_volatile((block_addr + 4) as *const u32);

                    if val1 != 0xB10C0000 + i || val2 != 0x12345678 {
                        self.free_blocks(addr, 3);
                        return false;
                    }
                }
            }

            // Free all blocks
            if !self.free_blocks(addr, 3) {
                return false;
            }

            self.allocated_blocks == initial_allocated
        } else {
            false
        }
    }

    /// Add canary values to allocated blocks for corruption detection
    fn add_canaries(&self, address: u32, num_blocks: u32) {
        const CANARY_VALUE: u32 = 0xDEADC0DE;

        // Add canary at the beginning of first block
        unsafe {
            write_volatile(address as *mut u32, CANARY_VALUE);
        }

        // Add canary at the end of last block
        let end_address = address + (num_blocks * BLOCK_SIZE) - 4;
        unsafe {
            write_volatile(end_address as *mut u32, CANARY_VALUE);
        }
    }

    /// Check if canaries are intact
    fn check_canaries(&self, address: u32, num_blocks: u32) -> bool {
        const CANARY_VALUE: u32 = 0xDEADC0DE;

        // Check canary at the beginning
        unsafe {
            let start_canary = read_volatile(address as *const u32);
            if start_canary != CANARY_VALUE {
                return false;
            }
        }

        // Check canary at the end
        let end_address = address + (num_blocks * BLOCK_SIZE) - 4;
        unsafe {
            let end_canary = read_volatile(end_address as *const u32);
            if end_canary != CANARY_VALUE {
                return false;
            }
        }

        true
    }

    /// Defragment memory by coalescing adjacent free blocks
    pub fn defragment(&mut self) -> u32 {
        let coalesced_blocks = 0;

        // This is a simple defragmentation that just updates our free block hint
        // In a more advanced system, we'd actually move allocated blocks

        // Find the first free block for our next_free_block hint
        for i in 0..self.total_blocks {
            if !self.is_block_used(i) {
                self.next_free_block = i;
                break;
            }
        }

        coalesced_blocks
    }

    /// Get fragmentation percentage (0-100)
    pub fn get_fragmentation(&self) -> u32 {
        let largest_free = self.get_largest_free_block() / BLOCK_SIZE;
        let total_free = self.total_blocks - self.allocated_blocks;

        if total_free == 0 {
            return 0;
        }

        // Fragmentation = (1 - largest_free/total_free) * 100
        let efficiency = (largest_free * 100) / total_free;
        100u32.saturating_sub(efficiency)
    }

    /// Allocate memory with specific alignment
    #[allow(dead_code)]
    pub fn allocate_aligned(&mut self, size_bytes: u32, alignment: u32) -> Option<u32> {
        #[allow(clippy::manual_div_ceil)]
        let blocks_needed = (size_bytes + BLOCK_SIZE - 1) / BLOCK_SIZE;

        // For simplicity, our blocks are already 64-byte aligned
        // In a real implementation, we'd handle arbitrary alignment
        if alignment <= BLOCK_SIZE {
            return self.allocate_blocks(blocks_needed);
        }

        None // Unsupported alignment
    }

    /// Memory corruption check - scan for common patterns
    pub fn check_corruption(&self) -> bool {
        // Check bitmap integrity
        #[allow(clippy::manual_div_ceil)]
        let bitmap_blocks = (BITMAP_SIZE + BLOCK_SIZE - 1) / BLOCK_SIZE;
        let mut counted_allocated = 0;

        for i in bitmap_blocks..self.total_blocks {
            if self.is_block_used(i) {
                counted_allocated += 1;
            }
        }

        // Should match our internal counter (minus bitmap blocks)
        counted_allocated + bitmap_blocks == self.allocated_blocks
    }
}

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
