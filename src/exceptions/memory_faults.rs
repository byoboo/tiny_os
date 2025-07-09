//! Memory Fault Analysis for TinyOS
//!
//! This module provides comprehensive memory fault analysis capabilities
//! for data aborts, instruction aborts, and other memory-related exceptions.
//! It implements Phase 1 memory fault handling as outlined in the enhancement plan.
//! In Phase 4, this integrates with MMU exception handling for advanced memory management.

use core::arch::asm;
use crate::uart::Uart;
use crate::memory::{MemoryManager, handle_mmu_exception_global, MmuRecoveryAction};

/// Memory fault types based on ESR_EL1 exception class
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryFaultType {
    /// Data abort from current exception level
    DataAbortCurrentEL,
    /// Data abort from lower exception level
    DataAbortLowerEL,
    /// Instruction abort from current exception level
    InstructionAbortCurrentEL,
    /// Instruction abort from lower exception level
    InstructionAbortLowerEL,
    /// Translation fault
    TranslationFault,
    /// Permission fault
    PermissionFault,
    /// Alignment fault
    AlignmentFault,
    /// Unknown fault type
    Unknown,
}

/// Memory access type that caused the fault
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryAccessType {
    /// Read access
    Read,
    /// Write access
    Write,
    /// Execute access
    Execute,
    /// Unknown access type
    Unknown,
}

/// Memory fault information extracted from system registers
#[derive(Debug, Clone, Copy)]
pub struct MemoryFaultInfo {
    /// Type of memory fault
    pub fault_type: MemoryFaultType,
    /// Address that caused the fault (from FAR_EL1)
    pub fault_address: u64,
    /// Access type that caused the fault
    pub access_type: MemoryAccessType,
    /// Instruction address where fault occurred
    pub instruction_address: u64,
    /// Additional fault status information
    pub fault_status: u32,
    /// Whether the fault is valid
    pub is_valid: bool,
}

impl MemoryFaultInfo {
    /// Create a new memory fault info structure
    pub fn new(fault_type: MemoryFaultType, fault_address: u64, access_type: MemoryAccessType) -> Self {
        Self {
            fault_type,
            fault_address,
            access_type,
            instruction_address: 0,
            fault_status: 0,
            is_valid: true,
        }
    }
    
    /// Create an invalid fault info
    pub fn invalid() -> Self {
        Self {
            fault_type: MemoryFaultType::Unknown,
            fault_address: 0,
            access_type: MemoryAccessType::Unknown,
            instruction_address: 0,
            fault_status: 0,
            is_valid: false,
        }
    }
}

/// Memory fault analyzer - decodes memory faults from ESR_EL1 and other registers
pub struct MemoryFaultAnalyzer;

impl MemoryFaultAnalyzer {
    /// Analyze a memory fault based on ESR_EL1 value
    pub fn analyze_fault(esr_el1: u32) -> MemoryFaultInfo {
        let exception_class = (esr_el1 >> 26) & 0x3F;
        let fault_address = Self::read_far_el1();
        
        let fault_type = match exception_class {
            0x24 => MemoryFaultType::DataAbortCurrentEL,
            0x25 => MemoryFaultType::DataAbortLowerEL,
            0x20 => MemoryFaultType::InstructionAbortCurrentEL,
            0x21 => MemoryFaultType::InstructionAbortLowerEL,
            _ => MemoryFaultType::Unknown,
        };
        
        // Extract access type from ISS (bits 0-24)
        let iss = esr_el1 & 0x1FFFFFF;
        let access_type = if (iss & (1 << 6)) != 0 {
            MemoryAccessType::Write
        } else {
            MemoryAccessType::Read
        };
        
        let mut fault_info = MemoryFaultInfo::new(fault_type, fault_address, access_type);
        fault_info.fault_status = iss;
        
        fault_info
    }
    
