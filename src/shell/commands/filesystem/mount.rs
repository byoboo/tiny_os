// TinyOS Shell Filesystem Mounting Operations
// Focused module for filesystem mounting and information

use crate::{filesystem::Fat32FileSystem, sdcard::SdCard, uart::Uart};

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
