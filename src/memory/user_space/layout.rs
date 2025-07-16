//! Memory Layout and Address Space Constants
//!
//! This module defines address space layouts, constants, and standard memory
//! layouts for user space processes.

use super::vma::{VirtualMemoryArea, VmaType};
use crate::memory::mmu::RegionType;

/// User space virtual address ranges
pub const USER_SPACE_START: u64 = 0x0000_0000_0000_0000;
pub const USER_SPACE_END: u64 = 0x0000_7FFF_FFFF_FFFF; // 128TB user space
pub const KERNEL_SPACE_START: u64 = 0xFFFF_8000_0000_0000;
pub const KERNEL_SPACE_END: u64 = 0xFFFF_FFFF_FFFF_FFFF;

/// Standard user process memory layout addresses
pub const STANDARD_CODE_START: u64 = 0x400000;
pub const STANDARD_CODE_SIZE: u64 = 0x100000; // 1MB

pub const STANDARD_DATA_START: u64 = 0x500000;
pub const STANDARD_DATA_SIZE: u64 = 0x100000; // 1MB

pub const STANDARD_HEAP_START: u64 = 0x600000;
pub const STANDARD_HEAP_SIZE: u64 = 0x100000; // 1MB, can grow

pub const STANDARD_STACK_START: u64 = 0x7FFFFFFFF000;
pub const STANDARD_STACK_SIZE: u64 = 0x1000; // 4KB, grows down

/// Create standard VMAs for a user process
pub fn create_standard_vmas() -> [VirtualMemoryArea; 4] {
    [
        // Code segment
        VirtualMemoryArea::new(
            STANDARD_CODE_START,
            STANDARD_CODE_START + STANDARD_CODE_SIZE,
            VmaType::Code,
            RegionType::UserCode,
        ),
        // Data segment
        VirtualMemoryArea::new(
            STANDARD_DATA_START,
            STANDARD_DATA_START + STANDARD_DATA_SIZE,
            VmaType::Data,
            RegionType::UserData,
        ),
        // Heap
        VirtualMemoryArea::new(
            STANDARD_HEAP_START,
            STANDARD_HEAP_START + STANDARD_HEAP_SIZE,
            VmaType::Heap,
            RegionType::UserData,
        ),
        // Stack
        VirtualMemoryArea::new(
            STANDARD_STACK_START,
            STANDARD_STACK_START + STANDARD_STACK_SIZE,
            VmaType::Stack,
            RegionType::UserData,
        ),
    ]
}

/// Validate that an address range is within user space
pub fn is_user_space_address(start: u64, size: u64) -> bool {
    // USER_SPACE_START is 0, so start is always >= 0, but we need to check overflow
    start.saturating_add(size) <= USER_SPACE_END
}

/// Validate that an address is within kernel space
pub fn is_kernel_space_address(addr: u64) -> bool {
    addr >= KERNEL_SPACE_START
    // KERNEL_SPACE_END is u64::MAX, so addr is always <= u64::MAX
}
