//! System command utility functions
//!
//! This module contains shared utility functions used across system command
//! modules for formatting, printing, and common operations.

use crate::uart::Uart;

/// Helper function to print time in a readable format
pub fn print_time(uart: &Uart, ms: u32) {
    let seconds = ms / 1000;
    let remaining_ms = ms % 1000;
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    let hours = minutes / 60;
    let remaining_minutes = minutes % 60;

    if hours > 0 {
        print_number(uart, hours);
        uart.puts("h ");
    }
    if remaining_minutes > 0 || hours > 0 {
        print_number(uart, remaining_minutes);
        uart.puts("m ");
    }
    print_number(uart, remaining_seconds);
    uart.puts(".");
    // Print milliseconds with leading zeros
    if remaining_ms < 100 {
        uart.puts("0");
    }
    if remaining_ms < 10 {
        uart.puts("0");
    }
    print_number(uart, remaining_ms);
    uart.puts("s");
}

/// Helper function to print numbers
#[inline]
pub fn print_number(uart: &Uart, mut num: u32) {
    if num == 0 {
        uart.putc(b'0');
        return;
    }

    let mut digits = [0u8; 10];
    let mut count = 0;

    while num > 0 {
        digits[count] = (num % 10) as u8 + b'0';
        num /= 10;
        count += 1;
    }

    for i in (0..count).rev() {
        uart.putc(digits[i]);
    }
}

/// Helper function to print hexadecimal numbers
pub fn print_hex(uart: &Uart, mut num: u64) {
    if num == 0 {
        uart.puts("0");
        return;
    }

    let mut digits = [0u8; 16];
    let mut count = 0;

    while num > 0 {
        let digit = (num % 16) as u8;
        digits[count] = if digit < 10 {
            digit + b'0'
        } else {
            digit - 10 + b'A'
        };
        num /= 16;
        count += 1;
    }

    // Print in reverse order
    for i in (0..count).rev() {
        uart.putc(digits[i]);
    }
}

/// Parse a string into a number (decimal only)
pub fn parse_number(s: &str) -> Option<u32> {
    let mut result = 0u32;
    let bytes = s.as_bytes();

    if bytes.is_empty() {
        return None;
    }

    for &byte in bytes {
        if !byte.is_ascii_digit() {
            return None;
        }
        result = result.checked_mul(10)?.checked_add((byte - b'0') as u32)?;
    }

    Some(result)
}
