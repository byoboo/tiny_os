//! Hardware command modules
//!
//! This module contains organized hardware command handlers, decomposed
//! from the original monolithic hardware.rs file for better maintainability.

pub mod led;
pub mod interrupts;
pub mod exceptions;
pub mod sdcard;
pub mod deferred;

// Re-export all command handlers for backward compatibility
pub use led::{handle_led_on, handle_led_off, handle_led_toggle};
pub use interrupts::{
    handle_interrupt_status, handle_interrupt_toggle, handle_interrupt_test,
    handle_irq_integration_test, handle_nested_interrupt_test,
};
pub use exceptions::{
    handle_exception_stats, handle_exception_test, handle_exception_test_advanced,
    handle_esr_test, handle_syscall_test, handle_memory_fault_test,
};
pub use sdcard::{handle_sdcard_info, handle_sdcard_read, handle_sdcard_write};
pub use deferred::{handle_deferred_processing_test};

// All major hardware command categories have been extracted into focused modules!
// The modular architecture is complete and ready for production use.
