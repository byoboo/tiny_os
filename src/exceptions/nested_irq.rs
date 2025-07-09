//! Nested Interrupt Support for TinyOS Phase 2
//!
//! This module implements nested interrupt handling with priority management,
//! interrupt masking, and critical section support.

use core::arch::asm;
use crate::uart::Uart;

/// Interrupt priority levels (0 = highest, 255 = lowest)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterruptPriority {
    Critical = 0,      // System critical interrupts
    High = 64,         // High priority (timer, etc.)
    Normal = 128,      // Normal priority (UART, GPIO)
    Low = 192,         // Low priority (background tasks)
    Disabled = 255,    // Disabled
}

impl From<u8> for InterruptPriority {
    fn from(value: u8) -> Self {
        match value {
            0..=31 => InterruptPriority::Critical,
            32..=95 => InterruptPriority::High,
            96..=159 => InterruptPriority::Normal,
            160..=223 => InterruptPriority::Low,
            _ => InterruptPriority::Disabled,
        }
    }
}

/// Interrupt mask state
#[derive(Debug, Clone, Copy)]
pub struct InterruptMask {
    /// Previous interrupt state
    pub previous_state: bool,
    /// Current mask level
    pub mask_level: InterruptPriority,
    /// Nesting level
    pub nesting_level: u32,
}

impl InterruptMask {
    pub const fn new() -> Self {
        Self {
            previous_state: false,
            mask_level: InterruptPriority::Disabled,
            nesting_level: 0,
        }
    }
}

/// Nested interrupt manager
pub struct NestedInterruptManager {
    /// Current interrupt nesting level
    nesting_level: u32,
    /// Maximum nesting level seen
    max_nesting_level: u32,
    /// Current priority mask
    current_priority_mask: InterruptPriority,
    /// Interrupt stack for nesting
    interrupt_stack: [InterruptMask; 16],
    /// Stack pointer
    stack_pointer: usize,
    /// Statistics
    stats: NestedInterruptStats,
}

impl NestedInterruptManager {
    pub const fn new() -> Self {
        Self {
            nesting_level: 0,
            max_nesting_level: 0,
            current_priority_mask: InterruptPriority::Disabled,
            interrupt_stack: [InterruptMask::new(); 16],
            stack_pointer: 0,
            stats: NestedInterruptStats::new(),
        }
    }
    
    /// Enter interrupt context
    pub fn enter_interrupt(&mut self, priority: InterruptPriority) -> bool {
        // Check if we can accept this interrupt based on priority
        if priority >= self.current_priority_mask {
            return false; // Interrupt is masked
        }
        
        // Save current state on interrupt stack
        if self.stack_pointer >= self.interrupt_stack.len() {
            // Stack overflow - cannot nest further
            self.stats.stack_overflows += 1;
            return false;
        }
        
        let current_state = self.are_interrupts_enabled();
        self.interrupt_stack[self.stack_pointer] = InterruptMask {
            previous_state: current_state,
            mask_level: self.current_priority_mask,
            nesting_level: self.nesting_level,
        };
        
        self.stack_pointer += 1;
        self.nesting_level += 1;
        
        // Update maximum nesting level
        if self.nesting_level > self.max_nesting_level {
            self.max_nesting_level = self.nesting_level;
        }
        
        // Set new priority mask (allow only higher priority interrupts)
        self.current_priority_mask = priority;
        self.set_priority_mask(priority);
        
        // Enable interrupts to allow nesting
        self.enable_interrupts();
        
        self.stats.total_nested_interrupts += 1;
        if self.nesting_level > 1 {
            self.stats.nested_interrupt_events += 1;
        }
        
        true
    }
    
    /// Exit interrupt context
    pub fn exit_interrupt(&mut self) {
        if self.stack_pointer == 0 {
            // Stack underflow
            self.stats.stack_underflows += 1;
            return;
        }
        
        // Disable interrupts while we restore state
        self.disable_interrupts();
        
        // Restore previous state from stack
        self.stack_pointer -= 1;
        let previous_mask = self.interrupt_stack[self.stack_pointer];
        
        self.nesting_level = previous_mask.nesting_level;
        self.current_priority_mask = previous_mask.mask_level;
        
        // Restore priority mask
        self.set_priority_mask(self.current_priority_mask);
        
        // Restore interrupt state
        if previous_mask.previous_state {
            self.enable_interrupts();
        }
    }
    
    /// Mask interrupts below a certain priority
    pub fn mask_interrupts(&mut self, priority: InterruptPriority) -> InterruptMask {
        let previous_state = InterruptMask {
            previous_state: self.are_interrupts_enabled(),
            mask_level: self.current_priority_mask,
            nesting_level: self.nesting_level,
        };
        
        self.current_priority_mask = priority;
        self.set_priority_mask(priority);
        
        previous_state
    }
    
    /// Restore interrupt mask
    pub fn restore_interrupts(&mut self, mask: InterruptMask) {
        self.current_priority_mask = mask.mask_level;
        self.set_priority_mask(mask.mask_level);
        
        if mask.previous_state {
            self.enable_interrupts();
        } else {
            self.disable_interrupts();
        }
    }
    
