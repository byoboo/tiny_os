//! Hardware command modules
//!
//! This module contains organized hardware command handlers, decomposed
//! from the original monolithic hardware.rs file for better maintainability.

pub mod deferred;
pub mod exceptions;
pub mod interrupts;
pub mod led;
pub mod sdcard;

// Note: Re-exports removed as these functions are not used in the current command-line shell
// They are kept as modules for future reference and automated testing
// pub use deferred::handle_deferred_processing_test;
// pub use exceptions::{
//     handle_esr_test, handle_exception_stats, handle_exception_test, handle_exception_test_advanced,
//     handle_memory_fault_test, handle_syscall_test,
// };
// pub use interrupts::{
//     handle_interrupt_status, handle_interrupt_test, handle_interrupt_toggle,
//     handle_irq_integration_test, handle_nested_interrupt_test,
// };
// pub use led::{handle_led_off, handle_led_on, handle_led_toggle};
// pub use sdcard::{handle_sdcard_info, handle_sdcard_read, handle_sdcard_write};

// All major hardware command categories have been extracted into focused
// modules! The modular architecture is complete and ready for production use.
