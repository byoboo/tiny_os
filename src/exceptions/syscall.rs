//! System Call Interface for TinyOS
//!
//! This module provides the foundation for system call handling in TinyOS.
//! It implements the SVC (SuperVisor Call) instruction handler and basic
//! system call dispatcher as outlined in Phase 1 of the enhancement plan.

use crate::uart::Uart;

/// System call numbers - Phase 1 basic implementation
#[repr(u64)]
#[derive(Debug, Clone, Copy)]
pub enum SyscallNumber {
    /// Debug print syscall - print a string to UART
    DebugPrint = 0,
    /// Get system time (placeholder - returns a dummy value)
    GetTime = 1,
    /// Get process ID (placeholder - returns 0)
    GetPid = 2,
    /// Exit process (placeholder - returns to shell)
    Exit = 3,
    /// Invalid syscall number
    Invalid = 0xFFFF,
}

impl From<u64> for SyscallNumber {
    fn from(value: u64) -> Self {
        match value {
            0 => SyscallNumber::DebugPrint,
            1 => SyscallNumber::GetTime,
            2 => SyscallNumber::GetPid,
            3 => SyscallNumber::Exit,
            _ => SyscallNumber::Invalid,
        }
    }
}

/// System call return codes
#[repr(i64)]
#[derive(Debug, Clone, Copy)]
pub enum SyscallResult {
    Success = 0,
    InvalidSyscall = -1,
    InvalidParameter = -2,
    NotImplemented = -3,
}

/// System call dispatcher - handles SVC exceptions
pub fn handle_syscall(syscall_number: u64, _args: &[u64; 6]) -> SyscallResult {
    let syscall = SyscallNumber::from(syscall_number);

    match syscall {
        SyscallNumber::DebugPrint => {
            // For now, just print a debug message
            // In a full implementation, args[0] would be a pointer to the string
            let mut uart = Uart::new();
            uart.init();
            uart.puts("SYSCALL: Debug print called\r\n");
            SyscallResult::Success
        }

        SyscallNumber::GetTime => {
            // Return a dummy timestamp for now
            // In a full implementation, this would return the actual system time
            SyscallResult::Success
        }

        SyscallNumber::GetPid => {
            // Return process ID 0 for now (kernel process)
            SyscallResult::Success
        }

        SyscallNumber::Exit => {
            // For now, just print exit message
            let mut uart = Uart::new();
            uart.init();
            uart.puts("SYSCALL: Process exit called\r\n");
            SyscallResult::Success
        }

        SyscallNumber::Invalid => SyscallResult::InvalidSyscall,
    }
}

/// Helper function to make a system call from user code
/// This is a placeholder for when we implement user/kernel mode separation
pub fn make_syscall(syscall_number: SyscallNumber, args: &[u64; 6]) -> i64 {
    // For now, just call the handler directly
    // In a full implementation, this would use the SVC instruction
    let result = handle_syscall(syscall_number as u64, args);
    result as i64
}

/// Test function to validate system call interface
pub fn test_syscall_interface() -> bool {
    let mut uart = Uart::new();
    uart.init();
    uart.puts("Testing system call interface...\r\n");

    // Test valid syscalls
    let args = [0; 6];
    let result = handle_syscall(SyscallNumber::DebugPrint as u64, &args);
    if result as i64 != SyscallResult::Success as i64 {
        uart.puts("❌ DebugPrint syscall failed\r\n");
        return false;
    }

    let result = handle_syscall(SyscallNumber::GetTime as u64, &args);
    if result as i64 != SyscallResult::Success as i64 {
        uart.puts("❌ GetTime syscall failed\r\n");
        return false;
    }

    // Test invalid syscall
    let result = handle_syscall(999, &args);
    if result as i64 != SyscallResult::InvalidSyscall as i64 {
        uart.puts("❌ Invalid syscall handling failed\r\n");
        return false;
    }

    uart.puts("✅ System call interface tests passed\r\n");
    true
}

/// System call statistics
#[derive(Debug, Clone, Copy)]
pub struct SyscallStats {
    pub total_syscalls: u64,
    pub debug_print_calls: u64,
    pub get_time_calls: u64,
    pub get_pid_calls: u64,
    pub exit_calls: u64,
    pub invalid_calls: u64,
}

impl SyscallStats {
    pub const fn new() -> Self {
        Self {
            total_syscalls: 0,
            debug_print_calls: 0,
            get_time_calls: 0,
            get_pid_calls: 0,
            exit_calls: 0,
            invalid_calls: 0,
        }
    }
}

/// Global system call statistics
pub static mut SYSCALL_STATS: SyscallStats = SyscallStats::new();

/// Get current system call statistics
pub fn get_syscall_stats() -> SyscallStats {
    unsafe { SYSCALL_STATS }
}
