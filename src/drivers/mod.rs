//! Hardware Driver Abstraction Layer

// Core driver modules (without config/traits dependencies)
// pub mod gpio;
// pub mod sdcard;
// pub mod timer;
// pub mod uart;

// Week 3: VideoCore GPU Integration drivers
pub mod mailbox;
pub mod videocore;
pub mod dma;
pub mod cache;

// Week 4: Advanced Hardware Features
pub mod week4_simple;
pub mod pcie; // Re-enabled

// Re-export commonly used types
pub use mailbox::{Mailbox, GpuMemoryFlags, test_mailbox};
pub use videocore::{VideoCore, GpuTaskType, GpuStatus};
pub use dma::DmaController;
pub use cache::CacheController;

/// Simple DriverError for compatibility
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DriverError {
    NotInitialized,
    HardwareError,
    InvalidConfig,
    Timeout,
    Busy,
    Unsupported,
    VideoCore(crate::drivers::videocore::GpuError),
}

impl From<crate::drivers::videocore::GpuError> for DriverError {
    fn from(err: crate::drivers::videocore::GpuError) -> Self {
        DriverError::VideoCore(err)
    }
}
