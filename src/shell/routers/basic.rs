//! Basic Command Router
//!
//! This module handles routing for basic system, hardware, and memory commands
//! that don't require complex submenu interactions.

use crate::shell::{commands, core::ShellContext};

/// Route basic system commands (h, t, s, c, b, z)
pub fn route_system_commands(ch: u8, context: &mut ShellContext, start_time: u64) -> bool {
    match ch {
        b'h' | b'H' => {
            commands::system::handle_help(context);
            true
        }
        b't' | b'T' => {
            commands::system::handle_time(context, start_time);
            true
        }
        b's' | b'S' => {
            commands::system::handle_system_info(context);
            true
        }
        b'c' | b'C' => {
            commands::system::handle_health_check(context);
            true
        }
        b'b' | b'B' => {
            // Benchmark commands menu
            handle_benchmark_menu(context);
            true
        }
        b'z' | b'Z' => {
            // Quick performance status
            commands::benchmark::cmd_perfstat(&[], context);
            true
        }
        _ => false,
    }
}

/// Route basic hardware commands (LED, basic interrupts, exceptions)
pub fn route_hardware_commands(ch: u8, context: &mut ShellContext) -> bool {
    match ch {
        b'1' => {
            commands::hardware::handle_led_on(context);
            context.led_state = true;
            true
        }
        b'0' => {
            commands::hardware::handle_led_off(context);
            context.led_state = false;
            true
        }
        b'l' | b'L' => {
            commands::hardware::handle_led_toggle(context);
            true
        }
        b'i' | b'I' => {
            commands::hardware::handle_interrupt_status(context);
            true
        }
        b'e' | b'E' => {
            commands::hardware::handle_interrupt_toggle(context);
            true
        }
        b'j' | b'J' => {
            commands::hardware::handle_interrupt_test(context);
            true
        }
        b'v' | b'V' => {
            commands::hardware::handle_exception_stats(context);
            true
        }
        b'w' | b'W' => {
            commands::hardware::handle_exception_test(context);
            true
        }
        b'p' | b'P' => {
            commands::hardware::handle_sdcard_info(context);
            true
        }
        b'q' | b'Q' => {
            commands::hardware::handle_sdcard_read(context);
            true
        }
        b'y' | b'Y' => {
            commands::hardware::handle_sdcard_write(context);
            true
        }
        _ => false,
    }
}

/// Route enhanced hardware testing commands
pub fn route_enhanced_hardware_commands(ch: u8, context: &mut ShellContext) -> bool {
    match ch {
        // Phase 1 enhanced exception testing commands
        b'7' => {
            commands::hardware::handle_exception_test_advanced(context);
            true
        }
        b'8' => {
            commands::hardware::handle_esr_test(context);
            true
        }
        // Phase 1 system call and memory fault testing
        b'9' => {
            commands::hardware::handle_syscall_test(context);
            true
        }
        b'!' => {
            commands::hardware::handle_memory_fault_test(context);
            true
        }
        // Phase 2 advanced IRQ and interrupt testing
        b'#' => {
            commands::hardware::handle_irq_integration_test(context);
            true
        }
        b'$' => {
            commands::hardware::handle_nested_interrupt_test(context);
            true
        }
        b'%' => {
            commands::hardware::handle_deferred_processing_test(context);
            true
        }
        // Week 4: Advanced Hardware Integration Commands
        b'4' => {
            handle_week4_menu(context);
            true
        }
        // Week 5: Network and Advanced I/O Commands
        b'5' => {
            handle_week5_menu(context);
            true
        }
        // Week 6: Advanced Security and Real-time Commands
        b'6' => {
            handle_week6_menu(context);
            true
        }
        _ => false,
    }
}

/// Route basic memory commands
pub fn route_memory_commands(ch: u8, context: &mut ShellContext) -> bool {
    match ch {
        b'm' | b'M' => {
            commands::memory::handle_memory_stats(&context.uart, &context.memory_manager);
            true
        }
        b'a' | b'A' => {
            commands::memory::handle_memory_allocate(&context.uart, &mut context.memory_manager);
            true
        }
        b'f' | b'F' => {
            commands::memory::handle_memory_free(&context.uart, &mut context.memory_manager);
            true
        }
        b'x' | b'X' => {
            commands::memory::handle_memory_test(&context.uart, &mut context.memory_manager);
            true
        }
        b'z' | b'Z' => {
            commands::memory::handle_comprehensive_memory_test(
                &context.uart,
                &mut context.memory_manager,
            );
            true
        }
        b'g' | b'G' => {
            commands::memory::handle_memory_corruption_check(
                &context.uart,
                &context.memory_manager,
            );
            true
        }
        b'r' | b'R' => {
            commands::memory::handle_memory_defragment(&context.uart, &mut context.memory_manager);
            true
        }
        _ => false,
    }
}

/// Handle benchmark menu interface
fn handle_benchmark_menu(context: &mut ShellContext) {
    use crate::shell::commands::benchmark::benchmark_menu;
    benchmark_menu(context);
}

