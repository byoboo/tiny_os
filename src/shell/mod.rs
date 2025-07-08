//! TinyOS Interactive Shell
//!
//! This module provides the interactive shell interface for TinyOS.
//! The shell handles command parsing, execution, and user interaction.

use core::option::Option::Some;

use crate::{
    fat32::Fat32FileSystem, gpio::Gpio, interrupts::InterruptController, memory::MemoryManager,
    sdcard::SdCard, timer::SystemTimer, uart::Uart,
};

mod commands;
use commands::*;

/// Shell context containing system components
pub struct ShellContext {
    pub uart: Uart,
    pub gpio: Gpio,
    pub timer: SystemTimer,
    pub memory_manager: MemoryManager,
    pub interrupt_controller: InterruptController,
    pub sdcard: SdCard,
    pub fat32_fs: Option<Fat32FileSystem>,
    pub led_state: bool,
}

impl ShellContext {
    /// Create a new shell context with initialized components
    pub fn new(
        uart: Uart,
        gpio: Gpio,
        timer: SystemTimer,
        memory_manager: MemoryManager,
        interrupt_controller: InterruptController,
        sdcard: SdCard,
        fat32_fs: Option<Fat32FileSystem>,
    ) -> Self {
        Self {
            uart,
            gpio,
            timer,
            memory_manager,
            interrupt_controller,
            sdcard,
            fat32_fs,
            led_state: false,
        }
    }
}

/// Main shell loop
pub fn run_shell(mut context: ShellContext) -> ! {
    let start_time = context.timer.get_time();

    loop {
        let _current_time = context.timer.get_time();

        // Check for UART input
        if let Some(ch) = context.uart.getc() {
            // Route command to appropriate handler
            match ch {
                // System commands
                b'h' | b'H' => system::handle_help(&context),
                b't' | b'T' => system::handle_time(&context, start_time),
                b's' | b'S' => system::handle_system_info(&context),
                b'c' | b'C' => system::handle_health_check(&mut context),

                // Hardware commands
                b'1' => {
                    hardware::handle_led_on(&mut context);
                    context.led_state = true;
                }
                b'0' => {
                    hardware::handle_led_off(&mut context);
                    context.led_state = false;
                }
                b'l' | b'L' => {
                    hardware::handle_led_toggle(&mut context);
                }
                b'i' | b'I' => hardware::handle_interrupt_status(&context),
                b'e' | b'E' => hardware::handle_interrupt_toggle(&mut context),
                b'j' | b'J' => hardware::handle_interrupt_test(&mut context),
                b'v' | b'V' => hardware::handle_exception_stats(&context),
                b'w' | b'W' => hardware::handle_exception_test(&context),
                b'p' | b'P' => hardware::handle_sdcard_info(&context),
                b'q' | b'Q' => hardware::handle_sdcard_read(&mut context),
                b'y' | b'Y' => hardware::handle_sdcard_write(&mut context),

                // Memory commands
                b'm' | b'M' => memory::handle_memory_stats(&context.uart, &context.memory_manager),
                b'a' | b'A' => {
                    memory::handle_memory_allocate(&context.uart, &mut context.memory_manager)
                }
                b'f' | b'F' => {
                    memory::handle_memory_free(&context.uart, &mut context.memory_manager)
                }
                b'x' | b'X' => {
                    memory::handle_memory_test(&context.uart, &mut context.memory_manager)
                }
                b'z' | b'Z' => memory::handle_comprehensive_memory_test(
                    &context.uart,
                    &mut context.memory_manager,
                ),
                b'g' | b'G' => {
                    memory::handle_memory_corruption_check(&context.uart, &context.memory_manager)
                }
                b'r' | b'R' => {
                    memory::handle_memory_defragment(&context.uart, &mut context.memory_manager)
                }

                // Filesystem commands
                b'd' | b'D' => {
                    filesystem::handle_directory_listing(&context.uart, &mut context.fat32_fs)
                }
                b'n' | b'N' => {
                    filesystem::handle_filesystem_mount_info(&context.uart, &mut context.fat32_fs)
                }
                b'o' | b'O' => {
                    filesystem::handle_change_directory(&context.uart, &mut context.fat32_fs)
                }
                b'u' | b'U' => filesystem::handle_read_file(&context.uart, &mut context.fat32_fs),
                b'k' | b'K' => {
                    filesystem::handle_change_to_root(&context.uart, &mut context.fat32_fs)
                }

                // Unknown command
                _ => {
                    if (32..=126).contains(&ch) {
                        context.uart.puts("Unknown command. Type 'h' for help.\r\n");
                    } else {
                        context
                            .uart
                            .puts("Non-printable character (control code)\r\n");
                    }
                }
            }
        }

        context.timer.delay_us(50);
    }
}
