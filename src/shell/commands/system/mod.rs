//! System command module
//!
//! This module provides modular organization for system-level commands,
//! breaking down the previous monolithic system.rs into focused submodules.

pub mod core;
pub mod cow;
pub mod stack;
pub mod utils;

// Note: Re-exports removed as these functions are not used in the current command-line shell
// They are kept as modules for future reference and automated testing
// pub use core::{handle_health_check, handle_help, handle_system_info, handle_time};
// pub use cow::{
//     cmd_cow_create, cmd_cow_protect, cmd_cow_stats, cmd_cow_status, cmd_cow_test, cmd_cow_unprotect,
// };
// pub use stack::{
//     cmd_stack_alloc, cmd_stack_dealloc, cmd_stack_status, cmd_stack_switch, cmd_stack_test,
// };
// pub use utils::{parse_number, print_hex, print_number, print_time};
