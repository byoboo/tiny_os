//! UART Driver Module
//!
//! This module provides a complete UART driver implementation with
//! hardware abstraction and high-level APIs.

pub mod driver;
pub mod hardware;

// Re-export main types
pub use driver::{Uart, UartConfig, UartDriver};
pub use hardware::UartHardware;
