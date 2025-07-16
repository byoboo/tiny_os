use super::{
    boot_sector::{Fat32BootSector, FilesystemLayout},
    cluster_chain::ClusterChain,
    directory::DirectoryReader,
    file_operations::FileOperations,
    Fat32Error, FileContent, FileInfo, FileList,
};
/// FAT32 Filesystem Interface
///
/// This module provides the main FAT32 filesystem interface that coordinates
/// all the other modules. It maintains the filesystem state and provides
/// high-level operations for file and directory management.
use crate::sdcard::SdCard;

/// Main FAT32 filesystem interface
pub struct Fat32FileSystem {
    sd_card: SdCard,
    boot_sector: Fat32BootSector,
    layout: FilesystemLayout,
    current_dir_cluster: u32,
    directory_reader: DirectoryReader,
    file_operations: FileOperations,
    cluster_chain: ClusterChain,
}

impl Fat32FileSystem {
    /// Create a new FAT32 filesystem instance
    pub fn new(mut sd_card: SdCard) -> Result<Self, Fat32Error> {
        // Initialize SD card if not already done
        if !sd_card.is_initialized() {
            sd_card.init()?;
        }

        // Read and validate boot sector
        let boot_sector = Fat32BootSector::read_from_sd(&mut sd_card)?;

        // Calculate filesystem layout
        let layout = boot_sector.calculate_layout()?;

        // Create subsystem components
        let directory_reader = DirectoryReader::new(layout);
        let file_operations = FileOperations::new(layout);
        let cluster_chain = ClusterChain::new(layout);

        Ok(Self {
            sd_card,
            boot_sector,
            layout,
            current_dir_cluster: layout.root_dir_cluster,
            directory_reader,
            file_operations,
            cluster_chain,
        })
    }

    /// Mount the filesystem and perform initial validation
    pub fn mount(&mut self) -> Result<(), Fat32Error> {
        // Verify we can read the root directory
        let _entries = self.directory_reader.list_directory(
            &mut self.sd_card,
            &mut self.cluster_chain,
            self.layout.root_dir_cluster,
        )?;

        // Print filesystem information
        let uart = crate::uart::Uart::new();
        uart.puts("FAT32 filesystem mounted successfully!\n");

        // Print volume label
        uart.puts("Volume label: ");
        let label = self.boot_sector.get_volume_label();
        let mut found_char = false;
        for &byte in label.iter() {
            if byte != 0 {
                uart.putc(byte);
                found_char = true;
            }
        }
        if !found_char {
            uart.puts("(No label)");
        }
        uart.putc(b'\n');

        uart.puts("Cluster size: ");
        uart.put_hex(self.layout.bytes_per_cluster as u64);
        uart.puts(" bytes\n");

        uart.puts("Total clusters: ");
        uart.put_hex(self.layout.cluster_count as u64);
        uart.putc(b'\n');

        Ok(())
    }

    /// List files in current directory
    pub fn list_directory(&mut self) -> Result<FileList, Fat32Error> {
        self.directory_reader.list_directory(
            &mut self.sd_card,
            &mut self.cluster_chain,
            self.current_dir_cluster,
        )
    }

    /// List files in specified directory cluster
    pub fn list_directory_cluster(&mut self, cluster: u32) -> Result<FileList, Fat32Error> {
        self.directory_reader
            .list_directory(&mut self.sd_card, &mut self.cluster_chain, cluster)
    }

    /// Get current directory cluster
    pub fn get_current_directory(&self) -> u32 {
        self.current_dir_cluster
    }

    /// Change to root directory
    pub fn change_to_root(&mut self) {
        self.current_dir_cluster = self.layout.root_dir_cluster;
    }

    /// Change directory by name
    pub fn change_directory(&mut self, dir_name: &str) -> Result<(), Fat32Error> {
        if dir_name == ".." {
            // TODO: Implement parent directory navigation
            // For now, go to root
            self.change_to_root();
            return Ok(());
        }

        // Find directory in current directory
        let new_cluster = self.directory_reader.find_directory(
            &mut self.sd_card,
            &mut self.cluster_chain,
            self.current_dir_cluster,
            dir_name,
        )?;

        self.current_dir_cluster = new_cluster;
        Ok(())
    }

