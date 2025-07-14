//! GPU Offload System
//! 
//! Intelligent delegation of computational tasks between CPU and GPU
//! based on workload characteristics and hardware capabilities.

use crate::drivers::{videocore::{self, GpuTaskType}, dma};
use crate::benchmarks::timing;
use super::{OptimizationContext, MemoryTransferMethod};

/// Task characteristics for offload decision
#[derive(Debug, Clone)]
pub struct TaskCharacteristics {
    /// Data size in bytes
    pub data_size: u32,
    /// Task type
    pub task_type: GpuTaskType,
    /// Expected parallelism factor
    pub parallelism: f32,
    /// Memory access pattern (sequential vs random)
    pub sequential_access: bool,
    /// Computation intensity (cycles per byte)
    pub compute_intensity: f32,
}

impl TaskCharacteristics {
    /// Create characteristics for memory operation
    pub fn memory_operation(size: u32, sequential: bool) -> Self {
        Self {
            data_size: size,
            task_type: GpuTaskType::Memory,
            parallelism: 1.0,
            sequential_access: sequential,
            compute_intensity: 1.0,
        }
    }
    
    /// Create characteristics for computation task
    pub fn compute_task(size: u32, intensity: f32, parallelism: f32) -> Self {
        Self {
            data_size: size,
            task_type: GpuTaskType::Compute,
            parallelism,
            sequential_access: true,
            compute_intensity: intensity,
        }
    }
}

/// Offload decision result
#[derive(Debug, Clone, PartialEq)]
pub enum OffloadDecision {
    /// Use CPU for processing
    Cpu,
    /// Use GPU for processing
    Gpu,
    /// Use DMA for memory operations
    Dma,
    /// Use hybrid approach (CPU + GPU coordination)
    Hybrid,
}

/// GPU offload system
pub struct GpuOffloadSystem {
    /// Optimization context
    context: OptimizationContext,
    /// Performance history for learning
    cpu_performance_history: [u64; 16],
    gpu_performance_history: [u64; 16],
    history_index: usize,
}

impl GpuOffloadSystem {
    /// Create new offload system
    pub fn new(context: OptimizationContext) -> Self {
        Self {
            context,
            cpu_performance_history: [0; 16],
            gpu_performance_history: [0; 16],
            history_index: 0,
        }
    }
    
    /// Make offload decision based on task characteristics
    pub fn make_decision(&self, characteristics: &TaskCharacteristics) -> OffloadDecision {
        // If hardware not available, use CPU
        if !self.context.gpu_available && !self.context.dma_available {
            return OffloadDecision::Cpu;
        }
        
        match characteristics.task_type {
            GpuTaskType::Memory => self.decide_memory_operation(characteristics),
            GpuTaskType::Compute => self.decide_compute_operation(characteristics),
            GpuTaskType::Io => self.decide_io_operation(characteristics),
            GpuTaskType::Graphics => {
                // Graphics always prefer GPU when available
                if self.context.gpu_available {
                    OffloadDecision::Gpu
                } else {
                    OffloadDecision::Cpu
                }
            }
        }
    }
    
    /// Decide on memory operation offload
    fn decide_memory_operation(&self, characteristics: &TaskCharacteristics) -> OffloadDecision {
        let size = characteristics.data_size;
        
        // Very small operations stay on CPU
        if size < 256 {
            return OffloadDecision::Cpu;
        }
        
        // Large sequential operations prefer DMA
        if characteristics.sequential_access && self.context.dma_available {
            let dma_threshold = if self.context.has_advanced_features { 1024 } else { 4096 };
            if size >= dma_threshold {
                return OffloadDecision::Dma;
            }
        }
        
        // Medium-size operations with parallelism potential
        if self.context.gpu_available && characteristics.parallelism > 1.5 {
            let gpu_threshold = if self.context.has_advanced_features { 2048 } else { 8192 };
            if size >= gpu_threshold {
                return OffloadDecision::Gpu;
            }
        }
        
        OffloadDecision::Cpu
    }
    
    /// Decide on compute operation offload
    fn decide_compute_operation(&self, characteristics: &TaskCharacteristics) -> OffloadDecision {
        if !self.context.gpu_available {
            return OffloadDecision::Cpu;
        }
        
        let size = characteristics.data_size;
        let parallelism = characteristics.parallelism;
        let intensity = characteristics.compute_intensity;
        
        // High parallelism and compute intensity favor GPU
        let gpu_score = parallelism * intensity * (size as f32 / 1024.0);
        
        let gpu_threshold = if self.context.has_advanced_features {
            5.0 // Pi 4/5: Lower threshold for GPU usage
        } else {
            15.0 // Pi 3: Higher threshold
        };
        
        if gpu_score > gpu_threshold {
            OffloadDecision::Gpu
        } else if gpu_score > gpu_threshold * 0.3 && size > 4096 {
            OffloadDecision::Hybrid
        } else {
            OffloadDecision::Cpu
        }
    }
    
    /// Decide on I/O operation offload
    fn decide_io_operation(&self, characteristics: &TaskCharacteristics) -> OffloadDecision {
        let size = characteristics.data_size;
        
        // Large I/O operations benefit from DMA
        if self.context.dma_available && size > 8192 {
            OffloadDecision::Dma
        } else {
            OffloadDecision::Cpu
        }
    }
    
