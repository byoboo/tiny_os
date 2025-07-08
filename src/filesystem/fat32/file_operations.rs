use super::{
    boot_sector::FilesystemLayout, cluster_chain::ClusterChain, Fat32Error, FileContent, FileInfo,
    MAX_FILE_SIZE,
};
/// FAT32 File Operations
///
/// This module handles FAT32 file read/write operations including reading file
/// content, following cluster chains, and managing file data.
/// It provides no_std-compliant file operations for embedded environments.
use crate::sdcard::SdCard;

/// File operations manager for FAT32
pub struct FileOperations {
    layout: FilesystemLayout,
}

impl FileOperations {
    /// Create new file operations manager
    pub fn new(layout: FilesystemLayout) -> Self {
        Self { layout }
    }

    /// Read file contents by following cluster chain
    pub fn read_file_content(
        &self,
        sd_card: &mut SdCard,
        cluster_chain: &mut ClusterChain,
        file_info: &FileInfo,
    ) -> Result<FileContent, Fat32Error> {
        if file_info.size == 0 {
            return Ok(FileContent::new());
        }

        // Check file size limit
        if file_info.size > MAX_FILE_SIZE {
            return Err(Fat32Error::FileTooLarge);
        }

        let mut content = FileContent::new();
        let mut current_cluster = file_info.first_cluster;
        let mut bytes_read = 0;

        // Calculate clusters needed
        let clusters_needed =
            (file_info.size + self.layout.bytes_per_cluster - 1) / self.layout.bytes_per_cluster;

        for _ in 0..clusters_needed {
            if !self.layout.is_valid_cluster(current_cluster) {
                break;
            }

            // Read cluster data
            let bytes_in_cluster = self.read_cluster_data(
                sd_card,
                current_cluster,
                &mut content,
                file_info.size - bytes_read,
            )?;

            bytes_read += bytes_in_cluster;

            if bytes_read >= file_info.size {
                break;
            }

            // Follow cluster chain
            let next_cluster = cluster_chain.get_next_cluster(current_cluster)?;
            if cluster_chain.is_end_of_chain(next_cluster) {
                break;
            }
            current_cluster = next_cluster;
        }

        Ok(content)
    }

    /// Read data from a single cluster
    fn read_cluster_data(
        &self,
        sd_card: &mut SdCard,
        cluster: u32,
        content: &mut FileContent,
        bytes_remaining: u32,
    ) -> Result<u32, Fat32Error> {
        let sector = self.layout.cluster_to_sector(cluster);
        let sectors_to_read = self
            .layout
            .sectors_per_cluster
            .min((bytes_remaining + 511) / 512);

        let mut bytes_read = 0;

        for sector_offset in 0..sectors_to_read {
            let mut sector_data = [0u8; 512];
            sd_card.read_block(sector + sector_offset, &mut sector_data)?;

            // Calculate bytes to copy from this sector
            let bytes_in_sector = if bytes_remaining - bytes_read >= 512 {
                512
            } else {
                bytes_remaining - bytes_read
            };

            // Copy data to content buffer
            for i in 0..bytes_in_sector {
                content.push_byte(sector_data[i as usize])?;
            }

            bytes_read += bytes_in_sector;

            if bytes_read >= bytes_remaining {
                break;
            }
        }

        Ok(bytes_read)
    }

    /// Read file by chunks (for large files)
    pub fn read_file_chunked<F>(
        &self,
        sd_card: &mut SdCard,
        cluster_chain: &mut ClusterChain,
        file_info: &FileInfo,
        mut chunk_handler: F,
    ) -> Result<(), Fat32Error>
    where
        F: FnMut(&[u8]) -> Result<(), Fat32Error>,
    {
        if file_info.size == 0 {
            return Ok(());
        }

        let mut current_cluster = file_info.first_cluster;
        let mut bytes_read = 0;

        // Calculate clusters needed
        let clusters_needed =
            (file_info.size + self.layout.bytes_per_cluster - 1) / self.layout.bytes_per_cluster;

        for _ in 0..clusters_needed {
            if !self.layout.is_valid_cluster(current_cluster) {
                break;
            }

            // Read cluster data in chunks
            let bytes_in_cluster = self.read_cluster_chunked(
                sd_card,
                current_cluster,
                file_info.size - bytes_read,
                &mut chunk_handler,
            )?;

            bytes_read += bytes_in_cluster;

            if bytes_read >= file_info.size {
                break;
            }

            // Follow cluster chain
            let next_cluster = cluster_chain.get_next_cluster(current_cluster)?;
            if cluster_chain.is_end_of_chain(next_cluster) {
                break;
            }
            current_cluster = next_cluster;
        }

        Ok(())
    }

    /// Read cluster data in chunks
    fn read_cluster_chunked<F>(
        &self,
        sd_card: &mut SdCard,
        cluster: u32,
        bytes_remaining: u32,
        chunk_handler: &mut F,
    ) -> Result<u32, Fat32Error>
    where
        F: FnMut(&[u8]) -> Result<(), Fat32Error>,
    {
        let sector = self.layout.cluster_to_sector(cluster);
        let sectors_to_read = self
            .layout
            .sectors_per_cluster
            .min((bytes_remaining + 511) / 512);

        let mut bytes_read = 0;

        for sector_offset in 0..sectors_to_read {
            let mut sector_data = [0u8; 512];
            sd_card.read_block(sector + sector_offset, &mut sector_data)?;

            // Calculate bytes to process from this sector
            let bytes_in_sector = if bytes_remaining - bytes_read >= 512 {
                512
            } else {
                bytes_remaining - bytes_read
            };

            // Process chunk
            chunk_handler(&sector_data[..bytes_in_sector as usize])?;

            bytes_read += bytes_in_sector;

            if bytes_read >= bytes_remaining {
                break;
            }
        }

        Ok(bytes_read)
    }

