// TinyOS Privilege Level Management
// Phase 3.2: User/Kernel Mode Separation

use crate::exceptions::types::ExceptionContext;

/// Privilege levels in ARM64
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrivilegeLevel {
    /// Exception Level 0 - User mode
    EL0,
    /// Exception Level 1 - Kernel mode  
    EL1,
    /// Exception Level 2 - Hypervisor mode (not used in TinyOS)
    EL2,
    /// Exception Level 3 - Secure monitor mode (not used in TinyOS)
    EL3,
}

impl PrivilegeLevel {
    /// Get privilege level from SPSR_EL1
    pub fn from_spsr(spsr: u64) -> Self {
        match (spsr >> 2) & 0x3 {
            0 => PrivilegeLevel::EL0,
            1 => PrivilegeLevel::EL1,
            2 => PrivilegeLevel::EL2,
            3 => PrivilegeLevel::EL3,
            _ => unreachable!(),
        }
    }

    /// Convert to SPSR_EL1 bits
    pub fn to_spsr_bits(self) -> u64 {
        match self {
            PrivilegeLevel::EL0 => 0x0,
            PrivilegeLevel::EL1 => 0x4,
            PrivilegeLevel::EL2 => 0x8,
            PrivilegeLevel::EL3 => 0xC,
        }
    }

    /// Check if this is user mode
    pub fn is_user_mode(self) -> bool {
        matches!(self, PrivilegeLevel::EL0)
    }

    /// Check if this is kernel mode
    pub fn is_kernel_mode(self) -> bool {
        matches!(self, PrivilegeLevel::EL1)
    }
}

/// EL0 to EL1 transition information
#[derive(Debug, Clone)]
pub struct EL0ToEL1Transition {
    /// Exception syndrome register
    pub esr_el1: u64,
    /// Exception link register (return address)
    pub elr_el1: u64,
    /// Saved program status register
    pub spsr_el1: u64,
    /// Fault address register (if applicable)
    pub far_el1: u64,
    /// System call number (if syscall)
    pub syscall_number: Option<u64>,
    /// System call arguments (if syscall)
    pub syscall_args: [u64; 6],
    /// Transition timestamp
    pub timestamp: u64,
}

impl EL0ToEL1Transition {
    /// Create a new EL0 to EL1 transition
    pub fn new(esr: u64, elr: u64, spsr: u64, far: u64) -> Self {
        Self {
            esr_el1: esr,
            elr_el1: elr,
            spsr_el1: spsr,
            far_el1: far,
            syscall_number: None,
            syscall_args: [0; 6],
            timestamp: crate::timer::driver::get_system_time(),
        }
    }

    /// Set syscall information
    pub fn set_syscall(&mut self, syscall_num: u64, args: [u64; 6]) {
        self.syscall_number = Some(syscall_num);
        self.syscall_args = args;
    }

    /// Check if this is a syscall transition
    pub fn is_syscall(&self) -> bool {
        self.syscall_number.is_some()
    }
}

/// Privilege manager for handling user/kernel mode transitions
pub struct PrivilegeManager {
    /// Current privilege level
    current_level: PrivilegeLevel,

    /// User mode stack pointer
    user_stack_pointer: u64,

    /// Kernel mode stack pointer
    kernel_stack_pointer: u64,

    /// Statistics
    el0_to_el1_transitions: u64,
    el1_to_el0_transitions: u64,
    privilege_violations: u64,
    syscall_count: u64,
}

impl PrivilegeManager {
    /// Create a new privilege manager
    pub const fn new() -> Self {
        Self {
            current_level: PrivilegeLevel::EL1, // Start in kernel mode
            user_stack_pointer: 0,
            kernel_stack_pointer: 0,
            el0_to_el1_transitions: 0,
            el1_to_el0_transitions: 0,
            privilege_violations: 0,
            syscall_count: 0,
        }
    }

    /// Initialize privilege manager
    pub fn init(&mut self) {
        self.current_level = PrivilegeLevel::EL1;
        self.setup_stack_pointers();
    }

