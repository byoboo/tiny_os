//! VideoCore GPU Driver
//! 
//! Provides high-level interface to VideoCore GPU for parallel processing and hardware acceleration.
//! Automatically detects Pi model and optimizes for VideoCore VI (Pi 4/5) vs VideoCore IV (Pi 3).

use crate::drivers::mailbox::{self, Mailbox, GpuMemoryFlags};
use crate::benchmarks::timing;

/// GPU task types for performance optimization
#[derive(Debug, Clone, Copy)]
pub enum GpuTaskType {
    /// Memory-intensive operations (memory copies, fills)
    Memory,
    /// Mathematical computations (matrix operations, FFT)
    Compute,
    /// I/O operations (DMA transfers)
    Io,
    /// Graphics operations (pixel manipulation)
    Graphics,
}

/// GPU execution context
pub struct GpuContext {
    /// GPU memory handle
    pub memory_handle: u32,
    /// GPU bus address
    pub bus_address: u32,
    /// Memory size
    pub size: u32,
    /// CPU-accessible pointer
    pub cpu_ptr: *mut u8,
}

impl GpuContext {
    /// Create new GPU context with allocated memory
    pub fn new(size: u32) -> Result<Self, &'static str> {
        let mailbox = mailbox::get_mailbox();
        
        // Allocate GPU memory
        let alignment = mailbox.get_gpu_memory_alignment();
        let memory_handle = mailbox.allocate_gpu_memory(size, alignment, GpuMemoryFlags::Coherent)?;
        
        // Lock memory and get bus address
        let bus_address = mailbox.lock_gpu_memory(memory_handle)?;
        
        // Convert bus address to CPU address
        let cpu_ptr = (bus_address & 0x3FFFFFFF) as *mut u8;
        
        Ok(Self {
            memory_handle,
            bus_address,
            size,
            cpu_ptr,
        })
    }
    
    /// Get CPU-accessible slice
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(self.cpu_ptr, self.size as usize)
        }
    }
    
    /// Get mutable CPU-accessible slice
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(self.cpu_ptr, self.size as usize)
        }
    }
}

impl Drop for GpuContext {
    fn drop(&mut self) {
        let mailbox = mailbox::get_mailbox();
        let _ = mailbox.unlock_gpu_memory(self.memory_handle);
        let _ = mailbox.release_gpu_memory(self.memory_handle);
    }
}

/// VideoCore GPU capabilities
#[derive(Debug, Clone)]
pub struct GpuCapabilities {
    /// Pi model
    pub pi_model: u32,
    /// VideoCore version (4 for Pi 3, 6 for Pi 4/5)
    pub videocore_version: u8,
    /// GPU memory base address
    pub gpu_memory_base: u32,
    /// GPU memory size
    pub gpu_memory_size: u32,
    /// Optimal memory alignment
    pub memory_alignment: u32,
    /// Supports advanced features (Pi 4/5)
    pub has_advanced_features: bool,
}

/// VideoCore GPU driver
pub struct VideoCore {
    /// Mailbox interface
    mailbox: Option<&'static Mailbox>,
    /// GPU capabilities
    capabilities: Option<GpuCapabilities>,
    /// Initialization status
    initialized: bool,
}

impl VideoCore {
    /// Create new VideoCore driver
    pub const fn new() -> Self {
        Self {
            mailbox: None,
            capabilities: None,
            initialized: false,
        }
    }
    
