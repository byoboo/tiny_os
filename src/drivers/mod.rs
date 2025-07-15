//! Hardware Driver Abstraction Layer

// Common driver infrastructure
pub mod config;
pub mod traits;

// Core driver modules (without config/traits dependencies)
pub mod gpio;
pub mod sdcard;
pub mod timer;
pub mod uart;

// Week 3: VideoCore GPU Integration drivers
pub mod mailbox;
pub mod videocore;
pub mod dma;
pub mod cache;

// Week 4: Advanced Hardware Features
pub mod pcie; // Re-enabled
pub mod power_management; // Re-enabled with no_std formatting

// Week 5: Network and Advanced I/O
pub mod network;

// Week 6: Advanced Security and Real-time
pub mod security;

// Performance monitoring and benchmarking
pub mod performance;

// Legacy week-specific modules (deprecated - use new modular structure)
#[deprecated(note = "Use drivers::network instead")]
pub mod week5_network;
#[deprecated(note = "Use drivers::security instead")]
pub mod week6_security;
#[deprecated(note = "Use drivers::performance instead")]
pub mod week4_simple;

// Re-export commonly used types
pub use mailbox::{Mailbox, GpuMemoryFlags, test_mailbox};
pub use videocore::{VideoCore, GpuTaskType, GpuStatus};
pub use dma::DmaController;
pub use cache::CacheController;

// Re-export core driver types
pub use gpio::{Gpio, GpioPin, GpioFunction};
pub use sdcard::{SdCard, SdCardError};
pub use timer::{SystemTimer, TimerChannel};
pub use uart::{Uart, UartConfig};

// Re-export Week 4-6 modular types
pub use network::{NetworkController, NetworkInterface, NetworkError};
pub use security::{SecurityController, SecurityLevel, SecurityError};
pub use performance::{BenchmarkSuite, PerformanceError, OptimizationLevel};

/// Simple DriverError for compatibility
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DriverError {
    NotInitialized,
    HardwareError,
    InvalidConfig,
    Timeout,
    Busy,
    Unsupported,
}

/// Hardware version detection for platform-specific optimizations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HardwareVersion {
    RaspberryPi3,
    RaspberryPi4,
    RaspberryPi5,
    Unknown,
}

/// Raspberry Pi 4 specific configuration
pub struct RaspberryPi4Config {
    pub cortex_a72: bool,
    pub enhanced_dma: bool,
    pub usb3_support: bool,
    pub pcie_available: bool,
}

impl Default for RaspberryPi4Config {
    fn default() -> Self {
        Self {
            cortex_a72: true,
            enhanced_dma: true,
            usb3_support: true,
            pcie_available: true,
        }
    }
}

/// Hardware version detection function
pub fn detect_hardware_version() -> HardwareVersion {
    // TODO: Implement actual hardware detection via device tree or CPU ID
    // For now, default to Pi 4 as our primary target
    HardwareVersion::RaspberryPi4
}
