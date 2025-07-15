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

// Legacy week-specific modules have been removed
// Use drivers::performance, drivers::network, and drivers::security instead

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
/// Uses ARM CPU ID registers and memory layout to detect Raspberry Pi version
pub fn detect_hardware_version() -> HardwareVersion {
    // Read ARM CPU ID register to detect hardware
    let midr: u64;
    unsafe {
        core::arch::asm!("mrs {}, midr_el1", out(reg) midr);
    }
    
    // Extract implementer and part number
    let implementer = (midr >> 24) & 0xFF;
    let part_number = (midr >> 4) & 0xFFF;
    
    // ARM implementer ID (0x41) with different part numbers
    if implementer == 0x41 {
        match part_number {
            // Cortex-A53 (Pi 3)
            0xD03 => HardwareVersion::RaspberryPi3,
            // Cortex-A72 (Pi 4)
            0xD08 => HardwareVersion::RaspberryPi4,
            // Cortex-A76 (Pi 5)
            0xD0B => HardwareVersion::RaspberryPi5,
            // Default to Pi 4 for unknown ARM cores
            _ => HardwareVersion::RaspberryPi4,
        }
    } else {
        // Non-ARM or unknown implementer, default to Pi 4
        HardwareVersion::RaspberryPi4
    }
}
