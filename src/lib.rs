//! TinyOS Library Interface
//!
//! This module provides a library interface for TinyOS components
//! that can be used in hosted environments for testing.

// Use std only for tests, no_std for embedded
#![cfg_attr(not(test), no_std)]

// Core modules (available in no_std environments)
pub mod exceptions;
pub mod fat32;
pub mod gpio;
pub mod interrupts;
pub mod memory;
pub mod sdcard;
pub mod shell;
pub mod timer;
pub mod uart;