    /// Initialize VideoCore driver
    pub fn initialize(&mut self) -> Result<(), &'static str> {
        // Get mailbox reference
        self.mailbox = Some(mailbox::get_mailbox());
        self.initialized = true;
        Ok(())
    }
    
    /// Initialize VideoCore GPU
    pub fn init(&mut self) -> Result<(), &'static str> {
        if self.initialized {
            return Ok(());
        }
        
        // Initialize mailbox first
        mailbox::init()?;
        self.mailbox = Some(mailbox::get_mailbox());
        
        let mailbox = self.mailbox.as_ref().unwrap();
        
        // Detect GPU capabilities
        let pi_model = mailbox.get_board_model()?;
        let (gpu_base, gpu_size) = mailbox.get_vc_memory()?;
        
        // Determine VideoCore version based on Pi model
        let videocore_version = if mailbox.is_pi4_or_5() { 6 } else { 4 };
        
        self.capabilities = Some(GpuCapabilities {
            pi_model,
            videocore_version,
            gpu_memory_base: gpu_base,
            gpu_memory_size: gpu_size,
            memory_alignment: mailbox.get_gpu_memory_alignment(),
            has_advanced_features: videocore_version >= 6,
        });
        
        self.initialized = true;
        Ok(())
    }
    
    /// Get GPU capabilities
    pub fn get_capabilities(&self) -> Option<&GpuCapabilities> {
        self.capabilities.as_ref()
    }
    
    /// Check if GPU is available and initialized
    pub fn is_available(&self) -> bool {
        self.initialized && self.capabilities.is_some()
    }
    
    /// Allocate GPU memory context
    pub fn allocate_memory(&self, size: u32) -> Result<GpuContext, &'static str> {
        if !self.is_available() {
            return Err("GPU not initialized");
        }
        
        GpuContext::new(size)
    }
    
    /// Determine if task should run on GPU vs CPU
    pub fn should_use_gpu(&self, task_type: GpuTaskType, data_size: u32) -> bool {
        if !self.is_available() {
            return false;
        }
        
        let caps = self.capabilities.as_ref().unwrap();
        
        // Pi 4/5 with VideoCore VI - more aggressive GPU usage
        if caps.has_advanced_features {
            match task_type {
                GpuTaskType::Memory => data_size > 1024, // Use GPU for larger memory ops
                GpuTaskType::Compute => data_size > 512,  // Math operations
                GpuTaskType::Io => data_size > 2048,      // Large I/O operations
                GpuTaskType::Graphics => true,            // Always use GPU for graphics
            }
        } else {
            // Pi 3 with VideoCore IV - conservative GPU usage
            match task_type {
                GpuTaskType::Memory => data_size > 4096,  // Only very large memory ops
                GpuTaskType::Compute => data_size > 2048, // Conservative math operations
                GpuTaskType::Io => data_size > 8192,      // Only very large I/O
                GpuTaskType::Graphics => data_size > 1024, // Basic graphics support
            }
        }
    }
    
    /// Perform memory fill operation (GPU accelerated when beneficial)
    pub fn memory_fill(&self, dst: &mut [u8], value: u8) -> Result<u64, &'static str> {
        let size = dst.len() as u32;
        let start_cycles = timing::get_cycles();
        
        if self.should_use_gpu(GpuTaskType::Memory, size) {
            self.gpu_memory_fill(dst, value)?;
        } else {
            self.cpu_memory_fill(dst, value);
        }
        
        let end_cycles = timing::get_cycles();
        Ok(end_cycles - start_cycles)
    }
    
    /// Perform memory copy operation (GPU accelerated when beneficial)
    pub fn memory_copy(&self, dst: &mut [u8], src: &[u8]) -> Result<u64, &'static str> {
        if dst.len() != src.len() {
            return Err("Source and destination size mismatch");
        }
        
        let size = dst.len() as u32;
        let start_cycles = timing::get_cycles();
        
        if self.should_use_gpu(GpuTaskType::Memory, size) {
            self.gpu_memory_copy(dst, src)?;
        } else {
            self.cpu_memory_copy(dst, src);
        }
        
        let end_cycles = timing::get_cycles();
        Ok(end_cycles - start_cycles)
    }
    
    /// CPU memory fill implementation
    fn cpu_memory_fill(&self, dst: &mut [u8], value: u8) {
        for byte in dst.iter_mut() {
            *byte = value;
        }
    }
    
    /// CPU memory copy implementation
    fn cpu_memory_copy(&self, dst: &mut [u8], src: &[u8]) {
        dst.copy_from_slice(src);
    }
    
    /// GPU memory fill implementation
    fn gpu_memory_fill(&self, dst: &mut [u8], value: u8) -> Result<(), &'static str> {
        // For now, fall back to CPU (actual GPU implementation would use QPU)
        // This is where we would submit a QPU program to fill memory
        self.cpu_memory_fill(dst, value);
        Ok(())
    }
    
    /// GPU memory copy implementation
    fn gpu_memory_copy(&self, dst: &mut [u8], src: &[u8]) -> Result<(), &'static str> {
        // For now, fall back to CPU (actual GPU implementation would use DMA)
        // This is where we would use DMA controller for large transfers
        self.cpu_memory_copy(dst, src);
        Ok(())
    }
    
    /// Perform parallel computation benchmark
    pub fn parallel_compute_benchmark(&self, iterations: u32) -> Result<(u64, u64), &'static str> {
        if !self.is_available() {
            return Err("GPU not initialized");
        }
        
        let data_size = 4096u32;
        
        // CPU baseline
        let start_cpu = timing::get_cycles();
        for _ in 0..iterations {
            self.cpu_compute_task(data_size);
        }
        let cpu_cycles = timing::get_cycles() - start_cpu;
        
        // GPU implementation
        let start_gpu = timing::get_cycles();
        for _ in 0..iterations {
            self.gpu_compute_task(data_size)?;
        }
        let gpu_cycles = timing::get_cycles() - start_gpu;
        
        Ok((cpu_cycles, gpu_cycles))
    }
    
    /// CPU computation task
    fn cpu_compute_task(&self, size: u32) {
        // Simple computation task - sum of squares
        let mut sum: u64 = 0;
        for i in 0..size {
            sum += (i as u64) * (i as u64);
        }
        // Prevent optimization
        core::hint::black_box(sum);
    }
    
    /// GPU computation task
    fn gpu_compute_task(&self, size: u32) -> Result<(), &'static str> {
        // For now, delegate to CPU
        // Real implementation would submit QPU shader
        self.cpu_compute_task(size);
        Ok(())
    }
    
    /// Get GPU status information
    pub fn get_status(&self) -> Result<GpuStatus, &'static str> {
        if !self.is_available() {
            return Err("GPU not initialized");
        }
        
        let temperature = self.mailbox.as_ref().unwrap().get_gpu_temperature().unwrap_or(0);
        let caps = self.capabilities.as_ref().unwrap();
        
        Ok(GpuStatus {
            initialized: true,
            pi_model: caps.pi_model,
            videocore_version: caps.videocore_version,
            gpu_memory_base: caps.gpu_memory_base,
            gpu_memory_size: caps.gpu_memory_size,
            temperature_millidegrees: temperature,
            has_advanced_features: caps.has_advanced_features,
        })
    }
}