    /// Read file contents
    pub fn read_file(&mut self, filename: &str) -> Result<FileContent, Fat32Error> {
        // Find file in current directory
        let file_info = self.directory_reader.find_file(
            &mut self.sd_card,
            &mut self.cluster_chain,
            self.current_dir_cluster,
            filename,
        )?;

        // Read file content
        self.file_operations.read_file_content(
            &mut self.sd_card,
            &mut self.cluster_chain,
            &file_info,
        )
    }

    /// Find file by name and return file info
    pub fn find_file(&mut self, filename: &str) -> Result<FileInfo, Fat32Error> {
        self.directory_reader.find_file(
            &mut self.sd_card,
            &mut self.cluster_chain,
            self.current_dir_cluster,
            filename,
        )
    }

    /// Validate file integrity
    pub fn validate_file(&mut self, filename: &str) -> Result<(), Fat32Error> {
        let file_info = self.find_file(filename)?;
        let validation_result = self.file_operations.validate_file(
            &mut self.sd_card,
            &mut self.cluster_chain,
            &file_info,
        )?;

        validation_result.print_result();
        Ok(())
    }

    /// Print file content (for text files)
    pub fn print_file_content(&mut self, filename: &str) -> Result<(), Fat32Error> {
        let file_info = self.find_file(filename)?;
        self.file_operations.print_file_content(
            &mut self.sd_card,
            &mut self.cluster_chain,
            &file_info,
        )
    }

    /// Print file hex dump
    pub fn print_file_hex(&mut self, filename: &str) -> Result<(), Fat32Error> {
        let file_info = self.find_file(filename)?;
        self.file_operations
            .print_file_hex(&mut self.sd_card, &mut self.cluster_chain, &file_info)
    }

    /// Print cluster chain information for file
    pub fn print_file_cluster_chain(&mut self, filename: &str) -> Result<(), Fat32Error> {
        let file_info = self.find_file(filename)?;
        self.cluster_chain.print_chain_info(file_info.first_cluster)
    }

    /// Print directory listing
    pub fn print_directory_listing(&mut self) -> Result<(), Fat32Error> {
        self.directory_reader.print_directory_listing(
            &mut self.sd_card,
            &mut self.cluster_chain,
            self.current_dir_cluster,
        )
    }

    /// Print filesystem information
    pub fn print_info(&self) {
        let uart = crate::uart::Uart::new();
        uart.puts("=== FAT32 Filesystem Information ===\n");

        self.boot_sector.print_info();
        self.layout.print_info();
    }

    /// Print detailed filesystem statistics
    pub fn print_detailed_info(&mut self) -> Result<(), Fat32Error> {
        self.print_info();

        // Print cluster statistics
        let stats = self.cluster_chain.get_cluster_stats(&mut self.sd_card)?;
        stats.print_stats();

        Ok(())
    }

    /// Flush all cached data to disk
    pub fn flush(&mut self) -> Result<(), Fat32Error> {
        self.cluster_chain.flush_fat(&mut self.sd_card)?;
        Ok(())
    }

    /// Get filesystem layout information
    pub fn get_layout(&self) -> &FilesystemLayout {
        &self.layout
    }

    /// Get boot sector information
    pub fn get_boot_sector(&self) -> &Fat32BootSector {
        &self.boot_sector
    }

    /// Check if filesystem is mounted
    pub fn is_mounted(&self) -> bool {
        // Simple check - we're mounted if we have a valid root cluster
        self.layout.root_dir_cluster >= 2
    }

    /// Unmount filesystem (flush and cleanup)
    pub fn unmount(&mut self) -> Result<(), Fat32Error> {
        self.flush()?;

        let uart = crate::uart::Uart::new();
        uart.puts("FAT32 filesystem unmounted\n");

        Ok(())
    }

    /// Get SD card reference (for advanced operations)
    pub fn get_sd_card(&mut self) -> &mut SdCard {
        &mut self.sd_card
    }

