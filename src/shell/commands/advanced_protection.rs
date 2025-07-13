//! Advanced Memory Protection Commands
//! 
//! Re-exports all advanced memory protection command handlers from the modular subsystem.
//! This maintains compatibility while providing organized, focused modules for
//! page permissions, ASLR, stack protection, testing, and statistics.

#[path = "advanced_protection_core.rs"]
mod advanced_protection_core;
#[path = "advanced_protection_status.rs"]
mod advanced_protection_status;
#[path = "advanced_protection_permissions.rs"]
mod advanced_protection_permissions;
#[path = "advanced_protection_aslr.rs"]
mod advanced_protection_aslr;
#[path = "advanced_protection_stack.rs"]
mod advanced_protection_stack;
#[path = "advanced_protection_testing.rs"]
mod advanced_protection_testing;
#[path = "advanced_protection_stats.rs"]
mod advanced_protection_stats;

// Re-export all command functions
pub use advanced_protection_core::cmd_advanced_protection;
pub use advanced_protection_status::cmd_advanced_protection_status;
pub use advanced_protection_permissions::cmd_advanced_protection_permissions;
pub use advanced_protection_aslr::cmd_advanced_protection_aslr;
pub use advanced_protection_stack::cmd_advanced_protection_stack;
pub use advanced_protection_testing::cmd_advanced_protection_test;
pub use advanced_protection_stats::cmd_advanced_protection_stats;
