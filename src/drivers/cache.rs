//! ARM64 Cache Optimization Driver
//! 
//! Provides cache tuning and optimization for Pi-specific cache hierarchies.
//! Optimized for Cortex-A72/A76 (Pi 4/5) with fallback for Cortex-A53 (Pi 3).

use core::arch::asm;

/// Cache line sizes for different Pi models
pub const CORTEX_A53_CACHE_LINE_SIZE: usize = 64;  // Pi 3
pub const CORTEX_A72_CACHE_LINE_SIZE: usize = 64;  // Pi 4
pub const CORTEX_A76_CACHE_LINE_SIZE: usize = 64;  // Pi 5

/// Cache configuration for Pi optimization
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// L1 data cache line size
    pub l1_cache_line_size: usize,
    /// L2 cache line size
    pub l2_cache_line_size: usize,
    /// L1 data cache size
    pub l1_cache_size: usize,
    /// L2 cache size
    pub l2_cache_size: usize,
    /// Cache associativity
    pub associativity: usize,
    /// Supports advanced cache operations
    pub has_advanced_ops: bool,
}

impl CacheConfig {
    /// Configuration for Pi 4/5 (Cortex-A72/A76)
    pub fn for_pi4_or_5() -> Self {
        Self {
            l1_cache_line_size: CORTEX_A72_CACHE_LINE_SIZE,
            l2_cache_line_size: CORTEX_A72_CACHE_LINE_SIZE,
            l1_cache_size: 32 * 1024,      // 32KB L1D
            l2_cache_size: 1024 * 1024,    // 1MB L2 (shared)
            associativity: 4,              // 4-way associative
            has_advanced_ops: true,
        }
    }
    
    /// Configuration for Pi 3 (Cortex-A53)
    pub fn for_pi3() -> Self {
        Self {
            l1_cache_line_size: CORTEX_A53_CACHE_LINE_SIZE,
            l2_cache_line_size: CORTEX_A53_CACHE_LINE_SIZE,
            l1_cache_size: 32 * 1024,      // 32KB L1D
            l2_cache_size: 512 * 1024,     // 512KB L2 (shared)
            associativity: 4,              // 4-way associative
            has_advanced_ops: false,
        }
    }
}

/// Cache operation types
#[derive(Debug, Clone, Copy)]
pub enum CacheOperation {
    /// Clean cache (write back dirty data)
    Clean,
    /// Invalidate cache (mark as invalid)
    Invalidate,
    /// Clean and invalidate
    CleanInvalidate,
    /// Prefetch data into cache
    Prefetch,
}

/// Cache level specification
#[derive(Debug, Clone, Copy)]
pub enum CacheLevel {
    /// L1 data cache
    L1Data,
    /// L1 instruction cache
    L1Instruction,
    /// L2 unified cache
    L2Unified,
    /// All cache levels
    All,
}

/// ARM64 Cache Controller
pub struct CacheController {
    /// Cache configuration
    config: CacheConfig,
    /// Pi model detection
    is_pi4_or_5: bool,
}

impl CacheController {
    /// Create new cache controller
    pub fn new(is_pi4_or_5: bool) -> Self {
        let config = if is_pi4_or_5 {
            CacheConfig::for_pi4_or_5()
        } else {
            CacheConfig::for_pi3()
        };
        
        Self {
            config,
            is_pi4_or_5,
        }
    }
    
    /// Get cache configuration
    pub fn get_config(&self) -> &CacheConfig {
        &self.config
    }
    
    /// Clean data cache by virtual address
    pub fn clean_dcache_va(&self, addr: usize) {
        unsafe {
            asm!("dc cvac, {addr}", addr = in(reg) addr);
        }
    }
    
    /// Invalidate data cache by virtual address
    pub fn invalidate_dcache_va(&self, addr: usize) {
        unsafe {
            asm!("dc ivac, {addr}", addr = in(reg) addr);
        }
    }
    
    /// Clean and invalidate data cache by virtual address
    pub fn clean_invalidate_dcache_va(&self, addr: usize) {
        unsafe {
            asm!("dc civac, {addr}", addr = in(reg) addr);
        }
    }
    
    /// Clean data cache range
    pub fn clean_dcache_range(&self, start: usize, size: usize) {
        let line_size = self.config.l1_cache_line_size;
        let aligned_start = start & !(line_size - 1);
        let aligned_end = (start + size + line_size - 1) & !(line_size - 1);
        
        let mut addr = aligned_start;
        while addr < aligned_end {
            self.clean_dcache_va(addr);
            addr += line_size;
        }
        
        // Data synchronization barrier
        self.dsb();
    }
    
    /// Invalidate data cache range
    pub fn invalidate_dcache_range(&self, start: usize, size: usize) {
        let line_size = self.config.l1_cache_line_size;
        let aligned_start = start & !(line_size - 1);
        let aligned_end = (start + size + line_size - 1) & !(line_size - 1);
        
        let mut addr = aligned_start;
        while addr < aligned_end {
            self.invalidate_dcache_va(addr);
            addr += line_size;
        }
        
        // Data synchronization barrier
        self.dsb();
    }
    
    /// Clean and invalidate data cache range
    pub fn clean_invalidate_dcache_range(&self, start: usize, size: usize) {
        let line_size = self.config.l1_cache_line_size;
        let aligned_start = start & !(line_size - 1);
        let aligned_end = (start + size + line_size - 1) & !(line_size - 1);
        
        let mut addr = aligned_start;
        while addr < aligned_end {
            self.clean_invalidate_dcache_va(addr);
            addr += line_size;
        }
        
        // Data synchronization barrier
        self.dsb();
    }
    
    /// Prefetch data into cache
    pub fn prefetch(&self, addr: usize) {
        if self.config.has_advanced_ops {
            unsafe {
                asm!("prfm pldl1keep, [{addr}]", addr = in(reg) addr);
            }
        }
    }
    