    /// Read the Fault Address Register (FAR_EL1)
    fn read_far_el1() -> u64 {
        #[cfg(target_arch = "aarch64")]
        {
            let far_el1: u64;
            unsafe {
                asm!("mrs {}, far_el1", out(reg) far_el1);
            }
            far_el1
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            // For unit tests on host platform, return a mock value
            0x0000_0000_DEAD_BEEF
        }
    }
    
    /// Read the Exception Link Register (ELR_EL1) to get instruction address
    fn read_elr_el1() -> u64 {
        #[cfg(target_arch = "aarch64")]
        {
            let elr_el1: u64;
            unsafe {
                asm!("mrs {}, elr_el1", out(reg) elr_el1);
            }
            elr_el1
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            // For unit tests on host platform, return a mock value
            0x0000_0000_CAFE_BABE
        }
    }
    
    /// Classify fault based on fault status
    pub fn classify_fault_detail(fault_status: u32) -> &'static str {
        let dfsc = fault_status & 0x3F; // Data Fault Status Code
        
        match dfsc {
            0b000100 => "Translation fault, level 0",
            0b000101 => "Translation fault, level 1", 
            0b000110 => "Translation fault, level 2",
            0b000111 => "Translation fault, level 3",
            0b001001 => "Access flag fault, level 1",
            0b001010 => "Access flag fault, level 2",
            0b001011 => "Access flag fault, level 3",
            0b001101 => "Permission fault, level 1",
            0b001110 => "Permission fault, level 2",
            0b001111 => "Permission fault, level 3",
            0b100001 => "Alignment fault",
            _ => "Unknown fault",
        }
    }
    
    /// Generate a detailed fault report
    pub fn generate_fault_report(fault_info: &MemoryFaultInfo) -> [u8; 512] {
        let mut report = [0u8; 512];
        let mut uart = Uart::new();
        uart.init();
        
        uart.puts("=== Memory Fault Analysis Report ===\r\n");
        uart.puts("Fault Type: ");
        match fault_info.fault_type {
            MemoryFaultType::DataAbortCurrentEL => uart.puts("Data Abort (Current EL)"),
            MemoryFaultType::DataAbortLowerEL => uart.puts("Data Abort (Lower EL)"),
            MemoryFaultType::InstructionAbortCurrentEL => uart.puts("Instruction Abort (Current EL)"),
            MemoryFaultType::InstructionAbortLowerEL => uart.puts("Instruction Abort (Lower EL)"),
            MemoryFaultType::TranslationFault => uart.puts("Translation Fault"),
            MemoryFaultType::PermissionFault => uart.puts("Permission Fault"),
            MemoryFaultType::AlignmentFault => uart.puts("Alignment Fault"),
            MemoryFaultType::Unknown => uart.puts("Unknown Fault"),
        }
        uart.puts("\r\n");
        
        uart.puts("Fault Address: 0x");
        Self::print_hex(&uart, fault_info.fault_address);
        uart.puts("\r\n");
        
        uart.puts("Access Type: ");
        match fault_info.access_type {
            MemoryAccessType::Read => uart.puts("Read"),
            MemoryAccessType::Write => uart.puts("Write"),
            MemoryAccessType::Execute => uart.puts("Execute"),
            MemoryAccessType::Unknown => uart.puts("Unknown"),
        }
        uart.puts("\r\n");
        
        uart.puts("Fault Detail: ");
        uart.puts(Self::classify_fault_detail(fault_info.fault_status));
        uart.puts("\r\n");
        
        report
    }
    
    /// Helper function to print hex values
    fn print_hex(uart: &Uart, value: u64) {
        for i in (0..16).rev() {
            let nibble = (value >> (i * 4)) & 0xF;
            let hex_char = match nibble {
                0..=9 => (b'0' + nibble as u8) as char,
                10..=15 => (b'A' + (nibble - 10) as u8) as char,
                _ => '?',
            };
            uart.putc(hex_char as u8);
        }
    }
}

