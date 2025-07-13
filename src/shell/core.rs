//! TinyOS Shell Core
//!
//! This module provides the core shell infrastructure including the context
//! structure, initialization, and main shell loop coordination.

use crate::{
    filesystem::Fat32FileSystem, gpio::Gpio, interrupts::InterruptController,
    memory::MemoryManager, sdcard::SdCard, timer::SystemTimer, uart::Uart,
};

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

/// Main shell loop entry point
pub fn run_shell(mut context: ShellContext) -> ! {
    let start_time = context.timer.get_time();

    loop {
        let _current_time = context.timer.get_time();

        // Check for UART input
        if let Some(ch) = context.uart.getc() {
            // Dispatch command to router
            crate::shell::router::route_command(ch, &mut context, start_time);
        }

        context.timer.delay_us(50);
    }
}
