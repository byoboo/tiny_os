//! System command module
//!
//! This module provides modular organization for system-level commands,
//! breaking down the previous monolithic system.rs into focused submodules.

pub mod core;
pub mod stack;
pub mod cow;
pub mod utils;

// Re-export all functions to maintain compatibility with the original system.rs interface
pub use core::{handle_help, handle_time, handle_system_info, handle_health_check};
pub use stack::{cmd_stack_status, cmd_stack_alloc, cmd_stack_dealloc, cmd_stack_switch, cmd_stack_test};
pub use cow::{cmd_cow_status, cmd_cow_stats, cmd_cow_create, cmd_cow_protect, cmd_cow_unprotect, cmd_cow_test};
pub use utils::{print_time, print_number, print_hex, parse_number};
