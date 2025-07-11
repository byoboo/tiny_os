//! TinyOS Library Interface
//!
//! This module provides a library interface for TinyOS components
//! that can be used in hosted environments for testing.

// Use std only for tests, no_std for embedded
#![cfg_attr(not(test), no_std)]

// Core modules (available in no_std environments)
pub mod exceptions;
pub mod gpio;
pub mod interrupts;
pub mod memory;
pub mod timer;
pub mod uart;

// Test modules (only compiled when testing)
#[cfg(test)]
mod simple_tests;

// Constants for testing
pub const HEAP_START: u32 = 0x100000;
pub const HEAP_SIZE: u32 = 0x400000;
pub const BLOCK_SIZE: u32 = 64;
