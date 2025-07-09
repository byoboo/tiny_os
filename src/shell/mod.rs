//! TinyOS Interactive Shell
//!
//! This module provides the interactive shell interface for TinyOS.
//! The shell handles command parsing, execution, and user interaction.

use core::option::Option::Some;

use crate::{
    filesystem::Fat32FileSystem, gpio::Gpio, interrupts::InterruptController,
    memory::MemoryManager, sdcard::SdCard, timer::SystemTimer, uart::Uart,
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
                // Phase 1 enhanced exception testing commands
                b'7' => hardware::handle_exception_test_advanced(&context),
                b'8' => hardware::handle_esr_test(&context),
                // Phase 1 system call and memory fault testing
                b'9' => hardware::handle_syscall_test(&context),
                b'!' => hardware::handle_memory_fault_test(&context), /* Phase 2 advanced IRQ and interrupt testing */
                b'#' => hardware::handle_irq_integration_test(&context),
                b'$' => hardware::handle_nested_interrupt_test(&context),
                b'%' => hardware::handle_deferred_processing_test(&context),

                // Phase 3 Process Management Testing
                b'&' => {
                    // Process management submenu
                    context.uart.puts("\r\nProcess Management Commands:\r\n");
                    context.uart.puts("  1 - Process Context Test\r\n");
                    context.uart.puts("  2 - Privilege Level Test\r\n");
                    context.uart.puts("  3 - Scheduler Test\r\n");
                    context.uart.puts("  4 - Process Stats\r\n");
                    context.uart.puts("  5 - Scheduler Stats\r\n");
                    context.uart.puts("  6 - Privilege Stats\r\n");
                    context.uart.puts("Select option: ");

                    if let Some(option) = context.uart.getc() {
                        match option {
                            b'1' => commands::process::handle_process_context_test(&context),
                            b'2' => commands::process::handle_privilege_test(&context),
                            b'3' => commands::process::handle_scheduler_test(&context),
                            b'4' => commands::process::handle_process_stats(&context),
                            b'5' => commands::process::handle_scheduler_stats(&context),
                            b'6' => commands::process::handle_privilege_stats(&context),
                            _ => context.uart.puts("Invalid option\r\n"),
                        }
                    }
                }

                // Phase 4 MMU and Exception Management
                b'^' => {
                    // Exception management submenu
                    context.uart.puts("\r\nException Management Commands:\r\n");
                    context.uart.puts("  1 - Exception Statistics\r\n");
                    context.uart.puts("  2 - MMU Exception Statistics\r\n");
                    context.uart.puts("  3 - MMU Control (on/off)\r\n");
                    context.uart.puts("  4 - Exception Testing (safe)\r\n");
                    context.uart.puts("  5 - Reset Exception Stats\r\n");
                    context.uart.puts("Select option: ");

                    if let Some(option) = context.uart.getc() {
                        match option {
                            b'1' => commands::exceptions::cmd_exception_stats(&["ex"], &mut context),
                            b'2' => commands::exceptions::cmd_mmu_stats(&["mmu"], &mut context),
                            b'3' => {
                                context.uart.puts("Enable (1) or Disable (2) MMU handling? ");
                                if let Some(choice) = context.uart.getc() {
                                    match choice {
                                        b'1' => commands::exceptions::cmd_mmu_control(&["mmuctl", "on"], &mut context),
                                        b'2' => commands::exceptions::cmd_mmu_control(&["mmuctl", "off"], &mut context),
                                        _ => context.uart.puts("Invalid choice\r\n"),
                                    }
                                }
                            }
                            b'4' => {
                                context.uart.puts("Test type: (1) Alignment, (2) Null deref: ");
                                if let Some(choice) = context.uart.getc() {
                                    match choice {
                                        b'1' => commands::exceptions::cmd_test_exceptions(&["extest", "alignment"], &mut context),
                                        b'2' => commands::exceptions::cmd_test_exceptions(&["extest", "nullderef"], &mut context),
                                        _ => context.uart.puts("Invalid choice\r\n"),
                                    }
                                }
                            }
                            b'5' => commands::exceptions::cmd_reset_exception_stats(&["exreset"], &mut context),
                            _ => context.uart.puts("Invalid option\r\n"),
                        }
                    }
                }

                // Direct process management test commands (for automated testing)
                b'[' => commands::process::handle_process_context_test(&context), // pctx
                b'\\' => commands::process::handle_privilege_test(&context),      // priv
                b']' => commands::process::handle_scheduler_test(&context),       // sched

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
