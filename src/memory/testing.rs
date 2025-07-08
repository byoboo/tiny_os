//! Memory Testing Utilities and Validation
//!
//! This module provides comprehensive testing utilities for validating
//! memory allocator functionality, performance, and corruption detection.
//! 
//! All testing is designed to be no_std compatible and uses static strings
//! and direct UART output for reporting instead of heap-allocated strings.

use super::{
    allocator::BlockAllocator,
    hardware::MemoryHardware,
    protection::MemoryProtection,
};
use crate::uart::Uart;

/// Memory testing utility - provides comprehensive memory testing functionality
pub struct MemoryTester<'a> {
    allocator: &'a mut BlockAllocator,
}

impl<'a> MemoryTester<'a> {
    /// Create a new memory tester for the given allocator
    pub fn new(allocator: &'a mut BlockAllocator) -> Self {
        Self { allocator }
    }

    /// Run basic memory allocation/deallocation test
    /// 
    /// Allocates several blocks, writes test patterns, verifies the patterns,
    /// then frees the blocks and verifies we're back to the initial state.
    pub fn run_basic_test(&mut self, uart: &Uart) -> bool {
        uart.puts("Running basic memory test...\r\n");
        let initial_allocated = self.allocator.allocated_blocks();

        // Test 1: Allocate 5 blocks
        let mut test_blocks = [0u32; 5];
        for i in 0..5 {
            if let Some(addr) = self.allocator.allocate_block() {
                test_blocks[i] = addr;

                // Write a test pattern
                unsafe {
                    MemoryHardware::write_u32(addr, 0xDEADBEEF + i as u32);
                }
            } else {
                uart.puts("✗ Failed to allocate test block\r\n");
                return false;
            }
        }

        uart.puts("✓ Successfully allocated 5 test blocks\r\n");

        // Test 2: Verify test patterns
        for i in 0..5 {
            let expected = 0xDEADBEEF + i as u32;
            let actual = unsafe { MemoryHardware::read_u32(test_blocks[i]) };
            if actual != expected {
                uart.puts("✗ Data corruption detected in test block\r\n");
                return false;
            }
        }

        uart.puts("✓ All test patterns verified successfully\r\n");

        // Test 3: Free all blocks
        for &addr in &test_blocks {
            self.allocator.free_block(addr);
        }

        uart.puts("✓ All test blocks freed\r\n");

        // Test 4: Verify we're back to initial state
        if self.allocator.allocated_blocks() == initial_allocated {
            uart.puts("✓ Memory state restored to initial condition\r\n");
            true
        } else {
            uart.puts("✗ Memory state not properly restored\r\n");
            false
        }
    }

    /// Run stress test - allocate many blocks rapidly
    pub fn run_stress_test(&mut self, uart: &Uart) -> bool {
        uart.puts("Running stress test...\r\n");
        let initial_allocated = self.allocator.allocated_blocks();
        let mut allocated_blocks = [0u32; 50];
        let mut allocated_count: usize = 0;

        // Allocate as many blocks as possible
        for i in 0..50 {
            if let Some(addr) = self.allocator.allocate_block() {
                allocated_blocks[i] = addr;
                allocated_count += 1;
            } else {
                break;
            }
        }

        uart.puts("Allocated ");
        print_number(uart, allocated_count as u32);
        uart.puts(" blocks in stress test\r\n");

        // Free half the blocks (creating fragmentation)
        let mut freed_count: usize = 0;
        for i in (0..allocated_count).step_by(2) {
            if allocated_blocks[i] != 0 {
                self.allocator.free_block(allocated_blocks[i]);
                allocated_blocks[i] = 0;
                freed_count += 1;
            }
        }

        uart.puts("Freed ");
        print_number(uart, freed_count as u32);
        uart.puts(" blocks in stress test\r\n");

        // Try to allocate more blocks
        let mut _additional_allocated = 0;
        for i in 0..allocated_count {
            if allocated_blocks[i] == 0 {
                if let Some(addr) = self.allocator.allocate_block() {
                    allocated_blocks[i] = addr;
                    _additional_allocated += 1;
                }
            }
        }

        // Clean up - free all remaining blocks
        for &addr in &allocated_blocks {
            if addr != 0 {
                self.allocator.free_block(addr);
            }
        }

        // Verify final state
        if self.allocator.allocated_blocks() == initial_allocated {
            uart.puts("✓ Stress test completed successfully\r\n");
            true
        } else {
            uart.puts("✗ Stress test failed - memory not properly restored\r\n");
            false
        }
    }

