//! Utility functions for TinyOS
//!
//! Provides no_std compatible utilities for formatting, string manipulation,
//! and other common operations needed throughout TinyOS.

pub mod formatting;

// Re-export commonly used formatting functions
pub use formatting::{
    write_number_to_buffer,
    write_hex_to_buffer, 
    write_number_with_text,
    write_hex_with_text,
    write_bool_with_text
};
