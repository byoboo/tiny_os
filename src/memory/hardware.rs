//! Hardware-Specific Memory Operations
//!
//! This module provides hardware abstraction for memory operations,
//! including volatile reads/writes and hardware-specific optimizations.

use core::ptr::{read_volatile, write_volatile};

/// Hardware abstraction for memory operations
pub struct MemoryHardware;

impl MemoryHardware {
    /// Read a 32-bit value from memory with volatile semantics
    #[inline]
    pub unsafe fn read_u32(address: u32) -> u32 {
        read_volatile(address as *const u32)
    }

    /// Write a 32-bit value to memory with volatile semantics
    #[inline]
    pub unsafe fn write_u32(address: u32, value: u32) {
        write_volatile(address as *mut u32, value);
    }

    /// Read an 8-bit value from memory with volatile semantics
    #[inline]
    pub unsafe fn read_u8(address: u32) -> u8 {
        read_volatile(address as *const u8)
    }

    /// Write an 8-bit value to memory with volatile semantics
    #[inline]
    pub unsafe fn write_u8(address: u32, value: u8) {
        write_volatile(address as *mut u8, value);
    }

    /// Clear a range of memory (set to zero)
    pub unsafe fn clear_memory_range(start: u32, size: u32) {
        for i in 0..size {
            Self::write_u8(start + i, 0);
        }
    }

    /// Copy memory from source to destination
    pub unsafe fn copy_memory(src: u32, dst: u32, size: u32) {
        for i in 0..size {
            let value = Self::read_u8(src + i);
            Self::write_u8(dst + i, value);
        }
    }

    /// Check if address is properly aligned
    #[inline]
    pub fn is_aligned(address: u32, alignment: u32) -> bool {
        (address % alignment) == 0
    }

    /// Align address up to the nearest alignment boundary
    #[inline]
    pub fn align_up(address: u32, alignment: u32) -> u32 {
        (address + alignment - 1) & !(alignment - 1)
    }

    /// Align address down to the nearest alignment boundary
    #[inline]
    pub fn align_down(address: u32, alignment: u32) -> u32 {
        address & !(alignment - 1)
    }
}

/// Hardware memory information
pub struct HardwareMemoryInfo {
    pub total_memory: u32,
    pub available_memory: u32,
    pub cache_line_size: u32,
    pub page_size: u32,
}

impl HardwareMemoryInfo {
    /// Get hardware memory information for Raspberry Pi 4/5
    pub fn get() -> Self {
        Self {
            total_memory: 0x40000000, // 1GB (minimum for Pi 4)
            available_memory: 0x3F000000, // ~1016MB (allowing for GPU)
            cache_line_size: 64,
            page_size: 4096,
        }
    }
}
