#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(target_arch = "aarch64")]
use core::arch::global_asm;
use core::panic::PanicInfo;

mod uart;
use uart::Uart;

// Include the boot assembly
#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn kernel_main() {
    // Initialize UART for output
    let uart = Uart::new();
    uart.init();

    uart.puts("TinyOS Basic Test!\r\n");
    uart.puts("If you see this, the kernel is working!\r\n");

    // Simple loop
    loop {
        if let Some(ch) = uart.getc() {
            uart.puts("You pressed: ");
            uart.putc(ch);
            uart.puts("\r\n");
            
            if ch == b'q' {
                uart.puts("Quitting...\r\n");
                break;
            }
        }
    }

    // Halt
    #[cfg(target_arch = "aarch64")]
    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let uart = Uart::new();
    uart.puts("PANIC occurred!\r\n");
    
    #[cfg(target_arch = "aarch64")]
    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    unreachable!()
}
