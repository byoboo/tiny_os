//! Driver Traits Module
//!
//! This module provides common traits and error types for the driver system.

/// Driver error types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DriverError {
    NotInitialized,
    HardwareError,
    HardwareFault,
    InvalidConfig,
    Timeout,
    Busy,
    Unsupported,
    InvalidInput,
    CommunicationFailed,
}

/// Driver status enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DriverStatus {
    Uninitialized,
    Initializing,
    Ready,
    Busy,
    Error(DriverError),
}

/// Initialization trait for drivers
pub trait Initialize {
    type Config;

    /// Initialize the driver with default configuration
    fn init(&mut self) -> Result<(), DriverError>;

    /// Initialize the driver with specific configuration
    fn init_with_config(&mut self, config: &Self::Config) -> Result<(), DriverError>;
}

/// Status trait for drivers
pub trait Status {
    /// Get the current driver status
    fn status(&self) -> DriverStatus;

    /// Check if the driver is ready for operations
    fn is_ready(&self) -> bool {
        self.status() == DriverStatus::Ready
    }

    /// Check if the driver is busy
    fn is_busy(&self) -> bool {
        self.status() == DriverStatus::Busy
    }

    /// Check if the driver has an error
    fn has_error(&self) -> bool {
        matches!(self.status(), DriverStatus::Error(_))
    }
}
