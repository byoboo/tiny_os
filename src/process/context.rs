// TinyOS Process Context Management
// Phase 3.1: Process Context Management

use crate::exceptions::types::ExceptionContext;

/// Process state enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    /// Process is ready to run
    Ready,
    /// Process is currently running
    Running,
    /// Process is blocked waiting for an event
    Blocked,
    /// Process is terminated
    Terminated,
}

/// Extended process context including FPU and vector registers
#[derive(Debug, Clone)]
pub struct ProcessContext {
    /// Basic exception context (general purpose registers)
    pub exception_context: ExceptionContext,
    
    /// Process state
    pub state: ProcessState,
    
    /// Process ID
    pub pid: u32,
    
    /// Stack pointer for user mode
    pub user_stack_pointer: u64,
    
    /// Stack pointer for kernel mode
    pub kernel_stack_pointer: u64,
    
    /// Program counter (ELR_EL1)
    pub program_counter: u64,
    
    /// Processor state (SPSR_EL1)
    pub processor_state: u64,
    
    /// FPU context (placeholder for now)
    pub fpu_context: [u64; 32], // 32 NEON/FPU registers
    
    /// Vector registers context (placeholder for now)
    pub vector_context: [u64; 32], // 32 vector registers
    
    /// Process priority
    pub priority: u8,
    
    /// Time slice remaining (in timer ticks)
    pub time_slice: u32,
    
    /// Total CPU time used (in timer ticks)
    pub cpu_time: u64,
    
    /// Context switch count
    pub context_switches: u64,
}

impl ProcessContext {
    /// Create a new process context
    pub fn new(pid: u32, user_stack: u64, kernel_stack: u64, entry_point: u64) -> Self {
        Self {
            exception_context: ExceptionContext::new(),
            state: ProcessState::Ready,
            pid,
            user_stack_pointer: user_stack,
            kernel_stack_pointer: kernel_stack,
            program_counter: entry_point,
            processor_state: 0x0000_0000_0000_0000, // EL0 mode, interrupts enabled
            fpu_context: [0; 32],
            vector_context: [0; 32],
            priority: 5, // Default priority
            time_slice: 1000, // Default time slice
            cpu_time: 0,
            context_switches: 0,
        }
    }
    
    /// Save current context from hardware registers
    pub fn save_context(&mut self) -> ContextSwitchResult {
        // Save general purpose registers from exception context
        // This would normally be done by the exception handler
        // For now, we'll just mark the context as saved
        
        // Save FPU context (ARM64 specific)
        self.save_fpu_context();
        
        // Save vector registers
        self.save_vector_context();
        
        // Update statistics
        self.context_switches += 1;
        crate::process::record_context_switch();
        
        ContextSwitchResult::Success
    }
    
    /// Restore context to hardware registers
    pub fn restore_context(&self) -> ContextSwitchResult {
        // Restore general purpose registers
        // This would normally be done by the exception handler
        // For now, we'll just return success
        
        // Restore FPU context
        self.restore_fpu_context();
        
        // Restore vector registers
        self.restore_vector_context();
        
        // Set stack pointers based on privilege level
        self.set_stack_pointers();
        
        ContextSwitchResult::Success
    }
    
    /// Save FPU context
    fn save_fpu_context(&mut self) {
        // ARM64 FPU context saving
        #[cfg(target_arch = "aarch64")]
        unsafe {
            // Save NEON/FPU registers
            for i in 0..32 {
                match i {
                    0 => core::arch::asm!("str q0, [{}]", in(reg) &mut self.fpu_context[i]),
                    1 => core::arch::asm!("str q1, [{}]", in(reg) &mut self.fpu_context[i]),
                    2 => core::arch::asm!("str q2, [{}]", in(reg) &mut self.fpu_context[i]),
                    3 => core::arch::asm!("str q3, [{}]", in(reg) &mut self.fpu_context[i]),
                    // ... would continue for all 32 registers
                    _ => self.fpu_context[i] = 0, // Placeholder for remaining registers
                }
            }
        }
        
        #[cfg(not(target_arch = "aarch64"))]
        {
            // Mock FPU context for unit tests
            for i in 0..32 {
                self.fpu_context[i] = 0xDEAD_BEEF_0000_0000 + i as u64;
            }
        }
    }
    
    /// Restore FPU context
    fn restore_fpu_context(&self) {
        // ARM64 FPU context restoration
        #[cfg(target_arch = "aarch64")]
        unsafe {
            // Restore NEON/FPU registers
            for i in 0..4 { // Just first 4 as example
                match i {
                    0 => core::arch::asm!("ldr q0, [{}]", in(reg) &self.fpu_context[i]),
                    1 => core::arch::asm!("ldr q1, [{}]", in(reg) &self.fpu_context[i]),
                    2 => core::arch::asm!("ldr q2, [{}]", in(reg) &self.fpu_context[i]),
                    3 => core::arch::asm!("ldr q3, [{}]", in(reg) &self.fpu_context[i]),
                    _ => {}
                }
            }
        }
        
        #[cfg(not(target_arch = "aarch64"))]
        {
            // Mock FPU context for unit tests - nothing to do
        }
    }
    
    /// Save vector registers context
    fn save_vector_context(&mut self) {
        // Placeholder for vector register saving
        // In real implementation, would save ARM64 vector registers
        for i in 0..32 {
            self.vector_context[i] = 0xCAFE_BABE_0000_0000 + i as u64;
        }
    }
    
