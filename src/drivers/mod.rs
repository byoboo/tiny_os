//! Hardware Driver Abstraction Layer
//!
//! This module provides a clean, modular driver architecture for TinyOS.
//! Each driver is organized into separate modules with the following structure:
//! - Hardware register definitions and low-level access
//! - High-level API with type-safe interfaces
//! - Zero-cost abstractions using const generics where applicable
//! - Embedded-friendly design with static allocation only

pub mod gpio;
pub mod sdcard;
pub mod timer;
pub mod uart;
// pub mod power_management; // Temporarily disabled until format! issues resolved

// Week 3: VideoCore GPU Integration drivers
pub mod mailbox;
pub mod videocore;
pub mod dma;
pub mod cache;

// Week 4: Advanced Hardware Features (Simplified)
pub mod week4_simple;

// Week 4: Full Implementation (under development)
// pub mod pcie; // Temporarily disabled until compilation issues resolved
// pub mod power_management; // Temporarily disabled until compilation issues resolved

// Re-export commonly used driver types for convenience
pub use gpio::{Gpio, GpioFunction};
pub use sdcard::SdCard;
pub use timer::SystemTimer;
pub use uart::Uart;

// Week 3: VideoCore and DMA exports
pub use mailbox::{Mailbox, GpuMemoryFlags, test_mailbox};
pub use videocore::{VideoCore, GpuTaskType, GpuStatus};
pub use dma::DmaController;
pub use cache::CacheController;

/// Common traits for hardware drivers
pub mod traits {
    /// Trait for drivers that can be initialized
    pub trait Initialize {
        /// Initialize the driver with default settings
        fn init(&mut self) -> Result<(), DriverError>;

        /// Initialize the driver with custom configuration
        fn init_with_config(&mut self, config: &Self::Config) -> Result<(), DriverError>
        where
            Self: Sized,
            Self::Config: Sized;

        /// Associated configuration type
        type Config;
    }

    /// Trait for drivers that support resetting
    pub trait Reset {
        /// Reset the driver to its initial state
        fn reset(&mut self) -> Result<(), DriverError>;
    }

    /// Trait for drivers that can report their status
    pub trait Status {
        /// Get the current driver status
        fn status(&self) -> DriverStatus;
    }

    /// Common driver error types
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DriverError {
        /// Hardware not present or not responding
        HardwareNotFound,
        /// Invalid configuration parameters
        InvalidConfig,
        /// Operation timeout
        Timeout,
        /// Hardware fault or error
        HardwareFault,
        /// Operation not supported
        NotSupported,
        /// Invalid input parameters
        InvalidInput,
    }

    /// Common driver status types
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DriverStatus {
        /// Driver is not initialized
        Uninitialized,
        /// Driver is ready for use
        Ready,
        /// Driver is busy with an operation
        Busy,
        /// Driver is in an error state
        Error(DriverError),
    }
}

/// Hardware configuration constants using const generics
pub mod config {
    /// Raspberry Pi hardware version configuration
    pub trait HardwareVersion {
        const GPIO_BASE: u32;
        const UART_BASE: u32;
        const TIMER_BASE: u32;
        const EMMC_BASE: u32;
    }

    /// Raspberry Pi 3 configuration
    pub struct RaspberryPi3;

    impl HardwareVersion for RaspberryPi3 {
        const GPIO_BASE: u32 = 0x3F200000;
        const UART_BASE: u32 = 0x3F201000;
        const TIMER_BASE: u32 = 0x3F003000;
        const EMMC_BASE: u32 = 0x3F300000;
    }

    /// Raspberry Pi 4 configuration
    pub struct RaspberryPi4;

    impl HardwareVersion for RaspberryPi4 {
        const GPIO_BASE: u32 = 0xFE200000;
        const UART_BASE: u32 = 0xFE201000;
        const TIMER_BASE: u32 = 0xFE003000;
        const EMMC_BASE: u32 = 0xFE300000;
    }

    /// Raspberry Pi 5 configuration (same as Pi 4 for most peripherals)
    pub struct RaspberryPi5;

    impl HardwareVersion for RaspberryPi5 {
        const GPIO_BASE: u32 = 0xFE200000;
        const UART_BASE: u32 = 0xFE201000;
        const TIMER_BASE: u32 = 0xFE003000;
        const EMMC_BASE: u32 = 0xFE300000;
    }

    /// Default hardware version selection based on target
    #[cfg(feature = "raspi3")]
    pub type DefaultHardware = RaspberryPi3;

    #[cfg(not(feature = "raspi3"))]
    pub type DefaultHardware = RaspberryPi4;
}
