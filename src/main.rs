#![no_std]
#![no_main]

#[cfg(target_arch = "aarch64")]
use core::arch::global_asm;
use core::{
    include_str,
    option::Option::{None, Some},
    panic::PanicInfo,
    result::Result::{Err, Ok},
};

// Import everything from the library crate
use tiny_os_lib::{
    drivers::{
        gpio::{Gpio, GpioFunction},
        sdcard::SdCard,
        timer::SystemTimer,
        uart::Uart,
    },
    exceptions::init_exceptions,
    fat32::Fat32FileSystem,
    interrupts::InterruptController,
    memory::{init_mmu_exceptions, MemoryManager},
    process,
    shell::{run_shell, ShellContext},
};

// Include the boot assembly
#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn kernel_main() {
    // Initialize UART for output
    let mut uart = Uart::new();
    uart.init();

    uart.puts("TinyOS Starting...\r\n");
    uart.puts("================================\r\n");

    // Initialize system components
    uart.puts("Initializing system components...\r\n");

    // Initialize exceptions
    init_exceptions();
    uart.puts("✓ Exception handling initialized\r\n");

    // Initialize MMU exception handling
    init_mmu_exceptions();
    uart.puts("✓ MMU exception handling initialized\r\n");

    // Initialize process management
    process::init_process_management();
    uart.puts("✓ Process management initialized\r\n");

    // Initialize GPIO
    let gpio = Gpio::new();

    // Configure LED pin (GPIO 42 on Raspberry Pi)
    gpio.set_function(42, GpioFunction::Output);
    uart.puts("✓ GPIO initialized (LED on pin 42)\r\n");

    // Initialize system timer
    let timer = SystemTimer::new();
    uart.puts("✓ System timer initialized\r\n");

    // Initialize memory manager
    let memory_manager = MemoryManager::new();
    uart.puts("✓ Memory manager initialized\r\n");

    // Initialize interrupt controller
    let mut interrupt_controller = InterruptController::new();
    interrupt_controller.init();
    uart.puts("✓ Interrupt controller initialized\r\n");

    // Initialize SD Card (defer FAT32 mounting to avoid stack overflow)
    uart.puts("About to initialize SD Card...\r\n");

    // Create a stub SD card to avoid hardware initialization in QEMU
    let mut sdcard = SdCard::new();
    uart.puts("SD Card object created\r\n");

    let fat32_fs: Option<Fat32FileSystem> = None;

    // For now, skip SD card initialization in QEMU to prevent hanging
    // This can be re-enabled when proper QEMU emulation is available
    let _sd_init_success = false;
    uart.puts("SD Card initialization skipped (QEMU compatibility)\r\n");

    // System ready
    uart.puts("================================\r\n");
    uart.puts("✓ TinyOS Ready!\r\n");
    uart.puts("Available commands (type 'h' for help):\r\n");
    uart.puts("================================\r\n");

    // Create shell context and start the shell
    let shell_context = ShellContext::new(
        uart,
        gpio,
        timer,
        memory_manager,
        interrupt_controller,
        sdcard,
        fat32_fs,
    );

    // Start the interactive shell (this never returns)
    run_shell(shell_context);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut uart = Uart::new();
    uart.init();
    uart.puts("KERNEL PANIC!\r\n");
    loop {}
}
