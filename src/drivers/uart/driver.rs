//! High-level UART Driver API
//!
//! This module provides a safe, high-level interface to the UART peripheral
//! with features like buffered I/O, line editing, and error handling.

use crate::drivers::config::{DefaultHardware, HardwareVersion};
use crate::drivers::traits::{DriverError, DriverStatus, Initialize, Status};
use super::hardware::UartHardware;

/// UART driver configuration
#[derive(Debug, Clone, Copy)]
pub struct UartConfig {
    /// Baud rate (bits per second)
    pub baud_rate: u32,
    /// Number of data bits (5-8)
    pub data_bits: u8,
    /// Enable parity checking
    pub parity: bool,
    /// Number of stop bits (1 or 2)
    pub stop_bits: u8,
}

impl Default for UartConfig {
    fn default() -> Self {
        Self {
            baud_rate: 115200,
            data_bits: 8,
            parity: false,
            stop_bits: 1,
        }
    }
}

/// High-level UART driver
pub struct UartDriver<H: HardwareVersion = DefaultHardware> {
    hardware: UartHardware<H>,
    status: DriverStatus,
}

impl<H: HardwareVersion> UartDriver<H> {
    /// Create a new UART driver instance
    pub const fn new() -> Self {
        Self {
            hardware: UartHardware::new(),
            status: DriverStatus::Uninitialized,
        }
    }
    
    /// Send a single character
    #[inline]
    pub fn putc(&self, c: u8) {
        // Wait for transmit FIFO to have space
        while self.hardware.is_tx_full() {
            // Busy wait
        }
        
        unsafe {
            self.hardware.write_data(c);
        }
    }
    
    /// Send a string
    #[inline]
    pub fn puts(&self, s: &str) {
        for byte in s.bytes() {
            self.putc(byte);
        }
    }
    
    /// Send a hexadecimal representation of a 64-bit value
    pub fn put_hex(&self, value: u64) {
        const HEX_CHARS: &[u8] = b"0123456789ABCDEF";
        
        for i in (0..16).rev() {
            let nibble = ((value >> (i * 4)) & 0xF) as usize;
            self.putc(HEX_CHARS[nibble]);
        }
    }
    
    /// Try to receive a character (non-blocking)
    #[inline]
    pub fn getc(&self) -> Option<u8> {
        if !self.hardware.is_rx_empty() {
            unsafe {
                Some(self.hardware.read_data())
            }
        } else {
            None
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
                            self.puts("\x08 \x08"); // Backspace, space, backspace
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
    
    /// Legacy init method for backward compatibility
    pub fn init(&mut self) {
        // Initialize the hardware directly
        self.hardware.init_hardware();
        self.status = DriverStatus::Ready;
    }
}

impl<H: HardwareVersion> Initialize for UartDriver<H> {
    type Config = UartConfig;
    
    fn init(&mut self) -> Result<(), DriverError> {
        let config = UartConfig::default();
        self.init_with_config(&config)
    }
    
    fn init_with_config(&mut self, _config: &Self::Config) -> Result<(), DriverError> {
        // For now, use the standard hardware initialization
        // In the future, this could be extended to support different baud rates, etc.
        self.hardware.init_hardware();
        self.status = DriverStatus::Ready;
        Ok(())
    }
}

impl<H: HardwareVersion> Status for UartDriver<H> {
    fn status(&self) -> DriverStatus {
        self.status
    }
}

/// Type alias for the default UART driver
pub type Uart = UartDriver<DefaultHardware>;