    /// Setup initial stack pointers
    fn setup_stack_pointers(&mut self) {
        // Use more conservative memory addresses for QEMU compatibility
        // These addresses should be within valid memory regions
        self.kernel_stack_pointer = 0x0800_0000; // Lower memory for kernel stack
        self.user_stack_pointer = 0x0400_0000; // Even lower memory for user stack

        #[cfg(target_arch = "aarch64")]
        unsafe {
            // Set SP_EL0 for user mode (but don't modify current stack pointer)
            core::arch::asm!("msr sp_el0, {}", in(reg) self.user_stack_pointer);

            // Don't change the current stack pointer during initialization
            // as we're already running on a valid stack
            // The kernel stack will be set up during actual context switches
        }
    }

    /// Get current privilege level
    pub fn get_current_level(&self) -> PrivilegeLevel {
        self.current_level
    }

    /// Check if currently in user mode
    pub fn is_user_mode(&self) -> bool {
        self.current_level.is_user_mode()
    }

    /// Check if currently in kernel mode
    pub fn is_kernel_mode(&self) -> bool {
        self.current_level.is_kernel_mode()
    }

    /// Transition from EL0 to EL1 (user to kernel)
    pub fn transition_to_el1(&mut self, context: &ExceptionContext) -> EL0ToEL1Transition {
        if self.current_level != PrivilegeLevel::EL0 {
            self.privilege_violations += 1;
            crate::process::record_privilege_violation();
        }

        // Read system registers
        let esr = self.read_esr_el1();
        let elr = self.read_elr_el1();
        let spsr = self.read_spsr_el1();
        let far = self.read_far_el1();

        let mut transition = EL0ToEL1Transition::new(esr, elr, spsr, far);

        // Check if this is a syscall
        if (esr >> 26) & 0x3F == 0x15 {
            // SVC instruction
            let syscall_num = context.gpr[8]; // ARM64 syscall convention (x8)
            let args = [
                context.gpr[0],
                context.gpr[1],
                context.gpr[2],
                context.gpr[3],
                context.gpr[4],
                context.gpr[5],
            ];
            transition.set_syscall(syscall_num, args);
            self.syscall_count += 1;
        }

        self.current_level = PrivilegeLevel::EL1;
        self.el0_to_el1_transitions += 1;
        crate::process::record_privilege_escalation();

        transition
    }

    /// Transition from EL1 to EL0 (kernel to user)
    pub fn transition_to_el0(
        &mut self,
        return_address: u64,
        return_value: u64,
    ) -> Result<(), &'static str> {
        if self.current_level != PrivilegeLevel::EL1 {
            self.privilege_violations += 1;
            return Err("Invalid privilege level for EL0 transition");
        }

        // Set up return to user mode
        self.write_elr_el1(return_address);
        self.write_spsr_el1(PrivilegeLevel::EL0.to_spsr_bits());

        // Set return value in x0
        #[cfg(target_arch = "aarch64")]
        unsafe {
            core::arch::asm!("mov x0, {}", in(reg) return_value);
        }

        self.current_level = PrivilegeLevel::EL0;
        self.el1_to_el0_transitions += 1;

