//! Basic Command Router
//!
//! This module handles routing for basic system, hardware, and memory commands
//! that don't require complex submenu interactions.

use crate::shell::{commands, core::ShellContext};

/// Route basic system commands (h, t, s, c)
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
