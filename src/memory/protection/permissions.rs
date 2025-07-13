//! Page Permission Management
//!
//! This module provides fine-grained page permission control for
//! advanced memory protection features.

/// Page permission flags for advanced protection
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PagePermissions {
    /// Page can be read
    pub read: bool,
    /// Page can be written
    pub write: bool,
    /// Page can be executed
    pub execute: bool,
    /// Page is accessible from user mode
    pub user_accessible: bool,
    /// Page is kernel-only
    pub kernel_only: bool,
    /// Page is protected by stack canary
    pub stack_protected: bool,
}

impl PagePermissions {
    /// Create default permissions for user data pages
    pub const fn user_data() -> Self {
        Self {
            read: true,
            write: true,
            execute: false,
            user_accessible: true,
            kernel_only: false,
            stack_protected: false,
        }
    }

    /// Create default permissions for user code pages
    pub const fn user_code() -> Self {
        Self {
            read: true,
            write: false,
            execute: true,
            user_accessible: true,
            kernel_only: false,
            stack_protected: false,
        }
    }

    /// Create default permissions for kernel pages
    pub const fn kernel_only() -> Self {
        Self {
            read: true,
            write: true,
            execute: false,
            user_accessible: false,
            kernel_only: true,
            stack_protected: false,
        }
    }

    /// Create permissions for stack pages with protection
    pub const fn stack_page() -> Self {
        Self {
            read: true,
            write: true,
            execute: false,
            user_accessible: true,
            kernel_only: false,
            stack_protected: true,
        }
    }

    /// Create read-only permissions
    pub const fn read_only() -> Self {
        Self {
            read: true,
            write: false,
            execute: false,
            user_accessible: true,
            kernel_only: false,
            stack_protected: false,
        }
    }

    /// Create no permissions
    pub const fn none() -> Self {
        Self {
            read: false,
            write: false,
            execute: false,
            user_accessible: false,
            kernel_only: false,
            stack_protected: false,
        }
    }

    /// Add read permission
    pub const fn add_read(mut self) -> Self {
        self.read = true;
        self
    }

    /// Add write permission
    pub const fn add_write(mut self) -> Self {
        self.write = true;
        self
    }

    /// Add execute permission
    pub const fn add_execute(mut self) -> Self {
        self.execute = true;
        self
    }

    /// Check if readable
    pub const fn is_readable(&self) -> bool {
        self.read
    }

    /// Check if writable
    pub const fn is_writable(&self) -> bool {
        self.write
    }

    /// Check if executable
    pub const fn is_executable(&self) -> bool {
        self.execute
    }
}

/// Types of permission faults
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PermissionFaultType {
    Read,
    Write,
    Execute,
    UserAccess,
    // Backward compatibility aliases
    ReadViolation,
    WriteViolation,
    ExecuteViolation,
    UserAccessViolation,
}

/// Results of permission fault handling
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PermissionFaultResult {
    Continue,
    Terminate,
    Retry,
}

/// Advanced memory protection statistics
#[derive(Debug, Clone, Copy)]
pub struct AdvancedProtectionStats {
    /// Number of pages with custom permissions
    pub protected_pages: usize,
    /// Number of permission faults handled
    pub permission_faults: usize,
    /// Number of stack canary violations
    pub canary_violations: usize,
    /// Number of CFI violations
    pub cfi_violations: usize,
    /// Whether ASLR is enabled
    pub aslr_enabled: bool,
    /// Whether CFI is enabled
    pub cfi_enabled: bool,
    /// Number of ASLR randomizations performed
    pub aslr_randomizations: usize,
    /// Number of successful stack protections
    pub stack_protections: usize,
    /// Number of return address mismatches
    pub return_address_mismatches: usize,
    
    // Backward compatibility fields
    /// Total number of protected pages (alias for protected_pages)
    pub total_protected_pages: usize,
    /// Number of read-only pages
    pub read_only_pages: usize,
    /// Number of non-executable pages
    pub non_executable_pages: usize,
    /// Number of user-accessible protected pages
    pub user_protected_pages: usize,
    /// Number of protected stacks (alias for stack_protections)
    pub protected_stacks: usize,
    /// Number of active stack canaries
    pub stack_canaries_active: usize,
    /// Number of stack violations (alias for canary_violations)
    pub stack_violations: usize,
    /// Number of ROP attacks blocked
    pub rop_attacks_blocked: usize,
    /// Number of faults handled
    pub faults_handled: usize,
    /// Number of faults that resulted in termination
    pub faults_terminated: usize,
}

impl AdvancedProtectionStats {
    /// Create new statistics with default values
    pub const fn new() -> Self {
        Self {
            protected_pages: 0,
            permission_faults: 0,
            canary_violations: 0,
            cfi_violations: 0,
            aslr_enabled: false,
            cfi_enabled: false,
            aslr_randomizations: 0,
            stack_protections: 0,
            return_address_mismatches: 0,
            total_protected_pages: 0,
            read_only_pages: 0,
            non_executable_pages: 0,
            user_protected_pages: 0,
            protected_stacks: 0,
            stack_canaries_active: 0,
            stack_violations: 0,
            rop_attacks_blocked: 0,
            faults_handled: 0,
            faults_terminated: 0,
        }
    }
}

/// Information about a protected memory page
#[derive(Debug, Clone, Copy)]
pub struct ProtectedPage {
    /// Virtual address of the page
    pub virtual_addr: u64,
    /// Physical address of the page
    pub physical_addr: u64,
    /// Page permissions
    pub permissions: PagePermissions,
    /// Process ID that owns this page (0 for kernel)
    pub process_id: usize,
}

impl ProtectedPage {
    /// Create a new protected page
    pub const fn new(
        virtual_addr: u64,
        physical_addr: u64,
        permissions: PagePermissions,
        process_id: usize,
    ) -> Self {
        Self {
            virtual_addr,
            physical_addr,
            permissions,
            process_id,
        }
    }
}
