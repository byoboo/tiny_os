//! GPU Performance Benchmarking
//! 
//! Specialized benchmarks for VideoCore GPU performance measurement
//! and CPU vs GPU comparison for various workload types.

use crate::benchmarks::timing;
use crate::drivers::{videocore::{self, GpuTaskType}, dma};
use crate::optimization::{self, gpu_offload::{self, TaskCharacteristics}};

/// GPU benchmark results
#[derive(Debug, Clone)]
pub struct GpuBenchmarkResult {
    /// CPU execution cycles
    pub cpu_cycles: u64,
    /// GPU execution cycles  
    pub gpu_cycles: u64,
    /// DMA execution cycles (if applicable)
    pub dma_cycles: Option<u64>,
    /// Speedup factor (cpu_cycles / gpu_cycles)
    pub speedup: f32,
    /// Task characteristics
    pub task_type: GpuTaskType,
    /// Data size
    pub data_size: u32,
}

impl GpuBenchmarkResult {
    /// Calculate speedup ratio
    pub fn calculate_speedup(cpu_cycles: u64, gpu_cycles: u64) -> f32 {
        if gpu_cycles > 0 {
            cpu_cycles as f32 / gpu_cycles as f32
        } else {
            0.0
        }
    }
    
    /// Create new result
    pub fn new(cpu_cycles: u64, gpu_cycles: u64, task_type: GpuTaskType, data_size: u32) -> Self {
        Self {
            cpu_cycles,
            gpu_cycles,
            dma_cycles: None,
            speedup: Self::calculate_speedup(cpu_cycles, gpu_cycles),
            task_type,
            data_size,
        }
    }
    
    /// Add DMA result
    pub fn with_dma(mut self, dma_cycles: u64) -> Self {
        self.dma_cycles = Some(dma_cycles);
        self
    }
}

/// GPU performance benchmark suite
pub struct GpuPerformanceBenchmark {
    /// GPU available
    gpu_available: bool,
    /// DMA available
    dma_available: bool,
    /// Pi model info
    is_pi4_or_5: bool,
}

impl GpuPerformanceBenchmark {
    /// Create new benchmark suite
    pub fn new() -> Self {
        let gpu = videocore::get_gpu();
        let dma = dma::get_dma_controller();
        
        let (gpu_available, is_pi4_or_5) = if let Some(caps) = gpu.get_capabilities() {
            (gpu.is_available(), caps.has_advanced_features)
        } else {
            (false, false)
        };
        
        Self {
            gpu_available,
            dma_available: dma.is_initialized(),
            is_pi4_or_5,
        }
    }
    
    /// Benchmark memory operations (CPU vs GPU vs DMA)
    pub fn benchmark_memory_operations(&self, size: u32) -> Result<GpuBenchmarkResult, &'static str> {
        let gpu = videocore::get_gpu();
        
        // Allocate test data (no-std compatible with fixed size)
        const MAX_SIZE: usize = 4096;
        let actual_size = core::cmp::min(size as usize, MAX_SIZE);
        let mut src_data = [0xAAu8; MAX_SIZE];
        let mut dst_cpu = [0u8; MAX_SIZE];
        let mut dst_gpu = [0u8; MAX_SIZE];
        
        // Initialize only the portion we're testing
        for i in 0..actual_size {
            src_data[i] = 0xAA;
        }
        
        // CPU benchmark
        let cpu_start = timing::get_cycles();
        dst_cpu[..actual_size].copy_from_slice(&src_data[..actual_size]);
        let cpu_cycles = timing::get_cycles() - cpu_start;
        
        // GPU benchmark
        let gpu_cycles = if self.gpu_available {
            gpu.memory_copy(&mut dst_gpu, &src_data)?
        } else {
            cpu_cycles // Fallback to CPU result
        };
        
        // DMA benchmark
        let dma_cycles = if self.dma_available {
            let dma = dma::get_dma_controller();
            let mut dst_dma = [0u8; MAX_SIZE];
            Some(dma.memory_copy(&mut dst_dma[..actual_size], &src_data[..actual_size])?)
        } else {
            None
        };
        
        let mut result = GpuBenchmarkResult::new(cpu_cycles, gpu_cycles, GpuTaskType::Memory, size);
        if let Some(dma_cycles) = dma_cycles {
            result = result.with_dma(dma_cycles);
        }
        
