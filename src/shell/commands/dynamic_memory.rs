//! Dynamic Memory Management Commands
//!
//! Re-exports all dynamic memory command handlers from focused modules.
//! This maintains compatibility while providing organized, focused components.

#[path = "dynamic_memory_context.rs"]
mod dynamic_memory_context;
#[path = "dynamic_memory_core.rs"]
mod dynamic_memory_core;
#[path = "dynamic_memory_growth.rs"]
mod dynamic_memory_growth;
#[path = "dynamic_memory_lazy.rs"]
mod dynamic_memory_lazy;
#[path = "dynamic_memory_pressure.rs"]
mod dynamic_memory_pressure;
#[path = "dynamic_memory_status.rs"]
mod dynamic_memory_status;

// Note: Re-exports removed as these functions are not used in the current
// command-line shell They are kept as modules for future reference and
// automated testing
