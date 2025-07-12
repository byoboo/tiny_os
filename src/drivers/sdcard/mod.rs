//! SD Card Driver Module
//!
//! This module provides a complete SD card driver implementation with
//! hardware abstraction and high-level APIs.

pub mod driver;
pub mod hardware;

// Re-export main types
pub use driver::{SdCard, SdCardConfig, SdCardDriver};
pub use hardware::{SdCardError, SdCardHardware};