    /// Restore vector registers context
    fn restore_vector_context(&self) {
        // Placeholder for vector register restoration
        // In real implementation, would restore ARM64 vector registers
    }
    
    /// Set stack pointers based on privilege level
    fn set_stack_pointers(&self) {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            // Set user stack pointer (SP_EL0)
            core::arch::asm!("msr sp_el0, {}", in(reg) self.user_stack_pointer);
            
            // Set kernel stack pointer (SP_EL1)
            core::arch::asm!("mov sp, {}", in(reg) self.kernel_stack_pointer);
        }
        
        #[cfg(not(target_arch = "aarch64"))]
        {
            // Mock for unit tests - nothing to do
        }
    }
    
    /// Update process state
    pub fn set_state(&mut self, new_state: ProcessState) {
        self.state = new_state;
    }
    
    /// Get process state
    pub fn get_state(&self) -> ProcessState {
        self.state
    }
    
    /// Update time slice
    pub fn set_time_slice(&mut self, time_slice: u32) {
        self.time_slice = time_slice;
    }
    
    /// Decrement time slice
    pub fn decrement_time_slice(&mut self) -> bool {
        if self.time_slice > 0 {
            self.time_slice -= 1;
            self.time_slice == 0
        } else {
            true
        }
    }
    
    /// Add CPU time
    pub fn add_cpu_time(&mut self, time: u64) {
        self.cpu_time += time;
    }
    
    /// Check if process is ready to run
    pub fn is_ready(&self) -> bool {
        self.state == ProcessState::Ready
    }
    
    /// Check if process is running
    pub fn is_running(&self) -> bool {
        self.state == ProcessState::Running
    }
    
    /// Check if process is blocked
    pub fn is_blocked(&self) -> bool {
        self.state == ProcessState::Blocked
    }
    
    /// Check if process is terminated
    pub fn is_terminated(&self) -> bool {
        self.state == ProcessState::Terminated
    }
}

/// Result of context switch operation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContextSwitchResult {
    /// Context switch successful
    Success,
    /// Context switch failed - invalid state
    InvalidState,
    /// Context switch failed - hardware error
    HardwareError,
    /// Context switch failed - memory error
    MemoryError,
}

/// Process context manager
pub struct ProcessContextManager {
    /// Current process context
    current_context: Option<ProcessContext>,
    
    /// Context switch statistics
    context_switches: u64,
    
    /// Context switch failures
    context_switch_failures: u64,
}

impl ProcessContextManager {
    /// Create a new process context manager
    pub const fn new() -> Self {
        Self {
            current_context: None,
            context_switches: 0,
            context_switch_failures: 0,
        }
    }
    
    /// Set current process context
    pub fn set_current_context(&mut self, context: ProcessContext) {
        self.current_context = Some(context);
    }
    
    /// Get current process context
    pub fn get_current_context(&self) -> Option<&ProcessContext> {
        self.current_context.as_ref()
    }
    
    /// Get mutable current process context
    pub fn get_current_context_mut(&mut self) -> Option<&mut ProcessContext> {
        self.current_context.as_mut()
    }
    
    /// Perform context switch
    pub fn context_switch(&mut self, new_context: ProcessContext) -> ContextSwitchResult {
        // Save current context if exists
        if let Some(ref mut current) = self.current_context {
            if let ContextSwitchResult::Success = current.save_context() {
                // Set current context to not running
                current.set_state(ProcessState::Ready);
            } else {
                self.context_switch_failures += 1;
                return ContextSwitchResult::HardwareError;
            }
        }
        
        // Restore new context
        match new_context.restore_context() {
            ContextSwitchResult::Success => {
                self.current_context = Some(new_context);
                if let Some(ref mut current) = self.current_context {
                    current.set_state(ProcessState::Running);
                }
                self.context_switches += 1;
                ContextSwitchResult::Success
            }
            error => {
                self.context_switch_failures += 1;
                error
            }
        }
    }
    
    /// Get context switch statistics
    pub fn get_stats(&self) -> (u64, u64) {
        (self.context_switches, self.context_switch_failures)
    }
}

/// Global process context manager
static mut CONTEXT_MANAGER: ProcessContextManager = ProcessContextManager::new();

/// Initialize process context management
pub fn init_process_context_management() {
    unsafe {
        CONTEXT_MANAGER = ProcessContextManager::new();
    }
}

/// Get current process context
pub fn get_current_context() -> Option<ProcessContext> {
    unsafe { 
        let manager = core::ptr::addr_of!(CONTEXT_MANAGER);
        (*manager).get_current_context().cloned()
    }
}

/// Set current process context
pub fn set_current_context(context: ProcessContext) {
    unsafe {
        let manager = core::ptr::addr_of_mut!(CONTEXT_MANAGER);
        (*manager).set_current_context(context);
    }
}

/// Perform context switch
pub fn context_switch(new_context: ProcessContext) -> ContextSwitchResult {
    unsafe { 
        let manager = core::ptr::addr_of_mut!(CONTEXT_MANAGER);
        (*manager).context_switch(new_context)
    }
}

/// Get context management statistics
pub fn get_context_stats() -> (u64, u64) {
    unsafe { 
        let manager = core::ptr::addr_of!(CONTEXT_MANAGER);
        (*manager).get_stats()
    }
}