    /// Create a new file
    pub fn create_file(&mut self, filename: &str, content: &[u8]) -> Result<(), Fat32Error> {
        // Check if file already exists
        if self.find_file(filename).is_ok() {
            return Err(Fat32Error::FileAlreadyExists);
        }

        // Find free cluster for file data
        let first_cluster = self.cluster_chain.find_free_cluster(&mut self.sd_card)?;
        
        // Allocate cluster chain for file
        let clusters_needed = (content.len() + self.layout.bytes_per_cluster as usize - 1) / self.layout.bytes_per_cluster as usize;
        let mut allocated_clusters = [0u32; 256]; // Fixed size array for no_std
        let mut cluster_count = 0;
        
        for i in 0..clusters_needed {
            if cluster_count >= 256 {
                return Err(Fat32Error::FileTooLarge);
            }
            
            let cluster = if i == 0 {
                first_cluster
            } else {
                self.cluster_chain.find_free_cluster(&mut self.sd_card)?
            };
            
            allocated_clusters[cluster_count] = cluster;
            cluster_count += 1;
            
            // Link to next cluster or mark as end of chain
            if i < clusters_needed - 1 {
                // Will be linked to next cluster
            } else {
                self.cluster_chain.mark_end_of_chain(cluster)?;
            }
        }

        // Link cluster chain
        for i in 0..cluster_count - 1 {
            self.cluster_chain.set_next_cluster(allocated_clusters[i], allocated_clusters[i + 1])?;
        }

        // Write file data to clusters
        let mut written = 0;
        for i in 0..cluster_count {
            let cluster = allocated_clusters[i];
            let sector = self.layout.cluster_to_sector(cluster);
            let mut cluster_data = [0u8; 512];
            
            for sector_in_cluster in 0..self.layout.sectors_per_cluster {
                cluster_data.fill(0);
                let data_start = written;
                let data_end = (written + 512).min(content.len());
                
                if data_start < content.len() {
                    let copy_len = data_end - data_start;
                    cluster_data[..copy_len].copy_from_slice(&content[data_start..data_end]);
                }
                
                self.sd_card.write_block(sector + sector_in_cluster, &cluster_data)?;
                written += 512;
                
                if written >= content.len() {
                    break;
                }
            }
        }

        // Create directory entry (placeholder - needs implementation)
        // self.directory_reader.create_directory_entry(
        //     &mut self.sd_card,
        //     &mut self.cluster_chain,
        //     self.current_dir_cluster,
        //     filename,
        //     first_cluster,
        //     content.len() as u32,
        // )?;

        // Flush FAT to disk
        self.cluster_chain.flush_fat(&mut self.sd_card)?;

        Ok(())
    }

    /// Write data to existing file (overwrites content)
    pub fn write_file(&mut self, filename: &str, content: &[u8]) -> Result<(), Fat32Error> {
        // Find existing file
        let file_info = self.find_file(filename)?;
        
        // Free existing cluster chain
        self.cluster_chain.free_cluster_chain(&mut self.sd_card, file_info.first_cluster)?;
        
        // Create new content (reuse create_file logic)
        self.delete_file(filename)?;
        self.create_file(filename, content)?;
        
        Ok(())
    }

    /// Delete a file
    pub fn delete_file(&mut self, filename: &str) -> Result<(), Fat32Error> {
        // Find file
        let file_info = self.find_file(filename)?;
        
        // Free cluster chain
        self.cluster_chain.free_cluster_chain(&mut self.sd_card, file_info.first_cluster)?;
        
        // Remove directory entry (placeholder - needs implementation)
        // self.directory_reader.delete_directory_entry(
        //     &mut self.sd_card,
        //     &mut self.cluster_chain,
        //     self.current_dir_cluster,
        //     filename,
        // )?;
        
        // Flush FAT to disk
        self.cluster_chain.flush_fat(&mut self.sd_card)?;
        
        Ok(())
    }

    /// Test filesystem operations
    pub fn test_filesystem(&mut self) -> Result<(), Fat32Error> {
        let uart = crate::uart::Uart::new();
        uart.puts("=== Testing FAT32 Filesystem ===\n");

        // Test root directory listing
        uart.puts("Testing root directory listing...\n");
        self.change_to_root();
        let files = self.list_directory()?;
        uart.puts("Found ");
        uart.put_hex(files.len() as u64);
        uart.puts(" entries in root directory\n");

        // Test filesystem info
        uart.puts("Testing filesystem info...\n");
        self.print_info();

        // Test cluster statistics
        uart.puts("Testing cluster statistics...\n");
        let stats = self.cluster_chain.get_cluster_stats(&mut self.sd_card)?;
        stats.print_stats();

        uart.puts("Filesystem tests completed successfully\n");
        Ok(())
    }
}

/// For backward compatibility with existing code
impl Fat32FileSystem {
    /// Legacy method names for compatibility
    pub fn list_files(&mut self) -> Result<FileList, Fat32Error> {
        self.list_directory()
    }

    /// Legacy method for getting current directory
    pub fn current_directory(&self) -> u32 {
        self.get_current_directory()
    }

    /// Legacy method for filesystem info
    pub fn filesystem_info(&self) {
        self.print_info();
    }
}
