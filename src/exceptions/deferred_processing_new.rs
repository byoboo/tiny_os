//! Deferred Processing Module - Coordinator
//!
//! This file serves as a coordinator for the modular deferred processing implementation.
//! The original monolithic deferred processing code has been broken down into focused modules
//! for better maintainability and architecture.

#[path = "deferred/mod.rs"]
pub mod deferred_module;

// Re-export all public items from the modular implementation
pub use deferred_module::*;
