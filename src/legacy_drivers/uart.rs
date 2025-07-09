// UART driver for Raspberry Pi 4/5
use core::ptr::{read_volatile, write_volatile};

// UART base addresses for Raspberry Pi 4/5
// Pi 4/5: 0xFE201000 (BCM2711/BCM2712)
const UART_BASE: u32 = 0xFE201000;
#[allow(clippy::identity_op)]
const UART_DR: u32 = UART_BASE + 0x00; // Data register
const UART_FR: u32 = UART_BASE + 0x18; // Flag register
const UART_IBRD: u32 = UART_BASE + 0x24; // Integer baud rate divisor
const UART_FBRD: u32 = UART_BASE + 0x28; // Fractional baud rate divisor
const UART_LCRH: u32 = UART_BASE + 0x2C; // Line control register
const UART_CR: u32 = UART_BASE + 0x30; // Control register
const UART_ICR: u32 = UART_BASE + 0x44; // Interrupt clear register

// Flag register bits
const UART_FR_TXFF: u32 = 1 << 5; // Transmit FIFO full
const UART_FR_RXFE: u32 = 1 << 4; // Receive FIFO empty

// Control register bits
const UART_CR_UARTEN: u32 = 1 << 0; // UART enable
const UART_CR_TXE: u32 = 1 << 8; // Transmit enable
const UART_CR_RXE: u32 = 1 << 9; // Receive enable

// Line control register bits
const UART_LCRH_WLEN_8BIT: u32 = 0b11 << 5; // 8-bit words
const UART_LCRH_FEN: u32 = 1 << 4; // Enable FIFOs

pub struct Uart {
    #[allow(dead_code)]
    base: u32,
}

impl Uart {
    pub fn new() -> Self {
        Self { base: UART_BASE }
    }

    pub fn init(&self) {
        unsafe {
            // Disable UART
            write_volatile(UART_CR as *mut u32, 0);

            // Clear all pending interrupts
            write_volatile(UART_ICR as *mut u32, 0x7FF);

            // Set baud rate to 115200 (assuming 48MHz UART clock)
            // Baud rate divisor = UART_CLK / (16 * baud_rate)
            // For 115200: divisor = 48000000 / (16 * 115200) = 26.04
            // Integer part = 26, fractional part = 0.04 * 64 = 2.56 ≈ 3
            write_volatile(UART_IBRD as *mut u32, 26);
            write_volatile(UART_FBRD as *mut u32, 3);

            // Set line control: 8-bit, no parity, 1 stop bit, FIFOs enabled
            write_volatile(UART_LCRH as *mut u32, UART_LCRH_WLEN_8BIT | UART_LCRH_FEN);

            // Enable UART, transmit, and receive
            write_volatile(
                UART_CR as *mut u32,
                UART_CR_UARTEN | UART_CR_TXE | UART_CR_RXE,
            );
        }
    }

    pub fn putc(&self, c: u8) {
        unsafe {
            // Wait for transmit FIFO to have space
            while (read_volatile(UART_FR as *const u32) & UART_FR_TXFF) != 0 {
                // Busy wait
            }

            // Write character to data register
            write_volatile(UART_DR as *mut u32, c as u32);
        }
    }

    pub fn puts(&self, s: &str) {
        for byte in s.bytes() {
            self.putc(byte);
        }
    }

    pub fn put_hex(&self, value: u64) {
        const HEX_CHARS: &[u8] = b"0123456789ABCDEF";

        for i in (0..16).rev() {
            let nibble = ((value >> (i * 4)) & 0xF) as usize;
            self.putc(HEX_CHARS[nibble]);
        }
    }

    pub fn getc(&self) -> Option<u8> {
        unsafe {
            // Check if receive FIFO has data
            if (read_volatile(UART_FR as *const u32) & UART_FR_RXFE) == 0 {
                Some(read_volatile(UART_DR as *const u32) as u8)
            } else {
                None
            }
        }
    }

    /// Read a line of input with basic editing support
    /// Returns the number of characters read (excluding null terminator)
    pub fn read_line(&self, buffer: &mut [u8], max_len: usize) -> usize {
        let mut pos = 0;
        let actual_max = max_len.min(buffer.len().saturating_sub(1));

        loop {
            if let Some(ch) = self.getc() {
                match ch {
                    // Enter/newline - finish input
                    b'\r' | b'\n' => {
                        self.puts("\r\n");
                        buffer[pos] = 0; // Null terminate
                        return pos;
                    }
                    // Backspace
                    8 | 127 => {
                        if pos > 0 {
                            pos -= 1;
                            self.puts("\x08 \x08"); // Backspace, space,
                                                    // backspace
                        }
                    }
                    // Ctrl+C - cancel input
                    3 => {
                        self.puts("^C\r\n");
                        buffer[0] = 0;
                        return 0;
                    }
                    // Printable characters
                    32..=126 => {
                        if pos < actual_max {
                            buffer[pos] = ch;
                            pos += 1;
                            self.putc(ch); // Echo character
                        }
                    }
                    // Ignore other control characters
                    _ => {}
                }
            }
        }
    }

    /// Wait for a single keypress and return it
    pub fn wait_for_key(&self) -> u8 {
        loop {
            if let Some(ch) = self.getc() {
                return ch;
            }
        }
    }
}
