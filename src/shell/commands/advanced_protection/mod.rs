//! Advanced Memory Protection Command Module
//!
//! This module provides comprehensive shell commands for managing advanced
//! memory protection features in TinyOS, including page permissions, ASLR,
//! stack protection, testing capabilities, and statistical monitoring.

mod aslr;
mod core;
mod permissions;
mod stack;
mod stats;
mod status;
mod testing;

// Re-export all command functions
pub use core::cmd_advanced_protection;

pub use aslr::cmd_advanced_protection_aslr;
pub use permissions::cmd_advanced_protection_permissions;
pub use stack::cmd_advanced_protection_stack;
pub use stats::cmd_advanced_protection_stats;
pub use status::cmd_advanced_protection_status;
pub use testing::cmd_advanced_protection_test;