/// Memory fault statistics
#[derive(Debug, Clone, Copy)]
pub struct MemoryFaultStats {
    pub total_faults: u64,
    pub data_aborts: u64,
    pub instruction_aborts: u64,
    pub translation_faults: u64,
    pub permission_faults: u64,
    pub alignment_faults: u64,
    pub unknown_faults: u64,
}

impl MemoryFaultStats {
    pub const fn new() -> Self {
        Self {
            total_faults: 0,
            data_aborts: 0,
            instruction_aborts: 0,
            translation_faults: 0,
            permission_faults: 0,
            alignment_faults: 0,
            unknown_faults: 0,
        }
    }
    
    /// Record a memory fault
    pub fn record_fault(&mut self, fault_type: MemoryFaultType) {
        self.total_faults += 1;
        
        match fault_type {
            MemoryFaultType::DataAbortCurrentEL | MemoryFaultType::DataAbortLowerEL => {
                self.data_aborts += 1;
            }
            MemoryFaultType::InstructionAbortCurrentEL | MemoryFaultType::InstructionAbortLowerEL => {
                self.instruction_aborts += 1;
            }
            MemoryFaultType::TranslationFault => {
                self.translation_faults += 1;
            }
            MemoryFaultType::PermissionFault => {
                self.permission_faults += 1;
            }
            MemoryFaultType::AlignmentFault => {
                self.alignment_faults += 1;
            }
            MemoryFaultType::Unknown => {
                self.unknown_faults += 1;
            }
        }
    }
}

/// Global memory fault statistics
pub static mut MEMORY_FAULT_STATS: MemoryFaultStats = MemoryFaultStats::new();

/// Get current memory fault statistics
pub fn get_memory_fault_stats() -> MemoryFaultStats {
    unsafe { MEMORY_FAULT_STATS }
}

/// Integrate memory fault analysis with MMU exception handling (Phase 4)
pub fn handle_memory_fault_with_mmu(
    esr_el1: u32,
    far_el1: u64,
    elr_el1: u64,
    user_mode: bool,
    memory_manager: &mut MemoryManager,
) -> MmuRecoveryAction {
    // First, analyze the fault using our existing system
    let fault_info = MemoryFaultAnalyzer::analyze_fault(esr_el1);
    
    // Update statistics
    unsafe {
        MEMORY_FAULT_STATS.record_fault(fault_info.fault_type);
    }
    
    // Print fault information for debugging
    let mut uart = Uart::new();
    uart.init();
    uart.puts("MMU Memory Fault Integration:\r\n");
    uart.puts("Fault Address: 0x");
    MemoryFaultAnalyzer::print_hex(&uart, far_el1);
    uart.puts("\r\nInstruction Address: 0x");
    MemoryFaultAnalyzer::print_hex(&uart, elr_el1);
    uart.puts("\r\n");
    
    // Delegate to MMU exception handler for advanced processing
    handle_mmu_exception_global(esr_el1, far_el1, user_mode, elr_el1, memory_manager)
}

/// Test function to validate memory fault analysis
pub fn test_memory_fault_analysis() -> bool {
    let mut uart = Uart::new();
    uart.init();
    uart.puts("Testing memory fault analysis...\r\n");
    
    // Test fault analysis with different ESR values
    let test_cases = [
        (0x24000000, MemoryFaultType::DataAbortCurrentEL),
        (0x25000000, MemoryFaultType::DataAbortLowerEL),
        (0x20000000, MemoryFaultType::InstructionAbortCurrentEL),
        (0x21000000, MemoryFaultType::InstructionAbortLowerEL),
    ];
    
    for (esr_value, expected_type) in test_cases.iter() {
        let fault_info = MemoryFaultAnalyzer::analyze_fault(*esr_value);
        if fault_info.fault_type != *expected_type {
            uart.puts("❌ Fault type analysis failed\r\n");
            return false;
        }
    }
    
    uart.puts("✅ Memory fault analysis tests passed\r\n");
    true
}
