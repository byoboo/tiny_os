//! TinyOS Shell Core
//!
//! This module provides the core shell infrastructure including the context
//! structure, initialization, and main shell loop with command-line interface.

use super::{
    executor::{CommandExecutor, CommandResult},
    parser::{CommandCompletion, CommandInput},
};
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
    let mut input = CommandInput::new();
    let mut executor = CommandExecutor::new();
    let _completion = CommandCompletion::new();

    // Show welcome message
    show_welcome(&mut context);

    loop {
        // Show prompt
        show_prompt(&executor, &mut context);

        // Process input until we get a complete command
        let mut command_ready = false;
        while !command_ready {
            if let Some(ch) = context.uart.getc() {
                if let Some(command) = input.process_char(ch, &mut context) {
                    if !command.is_empty() {
                        // Execute the command
                        let result = executor.execute(&command, &mut context);

                        // Handle result
                        match result {
                            CommandResult::Success => {
                                // Command executed successfully
                            }
                            CommandResult::Error(msg) => {
                                context.uart.puts("Error: ");
                                context.uart.puts(msg);
                                context.uart.puts("\r\n");
                            }
                            CommandResult::NotFound => {
                                context.uart.puts("Command not found: ");
                                context.uart.puts(command.name());
                                context.uart.puts("\r\n");
                                context.uart.puts("Type 'help' for available commands.\r\n");
                            }
                            CommandResult::Exit => {
                                context.uart.puts("Shell exiting...\r\n");
                                break;
                            }
                        }
                    }
                    command_ready = true;
                }
            }

            // Small delay to prevent busy waiting
            context.timer.delay_us(100);
        }

        // Check if we should exit
        if executor.should_exit() {
            break;
        }
    }

    // If we somehow get here, restart the shell
    run_shell(context);
}

/// Show welcome message
fn show_welcome(context: &mut ShellContext) {
    context.uart.puts("\r\n");
    context.uart.puts(
        "╔══════════════════════════════════════════════════════════════════════════════╗\r\n",
    );
    context.uart.puts(
        "║                            TinyOS Shell v2.0                                ║\r\n",
    );
    context.uart.puts(
        "║                      Enhanced Command Line Interface                        ║\r\n",
    );
    #[cfg(feature = "raspi3")]
    context
        .uart
        .puts("║                      Optimized for Raspberry Pi 3                          ║\r\n");
    #[cfg(not(feature = "raspi3"))]
    context
        .uart
        .puts("║                      Optimized for Raspberry Pi 4/5                        ║\r\n");
    context.uart.puts(
        "╚══════════════════════════════════════════════════════════════════════════════╝\r\n",
    );
    context.uart.puts("\r\n");
    context
        .uart
        .puts("Welcome to TinyOS! Type 'help' for available commands.\r\n");
    context.uart.puts("\r\n");
}

/// Show command prompt
fn show_prompt(executor: &CommandExecutor, context: &mut ShellContext) {
    context.uart.puts("\x1b[32m"); // Green color
    context.uart.puts("tinyos");
    context.uart.puts("\x1b[0m"); // Reset color
    context.uart.puts(":");
    context.uart.puts("\x1b[34m"); // Blue color
    context.uart.puts(executor.get_current_dir());
    context.uart.puts("\x1b[0m"); // Reset color
    context.uart.puts("$ ");
}