        Ok(())
    }

    /// Validate privilege level for operation
    pub fn validate_privilege(
        &mut self,
        required_level: PrivilegeLevel,
    ) -> Result<(), &'static str> {
        match (self.current_level, required_level) {
            (PrivilegeLevel::EL0, PrivilegeLevel::EL1) => {
                self.privilege_violations += 1;
                crate::process::record_privilege_violation();
                Err("Insufficient privilege: EL1 required")
            }
            (PrivilegeLevel::EL0, PrivilegeLevel::EL2) => {
                self.privilege_violations += 1;
                crate::process::record_privilege_violation();
                Err("Insufficient privilege: EL2 required")
            }
            (PrivilegeLevel::EL0, PrivilegeLevel::EL3) => {
                self.privilege_violations += 1;
                crate::process::record_privilege_violation();
                Err("Insufficient privilege: EL3 required")
            }
            _ => Ok(()),
        }
    }

    /// Set user stack pointer
    pub fn set_user_stack(&mut self, stack_pointer: u64) {
        self.user_stack_pointer = stack_pointer;

        #[cfg(target_arch = "aarch64")]
        unsafe {
            core::arch::asm!("msr sp_el0, {}", in(reg) stack_pointer);
        }
    }

    /// Set kernel stack pointer
    pub fn set_kernel_stack(&mut self, stack_pointer: u64) {
        self.kernel_stack_pointer = stack_pointer;

        #[cfg(target_arch = "aarch64")]
        unsafe {
            core::arch::asm!("mov sp, {}", in(reg) stack_pointer);
        }
    }

    /// Get statistics
    pub fn get_stats(&self) -> (u64, u64, u64, u64) {
        (
            self.el0_to_el1_transitions,
            self.el1_to_el0_transitions,
            self.privilege_violations,
            self.syscall_count,
        )
    }

    /// Read ESR_EL1
    fn read_esr_el1(&self) -> u64 {
        #[cfg(target_arch = "aarch64")]
        {
            let esr: u64;
            unsafe {
                core::arch::asm!("mrs {}, esr_el1", out(reg) esr);
            }
            esr
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            0x5600_0000 // Mock SVC exception
        }
    }

    /// Read ELR_EL1
    fn read_elr_el1(&self) -> u64 {
        #[cfg(target_arch = "aarch64")]
        {
            let elr: u64;
            unsafe {
                core::arch::asm!("mrs {}, elr_el1", out(reg) elr);
            }
            elr
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            0x0000_0000_1000_0000 // Mock return address
        }
    }

    /// Read SPSR_EL1
    fn read_spsr_el1(&self) -> u64 {
        #[cfg(target_arch = "aarch64")]
        {
            let spsr: u64;
            unsafe {
                core::arch::asm!("mrs {}, spsr_el1", out(reg) spsr);
            }
            spsr
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            0x0000_0000_0000_0000 // Mock EL0 state
        }
    }

    /// Read FAR_EL1
    fn read_far_el1(&self) -> u64 {
        #[cfg(target_arch = "aarch64")]
        {
            let far: u64;
            unsafe {
                core::arch::asm!("mrs {}, far_el1", out(reg) far);
            }
            far
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            0x0000_0000_DEAD_BEEF // Mock fault address
        }
    }

    /// Write ELR_EL1
    fn write_elr_el1(&self, value: u64) {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            core::arch::asm!("msr elr_el1, {}", in(reg) value);
        }
    }

    /// Write SPSR_EL1
    fn write_spsr_el1(&self, value: u64) {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            core::arch::asm!("msr spsr_el1, {}", in(reg) value);
        }
    }
}

/// Global privilege manager
static mut PRIVILEGE_MANAGER: PrivilegeManager = PrivilegeManager::new();

/// Initialize privilege management
pub fn init_privilege_management() {
    unsafe {
        PRIVILEGE_MANAGER.init();
    }
}

/// Get current privilege level
pub fn get_current_privilege_level() -> PrivilegeLevel {
    unsafe { PRIVILEGE_MANAGER.get_current_level() }
}

/// Check if in user mode
pub fn is_user_mode() -> bool {
    unsafe { PRIVILEGE_MANAGER.is_user_mode() }
}

/// Check if in kernel mode
pub fn is_kernel_mode() -> bool {
    unsafe { PRIVILEGE_MANAGER.is_kernel_mode() }
}

/// Transition to EL1 (kernel mode)
pub fn transition_to_kernel(context: &ExceptionContext) -> EL0ToEL1Transition {
    unsafe { PRIVILEGE_MANAGER.transition_to_el1(context) }
}

/// Transition to EL0 (user mode)
pub fn transition_to_user(return_address: u64, return_value: u64) -> Result<(), &'static str> {
    unsafe { PRIVILEGE_MANAGER.transition_to_el0(return_address, return_value) }
}

/// Validate privilege for operation
pub fn validate_privilege(required_level: PrivilegeLevel) -> Result<(), &'static str> {
    unsafe { PRIVILEGE_MANAGER.validate_privilege(required_level) }
}

/// Set user stack pointer
pub fn set_user_stack(stack_pointer: u64) {
    unsafe { PRIVILEGE_MANAGER.set_user_stack(stack_pointer) }
}

/// Set kernel stack pointer
pub fn set_kernel_stack(stack_pointer: u64) {
    unsafe { PRIVILEGE_MANAGER.set_kernel_stack(stack_pointer) }
}

/// Get privilege management statistics
pub fn get_privilege_stats() -> (u64, u64, u64, u64) {
    unsafe { PRIVILEGE_MANAGER.get_stats() }
}
