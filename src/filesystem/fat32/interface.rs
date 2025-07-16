use super::{
    boot_sector::{Fat32BootSector, FilesystemLayout},
    cluster_chain::ClusterChain,
    directory::DirectoryReader,
    file_operations::FileOperations,
    Fat32Error, FileContent, FileInfo, FileList,
};
/// FAT32 Filesystem Interface
///
/// This module provides the main FAT32 filesystem interface with complete write
/// support and memory safety. It coordinates all filesystem operations and
/// provides high-level operations for file and directory management.
///
/// # Key Features
///
/// - **Complete Write Support**: Create, modify, and delete files with proper directory entries
/// - **Memory Safety**: All operations use safe Rust with no unsafe code
/// - **Directory Management**: Safe directory entry creation and deletion
/// - **Cluster Chain Management**: Efficient FAT operations with cycle detection
/// - **Error Recovery**: Comprehensive error handling and validation
/// - **Standard Compatibility**: FAT32 format compatible with all operating systems
///
/// # Recent Improvements
///
/// - Implemented directory entry creation and deletion methods
/// - Added complete file write operations with proper cluster allocation
/// - Eliminated all unsafe memory operations
/// - Added cycle detection to prevent infinite loops in corrupted filesystems
/// - Improved error handling and recovery mechanisms
use crate::sdcard::SdCard;

/// Directory path entry for navigation
#[derive(Debug, Clone, Copy)]
struct DirectoryPathEntry {
    cluster: u32,
    parent_cluster: u32,
}

/// Main FAT32 filesystem interface
pub struct Fat32FileSystem {
    sd_card: SdCard,
    boot_sector: Fat32BootSector,
    layout: FilesystemLayout,
    current_dir_cluster: u32,
    directory_reader: DirectoryReader,
    file_operations: FileOperations,
    cluster_chain: ClusterChain,
    directory_path: [DirectoryPathEntry; 32], // Stack for directory navigation
    path_depth: usize,
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

        // Initialize directory path with root
        let mut directory_path = [DirectoryPathEntry {
            cluster: layout.root_dir_cluster,
            parent_cluster: 0,
        }; 32];
        
        directory_path[0] = DirectoryPathEntry {
            cluster: layout.root_dir_cluster,
            parent_cluster: 0, // Root has no parent
        };