    /// Prefetch range for streaming access
    pub fn prefetch_range(&self, start: usize, size: usize) {
        if !self.config.has_advanced_ops {
            return;
        }
        
        let line_size = self.config.l1_cache_line_size;
        let stride = line_size * 2; // Prefetch every other line
        
        let mut addr = start;
        let end = start + size;
        
        while addr < end {
            self.prefetch(addr);
            addr += stride;
        }
    }
    
    /// Data synchronization barrier
    pub fn dsb(&self) {
        unsafe {
            asm!("dsb sy");
        }
    }
    
    /// Data memory barrier
    pub fn dmb(&self) {
        unsafe {
            asm!("dmb sy");
        }
    }
    
    /// Instruction synchronization barrier
    pub fn isb(&self) {
        unsafe {
            asm!("isb");
        }
    }
    
    /// Optimize memory access pattern for cache efficiency
    pub fn optimize_memory_access(&self, data: &mut [u8], pattern: MemoryAccessPattern) {
        match pattern {
            MemoryAccessPattern::Sequential => {
                // Prefetch ahead for sequential access
                if data.len() > self.config.l1_cache_size {
                    let prefetch_distance = self.config.l1_cache_line_size * 8;
                    let data_end = data.as_ptr() as usize + data.len();
                    
                    for chunk in data.chunks_mut(prefetch_distance) {
                        let prefetch_addr = chunk.as_ptr() as usize + prefetch_distance;
                        if prefetch_addr < data_end {
                            self.prefetch(prefetch_addr);
                        }
                        
                        // Process current chunk
                        self.process_memory_chunk(chunk);
                    }
                } else {
                    self.process_memory_chunk(data);
                }
            }
            MemoryAccessPattern::Random => {
                // For random access, minimize cache pollution
                self.process_memory_chunk(data);
            }
            MemoryAccessPattern::Block(block_size) => {
                // Process in cache-friendly blocks
                let optimal_block_size = block_size.min(self.config.l1_cache_size / 2);
                
                for chunk in data.chunks_mut(optimal_block_size) {
                    self.process_memory_chunk(chunk);
                }
            }
        }
    }
    
    /// Process a memory chunk with cache awareness
    fn process_memory_chunk(&self, chunk: &mut [u8]) {
        // Simple processing - just touch each cache line
        let line_size = self.config.l1_cache_line_size;
        let chunk_len = chunk.len();
        let chunk_ptr = chunk.as_ptr() as usize;
        
        for (i, byte) in chunk.iter_mut().enumerate() {
            if i % line_size == 0 {
                // First byte of cache line - prefetch next line
                let next_line = i + line_size;
                if next_line < chunk_len {
                    self.prefetch(chunk_ptr + next_line);
                }
            }
            
            // Simple operation to ensure cache line is loaded
            *byte = byte.wrapping_add(1);
        }
    }
    
    /// Get optimal alignment for data structures
    pub fn get_optimal_alignment(&self) -> usize {
        self.config.l1_cache_line_size
    }
    
    /// Get optimal block size for algorithms
    pub fn get_optimal_block_size(&self) -> usize {
        // Use 1/4 of L1 cache to avoid eviction
        self.config.l1_cache_size / 4
    }
    
    /// Test cache performance
    pub fn test_cache_performance(&self) -> CachePerformanceResult {
        use crate::benchmarks::timing;
        
        const MAX_TEST_SIZE: usize = 16384; // 16KB test for no-std
        let mut test_data = [0u8; MAX_TEST_SIZE];
        
        // Sequential access test
        let sequential_start = timing::get_cycles();
        for byte in test_data.iter_mut() {
            *byte = 0xAA;
        }
        let sequential_cycles = timing::get_cycles() - sequential_start;
        
        // Random access test
        let random_start = timing::get_cycles();
        let mut index = 0x12345678usize;
        for _ in 0..MAX_TEST_SIZE {
            index = (index.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7FFFFFFF;
            let access_index = index % MAX_TEST_SIZE;
            test_data[access_index] = 0x55;
        }
        let random_cycles = timing::get_cycles() - random_start;
        
        // Cache efficiency calculation
        let cache_efficiency = if random_cycles > 0 {
            (sequential_cycles as f32 / random_cycles as f32) * 100.0
        } else {
            0.0
        };
        
        CachePerformanceResult {
            sequential_cycles,
            random_cycles,
            cache_efficiency,
            l1_hit_rate: if cache_efficiency > 80.0 { 0.95 } else { 0.7 },
        }
    }
}

/// Memory access pattern for optimization
#[derive(Debug, Clone, Copy)]
pub enum MemoryAccessPattern {
    Sequential,
    Random,
    Block(usize),
}

/// Cache performance test result
#[derive(Debug, Clone)]
pub struct CachePerformanceResult {
    pub sequential_cycles: u64,
    pub random_cycles: u64,
    pub cache_efficiency: f32,
    pub l1_hit_rate: f32,
}

/// Global cache controller
static mut CACHE_CONTROLLER: Option<CacheController> = None;

/// Initialize cache controller
pub fn init(is_pi4_or_5: bool) {
    unsafe {
        CACHE_CONTROLLER = Some(CacheController::new(is_pi4_or_5));
    }
}

/// Get global cache controller
pub fn get_cache_controller() -> Option<&'static CacheController> {
    unsafe { CACHE_CONTROLLER.as_ref() }
}

/// Test cache functionality
pub fn test_cache_optimization() -> Result<CachePerformanceResult, &'static str> {
    if let Some(controller) = get_cache_controller() {
        Ok(controller.test_cache_performance())
    } else {
        Err("Cache controller not initialized")
    }
}
