//! UART Hardware Registers and Low-level Access
//!
//! This module contains the hardware register definitions and low-level
//! memory-mapped I/O operations for the UART peripheral.

use core::ptr::{read_volatile, write_volatile};

use crate::drivers::config::HardwareVersion;

/// UART register offsets from base address
pub mod registers {
    /// Data register offset
    pub const DR: u32 = 0x00;
    /// Flag register offset
    pub const FR: u32 = 0x18;
    /// Integer baud rate divisor offset
    pub const IBRD: u32 = 0x24;
    /// Fractional baud rate divisor offset
    pub const FBRD: u32 = 0x28;
    /// Line control register offset
    pub const LCRH: u32 = 0x2C;
    /// Control register offset
    pub const CR: u32 = 0x30;
    /// Interrupt clear register offset
    pub const ICR: u32 = 0x44;
}

/// Flag register bit definitions
pub mod flags {
    /// Transmit FIFO full
    pub const TXFF: u32 = 1 << 5;
    /// Receive FIFO empty
    pub const RXFE: u32 = 1 << 4;
}

/// Control register bit definitions
pub mod control {
    /// UART enable
    pub const UARTEN: u32 = 1 << 0;
    /// Transmit enable
    pub const TXE: u32 = 1 << 8;
    /// Receive enable
    pub const RXE: u32 = 1 << 9;
}

/// Line control register bit definitions
pub mod line_control {
    /// 8-bit words
    pub const WLEN_8BIT: u32 = 0b11 << 5;
    /// Enable FIFOs
    pub const FEN: u32 = 1 << 4;
}

/// Low-level UART hardware access
pub struct UartHardware<H: HardwareVersion> {
    _phantom: core::marker::PhantomData<H>,
}

impl<H: HardwareVersion> UartHardware<H> {
    /// Create a new UART hardware interface
    #[inline]
    pub const fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }

    /// Get the base address for this hardware version
    #[inline]
    const fn base_addr() -> u32 {
        H::UART_BASE
    }

    /// Write to a UART register
    #[inline]
    pub unsafe fn write_register(&self, offset: u32, value: u32) {
        let addr = (Self::base_addr() + offset) as *mut u32;
        write_volatile(addr, value);
    }

    /// Read from a UART register
    #[inline]
    pub unsafe fn read_register(&self, offset: u32) -> u32 {
        let addr = (Self::base_addr() + offset) as *const u32;
        read_volatile(addr)
    }

    /// Write a byte to the data register
    #[inline]
    pub unsafe fn write_data(&self, data: u8) {
        self.write_register(registers::DR, data as u32);
    }

    /// Read a byte from the data register
    #[inline]
    pub unsafe fn read_data(&self) -> u8 {
        self.read_register(registers::DR) as u8
    }

    /// Check if transmit FIFO is full
    #[inline]
    pub fn is_tx_full(&self) -> bool {
        unsafe { (self.read_register(registers::FR) & flags::TXFF) != 0 }
    }

    /// Check if receive FIFO is empty
    #[inline]
    pub fn is_rx_empty(&self) -> bool {
        unsafe { (self.read_register(registers::FR) & flags::RXFE) != 0 }
    }

    /// Initialize UART hardware with standard settings
    pub fn init_hardware(&self) {
        unsafe {
            // Disable UART
            self.write_register(registers::CR, 0);

            // Clear all pending interrupts
            self.write_register(registers::ICR, 0x7FF);

            // Set baud rate to 115200 (assuming 48MHz UART clock)
            // Baud rate divisor = UART_CLK / (16 * baud_rate)
            // For 115200: divisor = 48000000 / (16 * 115200) = 26.04
            // Integer part = 26, fractional part = 0.04 * 64 = 2.56 ≈ 3
            self.write_register(registers::IBRD, 26);
            self.write_register(registers::FBRD, 3);

            // Set line control: 8-bit, no parity, 1 stop bit, FIFOs enabled
            self.write_register(registers::LCRH, line_control::WLEN_8BIT | line_control::FEN);

            // Enable UART, transmit, and receive
            self.write_register(registers::CR, control::UARTEN | control::TXE | control::RXE);
        }
    }
}
