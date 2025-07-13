//! Hardware command handlers
//!
//! This module provides a unified interface for all hardware-related commands
//! including LED control, interrupt management, exception handling, and SD card
//! operations.
//!
//! All implementations are organized in focused sub-modules for better
//! maintainability and clear separation of concerns.

// Re-export all hardware command handlers from the modular subsystem
pub use super::hardware::*;
