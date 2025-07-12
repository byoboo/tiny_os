//! Stack Management Assembly Functions
//!
//! ARM64 assembly functions for stack switching, privilege level management,
//! and stack pointer manipulation.

.section .text

//! Switch to a new stack
//! Args: x0 = new stack pointer
//! Returns: x0 = old stack pointer
.global switch_to_stack
.type switch_to_stack, %function
switch_to_stack:
    // Save current stack pointer
    mov x1, sp
    
    // Switch to new stack
    mov sp, x0
    
    // Return old stack pointer
    mov x0, x1
    ret

//! Get current stack pointer
//! Returns: x0 = current stack pointer
.global get_current_sp
.type get_current_sp, %function
get_current_sp:
    mov x0, sp
    ret

//! Setup EL0 stack pointer
//! Args: x0 = stack pointer value
.global setup_el0_stack
.type setup_el0_stack, %function
setup_el0_stack:
    msr sp_el0, x0
    ret

//! Setup EL1 stack pointer
//! Args: x0 = stack pointer value
.global setup_el1_stack
.type setup_el1_stack, %function
setup_el1_stack:
    msr sp_el1, x0
    ret

//! Switch to EL0 with specified stack
//! Args: x0 = EL0 stack pointer, x1 = entry point
.global switch_to_el0
.type switch_to_el0, %function
switch_to_el0:
    // Setup EL0 stack pointer
    msr sp_el0, x0
    
    // Setup EL0 entry point
    msr elr_el1, x1
    
    // Setup SPSR for EL0 (AArch64, interrupts enabled)
    mov x2, #0x0
    msr spsr_el1, x2
    
    // Return to EL0
    eret

//! Switch back to EL1 from EL0
//! This is typically handled by exception return
.global switch_to_el1
.type switch_to_el1, %function
switch_to_el1:
    // This function is called from EL0 via SVC
    // The actual switching is handled by the exception handler
    svc #0
    ret

//! Get current exception level
//! Returns: x0 = current exception level (0-3)
.global get_current_el
.type get_current_el, %function
get_current_el:
    mrs x0, CurrentEL
    lsr x0, x0, #2
    ret

//! Safe stack switching with context preservation
//! Args: x0 = new stack pointer, x1 = context save area
//! Returns: x0 = old stack pointer
.global safe_switch_stack
.type safe_switch_stack, %function
safe_switch_stack:
    // Save current context to provided area
    stp x29, x30, [x1, #0]    // Frame pointer and link register
    stp x27, x28, [x1, #16]   // Callee-saved registers
    stp x25, x26, [x1, #32]
    stp x23, x24, [x1, #48]
    stp x21, x22, [x1, #64]
    stp x19, x20, [x1, #80]
    
    // Save current stack pointer
    mov x2, sp
    str x2, [x1, #96]
    
    // Switch to new stack
    mov sp, x0
    
    // Return old stack pointer
    mov x0, x2
    ret

//! Restore context from safe stack switch
//! Args: x0 = context save area
.global restore_stack_context
.type restore_stack_context, %function
restore_stack_context:
    // Restore context from save area
    ldp x29, x30, [x0, #0]    // Frame pointer and link register
    ldp x27, x28, [x0, #16]   // Callee-saved registers
    ldp x25, x26, [x0, #32]
    ldp x23, x24, [x0, #48]
    ldp x21, x22, [x0, #64]
    ldp x19, x20, [x0, #80]
    
    // Restore stack pointer
    ldr x1, [x0, #96]
    mov sp, x1
    
    ret

//! Stack overflow detection check
//! Args: x0 = stack base, x1 = stack limit
//! Returns: x0 = 1 if overflow detected, 0 otherwise
.global check_stack_overflow
.type check_stack_overflow, %function
check_stack_overflow:
    // Get current stack pointer
    mov x2, sp
    
    // Check if SP is below stack limit
    cmp x2, x1
    b.lo overflow_detected
    
    // Check if SP is above stack base
    cmp x2, x0
    b.hi overflow_detected
    
    // No overflow
    mov x0, #0
    ret

overflow_detected:
    mov x0, #1
    ret
