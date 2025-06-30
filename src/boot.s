.section .text.boot

.global _start

_start:
    // Get CPU ID
    mrs x1, mpidr_el1
    and x1, x1, #3
    
    // Only CPU 0 should continue, others should halt
    cbz x1, setup_stack
    
halt:
    wfe
    b halt

setup_stack:
    // Set up stack pointer
    ldr x1, =__stack_end
    mov sp, x1
    
    // Clear BSS section
    ldr x1, =__bss_start
    ldr x2, =__bss_end
    
clear_bss:
    cmp x1, x2
    b.ge start_kernel
    str xzr, [x1], #8
    b clear_bss
    
start_kernel:
    // Jump to Rust code
    bl _start_rust
    
    // Should never reach here
    b halt

.global _start_rust