/// GPU status information
#[derive(Debug, Clone)]
pub struct GpuStatus {
    pub initialized: bool,
    pub pi_model: u32,
    pub videocore_version: u8,
    pub gpu_memory_base: u32,
    pub gpu_memory_size: u32,
    pub temperature_millidegrees: u32,
    pub has_advanced_features: bool,
}

/// Global VideoCore instance
static mut GPU: VideoCore = VideoCore::new();

/// Initialize VideoCore GPU
pub fn init() -> Result<(), &'static str> {
    unsafe {
        GPU.init()
    }
}

/// Get global VideoCore instance
pub fn get_gpu() -> &'static VideoCore {
    unsafe { &GPU }
}

/// Get mutable global VideoCore instance
pub fn get_gpu_mut() -> &'static mut VideoCore {
    unsafe { &mut GPU }
}

/// Test VideoCore functionality
pub fn test_gpu() -> Result<(), &'static str> {
    let gpu = get_gpu();
    
    if !gpu.is_available() {
        return Err("GPU not available");
    }
    
    // Test memory allocation
    let mut context = gpu.allocate_memory(4096)?;
    let data = context.as_slice_mut();
    
    // Test memory operations
    gpu.memory_fill(data, 0xAA)?;
    gpu.memory_fill(data, 0x00)?;
    
    // Test computation
    let (_cpu_cycles, _gpu_cycles) = gpu.parallel_compute_benchmark(100)?;
    
    Ok(())
}
