/// FAT32 Directory Operations
///
/// This module handles FAT32 directory entry parsing, reading, and management.
/// It provides no_std-compliant directory operations for embedded environments.

use crate::sdcard::SdCard;
use super::{Fat32Error, FileInfo, FileList, ATTR_LONG_NAME, ATTR_VOLUME_ID, ATTR_DIRECTORY};
use super::boot_sector::FilesystemLayout;
use super::cluster_chain::ClusterChain;

// FAT32 Directory Entry (32 bytes)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Fat32DirEntry {
    pub name: [u8; 11],          // 8.3 filename
    pub attr: u8,                // Attributes
    pub nt_reserved: u8,         // Reserved
    pub creation_time_tenth: u8, // Creation time (tenths of second)
    pub creation_time: u16,      // Creation time
    pub creation_date: u16,      // Creation date
    pub last_access_date: u16,   // Last access date
    pub first_cluster_high: u16, // High 16 bits of first cluster
    pub write_time: u16,         // Write time
    pub write_date: u16,         // Write date
    pub first_cluster_low: u16,  // Low 16 bits of first cluster
    pub file_size: u32,          // File size in bytes
}

// Long File Name (LFN) Entry (32 bytes)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Fat32LfnEntry {
    pub ord: u8,                // Order/sequence number
    pub name1: [u16; 5],        // First 5 characters (UTF-16)
    pub attr: u8,               // Attribute (always 0x0F for LFN)
    pub entry_type: u8,         // Type (0 for name entry)
    pub checksum: u8,           // Checksum of 8.3 name
    pub name2: [u16; 6],        // Next 6 characters
    pub first_cluster_low: u16, // Always 0 for LFN
    pub name3: [u16; 2],        // Last 2 characters
}

impl Fat32DirEntry {
    /// Convert directory entry to FileInfo
    pub fn to_file_info(&self) -> FileInfo {
        let mut file_info = FileInfo::new();

        // Copy short name
        file_info.short_name.copy_from_slice(&self.name);

        // Convert 8.3 name to readable format
        let mut name_len = 0;
        for i in 0..8 {
            if self.name[i] != 0x20 {
                file_info.name[name_len] = self.name[i];
                name_len += 1;
            }
        }

        // Add extension if present
        if self.name[8] != 0x20 {
            file_info.name[name_len] = b'.';
            name_len += 1;
            for i in 8..11 {
                if self.name[i] != 0x20 {
                    file_info.name[name_len] = self.name[i];
                    name_len += 1;
                }
            }
        }

        file_info.size = self.file_size;
        file_info.first_cluster = 
            ((self.first_cluster_high as u32) << 16) | (self.first_cluster_low as u32);
        file_info.attributes = self.attr;
        file_info.is_directory = (self.attr & ATTR_DIRECTORY) != 0;
        file_info.creation_time = self.creation_time;
        file_info.creation_date = self.creation_date;
        file_info.modified_time = self.write_time;
        file_info.modified_date = self.write_date;

        file_info
    }

    /// Check if this is a valid directory entry
    pub fn is_valid(&self) -> bool {
        // Skip empty entries
        if self.name[0] == 0x00 {
            return false; // End of directory
        }
        if self.name[0] == 0xE5 {
            return false; // Deleted entry
        }

        // Skip LFN entries and volume labels
        if self.attr & ATTR_LONG_NAME == ATTR_LONG_NAME || self.attr & ATTR_VOLUME_ID != 0 {
            return false;
        }

        true
    }

    /// Get the filename as a string (for display purposes)
    pub fn get_filename(&self) -> [u8; 13] {
        let mut result = [0u8; 13];
        let mut idx = 0;

        // Copy name part
        for i in 0..8 {
            if self.name[i] != 0x20 {
                result[idx] = self.name[i];
                idx += 1;
            }
        }

        // Add extension if present
        if self.name[8] != 0x20 {
            result[idx] = b'.';
            idx += 1;
            for i in 8..11 {
                if self.name[i] != 0x20 {
                    result[idx] = self.name[i];
                    idx += 1;
                }
            }
        }

        result
    }
}

/// Directory reader for FAT32 filesystem
pub struct DirectoryReader {
    layout: FilesystemLayout,
}

impl DirectoryReader {
    /// Create a new directory reader
    pub fn new(layout: FilesystemLayout) -> Self {
        Self { layout }
    }

