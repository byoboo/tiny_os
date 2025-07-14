//! TinyOS Library Interface
//!
//! This module provides a library interface for TinyOS components
//! that can be used in hosted environments for testing.

// Use no_std for embedded target
#![no_std]

// Core modules (available in no_std environments)
pub mod benchmarks; // Performance measurement and optimization validation
// (temporarily disabled)
pub mod drivers; // New modular driver system
pub mod exceptions;
pub mod filesystem; // New modular filesystem system
pub mod interrupts;
pub mod memory;
pub mod optimization; // Week 3: Hardware optimization framework
pub mod process; // New process management system
pub mod shell;
pub mod testing; // Testing framework
pub mod utils; // Utility functions for no_std environment

// Legacy filesystem module (for backward compatibility)
// This re-exports types from the new modular filesystem
pub mod fat32 {
    pub use crate::filesystem::fat32::*;
}

// Legacy driver modules (for backward compatibility)
// These re-export types from the new modular drivers
pub mod gpio {
    pub use crate::drivers::gpio::*;
}

pub mod sdcard {
    pub use crate::drivers::sdcard::*;

    // Backward compatibility alias
    pub type SdError = SdCardError;
}

pub mod timer {
    pub use crate::drivers::timer::*;
}

pub mod uart {
    pub use crate::drivers::uart::*;
}
