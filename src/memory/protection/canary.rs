//! Memory Canary Protection and Corruption Detection
//!
//! This module provides basic memory protection through canary values
//! and corruption pattern detection for debugging memory issues.

use super::super::{
    allocator::BlockAllocator,
    hardware::MemoryHardware,
    layout::{BLOCK_SIZE, CANARY_VALUE},
};

/// Basic memory protection utilities for canary management
pub struct MemoryProtection;

impl MemoryProtection {
    /// Add canary values to allocated blocks for corruption detection
    ///
    /// Places canary values at the beginning and end of the allocated region
    /// to detect buffer overruns and other memory corruption.
    pub fn add_canaries(address: u32, num_blocks: u32) {
        // Add canary at the beginning of first block
        unsafe {
            MemoryHardware::write_u32(address, CANARY_VALUE);
        }

        // Add canary at the end of last block
        let end_address = address + (num_blocks * BLOCK_SIZE) - 4;
        unsafe {
            MemoryHardware::write_u32(end_address, CANARY_VALUE);
        }
    }

    /// Check if canaries are intact
    ///
    /// Returns true if both canary values are intact, false if corruption is
    /// detected.
    pub fn check_canaries(address: u32, num_blocks: u32) -> bool {
        // Check canary at the beginning
        unsafe {
            let start_canary = MemoryHardware::read_u32(address);
            if start_canary != CANARY_VALUE {
                return false;
            }
        }

        // Check canary at the end
        let end_address = address + (num_blocks * BLOCK_SIZE) - 4;
        unsafe {
            let end_canary = MemoryHardware::read_u32(end_address);
            if end_canary != CANARY_VALUE {
                return false;
            }
        }

        true
    }

    /// Check for basic memory corruption in allocator
    ///
    /// Performs basic sanity checks on the allocator structure
    /// to detect obvious corruption.
    pub fn check_corruption(allocator: &BlockAllocator) -> bool {
        // Check that heap start is aligned
        if allocator.heap_start() % 4 != 0 {
            return false;
        }

        // Check that total blocks is reasonable
        if allocator.total_blocks() == 0 || allocator.total_blocks() > 0x100000 {
            return false;
        }

        true
    }

    /// Validate overall heap integrity
    ///
    /// Performs comprehensive checks on the heap structure including
    /// block allocation consistency and boundary validation.
    pub fn validate_heap_integrity(allocator: &BlockAllocator) -> bool {
        // Basic corruption check first
        if !Self::check_corruption(allocator) {
            return false;
        }

        // Check heap boundaries
        let heap_end = allocator.heap_start() + (allocator.total_blocks() * BLOCK_SIZE);
        if heap_end <= allocator.heap_start() {
            return false;
        }

        // Check that allocated blocks is within reasonable bounds
        if allocator.allocated_blocks() > allocator.total_blocks() {
            return false;
        }

        // All checks passed
        true
    }

    /// Scan for common corruption patterns
    ///
    /// Looks for patterns that might indicate memory corruption such as
    /// repeated values, null pointers, or invalid addresses.
    pub fn scan_corruption_patterns(start: u32, size: u32) -> CorruptionReport {
        let mut report = CorruptionReport::new();
        let mut consecutive_zeros = 0;
        let mut consecutive_ffs = 0;
        let max_consecutive_threshold = 64; // Threshold for suspicious patterns

        unsafe {
            for offset in (0..size).step_by(4) {
                if offset + 4 > size {
                    break;
                }

                let value = MemoryHardware::read_u32(start + offset);

                // Check for excessive zeros
                if value == 0 {
                    consecutive_zeros += 1;
                    consecutive_ffs = 0;
                } else if value == 0xFFFFFFFF {
                    consecutive_ffs += 1;
                    consecutive_zeros = 0;
                } else {
                    // Reset counters on non-pattern value
                    if consecutive_zeros > max_consecutive_threshold {
                        report.excessive_zeros = true;
                    }
                    if consecutive_ffs > max_consecutive_threshold {
                        report.excessive_ffs = true;
                    }
                    consecutive_zeros = 0;
                    consecutive_ffs = 0;
                }

                // Check for common corruption values
                if value == 0xDEADBEEF || value == 0xBADC0DE {
                    report.debug_patterns += 1;
                }
            }

            // Final check for patterns at the end
            if consecutive_zeros > max_consecutive_threshold {
                report.excessive_zeros = true;
            }
            if consecutive_ffs > max_consecutive_threshold {
                report.excessive_ffs = true;
            }
        }

        report
    }
}

/// Report structure for corruption detection results
#[derive(Debug)]
pub struct CorruptionReport {
    pub excessive_zeros: bool,
    pub excessive_ffs: bool,
    pub debug_patterns: u32,
}

impl CorruptionReport {
    /// Create a new corruption report with default values
    pub fn new() -> Self {
        Self {
            excessive_zeros: false,
            excessive_ffs: false,
            debug_patterns: 0,
        }
    }

    /// Check if any corruption patterns were detected
    pub fn has_corruption(&self) -> bool {
        self.excessive_zeros || self.excessive_ffs || self.debug_patterns > 10
    }
}

/// Extension trait to add corruption checking methods to BlockAllocator
pub trait CorruptionDetection {
    fn is_block_used(&self, block_number: u32) -> bool;
}

impl CorruptionDetection for BlockAllocator {
    fn is_block_used(&self, block_number: u32) -> bool {
        if block_number >= self.total_blocks() {
            return true; // Out of bounds blocks are considered "used"
        }

        let byte_index = block_number / 8;
        let bit_index = block_number % 8;
        let bitmap_address = self.heap_start() + byte_index;

        unsafe {
            let byte_value = MemoryHardware::read_u8(bitmap_address);
            (byte_value & (1 << bit_index)) != 0
        }
    }
}