        Ok(Self {
            sd_card,
            boot_sector,
            layout,
            current_dir_cluster: layout.root_dir_cluster,
            directory_reader,
            file_operations,
            cluster_chain,
            directory_path,
            path_depth: 0,
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
    
    /// Get current directory path as string
    pub fn get_current_path(&self) -> [u8; 256] {
        let mut path = [0u8; 256];
        let mut path_len;
        
        if self.path_depth == 0 {
            path[0] = b'/';
            path_len = 1;
        } else {
            path[0] = b'/';
            path_len = 1;
            
            // For simplicity, show depth as numbers
            // A full implementation would store directory names
            for i in 1..=self.path_depth {
                if path_len < 250 {
                    path[path_len] = b'd';
                    path[path_len + 1] = b'0' + (i as u8);
                    path[path_len + 2] = b'/';
                    path_len += 3;
                }
            }
        }
        
        path
    }
    
    /// Get parent directory cluster
    pub fn get_parent_directory(&self) -> Option<u32> {
        if self.path_depth > 0 {
            Some(self.directory_path[self.path_depth].parent_cluster)
        } else {
            None // Root has no parent
        }
    }

    /// Change to root directory
    pub fn change_to_root(&mut self) {
        self.current_dir_cluster = self.layout.root_dir_cluster;
        self.path_depth = 0;
        self.directory_path[0] = DirectoryPathEntry {
            cluster: self.layout.root_dir_cluster,
            parent_cluster: 0,
        };
    }

    /// Change directory by name with proper parent support
    pub fn change_directory(&mut self, dir_name: &str) -> Result<(), Fat32Error> {
        if dir_name == ".." {
            // Navigate to parent directory
            if self.path_depth > 0 {
                self.path_depth -= 1;
                self.current_dir_cluster = self.directory_path[self.path_depth].cluster;
            } else {
                // Already at root, stay at root
                self.change_to_root();
            }
            return Ok(());
        }
        
        if dir_name == "." {
            // Current directory - no change needed
            return Ok(());
        }

        // Find directory in current directory
        let new_cluster = self.directory_reader.find_directory(
            &mut self.sd_card,
            &mut self.cluster_chain,
            self.current_dir_cluster,
            dir_name,
        )?;

        // Add current directory to path stack
        if self.path_depth < 31 {
            self.path_depth += 1;
            self.directory_path[self.path_depth] = DirectoryPathEntry {
                cluster: new_cluster,
                parent_cluster: self.current_dir_cluster,
            };
        } else {
            return Err(Fat32Error::InvalidPath); // Path too deep
        }

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

        // Create directory entry
        self.directory_reader.create_directory_entry(
            &mut self.sd_card,
            &mut self.cluster_chain,
            self.current_dir_cluster,
            filename,
            first_cluster,
            content.len() as u32,
        )?;

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
        
        // Remove directory entry
        self.directory_reader.delete_directory_entry(
            &mut self.sd_card,
            &mut self.cluster_chain,
            self.current_dir_cluster,
            filename,
        )?;
        
        // Flush FAT to disk
        self.cluster_chain.flush_fat(&mut self.sd_card)?;
        
        Ok(())
    }
    
    /// Create a new directory
    pub fn create_directory(&mut self, dirname: &str) -> Result<(), Fat32Error> {
        // Check if directory already exists
        if self.directory_reader.find_directory(
            &mut self.sd_card,
            &mut self.cluster_chain,
            self.current_dir_cluster,
            dirname,
        ).is_ok() {
            return Err(Fat32Error::FileAlreadyExists);
        }
        
        // Find free cluster for directory
        let dir_cluster = self.cluster_chain.find_free_cluster(&mut self.sd_card)?;
        
        // Mark cluster as end of chain
        self.cluster_chain.mark_end_of_chain(dir_cluster)?;
        
        // Create directory entry
        self.directory_reader.create_directory_entry(
            &mut self.sd_card,
            &mut self.cluster_chain,
            self.current_dir_cluster,
            dirname,
            dir_cluster,
            0, // Directories have size 0
        )?;
        
        // Initialize directory with "." and ".." entries
        let sector = self.layout.cluster_to_sector(dir_cluster);
        let mut dir_data = [0u8; 512];
        
        // Create "." entry (current directory)
        let dot_name = b".          "; // 11 bytes
        dir_data[0..11].copy_from_slice(dot_name);
        dir_data[11] = super::ATTR_DIRECTORY;
        dir_data[26] = (dir_cluster & 0xFFFF) as u8;
        dir_data[27] = ((dir_cluster >> 8) & 0xFF) as u8;
        dir_data[20] = ((dir_cluster >> 16) & 0xFF) as u8;
        dir_data[21] = ((dir_cluster >> 24) & 0xFF) as u8;
        
        // Create ".." entry (parent directory)
        let dotdot_name = b"..         "; // 11 bytes
        dir_data[32..43].copy_from_slice(dotdot_name);
        dir_data[43] = super::ATTR_DIRECTORY;
        let parent_cluster = if self.current_dir_cluster == self.layout.root_dir_cluster {
            0 // Root directory parent is 0
        } else {
            self.current_dir_cluster
        };
        dir_data[58] = (parent_cluster & 0xFFFF) as u8;
        dir_data[59] = ((parent_cluster >> 8) & 0xFF) as u8;
        dir_data[52] = ((parent_cluster >> 16) & 0xFF) as u8;
        dir_data[53] = ((parent_cluster >> 24) & 0xFF) as u8;
        
        // Write directory data to disk
        self.sd_card.write_block(sector, &dir_data)?;
        
        // Flush FAT to disk
        self.cluster_chain.flush_fat(&mut self.sd_card)?;
        
        Ok(())
    }
    
    /// Remove a directory (must be empty)
    pub fn remove_directory(&mut self, dirname: &str) -> Result<(), Fat32Error> {
        // Find directory
        let dir_cluster = self.directory_reader.find_directory(
            &mut self.sd_card,
            &mut self.cluster_chain,
            self.current_dir_cluster,
            dirname,
        )?;
        
        // Check if directory is empty (only "." and ".." entries)
        let files = self.directory_reader.list_directory(
            &mut self.sd_card,
            &mut self.cluster_chain,
            dir_cluster,
        )?;
        
        // Directory should only contain "." and ".." entries if empty
        if files.len() > 2 {
            return Err(Fat32Error::DirectoryNotFound); // Directory not empty
        }
        
        // Free cluster chain
        self.cluster_chain.free_cluster_chain(&mut self.sd_card, dir_cluster)?;
        
        // Remove directory entry
        self.directory_reader.delete_directory_entry(
            &mut self.sd_card,
            &mut self.cluster_chain,
            self.current_dir_cluster,
            dirname,
        )?;
        
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
