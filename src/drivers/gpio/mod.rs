//! GPIO Driver Module
//!
//! This module provides a complete GPIO driver implementation with
//! hardware abstraction and high-level APIs.

pub mod driver;
pub mod hardware;

// Re-export main types
pub use driver::{Gpio, GpioConfig, GpioDriver, GpioPin};
pub use hardware::{GpioFunction, GpioHardware};