    /// Read directory entries from a cluster
    pub fn read_directory_cluster(
        &self,
        sd_card: &mut SdCard,
        cluster: u32,
    ) -> Result<[Fat32DirEntry; 16], Fat32Error> {
        let sector = self.layout.cluster_to_sector(cluster);
        if sector == 0 {
            return Err(Fat32Error::DirectoryNotFound);
        }

        let mut dir_data = [0u8; 512];
        sd_card.read_block(sector, &mut dir_data)?;

        // Convert to directory entries
        let entries = unsafe { 
            core::mem::transmute::<[u8; 512], [Fat32DirEntry; 16]>(dir_data) 
        };

        Ok(entries)
    }

    /// List files in specified directory cluster
    pub fn list_directory(
        &self,
        sd_card: &mut SdCard,
        cluster_chain: &mut ClusterChain,
        cluster: u32,
    ) -> Result<FileList, Fat32Error> {
        let mut files = FileList::new();
        let mut current_cluster = cluster;

        loop {
            let entries = self.read_directory_cluster(sd_card, current_cluster)?;

            for entry in &entries {
                // Check if this is a valid entry
                if !entry.is_valid() {
                    if entry.name[0] == 0x00 {
                        break; // End of directory
                    }
                    continue; // Skip invalid entries
                }

                let file_info = entry.to_file_info();

                if files.push(file_info).is_err() {
                    break; // Directory list full
                }
            }

            // Follow cluster chain
            let next_cluster = cluster_chain.get_next_cluster(current_cluster)?;
            if cluster_chain.is_end_of_chain(next_cluster) {
                break; // End of chain
            }
            current_cluster = next_cluster;
        }

        Ok(files)
    }

    /// Find a file or directory by name in the given directory cluster
    pub fn find_entry(
        &self,
        sd_card: &mut SdCard,
        cluster_chain: &mut ClusterChain,
        dir_cluster: u32,
        name: &str,
    ) -> Result<Option<FileInfo>, Fat32Error> {
        let files = self.list_directory(sd_card, cluster_chain, dir_cluster)?;

        for i in 0..files.len() {
            let file = &files[i];
            let name_len = file.name.iter().position(|&x| x == 0).unwrap_or(256);
            let file_name = core::str::from_utf8(&file.name[..name_len.min(name.len())])
                .map_err(|_| Fat32Error::InvalidFilename)?;

            if file_name.eq_ignore_ascii_case(name) {
                return Ok(Some(*file));
            }
        }

        Ok(None)
    }

    /// Find a directory by name and return its cluster
    pub fn find_directory(
        &self,
        sd_card: &mut SdCard,
        cluster_chain: &mut ClusterChain,
        parent_cluster: u32,
        dir_name: &str,
    ) -> Result<u32, Fat32Error> {
        if let Some(entry) = self.find_entry(sd_card, cluster_chain, parent_cluster, dir_name)? {
            if entry.is_directory {
                return Ok(entry.first_cluster);
            } else {
                return Err(Fat32Error::NotADirectory);
            }
        }

        Err(Fat32Error::DirectoryNotFound)
    }

    /// Find a file by name and return its info
    pub fn find_file(
        &self,
        sd_card: &mut SdCard,
        cluster_chain: &mut ClusterChain,
        dir_cluster: u32,
        filename: &str,
    ) -> Result<FileInfo, Fat32Error> {
        if let Some(entry) = self.find_entry(sd_card, cluster_chain, dir_cluster, filename)? {
            if !entry.is_directory {
                return Ok(entry);
            } else {
                return Err(Fat32Error::NotAFile);
            }
        }

        Err(Fat32Error::FileNotFound)
    }

    /// Print directory listing via UART
    pub fn print_directory_listing(
        &self,
        sd_card: &mut SdCard,
        cluster_chain: &mut ClusterChain,
        cluster: u32,
    ) -> Result<(), Fat32Error> {
        let files = self.list_directory(sd_card, cluster_chain, cluster)?;
        let uart = crate::uart::Uart::new();

        uart.puts("Directory listing:\n");
        uart.puts("Type  Size       Name\n");
        uart.puts("----  ----       ----\n");

        for i in 0..files.len() {
            let file = &files[i];
            
            // Print file type
            if file.is_directory {
                uart.puts("DIR   ");
            } else {
                uart.puts("FILE  ");
            }

            // Print file size
            if file.is_directory {
                uart.puts("         ");
            } else {
                uart.put_hex(file.size as u64);
                uart.puts("  ");
            }

            // Print filename
            let name_len = file.name.iter().position(|&x| x == 0).unwrap_or(256);
            for i in 0..name_len {
                uart.putc(file.name[i]);
            }
            uart.putc(b'\n');
        }

        Ok(())
    }
}
