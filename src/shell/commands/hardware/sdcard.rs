//! SD Card command handlers
//!
//! This module contains handlers for SD card-related commands including
//! card information, block reading, and block writing operations.

use crate::shell::ShellContext;

/// Handle SD card info command (p/P)
pub fn handle_sdcard_info(context: &ShellContext) {
    context.uart.puts("\r\n=== SD Card Information ===\r\n");

    if let Some(info) = context.sdcard.get_card_info() {
        context.uart.puts("SD Card Status: ✓ INITIALIZED\r\n");

        context.uart.puts("Card Type: ");
        if info.high_capacity {
            context.uart.puts("SDHC/SDXC (High Capacity)\r\n");
        } else {
            context.uart.puts("SDSC (Standard Capacity)\r\n");
        }

        context.uart.puts("RCA (Relative Card Address): 0x");
        context.uart.put_hex(info.rca as u64);
        context.uart.puts("\r\n");

        context.uart.puts("Card Initialized: ✓ YES\r\n");
    } else {
        context.uart.puts("SD Card Status: ✗ NOT INITIALIZED\r\n");
        context.uart.puts("Note: SD card may not be present or\r\n");
        context.uart.puts("      initialization failed in QEMU\r\n");
        context
            .uart
            .puts("      (Full functionality on real Pi)\r\n");
    }
    context.uart.puts("===========================\r\n");
}

/// Handle SD card read command (q/Q)
pub fn handle_sdcard_read(context: &mut ShellContext) {
    context.uart.puts("\r\n=== SD Card Block Read ===\r\n");
    context.uart.puts("Reading block 0 (boot sector)...\r\n");

    let mut buffer = [0u8; 512];
    match context.sdcard.read_block(0, &mut buffer) {
        Ok(()) => {
            context.uart.puts("✓ Block read successful!\r\n");
            context.uart.puts("Boot sector analysis:\r\n");

            // Check for FAT filesystem signature
            if buffer[510] == 0x55 && buffer[511] == 0xAA {
                context.uart.puts("  Boot signature: ✓ VALID (0x55AA)\r\n");

                // Check for FAT type
                if &buffer[54..62] == b"FAT32   " {
                    context.uart.puts("  Filesystem: FAT32\r\n");
                } else if &buffer[54..59] == b"FAT16" {
                    context.uart.puts("  Filesystem: FAT16\r\n");
                } else if &buffer[54..58] == b"FAT1" {
                    context.uart.puts("  Filesystem: FAT12\r\n");
                } else {
                    context.uart.puts("  Filesystem: Unknown\r\n");
                }
            } else {
                context.uart.puts("  Boot signature: ✗ INVALID\r\n");
                context.uart.puts("  (Not a bootable filesystem)\r\n");
            }

            context.uart.puts("First 16 bytes: ");
            for (i, &byte) in buffer.iter().enumerate().take(16) {
                if byte < 16 {
                    context.uart.putc(b'0');
                }
                context.uart.put_hex(byte as u64);
                if i < 15 {
                    context.uart.putc(b' ');
                }
            }
            context.uart.puts("\r\n");
        }
        Err(_) => {
            context.uart.puts("✗ Block read failed\r\n");
            context.uart.puts("SD card may not be initialized\r\n");
        }
    }
    context.uart.puts("==========================\r\n");
}

/// Handle SD card write command (y/Y)
pub fn handle_sdcard_write(context: &mut ShellContext) {
    context
        .uart
        .puts("\r\n=== SD Card Block Write Test ===\r\n");
    context
        .uart
        .puts("Writing test pattern to block 1000...\r\n");

    // Create a test pattern
    let mut test_buffer = [0u8; 512];
    for (i, byte) in test_buffer.iter_mut().enumerate() {
        *byte = (i % 256) as u8;
    }

    match context.sdcard.write_block(1000, &test_buffer) {
        Ok(()) => {
            context.uart.puts("✓ Test write successful!\r\n");

            // Verify by reading back
            context.uart.puts("Verifying write by reading back...\r\n");
            let mut verify_buffer = [0u8; 512];
            match context.sdcard.read_block(1000, &mut verify_buffer) {
                Ok(()) => {
                    if verify_buffer == test_buffer {
                        context.uart.puts("✓ Write verification passed!\r\n");
                        context.uart.puts("Data integrity confirmed.\r\n");
                    } else {
                        context.uart.puts("✗ Write verification failed!\r\n");
                        context.uart.puts("Data corruption detected.\r\n");
                    }
                }
                Err(_) => {
                    context.uart.puts("✗ Verification read failed\r\n");
                }
            }
        }
        Err(_) => {
            context.uart.puts("✗ Test write failed\r\n");
            context.uart.puts("SD card may not be initialized\r\n");
            context.uart.puts("or may be write-protected\r\n");
        }
    }
    context.uart.puts("===============================\r\n");
}
