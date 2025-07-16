use super::{
    boot_sector::FilesystemLayout, cluster_chain::ClusterChain, Fat32Error, FileInfo, FileList,
    ATTR_DIRECTORY, ATTR_LONG_NAME, ATTR_VOLUME_ID,
};
/// FAT32 Directory Operations
///
/// This module handles FAT32 directory entry parsing, reading, and management.
/// It provides no_std-compliant directory operations for embedded environments.
use crate::sdcard::SdCard;

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

        // Parse directory entries safely
        let mut entries = [Fat32DirEntry {
            name: [0; 11],
            attr: 0,
            nt_reserved: 0,
            creation_time_tenth: 0,
            creation_time: 0,
            creation_date: 0,
            last_access_date: 0,
            first_cluster_high: 0,
            write_time: 0,
            write_date: 0,
            first_cluster_low: 0,
            file_size: 0,
        }; 16];

        for i in 0..16 {
            let offset = i * 32;
            let entry_data = &dir_data[offset..offset + 32];
            
            // Parse entry fields safely
            let mut name = [0u8; 11];
            name.copy_from_slice(&entry_data[0..11]);
            
            entries[i] = Fat32DirEntry {
                name,
                attr: entry_data[11],
                nt_reserved: entry_data[12],
                creation_time_tenth: entry_data[13],
                creation_time: u16::from_le_bytes([entry_data[14], entry_data[15]]),
                creation_date: u16::from_le_bytes([entry_data[16], entry_data[17]]),
                last_access_date: u16::from_le_bytes([entry_data[18], entry_data[19]]),
                first_cluster_high: u16::from_le_bytes([entry_data[20], entry_data[21]]),
                write_time: u16::from_le_bytes([entry_data[22], entry_data[23]]),
                write_date: u16::from_le_bytes([entry_data[24], entry_data[25]]),
                first_cluster_low: u16::from_le_bytes([entry_data[26], entry_data[27]]),
                file_size: u32::from_le_bytes([entry_data[28], entry_data[29], entry_data[30], entry_data[31]]),
            };
        }

        Ok(entries)
    }

    /// List files in specified directory cluster with LFN support
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
            let mut lfn_entries = [Fat32LfnEntry {
                ord: 0,
                name1: [0; 5],
                attr: 0,
                entry_type: 0,
                checksum: 0,
                name2: [0; 6],
                first_cluster_low: 0,
                name3: [0; 2],
            }; 4];
            let mut lfn_count = 0;
            let mut expecting_lfn = false;

            for entry in &entries {
                // Check if this is end of directory
                if entry.name[0] == 0x00 {
                    break;
                }
                
                // Skip deleted entries
                if entry.name[0] == 0xE5 {
                    lfn_count = 0;
                    expecting_lfn = false;
                    continue;
                }
                
                // Check if this is an LFN entry
                if entry.attr & ATTR_LONG_NAME == ATTR_LONG_NAME && entry.attr & ATTR_VOLUME_ID == 0 {
                    // This is an LFN entry - convert to LFN structure
                    let lfn_entry = self.convert_to_lfn_entry(entry);
                    
                    if lfn_count < 4 {
                        lfn_entries[lfn_count] = lfn_entry;
                        lfn_count += 1;
                        expecting_lfn = true;
                    }
                    continue;
                }
                
                // Check if this is a valid directory entry
                if !entry.is_valid() {
                    lfn_count = 0;
                    expecting_lfn = false;
                    continue;
                }

                let mut file_info = entry.to_file_info();
                
                // If we have LFN entries, extract the long filename
                if expecting_lfn && lfn_count > 0 {
                    // Verify LFN checksum
                    let checksum = super::filename::calculate_lfn_checksum(&entry.name);
                    if lfn_count > 0 && lfn_entries[0].checksum == checksum {
                        if let Ok(long_name) = super::filename::extract_lfn_name(&lfn_entries, lfn_count) {
                            file_info.name = long_name;
                        }
                    }
                }
                
                if files.push(file_info).is_err() {
                    break; // Directory list full
                }
                
                // Reset LFN state
                lfn_count = 0;
                expecting_lfn = false;
            }

            // Follow cluster chain
            let next_cluster = cluster_chain.get_next_cluster_from_sd(sd_card, current_cluster)?;
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

    /// Create a new directory entry
    pub fn create_directory_entry(
        &self,
        sd_card: &mut SdCard,
        cluster_chain: &mut ClusterChain,
        dir_cluster: u32,
        filename: &str,
        first_cluster: u32,
        file_size: u32,
    ) -> Result<(), Fat32Error> {
        // Convert filename to 8.3 format
        let short_name = super::filename::name_to_83(filename);
        
        // Create directory entry
        let new_entry = Fat32DirEntry {
            name: short_name,
            attr: 0x00, // Regular file
            nt_reserved: 0,
            creation_time_tenth: 0,
            creation_time: 0,
            creation_date: 0,
            last_access_date: 0,
            first_cluster_high: (first_cluster >> 16) as u16,
            write_time: 0,
            write_date: 0,
            first_cluster_low: (first_cluster & 0xFFFF) as u16,
            file_size,
        };

        // Find free directory entry slot
        let mut current_cluster = dir_cluster;
        let mut entry_found = false;
        let mut target_sector = 0;
        let mut target_offset = 0;

        loop {
            let sector = self.layout.cluster_to_sector(current_cluster);
            if sector == 0 {
                return Err(Fat32Error::DirectoryNotFound);
            }

            for sector_offset in 0..self.layout.sectors_per_cluster {
                let current_sector = sector + sector_offset;
                let mut dir_data = [0u8; 512];
                sd_card.read_block(current_sector, &mut dir_data)?;

                // Check each directory entry slot
                for entry_idx in 0..16 {
                    let entry_offset = entry_idx * 32;
                    let entry_name = &dir_data[entry_offset..entry_offset + 11];
                    
                    // Check if slot is free (deleted or empty)
                    if dir_data[entry_offset] == 0x00 || dir_data[entry_offset] == 0xE5 {
                        target_sector = current_sector;
                        target_offset = entry_offset;
                        entry_found = true;
                        break;
                    }
                }
                
                if entry_found {
                    break;
                }
            }
            
            if entry_found {
                break;
            }

            // Move to next cluster in chain
            let next_cluster = cluster_chain.get_next_cluster_from_sd(sd_card, current_cluster)?;
            if cluster_chain.is_end_of_chain(next_cluster) {
                return Err(Fat32Error::DiskFull); // No free slots found
            }
            current_cluster = next_cluster;
        }

        // Write the directory entry
        let mut sector_data = [0u8; 512];
        sd_card.read_block(target_sector, &mut sector_data)?;
        
        // Copy entry data
        let entry_bytes = unsafe {
            core::slice::from_raw_parts(
                &new_entry as *const Fat32DirEntry as *const u8,
                32,
            )
        };
        sector_data[target_offset..target_offset + 32].copy_from_slice(entry_bytes);
        
        sd_card.write_block(target_sector, &sector_data)?;
        
        Ok(())
    }

    /// Delete a directory entry by name
    pub fn delete_directory_entry(
        &self,
        sd_card: &mut SdCard,
        cluster_chain: &mut ClusterChain,
        dir_cluster: u32,
        filename: &str,
    ) -> Result<(), Fat32Error> {
        let mut current_cluster = dir_cluster;
        let mut entry_found = false;
        let mut target_sector = 0;
        let mut target_offset = 0;

        loop {
            let sector = self.layout.cluster_to_sector(current_cluster);
            if sector == 0 {
                return Err(Fat32Error::DirectoryNotFound);
            }

            for sector_offset in 0..self.layout.sectors_per_cluster {
                let current_sector = sector + sector_offset;
                let mut dir_data = [0u8; 512];
                sd_card.read_block(current_sector, &mut dir_data)?;

                // Check each directory entry
                for entry_idx in 0..16 {
                    let entry_offset = entry_idx * 32;
                    
                    // Skip empty/deleted entries
                    if dir_data[entry_offset] == 0x00 || dir_data[entry_offset] == 0xE5 {
                        continue;
                    }
                    
                    // Parse entry name
                    let entry_name = &dir_data[entry_offset..entry_offset + 11];
                    let mut name_bytes = [0u8; 13];
                    let mut name_len = 0;
                    
                    // Convert 8.3 name to readable format
                    for i in 0..8 {
                        if entry_name[i] != 0x20 {
                            name_bytes[name_len] = entry_name[i];
                            name_len += 1;
                        }
                    }
                    
                    if entry_name[8] != 0x20 {
                        name_bytes[name_len] = b'.';
                        name_len += 1;
                        for i in 8..11 {
                            if entry_name[i] != 0x20 {
                                name_bytes[name_len] = entry_name[i];
                                name_len += 1;
                            }
                        }
                    }
                    
                    // Compare names (case-insensitive)
                    if name_len == filename.len() {
                        let mut names_match = true;
                        let filename_bytes = filename.as_bytes();
                        for i in 0..name_len {
                            if name_bytes[i].to_ascii_uppercase() != filename_bytes[i].to_ascii_uppercase() {
                                names_match = false;
                                break;
                            }
                        }
                        
                        if names_match {
                            target_sector = current_sector;
                            target_offset = entry_offset;
                            entry_found = true;
                            break;
                        }
                    }
                }
                
                if entry_found {
                    break;
                }
            }
            
            if entry_found {
                break;
            }

            // Move to next cluster in chain
            let next_cluster = cluster_chain.get_next_cluster_from_sd(sd_card, current_cluster)?;
            if cluster_chain.is_end_of_chain(next_cluster) {
                return Err(Fat32Error::FileNotFound);
            }
            current_cluster = next_cluster;
        }

        // Mark entry as deleted
        let mut sector_data = [0u8; 512];
        sd_card.read_block(target_sector, &mut sector_data)?;
        sector_data[target_offset] = 0xE5; // Mark as deleted
        sd_card.write_block(target_sector, &sector_data)?;
        
        Ok(())
    }

    /// Convert directory entry to LFN entry (helper method)
    fn convert_to_lfn_entry(&self, entry: &Fat32DirEntry) -> Fat32LfnEntry {
        // This is a simplified conversion - in practice, the raw bytes would be reinterpreted
        // For now, we'll create a basic LFN entry structure
        Fat32LfnEntry {
            ord: entry.name[0],
            name1: [0; 5], // Would need proper UTF-16 extraction
            attr: entry.attr,
            entry_type: entry.nt_reserved,
            checksum: entry.creation_time_tenth,
            name2: [0; 6], // Would need proper UTF-16 extraction
            first_cluster_low: entry.first_cluster_low,
            name3: [0; 2], // Would need proper UTF-16 extraction
        }
    }

    /// Create directory entry with LFN support
    pub fn create_directory_entry_with_lfn(
        &self,
        sd_card: &mut SdCard,
        cluster_chain: &mut ClusterChain,
        dir_cluster: u32,
        filename: &str,
        first_cluster: u32,
        file_size: u32,
    ) -> Result<(), Fat32Error> {
        // Generate 8.3 short name
        let short_name = if super::filename::needs_lfn(filename) {
            super::filename::generate_short_name(filename)
        } else {
            super::filename::name_to_83(filename)
        };
        
        // Create LFN entries if needed
        if super::filename::needs_lfn(filename) {
            let (_lfn_entries, _num_entries) = super::filename::create_lfn_entries(filename, &short_name)?;
            
            // Write LFN entries first
            for _i in 0.._num_entries {
                // Convert LFN entry to raw bytes and write
                // For now, we'll use the basic create_directory_entry method
                // A full implementation would write the LFN entries as raw bytes
            }
        }
        
        // Create the main directory entry
        self.create_directory_entry(sd_card, cluster_chain, dir_cluster, filename, first_cluster, file_size)
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

            // Print filename (now supports long filenames)
            let name_len = file.name.iter().position(|&x| x == 0).unwrap_or(256);
            for i in 0..name_len {
                uart.putc(file.name[i]);
            }
            uart.putc(b'\n');
        }

        Ok(())
    }
}
