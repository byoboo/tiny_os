//! Command Router Modules
//!
//! This module provides the command routing infrastructure for the TinyOS shell,
//! organizing routing logic by command complexity and functionality.

pub mod basic;
pub mod advanced;
pub mod specialized;

// Re-export router functions for clean imports
pub use basic::{route_system_commands, route_hardware_commands, route_enhanced_hardware_commands, route_memory_commands};
pub use advanced::{route_process_management, route_exception_management, route_virtual_memory_management};
pub use specialized::{route_stack_management, route_cow_management, route_testing_framework, 
                     route_user_space_management, route_advanced_protection, route_dynamic_memory_management};
