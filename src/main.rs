#![no_std]
#![no_main]

#[cfg(target_arch = "aarch64")]
use core::arch::global_asm;
use core::{
    include_str,
    option::Option::None,
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
    memory::{
        init_cow_manager, init_mmu_exceptions, init_stack_manager, init_user_space_manager,
        init_virtual_memory, MemoryManager,
    },
    process,
    shell::{run_shell, ShellContext},
};

// Include the boot assembly
#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("boot.s"));

// Include the stack management assembly
#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("stack_asm.s"));

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

    // Initialize virtual memory management
    match init_virtual_memory() {
        Ok(()) => uart.puts("✓ Virtual memory management initialized\r\n"),
        Err(e) => {
            uart.puts("⚠ Virtual memory initialization failed: ");
            uart.puts(e);
            uart.puts("\r\n");
        }
    }

    // Initialize stack management
    match init_stack_manager() {
        Ok(()) => uart.puts("✓ Stack management initialized\r\n"),
        Err(_e) => {
            uart.puts("⚠ Stack management initialization failed\r\n");
        }
    }

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
    let mut memory_manager = MemoryManager::new();
    uart.puts("✓ Memory manager initialized\r\n");

    // Initialize COW manager
    let memory_manager_ptr = &mut memory_manager as *mut MemoryManager;
    init_cow_manager(memory_manager_ptr);
    uart.puts("✓ COW manager initialized\r\n");

    // Initialize user space manager
    init_user_space_manager(memory_manager_ptr);
    uart.puts("✓ User space manager initialized\r\n");

    // Initialize advanced memory protection
    use tiny_os_lib::memory::init_advanced_memory_protection;
    init_advanced_memory_protection(memory_manager_ptr);
    uart.puts("✓ Advanced memory protection initialized\r\n");

    // Initialize dynamic memory management
    use tiny_os_lib::memory::init_dynamic_memory_manager;
    match init_dynamic_memory_manager() {
        Ok(()) => uart.puts("✓ Dynamic memory management initialized\r\n"),
        Err(_e) => uart.puts("⚠ Dynamic memory management initialization failed\r\n"),
    }

    // Initialize interrupt controller
    let mut interrupt_controller = InterruptController::new();
    interrupt_controller.init();
    uart.puts("✓ Interrupt controller initialized\r\n");

    // Week 3: Initialize VideoCore GPU integration
    uart.puts("Initializing Week 3 VideoCore GPU integration...\r\n");

    // Initialize mailbox communication
    use tiny_os_lib::drivers::mailbox;
    match mailbox::init() {
        Ok(()) => uart.puts("✓ VideoCore mailbox initialized\r\n"),
        Err(e) => {
            uart.puts("⚠ Mailbox initialization failed: ");
            uart.puts(e);
            uart.puts("\r\n");
        }
    }

    // Initialize VideoCore GPU
    use tiny_os_lib::drivers::videocore;
    match videocore::init() {
        Ok(()) => {
            uart.puts("✓ VideoCore GPU initialized\r\n");
            #[cfg(feature = "raspi3")]
            uart.puts("📝 Pi 3 VideoCore IV compatibility mode\r\n");
            #[cfg(not(feature = "raspi3"))]
            uart.puts("🚀 Pi 4/5 VideoCore VI features available\r\n");
        }
        Err(e) => {
            uart.puts("⚠ VideoCore initialization failed: ");
            uart.puts(e);
            uart.puts("\r\n");
        }
    }

    // Initialize DMA controller
    use tiny_os_lib::drivers::dma;
    let mailbox = mailbox::get_mailbox();
    // Use compile-time feature detection for hardware version
    #[cfg(feature = "raspi3")]
    let is_pi4_or_5 = false;
    #[cfg(not(feature = "raspi3"))]
    let is_pi4_or_5 = true;
    match dma::init(is_pi4_or_5) {
        Ok(()) => {
            uart.puts("✓ DMA controller initialized\r\n");
            #[cfg(feature = "raspi3")]
            uart.puts("📝 Pi 3 DMA compatibility mode\r\n");
            #[cfg(not(feature = "raspi3"))]
            uart.puts("🚀 Pi 4/5 enhanced DMA features enabled\r\n");
        }
        Err(e) => {
            uart.puts("⚠ DMA initialization failed: ");
            uart.puts(e);
            uart.puts("\r\n");
        }
    }

    // Initialize cache controller
    use tiny_os_lib::drivers::cache;
    cache::init(is_pi4_or_5);
    uart.puts("✓ Cache controller initialized\r\n");
    #[cfg(feature = "raspi3")]
    uart.puts("📝 Cortex-A53 cache compatibility mode\r\n");
    #[cfg(not(feature = "raspi3"))]
    uart.puts("🚀 Cortex-A72/A76 cache optimizations enabled\r\n");

    // Initialize optimization framework
    use tiny_os_lib::optimization;
    match optimization::init() {
        Ok(()) => uart.puts("✓ Hardware optimization framework initialized\r\n"),
        Err(e) => {
            uart.puts("⚠ Optimization framework failed: ");
            uart.puts(e);
            uart.puts("\r\n");
        }
    }

    // Initialize GPU benchmarks
    use tiny_os_lib::benchmarks::gpu_performance;
    match gpu_performance::init() {
        Ok(()) => uart.puts("✓ GPU performance benchmarks ready\r\n"),
        Err(e) => {
            uart.puts("⚠ GPU benchmarks initialization failed: ");
            uart.puts(e);
            uart.puts("\r\n");
        }
    }

    // Initialize SD Card (defer FAT32 mounting to avoid stack overflow)
    uart.puts("About to initialize SD Card...\r\n");

    // Initialize SD card with QEMU compatibility
    let mut sdcard = SdCard::new();
    uart.puts("SD Card object created\r\n");

    let mut fat32_fs: Option<Fat32FileSystem> = None;

    // Try to initialize SD card (works in QEMU with -drive option)
    match sdcard.init() {
        Ok(()) => {
            uart.puts("✓ SD Card initialized successfully\r\n");
            // Try to mount FAT32 filesystem
            match Fat32FileSystem::new(sdcard) {
                Ok(mut fs) => {
                    match fs.mount() {
                        Ok(()) => {
                            uart.puts("✓ FAT32 filesystem mounted successfully\r\n");
                            fat32_fs = Some(fs);
                            // Create a new SD card instance for shell since filesystem took ownership
                            sdcard = SdCard::new();
                        }
                        Err(_) => {
                            uart.puts("⚠ FAT32 filesystem mount failed\r\n");
                            // Create a new SD card instance for shell
                            sdcard = SdCard::new();
                        }
                    }
                }
                Err(_) => {
                    uart.puts("⚠ No FAT32 filesystem found\r\n");
                    // Create a new SD card instance for shell
                    sdcard = SdCard::new();
                }
            }
        }
        Err(_) => {
            uart.puts("⚠ SD Card initialization failed (normal in QEMU without -drive)\r\n");
            uart.puts("  Use 'make run-local' for full filesystem support\r\n");
        }
    }

    // System ready
    uart.puts("================================\r\n");
    uart.puts("✓ TinyOS Ready!\r\n");
    uart.puts("Type 'help' for available commands\r\n");
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
