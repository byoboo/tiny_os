//! TinyOS Interactive Shell
//!
//! This module provides the interactive shell interface for TinyOS.
//! The shell now features a modern command-line interface similar to Linux/Unix shells
//! with command parsing, execution, and standard filesystem operations.

mod commands;
mod core;
mod parser;
mod executor;

// Re-export core shell functionality
pub use core::{run_shell, ShellContext};
pub use parser::{Command, CommandInput, CommandCompletion};
pub use executor::{CommandExecutor, CommandResult};
