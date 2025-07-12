//! Memory Layout Constants and Hardware Configuration
//!
//! This module contains all memory layout constants and hardware-specific
//! memory configuration for TinyOS on Raspberry Pi 4/5.

/// Memory layout constants for Raspberry Pi 4/5
#[allow(dead_code)]
pub const KERNEL_START: u32 = 0x80000; // 512KB from start of RAM
pub const KERNEL_END: u32 = 0x100000; // 1MB mark - end of kernel space
pub const HEAP_START: u32 = KERNEL_END; // Heap starts after kernel
pub const HEAP_SIZE: u32 = 0x400000; // 4MB heap
pub const HEAP_END: u32 = HEAP_START + HEAP_SIZE;

/// Simple block allocator constants
pub const BLOCK_SIZE: u32 = 64; // 64-byte blocks (ARM64 cache-line optimized)
pub const TOTAL_BLOCKS: u32 = HEAP_SIZE / BLOCK_SIZE;

/// Bitmap size calculation (bits to bytes)
#[allow(clippy::manual_div_ceil)]
pub const BITMAP_SIZE: u32 = (TOTAL_BLOCKS + 7) / 8;

/// Canary value for memory corruption detection
pub const CANARY_VALUE: u32 = 0xDEADC0DE;

/// Memory alignment constants
pub const DEFAULT_ALIGNMENT: u32 = BLOCK_SIZE;
pub const CACHE_LINE_SIZE: u32 = 64;

/// Hardware memory configuration
pub struct MemoryHardwareConfig {
    pub heap_start: u32,
    pub heap_size: u32,
    pub block_size: u32,
    pub total_blocks: u32,
    pub bitmap_size: u32,
}

impl MemoryHardwareConfig {
    /// Get the default hardware configuration for Raspberry Pi 4/5
    pub const fn default() -> Self {
        Self {
            heap_start: HEAP_START,
            heap_size: HEAP_SIZE,
            block_size: BLOCK_SIZE,
            total_blocks: TOTAL_BLOCKS,
            bitmap_size: BITMAP_SIZE,
        }
    }

    /// Get heap end address
    #[inline]
    pub const fn heap_end(&self) -> u32 {
        self.heap_start + self.heap_size
    }

    /// Calculate bitmap blocks needed
    #[inline]
    pub const fn bitmap_blocks(&self) -> u32 {
        (self.bitmap_size + self.block_size - 1) / self.block_size
    }

    /// Get usable heap start (after bitmap)
    #[inline]
    pub const fn usable_heap_start(&self) -> u32 {
        self.heap_start + self.bitmap_size
    }
}
