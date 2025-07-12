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
                            b'1' => {
                                commands::exceptions::cmd_exception_stats(&["ex"], &mut context)
                            }
                            b'2' => commands::exceptions::cmd_mmu_stats(&["mmu"], &mut context),
                            b'3' => {
                                context
                                    .uart
                                    .puts("Enable (1) or Disable (2) MMU handling? ");
                                if let Some(choice) = context.uart.getc() {
                                    match choice {
                                        b'1' => commands::exceptions::cmd_mmu_control(
                                            &["mmuctl", "on"],
                                            &mut context,
                                        ),
                                        b'2' => commands::exceptions::cmd_mmu_control(
                                            &["mmuctl", "off"],
                                            &mut context,
                                        ),
                                        _ => context.uart.puts("Invalid choice\r\n"),
                                    }
                                }
                            }
                            b'4' => {
                                context
                                    .uart
                                    .puts("Test type: (1) Alignment, (2) Null deref: ");
                                if let Some(choice) = context.uart.getc() {
                                    match choice {
                                        b'1' => commands::exceptions::cmd_test_exceptions(
                                            &["extest", "alignment"],
                                            &mut context,
                                        ),
                                        b'2' => commands::exceptions::cmd_test_exceptions(
                                            &["extest", "nullderef"],
                                            &mut context,
                                        ),
                                        _ => context.uart.puts("Invalid choice\r\n"),
                                    }
                                }
                            }
                            b'5' => commands::exceptions::cmd_reset_exception_stats(
                                &["exreset"],
                                &mut context,
                            ),
                            _ => context.uart.puts("Invalid option\r\n"),
                        }
                    }
                }

                // Phase 4.2 Virtual Memory Management
                b'~' => {
                    // Virtual memory management submenu
                    context
                        .uart
                        .puts("\r\nVirtual Memory Management Commands:\r\n");
                    context.uart.puts("  1 - Virtual Memory Status\r\n");
                    context.uart.puts("  2 - Enable MMU\r\n");
                    context.uart.puts("  3 - Disable MMU\r\n");
                    context.uart.puts("  4 - Translate Address\r\n");
                    context.uart.puts("  5 - Flush TLB\r\n");
                    context.uart.puts("  6 - Virtual Memory Test\r\n");
                    context.uart.puts("Select option: ");

                    if let Some(option) = context.uart.getc() {
                        match option {
                            b'1' => commands::exceptions::cmd_virtual_memory_status(
                                &["vm"],
                                &mut context,
                            ),
                            b'2' => commands::exceptions::cmd_mmu_enable_disable(
                                &["mmuctl", "on"],
                                &mut context,
                            ),
                            b'3' => commands::exceptions::cmd_mmu_enable_disable(
                                &["mmuctl", "off"],
                                &mut context,
                            ),
                            b'4' => {
                                context.uart.puts("Enter address (hex): 0x");
                                // For now, test with a common address
                                commands::exceptions::cmd_translate_address(
                                    &["translate", "0x80000"],
                                    &mut context,
                                );
                            }
                            b'5' => commands::exceptions::cmd_invalidate_tlb(
                                &["tlbflush"],
                                &mut context,
                            ),
                            b'6' => commands::exceptions::cmd_virtual_memory_test(
                                &["vmtest"],
                                &mut context,
                            ),
                            _ => context.uart.puts("Invalid option\r\n"),
                        }
                    }
                }

                // Phase 4.3 Stack Management
                b'`' => {
                    // Stack management submenu
                    context.uart.puts("\r\nStack Management Commands:\r\n");
                    context.uart.puts("  1 - Stack Status\r\n");
                    context.uart.puts("  2 - Allocate Kernel Stack\r\n");
                    context.uart.puts("  3 - Allocate User Stack\r\n");
                    context.uart.puts("  4 - Deallocate Stack\r\n");
                    context.uart.puts("  5 - Switch Stack\r\n");
                    context.uart.puts("  6 - Stack Test\r\n");
                    context.uart.puts("Select option: ");

                    if let Some(option) = context.uart.getc() {
                        match option {
                            b'1' => {
                                commands::system::cmd_stack_status(&["stack_status"], &mut context)
                            }
                            b'2' => commands::system::cmd_stack_alloc(
                                &["stack_alloc", "kernel"],
                                &mut context,
                            ),
                            b'3' => commands::system::cmd_stack_alloc(
                                &["stack_alloc", "user"],
                                &mut context,
                            ),
                            b'4' => {
                                context.uart.puts("Enter stack ID: ");
                                // For now, test with stack ID 1
                                commands::system::cmd_stack_dealloc(
                                    &["stack_dealloc", "1"],
                                    &mut context,
                                );
                            }
                            b'5' => {
                                context.uart.puts("Enter stack ID: ");
                                // For now, test with stack ID 0
                                commands::system::cmd_stack_switch(
                                    &["stack_switch", "0"],
                                    &mut context,
                                );
                            }
                            b'6' => commands::system::cmd_stack_test(&["stack_test"], &mut context),
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

                // Phase 4.4.2 User Space Page Table Management
                b'|' => {
                    // User space page table management submenu
                    context
                        .uart
                        .puts("\r\nUser Space Page Table Management:\r\n");
                    context.uart.puts("  1 - User Space Status\r\n");
                    context.uart.puts("  2 - Create User Page Table\r\n");
                    context.uart.puts("  3 - Destroy User Page Table\r\n");
                    context.uart.puts("  4 - Switch User Page Table\r\n");
                    context.uart.puts("  5 - VMA Management\r\n");
                    context.uart.puts("  6 - User Space Test\r\n");
                    context.uart.puts("  7 - Initialize User Space Manager\r\n");
                    context.uart.puts("Select option: ");

                    if let Some(option) = context.uart.getc() {
                        match option {
                            b'1' => commands::user_space::handle_user_space_status(&context),
                            b'2' => commands::user_space::handle_create_user_page_table(&context),
                            b'3' => commands::user_space::handle_destroy_user_page_table(&context),
                            b'4' => commands::user_space::handle_switch_user_page_table(&context),
                            b'5' => commands::user_space::handle_vma_management(&context),
                            b'6' => commands::user_space::handle_user_space_test(&mut context),
                            b'7' => commands::user_space::handle_user_space_init(&mut context),
                            _ => context.uart.puts("Invalid option\r\n"),
                        }
                    }
                }

                // Phase 4.4.3 Advanced Memory Protection
                b'@' => {
                    // Advanced memory protection submenu
                    context.uart.puts("\r\nAdvanced Memory Protection:\r\n");
                    context.uart.puts("  1 - Protection Status\r\n");
                    context.uart.puts("  2 - Page Permissions\r\n");
                    context.uart.puts("  3 - ASLR Information\r\n");
                    context.uart.puts("  4 - Stack Protection\r\n");
                    context.uart.puts("  5 - Protection Test\r\n");
                    context.uart.puts("  6 - Protection Statistics\r\n");
                    context.uart.puts("  h - Help\r\n");
                    context.uart.puts("Select option: ");

                    if let Some(option) = context.uart.getc() {
                        match option {
                            b'1' => commands::advanced_protection::cmd_advanced_protection_status(
                                &[],
                                &mut context,
                            ),
                            b'2' => {
                                context.uart.puts("Page permissions commands:\r\n");
                                context.uart.puts("  s - Set permissions\r\n");
                                context.uart.puts("  g - Get permissions\r\n");
                                context.uart.puts("Select: ");
                                if let Some(perm_option) = context.uart.getc() {
                                    match perm_option {
                                        b's' => {
                                            context.uart.puts("Set permissions (addr perms): ");
                                            // For simplicity, use test values
                                            let args = ["permissions", "set", "0x1000000", "rw"];
                                            commands::advanced_protection::cmd_advanced_protection_permissions(&args, &mut context);
                                        }
                                        b'g' => {
                                            context.uart.puts("Get permissions (addr): ");
                                            // For simplicity, use test values
                                            let args = ["permissions", "get", "0x1000000"];
                                            commands::advanced_protection::cmd_advanced_protection_permissions(&args, &mut context);
                                        }
                                        _ => context.uart.puts("Invalid option\r\n"),
                                    }
                                }
                            }
                            b'3' => commands::advanced_protection::cmd_advanced_protection_aslr(
                                &[],
                                &mut context,
                            ),
                            b'4' => {
                                context.uart.puts("Stack protection commands:\r\n");
                                context.uart.puts("  s - Setup protection\r\n");
                                context.uart.puts("  i - Stack info\r\n");
                                context.uart.puts("Select: ");
                                if let Some(stack_option) = context.uart.getc() {
                                    match stack_option {
                                        b's' => {
                                            // For simplicity, use test values
                                            let args =
                                                ["stack", "setup", "1", "0x2000000", "0x10000"];
                                            commands::advanced_protection::cmd_advanced_protection_stack(&args, &mut context);
                                        }
                                        b'i' => {
                                            let args = ["stack", "info"];
                                            commands::advanced_protection::cmd_advanced_protection_stack(&args, &mut context);
                                        }
                                        _ => context.uart.puts("Invalid option\r\n"),
                                    }
                                }
                            }
                            b'5' => commands::advanced_protection::cmd_advanced_protection_test(
                                &[],
                                &mut context,
                            ),
                            b'6' => commands::advanced_protection::cmd_advanced_protection_stats(
                                &[],
                                &mut context,
                            ),
                            b'h' => {
                                context.uart.puts("Advanced Memory Protection Help:\r\n");
                                context.uart.puts("  1 - Show protection system status\r\n");
                                context.uart.puts("  2 - Manage page permissions\r\n");
                                context.uart.puts("  3 - View ASLR information\r\n");
                                context.uart.puts("  4 - Stack protection features\r\n");
                                context.uart.puts("  5 - Run protection tests\r\n");
                                context.uart.puts("  6 - Show protection statistics\r\n");
                            }
                            _ => context.uart.puts("Invalid option\r\n"),
                        }
                    }
                }

                // Phase 4.4.4 Dynamic Memory Management
                b'*' => {
                    // Dynamic memory management submenu
                    context.uart.puts("\r\nDynamic Memory Management:\r\n");
                    context.uart.puts("  1 - System Status\r\n");
                    context.uart.puts("  2 - Stack Growth Management\r\n");
                    context.uart.puts("  3 - Lazy Page Allocation\r\n");
                    context.uart.puts("  4 - Memory Pressure Monitoring\r\n");
                    context.uart.puts("  5 - Memory Optimization\r\n");
                    context.uart.puts("  6 - Context Switching\r\n");
                    context.uart.puts("  7 - Statistics\r\n");
                    context.uart.puts("  h - Help\r\n");
                    context.uart.puts("Select option: ");

                    if let Some(option) = context.uart.getc() {
                        match option {
                            b'1' => commands::dynamic_memory::cmd_dynamic_memory_status(
                                &[],
                                &mut context,
                            ),
                            b'2' => {
                                context.uart.puts("Stack growth commands:\r\n");
                                context.uart.puts("  c - Create dynamic stack\r\n");
                                context.uart.puts("  s - Show stack status\r\n");
                                context.uart.puts("Select option: ");

                                if let Some(stack_option) = context.uart.getc() {
                                    match stack_option {
                                        b'c' => {
                                            let args = ["growth", "create"];
                                            commands::dynamic_memory::cmd_dynamic_memory_growth(
                                                &args,
                                                &mut context,
                                            );
                                        }
                                        b's' => {
                                            let args = ["growth", "status"];
                                            commands::dynamic_memory::cmd_dynamic_memory_growth(
                                                &args,
                                                &mut context,
                                            );
                                        }
                                        _ => context.uart.puts("Invalid option\r\n"),
                                    }
                                }
                            }
                            b'3' => {
                                context.uart.puts("Lazy allocation commands:\r\n");
                                context.uart.puts("  a - Add lazy page\r\n");
                                context.uart.puts("  s - Show lazy status\r\n");
                                context.uart.puts("Select option: ");

                                if let Some(lazy_option) = context.uart.getc() {
                                    match lazy_option {
                                        b'a' => {
                                            let args = ["lazy", "add"];
                                            commands::dynamic_memory::cmd_dynamic_memory_lazy(
                                                &args,
                                                &mut context,
                                            );
                                        }
                                        b's' => {
                                            let args = ["lazy", "status"];
                                            commands::dynamic_memory::cmd_dynamic_memory_lazy(
                                                &args,
                                                &mut context,
                                            );
                                        }
                                        _ => context.uart.puts("Invalid option\r\n"),
                                    }
                                }
                            }
                            b'4' => commands::dynamic_memory::cmd_dynamic_memory_pressure(
                                &[],
                                &mut context,
                            ),
                            b'5' => commands::dynamic_memory::cmd_dynamic_memory_optimize(
                                &[],
                                &mut context,
                            ),
                            b'6' => {
                                context.uart.puts("Context switching commands:\r\n");
                                context.uart.puts("  s - Perform demo context switch\r\n");
                                context.uart.puts("  t - Show context switch status\r\n");
                                context.uart.puts("Select option: ");

                                if let Some(context_option) = context.uart.getc() {
                                    match context_option {
                                        b's' => {
                                            let args = ["context", "switch"];
                                            commands::dynamic_memory::cmd_dynamic_memory_context(
                                                &args,
                                                &mut context,
                                            );
                                        }
                                        b't' => {
                                            let args = ["context", "status"];
                                            commands::dynamic_memory::cmd_dynamic_memory_context(
                                                &args,
                                                &mut context,
                                            );
                                        }
                                        _ => context.uart.puts("Invalid option\r\n"),
                                    }
                                }
                            }
                            b'7' => commands::dynamic_memory::cmd_dynamic_memory_stats(
                                &[],
                                &mut context,
                            ),
                            b'h' => {
                                context.uart.puts("Dynamic Memory Management Help:\r\n");
                                context
                                    .uart
                                    .puts("  1 - Show system status and overview\r\n");
                                context.uart.puts("  2 - Manage dynamic stack growth\r\n");
                                context.uart.puts("  3 - Control lazy page allocation\r\n");
                                context.uart.puts("  4 - Monitor memory pressure\r\n");
                                context.uart.puts("  5 - Trigger memory optimization\r\n");
                                context
                                    .uart
                                    .puts("  6 - Hardware-assisted context switching\r\n");
                                context.uart.puts("  7 - Show detailed statistics\r\n");
                            }
                            _ => context.uart.puts("Invalid option\r\n"),
                        }
                    }
                }

                // Testing Framework Commands (Phase 5)
                b'T' => {
                    // Read next character for testing subcommand
                    if let Some(sub_ch) = context.uart.getc() {
                        match sub_ch {
                            b'K' | b'k' => commands::testing::handle_kernel_tests(&context),
                            b'M' | b'm' => commands::testing::handle_mmu_tests(&context),
                            b'P' | b'p' => commands::testing::handle_process_tests(&context),
                            b'S' | b's' => commands::testing::handle_syscall_tests(&context),
                            b'I' | b'i' => commands::testing::handle_integration_tests(&context),
                            b'A' | b'a' => commands::testing::handle_all_tests(&context),
                            b'H' | b'h' => commands::testing::handle_testing_help(&context),
                            _ => {
                                context.uart.puts("Invalid testing command. Use TH for help.\r\n");
                            }
                        }
                    } else {
                        context.uart.puts("Testing commands: TK, TM, TP, TS, TI, TA, TH\r\n");
                    }
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
