//! Memory Protection Module
//!
//! This module provides comprehensive memory protection features including:
//! - Canary-based corruption detection
//! - Fine-grained page permissions
//! - Address Space Layout Randomization (ASLR)
//! - Advanced stack protection with guard pages
//! - Control Flow Integrity (CFI)
//! - Central protection management

pub mod aslr;
pub mod canary;
pub mod cfi;
pub mod manager;
pub mod permissions;
pub mod stack;

// Re-export key types and functions for compatibility
pub use canary::{CorruptionDetection, CorruptionReport, MemoryProtection};
pub use permissions::{
    AdvancedProtectionStats, PagePermissions, PermissionFaultResult, PermissionFaultType,
    ProtectedPage,
};
pub use aslr::AslrManager;
pub use stack::AdvancedStackProtection;
pub use cfi::CfiManager;
pub use manager::{
    AdvancedMemoryProtection, get_advanced_page_permissions, get_advanced_protection_stats,
    get_aslr_offset, handle_advanced_permission_fault, init_advanced_memory_protection,
    set_advanced_page_permissions, setup_advanced_stack_protection,
    verify_advanced_stack_canary, with_advanced_memory_protection,
};

// For backward compatibility, re-export everything that was in the original module
pub use canary::MemoryProtection as MemoryProtectionCompat;
pub use permissions::PagePermissions as PagePermissionsCompat;
pub use manager::AdvancedMemoryProtection as AdvancedMemoryProtectionCompat;