        Ok(result)
    }
    
    /// Benchmark parallel computation (CPU vs GPU)
    pub fn benchmark_parallel_computation(&self, iterations: u32) -> Result<GpuBenchmarkResult, &'static str> {
        let gpu = videocore::get_gpu();
        
        if self.gpu_available {
            let (cpu_cycles, gpu_cycles) = gpu.parallel_compute_benchmark(iterations)?;
            Ok(GpuBenchmarkResult::new(cpu_cycles, gpu_cycles, GpuTaskType::Compute, iterations))
        } else {
            // CPU-only benchmark
            let cpu_cycles = self.cpu_parallel_computation(iterations);
            Ok(GpuBenchmarkResult::new(cpu_cycles, cpu_cycles, GpuTaskType::Compute, iterations))
        }
    }
    
    /// CPU parallel computation simulation
    fn cpu_parallel_computation(&self, iterations: u32) -> u64 {
        let start_cycles = timing::get_cycles();
        
        for _ in 0..iterations {
            let mut sum = 0u64;
            for i in 0..1000 {
                sum += (i * i) as u64;
            }
            core::hint::black_box(sum);
        }
        
        timing::get_cycles() - start_cycles
    }
    
    /// Benchmark GPU memory fill operations
    pub fn benchmark_memory_fill(&self, size: u32, value: u8) -> Result<GpuBenchmarkResult, &'static str> {
        let gpu = videocore::get_gpu();
        
        // Allocate test data (no-std compatible)
        const MAX_SIZE: usize = 4096;
        let actual_size = core::cmp::min(size as usize, MAX_SIZE);
        let mut data_cpu = [0u8; MAX_SIZE];
        let mut data_gpu = [0u8; MAX_SIZE];
        
        // CPU benchmark
        let cpu_start = timing::get_cycles();
        for byte in data_cpu[..actual_size].iter_mut() {
            *byte = value;
        }
        let cpu_cycles = timing::get_cycles() - cpu_start;
        
        // GPU benchmark
        let gpu_cycles = if self.gpu_available {
            gpu.memory_fill(&mut data_gpu, value)?
        } else {
            cpu_cycles
        };
        
        Ok(GpuBenchmarkResult::new(cpu_cycles, gpu_cycles, GpuTaskType::Memory, size))
    }
    
    /// Comprehensive GPU vs CPU benchmark suite (no-std compatible)
    pub fn comprehensive_benchmark(&self) -> Result<u32, &'static str> {
        let mut completed_tests = 0;
        
        // Memory operation benchmarks (limited for no-std)
        let memory_sizes = [1024, 4096];
        for &size in &memory_sizes {
            let _result1 = self.benchmark_memory_operations(size)?;
            completed_tests += 1;
            
            let _result2 = self.benchmark_memory_fill(size, 0xAA)?;
            completed_tests += 1;
        }
        
        // Computation benchmarks
        let compute_iterations = [100, 500];
        for &iterations in &compute_iterations {
            let _result = self.benchmark_parallel_computation(iterations)?;
            completed_tests += 1;
        }
        
        Ok(completed_tests)
    }
    
    /// Test GPU offload decision making (no-std compatible)
    pub fn test_offload_decisions(&self) -> Result<u32, &'static str> {
        if let Some(offload_system) = gpu_offload::get_offload_system() {
            let mut decisions_tested = 0;
            
            // Test various task characteristics
            let test_cases = [
                TaskCharacteristics::memory_operation(1024, true),
                TaskCharacteristics::memory_operation(8192, true),
                TaskCharacteristics::memory_operation(1024, false),
                TaskCharacteristics::compute_task(2048, 2.0, 4.0),
                TaskCharacteristics::compute_task(8192, 5.0, 8.0),
            ];
            
            for characteristics in &test_cases {
                let _decision = offload_system.make_decision(characteristics);
                decisions_tested += 1;
            }
            
            Ok(decisions_tested)
        } else {
            Err("GPU offload system not initialized")
        }
    }
    
    /// Get benchmark summary
    pub fn get_summary(&self) -> GpuBenchmarkSummary {
        GpuBenchmarkSummary {
            gpu_available: self.gpu_available,
            dma_available: self.dma_available,
            is_pi4_or_5: self.is_pi4_or_5,
            videocore_version: if self.is_pi4_or_5 { 6 } else { 4 },
        }
    }
}

/// GPU benchmark summary
#[derive(Debug, Clone)]
pub struct GpuBenchmarkSummary {
    pub gpu_available: bool,
    pub dma_available: bool,
    pub is_pi4_or_5: bool,
    pub videocore_version: u8,
}

/// Initialize GPU benchmarks
pub fn init() -> Result<(), &'static str> {
    // Initialize optimization framework
    optimization::init()?;
    
    // Initialize GPU offload system
    let context = optimization::get_context();
    gpu_offload::init(context);
    
    Ok(())
}

/// Create GPU benchmark suite
pub fn create_benchmark_suite() -> GpuPerformanceBenchmark {
    GpuPerformanceBenchmark::new()
}

/// Quick GPU vs CPU performance test
pub fn quick_gpu_test() -> Result<(u64, u64), &'static str> {
    let benchmark = create_benchmark_suite();
    let result = benchmark.benchmark_memory_operations(4096)?;
    Ok((result.cpu_cycles, result.gpu_cycles))
}

/// VideoCore communication test
pub fn test_videocore_communication() -> Result<bool, &'static str> {
    let gpu = videocore::get_gpu();
    
    if !gpu.is_available() {
        return Ok(false);
    }
    
    // Test GPU status retrieval
    let _status = gpu.get_status()?;
    
    // Test memory allocation
    let _context = gpu.allocate_memory(4096)?;
    
    Ok(true)
}

/// DMA transfer efficiency test
pub fn test_dma_efficiency() -> Result<(u64, u64), &'static str> {
    let dma = dma::get_dma_controller();
    
    // Try DMA benchmark and let it handle initialization check internally
    dma.benchmark_memory_copy(8192)
}
