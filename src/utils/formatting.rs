// No-std formatting utilities for TinyOS
// Provides basic string formatting without the std format! macro

/// Simple number to string conversion for no_std environment
pub fn write_number_to_buffer(mut num: u64, buffer: &mut [u8]) -> usize {
    if num == 0 {
        if buffer.len() > 0 {
            buffer[0] = b'0';
            return 1;
        } else {
            return 0;
        }
    }

    let mut digits = 0;
    let mut temp = num;
    
    // Count digits
    while temp > 0 {
        digits += 1;
        temp /= 10;
    }
    
    if digits > buffer.len() {
        return 0; // Buffer too small
    }
    
    // Fill buffer from right to left
    for i in (0..digits).rev() {
        buffer[i] = b'0' + (num % 10) as u8;
        num /= 10;
    }
    
    digits
}

/// Simple hex number to string conversion
pub fn write_hex_to_buffer(mut num: u64, buffer: &mut [u8]) -> usize {
    if num == 0 {
        if buffer.len() >= 3 {
            buffer[0] = b'0';
            buffer[1] = b'x';
            buffer[2] = b'0';
            return 3;
        } else {
            return 0;
        }
    }

    let mut digits = 0;
    let mut temp = num;
    
    // Count hex digits
    while temp > 0 {
        digits += 1;
        temp /= 16;
    }
    
    if digits + 2 > buffer.len() {
        return 0; // Buffer too small
    }
    
    buffer[0] = b'0';
    buffer[1] = b'x';
    
    // Fill buffer from right to left
    for i in (2..digits + 2).rev() {
        let digit = (num % 16) as u8;
        buffer[i] = if digit < 10 {
            b'0' + digit
        } else {
            b'A' + (digit - 10)
        };
        num /= 16;
    }
    
    digits + 2
}

/// Helper to write a number and string to UART
pub fn write_number_with_text(context: &mut crate::shell::ShellContext, prefix: &str, number: u64, suffix: &str) {
    context.uart.puts(prefix);
    
    let mut buffer = [0u8; 32];
    let len = write_number_to_buffer(number, &mut buffer);
    
    for i in 0..len {
        context.uart.putc(buffer[i]);
    }
    
    context.uart.puts(suffix);
}

/// Helper to write a hex number and string to UART
pub fn write_hex_with_text(context: &mut crate::shell::ShellContext, prefix: &str, number: u64, suffix: &str) {
    context.uart.puts(prefix);
    
    let mut buffer = [0u8; 32];
    let len = write_hex_to_buffer(number, &mut buffer);
    
    for i in 0..len {
        context.uart.putc(buffer[i]);
    }
    
    context.uart.puts(suffix);
}

/// Helper to write boolean as string
pub fn write_bool_with_text(context: &mut crate::shell::ShellContext, prefix: &str, value: bool, suffix: &str) {
    context.uart.puts(prefix);
    if value {
        context.uart.puts("true");
    } else {
        context.uart.puts("false");
    }
    context.uart.puts(suffix);
}