/// Handle week 4 advanced hardware integration menu
fn handle_week4_menu(context: &mut ShellContext) {
    context.uart.puts("\n🚀 WEEK 4 ADVANCED HARDWARE FEATURES\n");
    context.uart.puts("=====================================\n");
    context.uart.puts("  1 - Initialize Week 4 features\n");
    context.uart.puts("  2 - Show system status\n");
    context.uart.puts("  3 - Run benchmarks\n");
    context.uart.puts("  4 - Power management\n");
    context.uart.puts("  5 - PCIe devices\n");
    context.uart.puts("  6 - Thermal management\n");
    context.uart.puts("  h - Help\n");
    context.uart.puts("Select option: ");

    if let Some(option) = context.uart.getc() {
        context.uart.putc(option);
        context.uart.puts("\n");
        
        match option {
            b'1' => {
                commands::week4_simple::cmd_week4_init(&["init"], context);
            }
            b'2' => {
                commands::week4_simple::cmd_week4_status(&["status"], context);
            }
            b'3' => {
                commands::week4_simple::cmd_week4_benchmark(&["benchmark"], context);
            }
            b'4' => {
                handle_week4_power_submenu(context);
            }
            b'5' => {
                commands::week4_simple::cmd_week4_devices(&["devices"], context);
            }
            b'6' => {
                commands::week4_simple::cmd_week4_thermal(&["thermal"], context);
            }
            b'h' | b'H' => {
                commands::week4_simple::cmd_week4_help(&["help"], context);
            }
            _ => {
                context.uart.puts("Invalid option\n");
            }
        }
    }
}

/// Handle Week 4 power management submenu
fn handle_week4_power_submenu(context: &mut ShellContext) {
    context.uart.puts("\n⚡ Power Management Options:\n");
    context.uart.puts("  1 - CPU frequency (min)\n");
    context.uart.puts("  2 - CPU frequency (medium)\n");
    context.uart.puts("  3 - CPU frequency (max)\n");
    context.uart.puts("  4 - GPU power (idle)\n");
    context.uart.puts("  5 - GPU power (full)\n");
    context.uart.puts("Select option: ");

    if let Some(option) = context.uart.getc() {
        context.uart.putc(option);
        context.uart.puts("\n");

        match option {
            b'1' => commands::week4_simple::cmd_week4_cpu_freq(&["cpu-freq", "min"], context),
            b'2' => commands::week4_simple::cmd_week4_cpu_freq(&["cpu-freq", "medium"], context),
            b'3' => commands::week4_simple::cmd_week4_cpu_freq(&["cpu-freq", "max"], context),
            b'4' => commands::week4_simple::cmd_week4_gpu_power(&["gpu-power", "idle"], context),
            b'5' => commands::week4_simple::cmd_week4_gpu_power(&["gpu-power", "full"], context),
            _ => context.uart.puts("Invalid option\n"),
        }
    }
}

/// Handle week 5 network and advanced I/O menu
fn handle_week5_menu(context: &mut ShellContext) {
    context.uart.puts("\n🌐 WEEK 5 NETWORK & ADVANCED I/O\n");
    context.uart.puts("==================================\n");
    context.uart.puts("  1 - Network Overview\n");
    context.uart.puts("  2 - Network Status\n");
    context.uart.puts("  3 - I/O Performance\n");
    context.uart.puts("  4 - Comprehensive Benchmark\n");
    context.uart.puts("  5 - Network Interfaces\n");
    context.uart.puts("  6 - I/O Protocols\n");
    context.uart.puts("  7 - Week 5 Capabilities\n");
    context.uart.puts("  h - Help\n");
    context.uart.puts("Choose option: ");

    if let Some(option) = context.uart.getc() {
        context.uart.putc(option);
        context.uart.puts("\n");
        
        match option {
            b'1' => {
                commands::week5::cmd_week5(&["overview"], context);
            }
            b'2' => {
                commands::week5::cmd_week5_network(&["status"], context);
            }
            b'3' => {
                commands::week5::cmd_week5_io(&["performance"], context);
            }
            b'4' => {
                commands::week5::cmd_week5(&["benchmark"], context);
            }
            b'5' => {
                commands::week5::cmd_week5_network(&["interfaces"], context);
            }
            b'6' => {
                commands::week5::cmd_week5_io(&["protocols"], context);
            }
            b'7' => {
                commands::week5::cmd_week5(&["capabilities"], context);
            }
            b'h' | b'H' => {
                commands::week5::cmd_week5(&["help"], context);
            }
            _ => {
                context.uart.puts("Invalid option\n");
            }
        }
    }
}

/// Handle week 6 security and real-time menu
fn handle_week6_menu(context: &mut ShellContext) {
    context.uart.puts("\n🔒 WEEK 6 SECURITY & REAL-TIME\n");
    context.uart.puts("=================================\n");
    context.uart.puts("  1 - Security Overview\n");
    context.uart.puts("  2 - Security Status\n");
    context.uart.puts("  3 - Real-time Metrics\n");
    context.uart.puts("  4 - System Hardening\n");
    context.uart.puts("  5 - Security Scan\n");
    context.uart.puts("  6 - RT Performance Test\n");
    context.uart.puts("  7 - Comprehensive Benchmark\n");
    context.uart.puts("  8 - Week 6 Capabilities\n");
    context.uart.puts("  h - Help\n");
    context.uart.puts("Choose option: ");

    if let Some(option) = context.uart.getc() {
        context.uart.putc(option);
        context.uart.puts("\n");
        
        match option {
            b'1' => {
                commands::week6::cmd_week6(&["overview"], context);
            }
            b'2' => {
                commands::week6::cmd_week6_security(&["status"], context);
            }
            b'3' => {
                commands::week6::cmd_week6_realtime(&["metrics"], context);
            }
            b'4' => {
                commands::week6::cmd_week6_hardening(&["status"], context);
            }
            b'5' => {
                commands::week6::cmd_week6_security(&["scan"], context);
            }
            b'6' => {
                commands::week6::cmd_week6_realtime(&["latency"], context);
            }
            b'7' => {
                commands::week6::cmd_week6(&["benchmark"], context);
            }
            b'8' => {
                commands::week6::cmd_week6(&["capabilities"], context);
            }
            b'h' | b'H' => {
                commands::week6::cmd_week6(&["help"], context);
            }
            _ => {
                context.uart.puts("Invalid option\n");
            }
        }
    }
}
