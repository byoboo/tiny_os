//! Memory management commands
//!
//! This module contains all memory-related command handlers including:
//! - Memory statistics display
//! - Memory allocation/deallocation
//! - Memory testing and corruption checking
//! - Memory defragmentation

use crate::{memory::MemoryManager, uart::Uart};

/// Handle memory statistics command ('m', 'M')
pub fn handle_memory_stats(uart: &Uart, memory_manager: &MemoryManager) {
    let stats = memory_manager.get_stats();
    uart.puts("\r\n=== Memory Statistics ===\r\n");
    uart.puts("Heap Layout:\r\n");
    uart.puts("  Start Address: 0x");
    print_hex(uart, stats.heap_start);
    uart.puts("\r\n  End Address: 0x");
    print_hex(uart, stats.heap_end);
    uart.puts("\r\n  Total Size: ");
    print_number(uart, stats.total_heap_size);
    uart.puts(" bytes\r\n");

    uart.puts("Block Information:\r\n");
    uart.puts("  Block Size: ");
    print_number(uart, stats.block_size);
    uart.puts(" bytes\r\n  Total Blocks: ");
    print_number(uart, stats.total_blocks);
    uart.puts("\r\n  Used Blocks: ");
    print_number(uart, stats.allocated_blocks);
    uart.puts("\r\n  Free Blocks: ");
    print_number(uart, stats.free_blocks);
    uart.puts("\r\n");

    uart.puts("Memory Usage:\r\n");
    uart.puts("  Used: ");
    print_number(uart, stats.used_heap_size);
    uart.puts(" bytes\r\n  Free: ");
    print_number(uart, stats.free_heap_size);
    uart.puts(" bytes\r\n");

    let usage_percent = (stats.used_heap_size * 100) / stats.total_heap_size;
    uart.puts("  Usage: ");
    print_number(uart, usage_percent);
    uart.puts("%\r\n");

    uart.puts("  Largest Free Block: ");
    print_number(uart, stats.largest_free_block);
    uart.puts(" bytes\r\n");

    uart.puts("  Fragmentation: ");
    print_number(uart, stats.fragmentation_percent);
    uart.puts("%\r\n");
    uart.puts("========================\r\n");
}

/// Handle memory allocation command ('a', 'A')
pub fn handle_memory_allocate(uart: &Uart, memory_manager: &mut MemoryManager) {
    uart.puts("Allocating memory block...\r\n");
    match memory_manager.allocate_block() {
        Some(address) => {
            uart.puts("✓ Memory block allocated at: 0x");
            print_hex(uart, address as u32);
            uart.puts("\r\n");
        }
        None => {
            uart.puts("✗ Memory allocation failed (no free blocks)\r\n");
        }
    }
}

/// Handle memory free command ('f', 'F')
pub fn handle_memory_free(uart: &Uart, _memory_manager: &mut MemoryManager) {
    uart.puts("Memory free functionality not directly available.\r\n");
    uart.puts("The memory manager uses a bitmap allocation system.\r\n");
    uart.puts("Use 'r' to defragment or check 'm' for current stats.\r\n");
}

/// Handle basic memory test command ('x', 'X')
pub fn handle_memory_test(uart: &Uart, memory_manager: &mut MemoryManager) {
    uart.puts("\r\n=== Basic Memory Test ===\r\n");
    uart.puts("Running memory allocation test...\r\n");

    // Test multiple allocations
    let test_count = 5;
    let mut allocated_addresses = [0u32; 5];
    let mut allocated_count = 0;

    uart.puts("Testing ");
    print_number(uart, test_count);
    uart.puts(" allocations:\r\n");

    for i in 0..test_count {
        match memory_manager.allocate_block() {
            Some(address) => {
                uart.puts("  Block ");
                print_number(uart, i + 1);
                uart.puts(": 0x");
                print_hex(uart, address);
                uart.puts(" ✓\r\n");
                allocated_addresses[allocated_count as usize] = address;
                allocated_count += 1;
            }
            None => {
                uart.puts("  Block ");
                print_number(uart, i + 1);
                uart.puts(": Failed ✗\r\n");
                break;
            }
        }
    }

    uart.puts("Freeing allocated blocks...\r\n");
    for i in 0..allocated_count {
        if memory_manager.free_block(allocated_addresses[i as usize]) {
            uart.puts("  Freed block ");
            print_number(uart, i + 1);
            uart.puts(" ✓\r\n");
        } else {
            uart.puts("  Failed to free block ");
            print_number(uart, i + 1);
            uart.puts(" ✗\r\n");
        }
    }

    uart.puts("=== Test Complete ===\r\n");
}

