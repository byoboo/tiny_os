ENTRY(_start)

SECTIONS
{
    /* Raspberry Pi 4/5 loads kernel at 0x80000 */
    . = 0x80000;
    
    /* Place all code sections at 0x80000, with _start first */
    .text : {
        /* Force _start to be at 0x80000 */
        KEEP(*(.text._start*))
        KEEP(*(.text.boot))
        *(.text*)
    }
    
    /* Read-only data after code */
    .rodata : {
        *(.rodata)
    }
    
    .data : {
        *(.data)
    }
    
    .bss : {
        __bss_start = .;
        *(.bss)
        __bss_end = .;
    }
    
    /* Stack grows downward from 0x80000 */
    __stack_end = 0x80000;
}
