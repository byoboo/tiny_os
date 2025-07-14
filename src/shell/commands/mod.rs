//! Shell command handlers
//!
//! This module contains all the individual command handlers organized by
//! category.

pub mod advanced_protection;
pub mod benchmark; // Performance benchmarking commands
pub mod dynamic_memory;
pub mod dynamic_memory_context;
pub mod dynamic_memory_core;
pub mod dynamic_memory_growth;
pub mod dynamic_memory_lazy;
pub mod dynamic_memory_pressure;
pub mod dynamic_memory_status;
pub mod exceptions;
pub mod filesystem;

pub mod hardware;
pub mod memory;
pub mod process;
pub mod system;
pub mod testing;
pub mod user_space;

// Week 4: Advanced hardware integration commands
// pub mod week4_advanced; // Temporarily disabled until format! issues resolved

// Week 4: Simple hardware integration commands
pub mod week4_simple;
