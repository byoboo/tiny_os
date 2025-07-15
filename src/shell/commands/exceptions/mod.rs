// TinyOS Shell Exception Commands Module
// Central coordination for exception command subsystem

pub mod control;
pub mod stats;
pub mod status;
pub mod testing;
pub mod translation;

// Re-export all command handlers for shell integration
// Note: Re-exports removed as these functions are not used in the current command-line shell
// They are kept as modules for future reference and automated testing
// pub use control::{cmd_mmu_control, cmd_mmu_enable_disable};
// pub use stats::{cmd_exception_stats, cmd_mmu_stats, cmd_reset_exception_stats};
// pub use status::cmd_virtual_memory_status;
// pub use testing::{cmd_test_exceptions, cmd_virtual_memory_test};
// pub use translation::{cmd_invalidate_tlb, cmd_translate_address};
