//! LED command handlers
//!
//! This module contains handlers for LED-related commands including
//! LED on/off control and toggle functionality.

use crate::shell::ShellContext;

/// Handle LED ON command (1)
pub fn handle_led_on(context: &mut ShellContext) {
    context.gpio.set_high(42);
    context.led_state = true;
    context.uart.puts("LED turned ON\r\n");
}

/// Handle LED OFF command (0)
pub fn handle_led_off(context: &mut ShellContext) {
    context.gpio.set_low(42);
    context.led_state = false;
    context.uart.puts("LED turned OFF\r\n");
}

/// Handle LED toggle command (l/L)
pub fn handle_led_toggle(context: &mut ShellContext) {
    context.led_state = !context.led_state;
    if context.led_state {
        context.gpio.set_high(42);
        context.uart.puts("LED toggled ON\r\n");
    } else {
        context.gpio.set_low(42);
        context.uart.puts("LED toggled OFF\r\n");
    }
}
