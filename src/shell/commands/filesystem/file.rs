// TinyOS Shell File Operations
// Focused module for file reading and manipulation

use crate::{filesystem::Fat32FileSystem, uart::Uart};

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
                super::utils::print_number(uart, content.len() as u32);
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
