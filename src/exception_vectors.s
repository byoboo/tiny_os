//! ARM64 Exception Vector Table
//! 
//! The ARM64 exception vector table contains 16 entries, each 128 bytes apart.
//! The table is organized as 4 groups of 4 exception types:
//! 
//! Group 1: Current EL with SP_EL0
//! Group 2: Current EL with SP_ELx  
//! Group 3: Lower EL using AArch64
//! Group 4: Lower EL using AArch32
//!
//! Each group has 4 exception types:
//! - Synchronous
//! - IRQ
//! - FIQ
//! - SError

.macro save_context
    // Save all general purpose registers
    stp x0, x1, [sp, #-16]!
    stp x2, x3, [sp, #-16]!
    stp x4, x5, [sp, #-16]!
    stp x6, x7, [sp, #-16]!
    stp x8, x9, [sp, #-16]!
    stp x10, x11, [sp, #-16]!
    stp x12, x13, [sp, #-16]!
    stp x14, x15, [sp, #-16]!
    stp x16, x17, [sp, #-16]!
    stp x18, x19, [sp, #-16]!
    stp x20, x21, [sp, #-16]!
    stp x22, x23, [sp, #-16]!
    stp x24, x25, [sp, #-16]!
    stp x26, x27, [sp, #-16]!
    stp x28, x29, [sp, #-16]!
    str x30, [sp, #-8]!
    
    // Save system registers
    mrs x0, elr_el1
    mrs x1, spsr_el1
    mrs x2, esr_el1
    mrs x3, far_el1
    stp x0, x1, [sp, #-16]!
    stp x2, x3, [sp, #-16]!
.endm

.macro restore_context
    // Restore system registers
    ldp x2, x3, [sp], #16
    ldp x0, x1, [sp], #16
    msr far_el1, x3
    msr esr_el1, x2
    msr spsr_el1, x1
    msr elr_el1, x0
    
    // Restore general purpose registers
    ldr x30, [sp], #8
    ldp x28, x29, [sp], #16
    ldp x26, x27, [sp], #16
    ldp x24, x25, [sp], #16
    ldp x22, x23, [sp], #16
    ldp x20, x21, [sp], #16
    ldp x18, x19, [sp], #16
    ldp x16, x17, [sp], #16
    ldp x14, x15, [sp], #16
    ldp x12, x13, [sp], #16
    ldp x10, x11, [sp], #16
    ldp x8, x9, [sp], #16
    ldp x6, x7, [sp], #16
    ldp x4, x5, [sp], #16
    ldp x2, x3, [sp], #16
    ldp x0, x1, [sp], #16
.endm

.macro exception_entry handler:req, level:req
    save_context
    
    // Prepare arguments for handler
    mov x0, sp          // ExceptionContext pointer
    mov x1, #\level     // Exception level
    
    // Call the handler
    bl \handler
    
    restore_context
    eret
.endm

// Align vector table to 2KB boundary (required by ARM64)
.align 11
.global exception_vector_table
exception_vector_table:

//
// Current EL with SP_EL0
//
.align 7
curr_el_sp0_sync:
    exception_entry handle_sync_exception, 0

.align 7
curr_el_sp0_irq:
    exception_entry handle_irq_exception, 0

.align 7
curr_el_sp0_fiq:
    exception_entry handle_fiq_exception, 0

.align 7
curr_el_sp0_serror:
    exception_entry handle_serror_exception, 0

//
// Current EL with SP_ELx
//
.align 7
curr_el_spx_sync:
    exception_entry handle_sync_exception, 1

.align 7
curr_el_spx_irq:
    exception_entry handle_irq_exception, 1

.align 7
curr_el_spx_fiq:
    exception_entry handle_fiq_exception, 1

.align 7
curr_el_spx_serror:
    exception_entry handle_serror_exception, 1

//
// Lower EL using AArch64
//
.align 7
lower_el_aarch64_sync:
    exception_entry handle_sync_exception, 2

.align 7
lower_el_aarch64_irq:
    exception_entry handle_irq_exception, 2

.align 7
lower_el_aarch64_fiq:
    exception_entry handle_fiq_exception, 2

.align 7
lower_el_aarch64_serror:
    exception_entry handle_serror_exception, 2

//
// Lower EL using AArch32
//
.align 7
lower_el_aarch32_sync:
    exception_entry handle_sync_exception, 3

.align 7
lower_el_aarch32_irq:
    exception_entry handle_irq_exception, 3

.align 7
lower_el_aarch32_fiq:
    exception_entry handle_fiq_exception, 3

.align 7
lower_el_aarch32_serror:
    exception_entry handle_serror_exception, 3