    /// Validate file content integrity
    pub fn validate_file(
        &self,
        sd_card: &mut SdCard,
        cluster_chain: &mut ClusterChain,
        file_info: &FileInfo,
    ) -> Result<FileValidationResult, Fat32Error> {
        let mut result = FileValidationResult::new();

        if file_info.size == 0 {
            result.is_valid = true;
            return Ok(result);
        }

        let mut current_cluster = file_info.first_cluster;
        let mut bytes_validated = 0;

        // Follow cluster chain and validate each cluster
        loop {
            if !self.layout.is_valid_cluster(current_cluster) {
                result.is_valid = false;
                result.error_message = "Invalid cluster in chain";
                break;
            }

            result.clusters_validated += 1;

            // Validate cluster exists on disk
            let sector = self.layout.cluster_to_sector(current_cluster);
            let mut sector_data = [0u8; 512];

            // Try to read first sector of cluster
            if sd_card.read_block(sector, &mut sector_data).is_err() {
                result.is_valid = false;
                result.error_message = "Cannot read cluster data";
                break;
            }

            // Count bytes in this cluster
            let bytes_in_cluster = self
                .layout
                .bytes_per_cluster
                .min(file_info.size - bytes_validated);
            bytes_validated += bytes_in_cluster;

            if bytes_validated >= file_info.size {
                result.is_valid = true;
                break;
            }

            // Follow cluster chain
            let next_cluster = cluster_chain.get_next_cluster(current_cluster)?;
            if cluster_chain.is_end_of_chain(next_cluster) {
                result.is_valid = (bytes_validated == file_info.size);
                if !result.is_valid {
                    result.error_message = "File size mismatch";
                }
                break;
            }
            current_cluster = next_cluster;
        }

        result.bytes_validated = bytes_validated;
        Ok(result)
    }

    /// Print file content via UART (for text files)
    pub fn print_file_content(
        &self,
        sd_card: &mut SdCard,
        cluster_chain: &mut ClusterChain,
        file_info: &FileInfo,
    ) -> Result<(), Fat32Error> {
        let uart = crate::uart::Uart::new();

        uart.puts("=== File Content ===\n");
        uart.puts("Size: ");
        uart.put_hex(file_info.size as u64);
        uart.puts(" bytes\n");

        if file_info.size == 0 {
            uart.puts("(Empty file)\n");
            return Ok(());
        }

        // Read file in chunks and print
        self.read_file_chunked(sd_card, cluster_chain, file_info, |chunk| {
            for &byte in chunk {
                // Print printable ASCII characters
                if byte >= 32 && byte < 127 {
                    uart.putc(byte);
                } else if byte == b'\n' {
                    uart.putc(b'\n');
                } else if byte == b'\r' {
                    uart.putc(b'\r');
                } else if byte == b'\t' {
                    uart.putc(b'\t');
                } else {
                    uart.putc(b'?'); // Non-printable character
                }
            }
            Ok(())
        })?;

        uart.puts("\n=== End of File ===\n");
        Ok(())
    }

    /// Print file hex dump via UART
    pub fn print_file_hex(
        &self,
        sd_card: &mut SdCard,
        cluster_chain: &mut ClusterChain,
        file_info: &FileInfo,
    ) -> Result<(), Fat32Error> {
        let uart = crate::uart::Uart::new();

        uart.puts("=== File Hex Dump ===\n");
        uart.puts("Size: ");
        uart.put_hex(file_info.size as u64);
        uart.puts(" bytes\n");

        if file_info.size == 0 {
            uart.puts("(Empty file)\n");
            return Ok(());
        }

        let mut offset = 0;

        // Read file in chunks and print hex
        self.read_file_chunked(sd_card, cluster_chain, file_info, |chunk| {
            for &byte in chunk {
                if offset % 16 == 0 {
                    uart.put_hex(offset as u64);
                    uart.puts(": ");
                }

                uart.put_hex(byte as u64);
                uart.putc(b' ');

                if offset % 16 == 15 {
                    uart.putc(b'\n');
                }

                offset += 1;
            }
            Ok(())
        })?;

        if offset % 16 != 0 {
            uart.putc(b'\n');
        }

        uart.puts("=== End of Hex Dump ===\n");
        Ok(())
    }
}

/// File validation result
#[derive(Debug)]
pub struct FileValidationResult {
    pub is_valid: bool,
    pub clusters_validated: u32,
    pub bytes_validated: u32,
    pub error_message: &'static str,
}

impl FileValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: false,
            clusters_validated: 0,
            bytes_validated: 0,
            error_message: "",
        }
    }

    pub fn print_result(&self) {
        let uart = crate::uart::Uart::new();
        uart.puts("=== File Validation Result ===\n");

        if self.is_valid {
            uart.puts("File is valid\n");
        } else {
            uart.puts("File is invalid: ");
            uart.puts(self.error_message);
            uart.putc(b'\n');
        }

        uart.puts("Clusters validated: ");
        uart.put_hex(self.clusters_validated as u64);
        uart.putc(b'\n');

        uart.puts("Bytes validated: ");
        uart.put_hex(self.bytes_validated as u64);
        uart.putc(b'\n');
    }
}
