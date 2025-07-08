//! Filesystem and storage commands
//!
//! This module contains all filesystem-related command handlers including:
//! - FAT32 filesystem operations
//! - Directory listing and navigation
//! - File reading and manipulation
//! - Storage device information

use crate::{fat32::Fat32FileSystem, sdcard::SdCard, uart::Uart};

/// Handle directory listing command ('d', 'D')
pub fn handle_directory_listing(uart: &Uart, fat32_fs: &mut Option<Fat32FileSystem>) {
    uart.puts("\r\n=== Directory Listing ===\r\n");
    if let Some(ref mut fs) = fat32_fs {
        match fs.list_directory() {
            Ok(files) => {
                if files.is_empty() {
                    uart.puts("Directory is empty.\r\n");
                } else {
                    uart.puts("Name             Size      Type\r\n");
                    uart.puts("--------------------------------\r\n");
                    for i in 0..files.len() {
                        let file = files.get(i).unwrap();

                        // Print filename (up to 12 chars)
                        let name_len = file
                            .name
                            .iter()
                            .position(|&x| x == 0)
                            .unwrap_or(256)
                            .min(12);
                        for j in 0..name_len {
                            uart.putc(file.name[j]);
                        }
                        for _ in name_len..13 {
                            uart.putc(b' ');
                        }

                        // Print size
                        if file.is_directory {
                            uart.puts("<DIR>    ");
                        } else {
                            print_number(uart, file.size);
                            uart.puts("     ");
                        }

                        // Print type
                        if file.is_directory {
                            uart.puts("Directory");
                        } else {
                            uart.puts("File");
                        }
                        uart.puts("\r\n");
                    }
                }
            }
            Err(_) => {
                uart.puts("Error reading directory\r\n");
            }
        }
    } else {
        uart.puts("FAT32 filesystem not available\r\n");
    }
}

/// Handle filesystem mount/info command ('n', 'N')
pub fn handle_filesystem_mount_info(uart: &Uart, fat32_fs: &mut Option<Fat32FileSystem>) {
    if fat32_fs.is_some() {
        uart.puts("\r\n=== FAT32 Filesystem Info ===\r\n");
        if let Some(ref fs) = fat32_fs {
            fs.print_info();
        }
    } else {
        uart.puts("\r\nMounting FAT32 filesystem...\r\n");
        // Create a new SD card instance for the filesystem
        let mut fs_sdcard = SdCard::new();
        match fs_sdcard.init() {
            Ok(()) => match Fat32FileSystem::new(fs_sdcard) {
                Ok(mut fs) => match fs.mount() {
                    Ok(()) => {
                        uart.puts("✓ FAT32 filesystem mounted successfully!\r\n");
                        *fat32_fs = Some(fs);
                    }
                    Err(_) => {
                        uart.puts("Failed to mount FAT32 filesystem\r\n");
                    }
                },
                Err(_) => {
                    uart.puts("No FAT32 filesystem found\r\n");
                }
            },
            Err(_) => {
                uart.puts("Failed to initialize SD card for filesystem\r\n");
            }
        }
    }
}

/// Handle change directory command ('o', 'O')
pub fn handle_change_directory(uart: &Uart, fat32_fs: &mut Option<Fat32FileSystem>) {
    uart.puts("\r\nChange directory - enter 'test' or '..' for demo:\r\n");
    uart.puts("Directory name: ");
    // For demo, just try 'test' directory
    if let Some(ref mut fs) = fat32_fs {
        match fs.change_directory("test") {
            Ok(()) => {
                uart.puts("Changed to directory 'test'\r\n");
            }
            Err(_) => {
                uart.puts("Directory 'test' not found, trying '..'\r\n");
                match fs.change_directory("..") {
                    Ok(()) => {
                        uart.puts("Changed to parent directory\r\n");
                    }
                    Err(_) => {
                        uart.puts("Could not change directory\r\n");
                    }
                }
            }
        }
    } else {
        uart.puts("FAT32 filesystem not available\r\n");
    }
}

/// Handle file read command ('u', 'U')
pub fn handle_read_file(uart: &Uart, fat32_fs: &mut Option<Fat32FileSystem>) {
    uart.puts("\r\nRead file - trying 'readme.txt':\r\n");
    if let Some(ref mut fs) = fat32_fs {
        match fs.read_file("readme.txt") {
            Ok(content) => {
                uart.puts("File contents:\r\n");
                uart.puts("==============\r\n");
                if let Ok(text) = content.as_str() {
                    uart.puts(text);
                } else {
                    uart.puts("Binary file - showing first 256 bytes:\r\n");
                    let bytes_to_show = content.len().min(256);
                    for i in 0..bytes_to_show {
                        let byte = content.as_slice()[i];
                        if (32..=126).contains(&byte) {
                            uart.putc(byte);
                        } else {
                            uart.putc(b'.');
                        }
                    }
                }
                uart.puts("\r\n==============\r\n");
                uart.puts("File size: ");
                print_number(uart, content.len() as u32);
                uart.puts(" bytes\r\n");
            }
            Err(_) => {
                uart.puts("File 'readme.txt' not found\r\n");
            }
        }
    } else {
        uart.puts("FAT32 filesystem not available\r\n");
    }
}

/// Handle change to root directory command ('k', 'K')
pub fn handle_change_to_root(uart: &Uart, fat32_fs: &mut Option<Fat32FileSystem>) {
    uart.puts("\r\nChanging to root directory...\r\n");
    if let Some(ref mut fs) = fat32_fs {
        fs.change_to_root();
        uart.puts("Now in root directory\r\n");
    } else {
        uart.puts("FAT32 filesystem not available\r\n");
    }
}

/// Helper function to print a number
fn print_number(uart: &Uart, mut num: u32) {
    if num == 0 {
        uart.puts("0");
        return;
    }

    let mut buffer = [0u8; 10];
    let mut index = 0;

    while num > 0 {
        buffer[index] = (num % 10) as u8 + b'0';
        num /= 10;
        index += 1;
    }

    for i in (0..index).rev() {
        uart.putc(buffer[i]);
    }
}