    /// Execute task with optimal method
    pub fn execute_task(&mut self, characteristics: &TaskCharacteristics, 
                       operation: &dyn Fn() -> Result<u64, &'static str>) -> Result<u64, &'static str> {
        let decision = self.make_decision(characteristics);
        let start_cycles = timing::get_cycles();
        
        let result = match decision {
            OffloadDecision::Cpu => self.execute_cpu_task(operation),
            OffloadDecision::Gpu => self.execute_gpu_task(characteristics, operation),
            OffloadDecision::Dma => self.execute_dma_task(characteristics, operation),
            OffloadDecision::Hybrid => self.execute_hybrid_task(characteristics, operation),
        };
        
        let end_cycles = timing::get_cycles();
        let execution_cycles = end_cycles - start_cycles;
        
        // Update performance history
        self.update_performance_history(decision, execution_cycles);
        
        result
    }
    
    /// Execute task on CPU
    fn execute_cpu_task(&self, operation: &dyn Fn() -> Result<u64, &'static str>) -> Result<u64, &'static str> {
        operation()
    }
    
    /// Execute task on GPU
    fn execute_gpu_task(&self, _characteristics: &TaskCharacteristics, 
                       operation: &dyn Fn() -> Result<u64, &'static str>) -> Result<u64, &'static str> {
        // For now, delegate to CPU (actual GPU implementation would submit QPU programs)
        operation()
    }
    
    /// Execute task using DMA
    fn execute_dma_task(&self, _characteristics: &TaskCharacteristics, 
                       operation: &dyn Fn() -> Result<u64, &'static str>) -> Result<u64, &'static str> {
        // For memory operations, this would use DMA controller
        operation()
    }
    
    /// Execute task using hybrid approach
    fn execute_hybrid_task(&self, _characteristics: &TaskCharacteristics, 
                          operation: &dyn Fn() -> Result<u64, &'static str>) -> Result<u64, &'static str> {
        // For now, delegate to CPU (actual implementation would split work)
        operation()
    }
    
    /// Update performance history for learning
    fn update_performance_history(&mut self, decision: OffloadDecision, cycles: u64) {
        match decision {
            OffloadDecision::Cpu => {
                self.cpu_performance_history[self.history_index] = cycles;
            }
            OffloadDecision::Gpu | OffloadDecision::Hybrid => {
                self.gpu_performance_history[self.history_index] = cycles;
            }
            OffloadDecision::Dma => {
                // DMA performance tracked separately
            }
        }
        
        self.history_index = (self.history_index + 1) % 16;
    }
    
    /// Get average CPU performance
    pub fn get_avg_cpu_performance(&self) -> u64 {
        let sum: u64 = self.cpu_performance_history.iter().sum();
        let count = self.cpu_performance_history.iter().filter(|&&x| x > 0).count();
        if count > 0 { sum / count as u64 } else { 0 }
    }
    
    /// Get average GPU performance
    pub fn get_avg_gpu_performance(&self) -> u64 {
        let sum: u64 = self.gpu_performance_history.iter().sum();
        let count = self.gpu_performance_history.iter().filter(|&&x| x > 0).count();
        if count > 0 { sum / count as u64 } else { 0 }
    }
    
    /// Benchmark CPU vs GPU for specific task type
    pub fn benchmark_offload(&mut self, task_type: GpuTaskType, size: u32) -> Result<(u64, u64), &'static str> {
        let characteristics = match task_type {
            GpuTaskType::Memory => TaskCharacteristics::memory_operation(size, true),
            GpuTaskType::Compute => TaskCharacteristics::compute_task(size, 2.0, 4.0),
            _ => TaskCharacteristics::memory_operation(size, true),
        };
        
        // CPU benchmark
        let cpu_start = timing::get_cycles();
        self.execute_cpu_task(&|| {
            // Simple benchmark task
            let mut sum = 0u64;
            for i in 0..size {
                sum += i as u64;
            }
            core::hint::black_box(sum);
            Ok(0)
        })?;
        let cpu_cycles = timing::get_cycles() - cpu_start;
        
        // GPU benchmark (currently delegates to CPU)
        let gpu_start = timing::get_cycles();
        self.execute_gpu_task(&characteristics, &|| {
            let mut sum = 0u64;
            for i in 0..size {
                sum += i as u64;
            }
            core::hint::black_box(sum);
            Ok(0)
        })?;
        let gpu_cycles = timing::get_cycles() - gpu_start;
        
        Ok((cpu_cycles, gpu_cycles))
    }
}

/// Global offload system instance
static mut OFFLOAD_SYSTEM: Option<GpuOffloadSystem> = None;

/// Initialize GPU offload system
pub fn init(context: OptimizationContext) {
    unsafe {
        OFFLOAD_SYSTEM = Some(GpuOffloadSystem::new(context));
    }
}

/// Get global offload system
pub fn get_offload_system() -> Option<&'static mut GpuOffloadSystem> {
    unsafe { OFFLOAD_SYSTEM.as_mut() }
}
