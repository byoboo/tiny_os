// TinyOS Shell Directory Operations
// Focused module for directory listing and navigation

use crate::{filesystem::Fat32FileSystem, uart::Uart};

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
                            super::utils::print_number(uart, file.size);
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