    /// Run corruption detection test
    pub fn run_corruption_test(&mut self, uart: &Uart) -> bool {
        uart.puts("Running corruption detection test...\r\n");

        // Allocate a test block
        if let Some(addr) = self.allocator.allocate_block() {
            // Set up canaries
            MemoryProtection::add_canaries(addr, 1);

            // Verify canaries are initially valid
            if !MemoryProtection::check_canaries(addr, 1) {
                uart.puts("✗ Initial canaries are invalid\r\n");
                self.allocator.free_block(addr);
                return false;
            }

            // Deliberately corrupt the end canary
            let end_canary_addr = addr + self.allocator.config().block_size - 4;
            unsafe {
                MemoryHardware::write_u32(end_canary_addr, 0xBADC0DE0);
            }

            // Verify corruption is detected
            if !MemoryProtection::check_canaries(addr, 1) {
                uart.puts("✓ Corruption successfully detected\r\n");
            } else {
                uart.puts("✗ Failed to detect deliberate corruption\r\n");
                self.allocator.free_block(addr);
                return false;
            }

            // Clean up
            self.allocator.free_block(addr);
            uart.puts("✓ Corruption test completed successfully\r\n");
            true
        } else {
            uart.puts("✗ Failed to allocate block for corruption test\r\n");
            false
        }
    }

    /// Run fragmentation test
    pub fn run_fragmentation_test(&mut self, uart: &Uart) -> bool {
        uart.puts("Running fragmentation test...\r\n");
        
        let initial_allocated = self.allocator.allocated_blocks();
        let mut test_blocks = [0u32; 20];
        
        // Allocate many blocks
        for i in 0..20 {
            if let Some(addr) = self.allocator.allocate_block() {
                test_blocks[i] = addr;
            } else {
                break;
            }
        }
        
        // Free every other block to create fragmentation
        let mut freed_count: usize = 0;
        for i in (1..20).step_by(2) {
            if test_blocks[i] != 0 {
                self.allocator.free_block(test_blocks[i]);
                test_blocks[i] = 0;
                freed_count += 1;
            }
        }

        uart.puts("Created fragmentation pattern (");
        print_number(uart, freed_count as u32);
        uart.puts(" blocks freed)\r\n");

        // For now, just report basic statistics
        uart.puts("Fragmentation: Test not yet implemented\r\n");
        uart.puts("Largest free block: Test not yet implemented\r\n");

        // Clean up remaining blocks
        for &addr in &test_blocks {
            if addr != 0 {
                self.allocator.free_block(addr);
            }
        }

        // Verify final state
        if self.allocator.allocated_blocks() == initial_allocated {
            uart.puts("✓ Fragmentation test completed successfully\r\n");
            true
        } else {
            uart.puts("✗ Fragmentation test failed - memory not properly restored\r\n");
            false
        }
    }

    /// Run boundary test (test allocation limits)
    pub fn run_boundary_test(&mut self, uart: &Uart) -> bool {
        uart.puts("Running boundary test...\r\n");
        
        let initial_allocated = self.allocator.allocated_blocks();
        let mut allocated_blocks = [0u32; 100];
        let mut allocated_count: usize = 0;

        // Try to allocate until we run out of memory
        for i in 0..100 {
            if let Some(addr) = self.allocator.allocate_block() {
                allocated_blocks[i] = addr;
                allocated_count += 1;
            } else {
                break;
            }
        }

        uart.puts("Allocated ");
        print_number(uart, allocated_count as u32);
        uart.puts(" blocks before exhaustion\r\n");

        // Verify that further allocation fails
        if self.allocator.allocate_block().is_none() {
            uart.puts("✓ Allocation correctly fails when memory exhausted\r\n");
        } else {
            uart.puts("✗ Allocation should fail when memory exhausted\r\n");
            return false;
        }

        // Free all blocks
        for i in 0..allocated_count {
            self.allocator.free_block(allocated_blocks[i]);
        }

        // Verify we can allocate again
        if let Some(addr) = self.allocator.allocate_block() {
            self.allocator.free_block(addr);
            uart.puts("✓ Can allocate again after freeing all blocks\r\n");
        } else {
            uart.puts("✗ Cannot allocate after freeing all blocks\r\n");
            return false;
        }

        // Verify final state
        if self.allocator.allocated_blocks() == initial_allocated {
            uart.puts("✓ Boundary test completed successfully\r\n");
            true
        } else {
            uart.puts("✗ Boundary test failed - memory not properly restored\r\n");
            false
        }
    }

    /// Run comprehensive test suite
    pub fn run_comprehensive_test(&mut self, uart: &Uart) -> bool {
        uart.puts("\r\n=== Comprehensive Memory Test Suite ===\r\n");
        
        let mut all_passed = true;
        
        all_passed &= self.run_basic_test(uart);
        uart.puts("\r\n");
        
        all_passed &= self.run_stress_test(uart);
        uart.puts("\r\n");
        
        all_passed &= self.run_corruption_test(uart);
        uart.puts("\r\n");
        
        all_passed &= self.run_fragmentation_test(uart);
        uart.puts("\r\n");
        
        all_passed &= self.run_boundary_test(uart);
        uart.puts("\r\n");

        if all_passed {
            uart.puts("🎉 All memory tests PASSED!\r\n");
        } else {
            uart.puts("❌ Some memory tests FAILED!\r\n");
        }
        
        uart.puts("=====================================\r\n");
        all_passed
    }
}

/// Helper function to print a number
#[inline]
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