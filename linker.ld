ENTRY(_start)

SECTIONS
{
    /* Raspberry Pi 4/5 loads kernel at 0x80000 */
    . = 0x80000;
    
    .text : {
        KEEP(*(.text.boot))
        *(.text)
    }
    
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
