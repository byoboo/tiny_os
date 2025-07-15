//! Shell command handlers
//!
//! This module contains all the individual command handlers organized by
//! category.

pub mod benchmark; // Performance benchmarking commands
pub mod dynamic_memory;
pub mod editor; // Text editor commands
pub mod exceptions;
pub mod filesystem;

pub mod hardware;
pub mod memory;
pub mod process;
pub mod system;
pub mod testing;
pub mod user_space;

// Modular feature commands (Project Baseline)
pub mod performance; // Performance monitoring and power management
pub mod network;     // Network and high-speed I/O
pub mod security;    // Security, real-time, and system hardening
