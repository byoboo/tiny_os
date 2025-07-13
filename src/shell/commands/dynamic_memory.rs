//! Dynamic Memory Management Commands
//! 
//! Re-exports all dynamic memory command handlers from focused modules.
//! This maintains compatibility while providing organized, focused components.

#[path = "dynamic_memory_core.rs"]
mod dynamic_memory_core;
#[path = "dynamic_memory_status.rs"]
mod dynamic_memory_status;
#[path = "dynamic_memory_growth.rs"]
mod dynamic_memory_growth;
#[path = "dynamic_memory_lazy.rs"]
mod dynamic_memory_lazy;
#[path = "dynamic_memory_pressure.rs"]
mod dynamic_memory_pressure;
#[path = "dynamic_memory_context.rs"]
mod dynamic_memory_context;

// Re-export all command functions
pub use dynamic_memory_core::cmd_dynamic_memory;
pub use dynamic_memory_status::cmd_dynamic_memory_status;
pub use dynamic_memory_growth::cmd_dynamic_memory_growth;
pub use dynamic_memory_lazy::cmd_dynamic_memory_lazy;
pub use dynamic_memory_pressure::{cmd_dynamic_memory_pressure, cmd_dynamic_memory_optimize};
pub use dynamic_memory_context::{cmd_dynamic_memory_context, cmd_dynamic_memory_stats};
