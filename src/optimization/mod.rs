//! Hardware Optimization Framework
//!
//! Coordination layer for Pi-specific hardware optimizations including
//! GPU offload, memory pattern optimization, and cache tuning.

pub mod gpu_offload;
pub mod memory_patterns;

use crate::drivers::{dma, mailbox, videocore};

/// Optimization context for hardware-specific tuning
pub struct OptimizationContext {
    /// Pi model information
    pub pi_model: u32,
    /// Has advanced features (Pi 4/5)
    pub has_advanced_features: bool,
    /// VideoCore version
    pub videocore_version: u8,
    /// GPU available
    pub gpu_available: bool,
    /// DMA available
    pub dma_available: bool,
}

impl OptimizationContext {
    /// Create optimization context from hardware detection
    pub fn from_hardware() -> Self {
        let gpu = videocore::get_gpu();
        let dma = dma::get_dma_controller();

        if let Some(caps) = gpu.get_capabilities() {
            Self {
                pi_model: caps.pi_model,
                has_advanced_features: caps.has_advanced_features,
                videocore_version: caps.videocore_version,
                gpu_available: gpu.is_available(),
                dma_available: dma.is_initialized(),
            }
        } else {
            Self {
                pi_model: 0,
                has_advanced_features: false,
                videocore_version: 4,
                gpu_available: false,
                dma_available: dma.is_initialized(),
            }
        }
    }

    /// Get recommended memory transfer method
    pub fn get_memory_transfer_method(&self, size: u32) -> MemoryTransferMethod {
        if self.dma_available && size >= self.get_dma_threshold() {
            MemoryTransferMethod::Dma
        } else if self.gpu_available && self.should_use_gpu_memory(size) {
            MemoryTransferMethod::Gpu
        } else {
            MemoryTransferMethod::Cpu
        }
    }

    /// Get DMA threshold based on Pi model
    fn get_dma_threshold(&self) -> u32 {
        if self.has_advanced_features {
            1024 // Pi 4/5: Lower threshold due to better DMA
        } else {
            4096 // Pi 3: Higher threshold
        }
    }

    /// Check if GPU memory operations are beneficial
    fn should_use_gpu_memory(&self, size: u32) -> bool {
        if !self.gpu_available {
            return false;
        }

        if self.has_advanced_features {
            size > 2048 // Pi 4/5: More aggressive GPU usage
        } else {
            size > 8192 // Pi 3: Conservative GPU usage
        }
    }
}

/// Memory transfer method recommendation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryTransferMethod {
    Cpu,
    Dma,
    Gpu,
}

/// Initialize optimization framework
pub fn init() -> Result<(), &'static str> {
    // Initialize GPU and DMA systems
    videocore::init()?;

    // Detect Pi model for DMA initialization
    let mailbox = mailbox::get_mailbox();
    let is_pi4_or_5 = mailbox.is_pi4_or_5();
    dma::init(is_pi4_or_5)?;

    Ok(())
}

/// Get optimization context
pub fn get_context() -> OptimizationContext {
    OptimizationContext::from_hardware()
}
