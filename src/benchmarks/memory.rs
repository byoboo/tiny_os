//! Memory Performance Benchmarking Module
//!
//! This module provides comprehensive memory allocation and management
//! performance testing for TinyOS efficiency validation.

use crate::benchmarks::timing;
use crate::memory::MemoryManager;

/// Memory benchmark suite for allocation performance testing
pub struct MemoryBenchmarks {
    /// Allocations performed in current test
    pub allocations_done: u32,
    /// Total bytes allocated during testing
    pub bytes_allocated: u64,
}

impl MemoryBenchmarks {
    /// Create new memory benchmark suite
    pub fn new() -> Self {
        Self {
            allocations_done: 0,
            bytes_allocated: 0,
        }
    }

    /// Benchmark sequential allocation performance
    pub fn benchmark_sequential_allocation(&mut self, manager: &mut MemoryManager) -> u64 {
        let start = timing::get_cycles();
        
        // Perform sequential allocations
        for i in 0..10 {
            if let Some(_ptr) = manager.allocate_block() {
                self.allocations_done += 1;
                self.bytes_allocated += 4096; // Assuming 4KB blocks
            }
        }
        
        let end = timing::get_cycles();
        end.saturating_sub(start)
    }

    /// Benchmark fragmentation patterns
    pub fn benchmark_fragmentation(&mut self, manager: &mut MemoryManager) -> u64 {
        let start = timing::get_cycles();
        
        // Allocate varying sizes to create fragmentation
        let sizes = [1, 2, 4, 8];
        for &size in &sizes {
            for _ in 0..5 {
                if let Some(_ptr) = manager.allocate_blocks(size) {
                    self.allocations_done += 1;
                    self.bytes_allocated += (size * 4096) as u64;
                }
            }
        }
        
        let end = timing::get_cycles();
        end.saturating_sub(start)
    }

    /// Benchmark mixed allocation/deallocation workload
    pub fn benchmark_mixed_workload(&mut self, manager: &mut MemoryManager) -> u64 {
        let start = timing::get_cycles();
        
        let mut allocated_blocks = [None; 10];
        
        // Mixed allocation/deallocation pattern
        for round in 0..3 {
            // Allocation phase
            for i in 0..5 {
                if let Some(ptr) = manager.allocate_block() {
                    allocated_blocks[i] = Some(ptr);
                    self.allocations_done += 1;
                    self.bytes_allocated += 4096;
                }
            }
            
            // Deallocation phase
            for i in 0..3 {
                if let Some(ptr) = allocated_blocks[i] {
                    manager.free_block(ptr);
                    allocated_blocks[i] = None;
                }
            }
        }
        
        // Clean up remaining allocations
        for i in 0..10 {
            if let Some(ptr) = allocated_blocks[i] {
                manager.free_block(ptr);
            }
        }
        
        let end = timing::get_cycles();
        end.saturating_sub(start)
    }

    /// Reset benchmark statistics
    pub fn reset(&mut self) {
        self.allocations_done = 0;
        self.bytes_allocated = 0;
    }
}

/// Run comprehensive memory performance test suite
pub fn run_memory_benchmarks(manager: &mut MemoryManager) -> MemoryBenchmarkResults {
    let mut benchmarks = MemoryBenchmarks::new();
    
    let sequential_cycles = benchmarks.benchmark_sequential_allocation(manager);
    let fragmentation_cycles = benchmarks.benchmark_fragmentation(manager);
    let mixed_cycles = benchmarks.benchmark_mixed_workload(manager);
    
    MemoryBenchmarkResults {
        sequential_allocation_cycles: sequential_cycles,
        fragmentation_cycles,
        mixed_workload_cycles: mixed_cycles,
        total_allocations: benchmarks.allocations_done,
        total_bytes_allocated: benchmarks.bytes_allocated,
    }
}

/// Results from memory benchmarking
pub struct MemoryBenchmarkResults {
    pub sequential_allocation_cycles: u64,
    pub fragmentation_cycles: u64,
    pub mixed_workload_cycles: u64,
    pub total_allocations: u32,
    pub total_bytes_allocated: u64,
}

impl MemoryBenchmarkResults {
    /// Calculate allocations per second based on cycle count
    pub fn allocations_per_second(&self) -> u32 {
        if self.sequential_allocation_cycles == 0 {
            return 0;
        }
        
        // Approximate calculation assuming 1GHz clock
        let seconds = self.sequential_allocation_cycles / 1_000_000_000;
        if seconds == 0 {
            return self.total_allocations * 1000; // Estimate based on milliseconds
        }
        
        (self.total_allocations as u64 / seconds) as u32
    }
}