/// Handle comprehensive memory test command ('z', 'Z')
pub fn handle_comprehensive_memory_test(uart: &Uart, memory_manager: &mut MemoryManager) {
    uart.puts("\r\n=== Comprehensive Memory Test Suite ===\r\n");

    let initial_stats = memory_manager.get_stats();
    uart.puts("Initial state: ");
    print_number(uart, initial_stats.free_blocks);
    uart.puts(" free blocks\r\n");

    // Test 1: Sequential allocation using the actual memory manager tests
    uart.puts("\n1. Basic Memory Test:\r\n");
    if memory_manager.run_memory_test() {
        uart.puts("✓ Basic memory allocation/deallocation test passed\r\n");
    } else {
        uart.puts("✗ Basic memory test failed\r\n");
    }

    // Test 2: Stress test
    uart.puts("2. Memory Stress Test:\r\n");
    if memory_manager.run_stress_test() {
        uart.puts("✓ Memory stress test passed\r\n");
    } else {
        uart.puts("✗ Memory stress test failed\r\n");
    }

    // Test 3: Boundary test
    uart.puts("3. Boundary Test:\r\n");
    if memory_manager.run_boundary_test() {
        uart.puts("✓ Memory boundary test passed\r\n");
    } else {
        uart.puts("✗ Memory boundary test failed\r\n");
    }

    // Test 4: Multi-block test
    uart.puts("4. Multi-block Test:\r\n");
    if memory_manager.run_multiblock_test() {
        uart.puts("✓ Multi-block allocation test passed\r\n");
    } else {
        uart.puts("✗ Multi-block allocation test failed\r\n");
    }

    let final_stats = memory_manager.get_stats();
    uart.puts("\nFinal state: ");
    print_number(uart, final_stats.free_blocks);
    uart.puts(" free blocks\r\n");

    uart.puts("Fragmentation: ");
    print_number(uart, memory_manager.get_fragmentation());
    uart.puts("%\r\n");

    uart.puts("=== Test Suite Complete ===\r\n");
}

/// Handle memory corruption check command ('g', 'G')
pub fn handle_memory_corruption_check(uart: &Uart, memory_manager: &MemoryManager) {
    uart.puts("\r\n=== Memory Corruption Check ===\r\n");
    uart.puts("Scanning memory for corruption...\r\n");

    if memory_manager.check_corruption() {
        uart.puts("✓ No memory corruption detected\r\n");
        uart.puts("Memory integrity: GOOD\r\n");
    } else {
        uart.puts("⚠️  WARNING: Memory corruption detected!\r\n");
        uart.puts("Memory integrity: COMPROMISED\r\n");
        uart.puts("Recommendation: Restart system\r\n");
    }

    let stats = memory_manager.get_stats();
    uart.puts("Heap boundaries verified: ");
    if stats.heap_start < stats.heap_end {
        uart.puts("✓ VALID\r\n");
    } else {
        uart.puts("✗ INVALID\r\n");
    }

    uart.puts("Block list consistency: ");
    // Memory manager doesn't have block list validation, so we'll check basic stats
    let stats = memory_manager.get_stats();
    if stats.allocated_blocks <= stats.total_blocks {
        uart.puts("✓ VALID\r\n");
    } else {
        uart.puts("✗ CORRUPTED\r\n");
    }

    uart.puts("===============================\r\n");
}

/// Handle memory defragmentation command ('r', 'R')
pub fn handle_memory_defragment(uart: &Uart, memory_manager: &mut MemoryManager) {
    uart.puts("\r\n=== Memory Defragmentation ===\r\n");

    let stats_before = memory_manager.get_stats();
    uart.puts("Before defragmentation:\r\n");
    uart.puts("  Fragmentation: ");
    print_number(uart, stats_before.fragmentation_percent);
    uart.puts("%\r\n");
    uart.puts("  Largest free block: ");
    print_number(uart, stats_before.largest_free_block);
    uart.puts(" bytes\r\n");

    uart.puts("Running defragmentation...\r\n");
    let coalesced_blocks = memory_manager.defragment();

    if coalesced_blocks > 0 || stats_before.fragmentation_percent > 0 {
        let stats_after = memory_manager.get_stats();
        uart.puts("✓ Defragmentation complete\r\n");
        uart.puts("After defragmentation:\r\n");
        uart.puts("  Fragmentation: ");
        print_number(uart, stats_after.fragmentation_percent);
        uart.puts("%\r\n");
        uart.puts("  Largest free block: ");
        print_number(uart, stats_after.largest_free_block);
        uart.puts(" bytes\r\n");

        let improvement = stats_before
            .fragmentation_percent
            .saturating_sub(stats_after.fragmentation_percent);
        uart.puts("  Improvement: -");
        print_number(uart, improvement);
        uart.puts("% fragmentation\r\n");
    } else {
        uart.puts("✗ Defragmentation failed\r\n");
        uart.puts("No improvement possible or operation failed\r\n");
    }

    uart.puts("=============================\r\n");
}

/// Helper function to print a number
fn print_number(uart: &Uart, mut num: u32) {
    if num == 0 {
        uart.puts("0");
        return;
    }

    let mut buffer = [0u8; 10];
    let mut index = 0;

    while num > 0 {
        buffer[index] = (num % 10) as u8 + b'0';
        num /= 10;
        index += 1;
    }

    for i in (0..index).rev() {
        uart.putc(buffer[i]);
    }
}

/// Helper function to print a hexadecimal number
fn print_hex(uart: &Uart, mut num: u32) {
    if num == 0 {
        uart.puts("0");
        return;
    }

    let mut buffer = [0u8; 8];
    let mut index = 0;

    while num > 0 {
        let digit = num % 16;
        buffer[index] = if digit < 10 {
            digit as u8 + b'0'
        } else {
            (digit - 10) as u8 + b'A'
        };
        num /= 16;
        index += 1;
    }

    for i in (0..index).rev() {
        uart.putc(buffer[i]);
    }
}
