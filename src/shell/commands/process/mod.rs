// TinyOS Shell Process Commands Module
// Central coordination for process command subsystem

pub mod context;
pub mod privilege;
pub mod scheduler;
pub mod stats;

// Re-export all command handlers for shell integration
pub use context::handle_process_context_test;
pub use privilege::{handle_privilege_test, handle_privilege_stats};
pub use scheduler::{handle_scheduler_test, handle_scheduler_stats};
pub use stats::handle_process_stats;
