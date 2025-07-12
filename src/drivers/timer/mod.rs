//! Timer Driver Module
//!
//! This module provides a complete System Timer driver implementation with
//! hardware abstraction and high-level APIs.

pub mod driver;
pub mod hardware;

// Re-export main types
pub use driver::{SystemTimer, TimerChannel, TimerConfig, TimerDriver};
pub use hardware::TimerHardware;