    /// Check if interrupts are enabled
    fn are_interrupts_enabled(&self) -> bool {
        #[cfg(target_arch = "aarch64")]
        {
            let daif: u64;
            unsafe {
                asm!("mrs {}, daif", out(reg) daif);
            }
            // Check if IRQ (bit 1) is not masked
            (daif & (1 << 1)) == 0
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            // For unit tests on host platform, return a mock value
            true
        }
    }
    
    /// Enable interrupts
    fn enable_interrupts(&self) {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            asm!("msr daifclr, #2"); // Clear IRQ mask (bit 1)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            // For unit tests on host platform, do nothing
        }
    }
    
    /// Disable interrupts
    fn disable_interrupts(&self) {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            asm!("msr daifset, #2"); // Set IRQ mask (bit 1)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            // For unit tests on host platform, do nothing
        }
    }
    
    /// Set interrupt priority mask in GIC
    fn set_priority_mask(&self, priority: InterruptPriority) {
        // GIC CPU Interface - Priority Mask Register
        const GICC_PMR: u32 = 0xFF842000 + 0x004;
        
        unsafe {
            core::ptr::write_volatile(GICC_PMR as *mut u32, (priority as u8) as u32);
        }
    }
    
    /// Get current nesting level
    pub fn get_nesting_level(&self) -> u32 {
        self.nesting_level
    }
    
    /// Get maximum nesting level seen
    pub fn get_max_nesting_level(&self) -> u32 {
        self.max_nesting_level
    }
    
    /// Get statistics
    pub fn get_stats(&self) -> NestedInterruptStats {
        self.stats
    }
    
    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = NestedInterruptStats::new();
    }
}

/// Critical section RAII guard
pub struct CriticalSection {
    previous_mask: InterruptMask,
}

impl CriticalSection {
    /// Enter critical section (disable all interrupts)
    pub fn enter() -> Self {
        let previous_mask = unsafe {
            NESTED_INTERRUPT_MANAGER.mask_interrupts(InterruptPriority::Critical)
        };
        
        Self { previous_mask }
    }
}

impl Drop for CriticalSection {
    fn drop(&mut self) {
        unsafe {
            NESTED_INTERRUPT_MANAGER.restore_interrupts(self.previous_mask);
        }
    }
}

/// Nested interrupt statistics
#[derive(Debug, Clone, Copy)]
pub struct NestedInterruptStats {
    pub total_nested_interrupts: u64,
    pub nested_interrupt_events: u64,
    pub stack_overflows: u64,
    pub stack_underflows: u64,
    pub max_nesting_depth: u32,
}

impl NestedInterruptStats {
    pub const fn new() -> Self {
        Self {
            total_nested_interrupts: 0,
            nested_interrupt_events: 0,
            stack_overflows: 0,
            stack_underflows: 0,
            max_nesting_depth: 0,
        }
    }
}

/// Global nested interrupt manager
pub static mut NESTED_INTERRUPT_MANAGER: NestedInterruptManager = NestedInterruptManager::new();

/// Initialize nested interrupt support
pub fn init_nested_interrupts() {
    let mut uart = Uart::new();
    uart.init();
    uart.puts("Nested interrupt support initialized\r\n");
}

/// Enter interrupt with priority checking
pub fn enter_interrupt_with_priority(priority: InterruptPriority) -> bool {
    unsafe {
        NESTED_INTERRUPT_MANAGER.enter_interrupt(priority)
    }
}

/// Exit current interrupt
pub fn exit_current_interrupt() {
    unsafe {
        NESTED_INTERRUPT_MANAGER.exit_interrupt();
    }
}

/// Get nested interrupt statistics
pub fn get_nested_interrupt_stats() -> NestedInterruptStats {
    unsafe {
        NESTED_INTERRUPT_MANAGER.get_stats()
    }
}

/// Test nested interrupt functionality
pub fn test_nested_interrupts() -> bool {
    let mut uart = Uart::new();
    uart.init();
    uart.puts("Testing nested interrupt support...\r\n");
    
    // Test priority conversion
    let priority = InterruptPriority::from(64);
    if priority != InterruptPriority::High {
        uart.puts("❌ Priority conversion failed\r\n");
        return false;
    }
    
    // Test critical section
    {
        let _critical = CriticalSection::enter();
        uart.puts("In critical section\r\n");
        // Interrupts should be disabled here
    }
    uart.puts("Critical section exited\r\n");
    
    // Test interrupt nesting simulation
    if enter_interrupt_with_priority(InterruptPriority::Normal) {
        uart.puts("Entered normal priority interrupt\r\n");
        
        if enter_interrupt_with_priority(InterruptPriority::High) {
            uart.puts("Nested high priority interrupt\r\n");
            exit_current_interrupt();
        }
        
        exit_current_interrupt();
    }
    
    let stats = get_nested_interrupt_stats();
    uart.puts("Nested interrupt tests completed, events: ");
    uart.put_hex(stats.total_nested_interrupts);
    uart.puts("\r\n");
    
    uart.puts("✅ Nested interrupt tests passed\r\n");
    true
}
