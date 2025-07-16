//! Utilities Module
//!
//! This module provides utility functions for exception command handling,
//! including number formatting and common helper functions.

use crate::drivers::uart::Uart;

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
