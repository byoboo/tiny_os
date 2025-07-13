//! TinyOS Interactive Shell
//!
//! This module provides the interactive shell interface for TinyOS.
//! The shell is now organized using a modular architecture with focused
//! components for core functionality, routing, and command handling.

mod commands;
mod core;
mod router;
mod routers;

// Re-export core shell functionality
pub use core::{ShellContext, run_shell};