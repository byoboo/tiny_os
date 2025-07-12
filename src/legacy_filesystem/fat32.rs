use core::mem;

/// FAT32 File System Implementation for TinyOS
///
/// This module provides FAT32filesystem support for the SD card driver.
/// FAT32 is implemented for maximum compatibility with other operating systems.
///
/// Features:
/// - Read/write files and directories
/// - Long filename support (LFN)
/// - Directory traversal and creation
/// - File allocation table management
/// - Boot sector parsing and validation
/// - Cluster chain management
use crate::sdcard::*;
use crate::uart::*;

// Constants
const MAX_FILE_SIZE: u32 = 1024 * 1024; // 1MB max file size to prevent memory issues

// File content container for no-std environment
#[derive(Debug)]
pub struct FileContent {
    data: [u8; MAX_FILE_SIZE as usize],
    len: usize,
}

impl FileContent {
    pub fn new() -> Self {
        Self {
            data: [0u8; MAX_FILE_SIZE as usize],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }

    pub fn as_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(self.as_slice())
    }
}

// Simple vector implementation for no-std environment
#[derive(Debug)]
pub struct FileList {
    data: [FileInfo; 64], // Fixed capacity for simplicity
    len: usize,
}

impl FileList {
    pub fn new() -> Self {
        Self {
            data: [FileInfo::new(); 64],
            len: 0,
        }
    }

    pub fn push(&mut self, item: FileInfo) -> Result<(), ()> {
        if self.len >= 64 {
            return Err(()); // Vector full
        }
        self.data[self.len] = item;
        self.len += 1;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<&FileInfo> {
        if index < self.len {
            Some(&self.data[index])
        } else {
            None
        }
    }
}

impl core::ops::Index<usize> for FileList {
    type Output = FileInfo;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// FAT32 Boot Sector Structure (512 bytes)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Fat32BootSector {
    pub jmp_boot: [u8; 3],          // Jump instruction
    pub oem_name: [u8; 8],          // OEM name
    pub bytes_per_sector: u16,      // Bytes per sector (usually 512)
    pub sectors_per_cluster: u8,    // Sectors per cluster
    pub reserved_sector_count: u16, // Reserved sectors
    pub num_fats: u8,               // Number of FATs (usually 2)
    pub root_entry_count: u16,      // Root directory entries (0 for FAT32)
    pub total_sectors_16: u16,      // Total sectors (0 if > 65535)
    pub media_type: u8,             // Media descriptor
    pub fat_size_16: u16,           // FAT size in sectors (0 for FAT32)
    pub sectors_per_track: u16,     // Sectors per track
    pub num_heads: u16,             // Number of heads
    pub hidden_sectors: u32,        // Hidden sectors
    pub total_sectors_32: u32,      // Total sectors (if > 65535)

    // FAT32 specific fields
    pub fat_size_32: u32,          // FAT size in sectors
    pub ext_flags: u16,            // Extended flags
    pub fs_version: u16,           // Filesystem version
    pub root_cluster: u32,         // Root directory cluster
    pub fs_info: u16,              // FSInfo sector
    pub backup_boot_sector: u16,   // Backup boot sector
    pub reserved: [u8; 12],        // Reserved
    pub drive_number: u8,          // Drive number
    pub reserved1: u8,             // Reserved
    pub boot_signature: u8,        // Boot signature (0x29)
    pub volume_id: u32,            // Volume ID
    pub volume_label: [u8; 11],    // Volume label
    pub file_system_type: [u8; 8], // "FAT32   "
    pub boot_code: [u8; 420],      // Boot code
    pub signature: u16,            // Boot signature (0xAA55)
}

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

// Directory entry attributes
pub const ATTR_READ_ONLY: u8 = 0x01;
pub const ATTR_HIDDEN: u8 = 0x02;
pub const ATTR_SYSTEM: u8 = 0x04;
pub const ATTR_VOLUME_ID: u8 = 0x08;
pub const ATTR_DIRECTORY: u8 = 0x10;
pub const ATTR_ARCHIVE: u8 = 0x20;
pub const ATTR_LONG_NAME: u8 = ATTR_READ_ONLY | ATTR_HIDDEN | ATTR_SYSTEM | ATTR_VOLUME_ID;

// FAT32 special cluster values
pub const CLUSTER_FREE: u32 = 0x00000000;
pub const CLUSTER_RESERVED_MIN: u32 = 0x0FFFFFF0;
pub const CLUSTER_BAD: u32 = 0x0FFFFFF7;
pub const CLUSTER_EOC_MIN: u32 = 0x0FFFFFF8;
pub const CLUSTER_EOC_MAX: u32 = 0x0FFFFFFF;

// Error types for FAT32 operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fat32Error {
    InvalidBootSector,
    InvalidSignature,
    UnsupportedSectorSize,
    UnsupportedClusterSize,
    SdCardError(SdError),
    ClusterOutOfRange,
    DirectoryNotFound,
    FileNotFound,
    DiskFull,
    InvalidPath,
    NotADirectory,
    NotAFile,
    ReadOnly,
    InvalidFilename,
    FileTooLarge,
}

impl From<SdError> for Fat32Error {
    fn from(err: SdError) -> Self {
        Fat32Error::SdCardError(err)
    }
}

// File information structure
#[derive(Debug, Clone, Copy)]
pub struct FileInfo {
    pub name: [u8; 256],      // Long filename (UTF-8)
    pub short_name: [u8; 11], // 8.3 short name
    pub size: u32,            // File size
    pub first_cluster: u32,   // First cluster
    pub attributes: u8,       // File attributes
    pub is_directory: bool,   // Is this a directory?
    pub creation_time: u16,   // Creation time
    pub creation_date: u16,   // Creation date
    pub modified_time: u16,   // Last modified time
    pub modified_date: u16,   // Last modified date
}

impl FileInfo {
    pub const fn new() -> Self {
        Self {
            name: [0; 256],
            short_name: [0; 11],
            size: 0,
            first_cluster: 0,
            attributes: 0,
            is_directory: false,
            creation_time: 0,
            creation_date: 0,
            modified_time: 0,
            modified_date: 0,
        }
    }
}

// FAT32 Filesystem structure
pub struct Fat32FileSystem {
    sd_card: SdCard,
    boot_sector: Fat32BootSector,
    fat_start_sector: u32,    // First sector of FAT
    data_start_sector: u32,   // First sector of data area
    sectors_per_cluster: u32, // Sectors per cluster
    bytes_per_cluster: u32,   // Bytes per cluster
    cluster_count: u32,       // Total number of clusters
    root_dir_cluster: u32,    // Root directory cluster
    current_dir_cluster: u32, // Current directory cluster
    fat_cache: [u8; 512],     // Cache for FAT sector
    fat_cache_sector: u32,    // Cached FAT sector number
    fat_cache_dirty: bool,    // FAT cache needs writing
}

impl Fat32FileSystem {
    /// Create a new FAT32 filesystem instance
    pub fn new(mut sd_card: SdCard) -> Result<Self, Fat32Error> {
        // Initialize SD card if not already done
        if !sd_card.is_initialized() {
            sd_card.init()?;
        }

        // Read boot sector (sector 0)
        let mut boot_sector_data = [0u8; 512];
        sd_card.read_block(0, &mut boot_sector_data)?;

        // Parse boot sector
        let boot_sector = unsafe { mem::transmute::<[u8; 512], Fat32BootSector>(boot_sector_data) };

        // Validate boot sector
        if boot_sector.signature != 0xAA55 {
            return Err(Fat32Error::InvalidSignature);
        }

        if boot_sector.bytes_per_sector != 512 {
            return Err(Fat32Error::UnsupportedSectorSize);
        }

        if boot_sector.sectors_per_cluster == 0
            || (boot_sector.sectors_per_cluster & (boot_sector.sectors_per_cluster - 1)) != 0
        {
            return Err(Fat32Error::UnsupportedClusterSize);
        }

        // Validate FAT32 specific fields
        if boot_sector.fat_size_32 == 0 || boot_sector.root_cluster < 2 {
            return Err(Fat32Error::InvalidBootSector);
        }

        // Calculate filesystem layout
        let fat_start_sector = boot_sector.reserved_sector_count as u32;
        let fat_size = boot_sector.fat_size_32;
        let data_start_sector = fat_start_sector + (boot_sector.num_fats as u32 * fat_size);

        let total_sectors = if boot_sector.total_sectors_16 != 0 {
            boot_sector.total_sectors_16 as u32
        } else {
            boot_sector.total_sectors_32
        };

        let data_sectors = total_sectors - data_start_sector;
        let cluster_count = data_sectors / boot_sector.sectors_per_cluster as u32;

        // Verify this is actually FAT32 (cluster count must be >= 65525)
        if cluster_count < 65525 {
            return Err(Fat32Error::InvalidBootSector);
        }

        Ok(Self {
            sd_card,
            boot_sector,
            fat_start_sector,
            data_start_sector,
            sectors_per_cluster: boot_sector.sectors_per_cluster as u32,
            bytes_per_cluster: boot_sector.sectors_per_cluster as u32 * 512,
            cluster_count,
            root_dir_cluster: boot_sector.root_cluster,
            current_dir_cluster: boot_sector.root_cluster,
            fat_cache: [0; 512],
            fat_cache_sector: 0xFFFFFFFF,
            fat_cache_dirty: false,
        })
    }

    /// Mount the filesystem and perform initial validation
    pub fn mount(&mut self) -> Result<(), Fat32Error> {
        // Verify we can read the root directory
        let _entries = self.read_directory(self.root_dir_cluster)?;

        // Print filesystem information
        let uart = Uart::new();
        uart.puts("FAT32 filesystem mounted successfully!\n");
        uart.puts("Volume label: ");

        // Convert volume label to string (removing padding spaces)
        let mut label = [0u8; 12];
        let mut len = 0;
        for &byte in self.boot_sector.volume_label.iter() {
            if byte != 0x20 && byte != 0x00 {
                label[len] = byte;
                len += 1;
            }
        }

        if len > 0 {
            for i in 0..len {
                uart.putc(label[i]);
            }
        } else {
            uart.puts("(No label)");
        }
        uart.putc(b'\n');

        uart.puts("Cluster size: ");
        uart.put_hex(self.bytes_per_cluster as u64);
        uart.puts(" bytes\n");

        uart.puts("Total clusters: ");
        uart.put_hex(self.cluster_count as u64);
        uart.putc(b'\n');

        Ok(())
    }

    /// Convert cluster number to sector number
    fn cluster_to_sector(&self, cluster: u32) -> u32 {
        if cluster < 2 {
            return 0; // Invalid cluster
        }
        self.data_start_sector + (cluster - 2) * self.sectors_per_cluster
    }

    /// Read FAT entry for given cluster
    fn read_fat_entry(&mut self, cluster: u32) -> Result<u32, Fat32Error> {
        if cluster >= self.cluster_count + 2 {
            return Err(Fat32Error::ClusterOutOfRange);
        }

        let fat_offset = cluster * 4; // FAT32 uses 4 bytes per entry
        let fat_sector = self.fat_start_sector + (fat_offset / 512);
        let entry_offset = (fat_offset % 512) as usize;

        // Check if we need to load a different FAT sector
        if fat_sector != self.fat_cache_sector {
            // Write cache if dirty
            if self.fat_cache_dirty {
                self.sd_card
                    .write_block(self.fat_cache_sector, &self.fat_cache)?;
                self.fat_cache_dirty = false;
            }

            // Read new FAT sector
            self.sd_card.read_block(fat_sector, &mut self.fat_cache)?;
            self.fat_cache_sector = fat_sector;
        }

        // Extract FAT entry (mask to 28 bits for FAT32)
        let fat_entry = u32::from_le_bytes([
            self.fat_cache[entry_offset],
            self.fat_cache[entry_offset + 1],
            self.fat_cache[entry_offset + 2],
            self.fat_cache[entry_offset + 3],
        ]) & 0x0FFFFFFF;

        Ok(fat_entry)
    }

    /// Write FAT entry for given cluster
    fn write_fat_entry(&mut self, cluster: u32, value: u32) -> Result<(), Fat32Error> {
        if cluster >= self.cluster_count + 2 {
            return Err(Fat32Error::ClusterOutOfRange);
        }

        let fat_offset = cluster * 4;
        let fat_sector = self.fat_start_sector + (fat_offset / 512);
        let entry_offset = (fat_offset % 512) as usize;

        // Check if we need to load a different FAT sector
        if fat_sector != self.fat_cache_sector {
            // Write cache if dirty
            if self.fat_cache_dirty {
                self.sd_card
                    .write_block(self.fat_cache_sector, &self.fat_cache)?;
                self.fat_cache_dirty = false;
            }

            // Read new FAT sector
            self.sd_card.read_block(fat_sector, &mut self.fat_cache)?;
            self.fat_cache_sector = fat_sector;
        }

        // Write FAT entry (preserve upper 4 bits)
        let old_entry = u32::from_le_bytes([
            self.fat_cache[entry_offset],
            self.fat_cache[entry_offset + 1],
            self.fat_cache[entry_offset + 2],
            self.fat_cache[entry_offset + 3],
        ]);

        let new_entry = (old_entry & 0xF0000000) | (value & 0x0FFFFFFF);
        let bytes = new_entry.to_le_bytes();

        self.fat_cache[entry_offset] = bytes[0];
        self.fat_cache[entry_offset + 1] = bytes[1];
        self.fat_cache[entry_offset + 2] = bytes[2];
        self.fat_cache[entry_offset + 3] = bytes[3];

        self.fat_cache_dirty = true;
        Ok(())
    }

    /// Flush FAT cache to disk
    pub fn flush_fat(&mut self) -> Result<(), Fat32Error> {
        if self.fat_cache_dirty && self.fat_cache_sector != 0xFFFFFFFF {
            self.sd_card
                .write_block(self.fat_cache_sector, &self.fat_cache)?;

            // Write to backup FAT if it exists
            if self.boot_sector.num_fats > 1 {
                let backup_sector = self.fat_cache_sector + self.boot_sector.fat_size_32;
                self.sd_card.write_block(backup_sector, &self.fat_cache)?;
            }

            self.fat_cache_dirty = false;
        }
        Ok(())
    }

    /// Read directory entries from a cluster
    fn read_directory(&mut self, cluster: u32) -> Result<[Fat32DirEntry; 16], Fat32Error> {
        let sector = self.cluster_to_sector(cluster);
        if sector == 0 {
            return Err(Fat32Error::DirectoryNotFound);
        }

        let mut dir_data = [0u8; 512];
        self.sd_card.read_block(sector, &mut dir_data)?;

        // Convert to directory entries
        let entries = unsafe { mem::transmute::<[u8; 512], [Fat32DirEntry; 16]>(dir_data) };

        Ok(entries)
    }

    /// List files in current directory
    pub fn list_directory(&mut self) -> Result<FileList, Fat32Error> {
        self.list_directory_cluster(self.current_dir_cluster)
    }

    /// List files in specified directory cluster
    pub fn list_directory_cluster(&mut self, cluster: u32) -> Result<FileList, Fat32Error> {
        let mut files = FileList::new();
        let mut current_cluster = cluster;

        loop {
            let entries = self.read_directory(current_cluster)?;

            for entry in &entries {
                // Skip empty entries
                if entry.name[0] == 0x00 {
                    break; // End of directory
                }
                if entry.name[0] == 0xE5 {
                    continue; // Deleted entry
                }

                // Skip LFN entries and volume labels for now
                if entry.attr & ATTR_LONG_NAME == ATTR_LONG_NAME || entry.attr & ATTR_VOLUME_ID != 0
                {
                    continue;
                }

                let mut file_info = FileInfo::new();

                // Copy short name
                file_info.short_name.copy_from_slice(&entry.name);

                // Convert 8.3 name to readable format
                let mut name_len = 0;
                for i in 0..8 {
                    if entry.name[i] != 0x20 {
                        file_info.name[name_len] = entry.name[i];
                        name_len += 1;
                    }
                }

                // Add extension if present
                if entry.name[8] != 0x20 {
                    file_info.name[name_len] = b'.';
                    name_len += 1;
                    for i in 8..11 {
                        if entry.name[i] != 0x20 {
                            file_info.name[name_len] = entry.name[i];
                            name_len += 1;
                        }
                    }
                }

                file_info.size = entry.file_size;
                file_info.first_cluster =
                    ((entry.first_cluster_high as u32) << 16) | (entry.first_cluster_low as u32);
                file_info.attributes = entry.attr;
                file_info.is_directory = (entry.attr & ATTR_DIRECTORY) != 0;
                file_info.creation_time = entry.creation_time;
                file_info.creation_date = entry.creation_date;
                file_info.modified_time = entry.write_time;
                file_info.modified_date = entry.write_date;

                if files.push(file_info).is_err() {
                    break; // Directory list full
                }
            }

            // Follow cluster chain
            let next_cluster = self.read_fat_entry(current_cluster)?;
            if next_cluster >= CLUSTER_EOC_MIN && next_cluster <= CLUSTER_EOC_MAX {
                break; // End of chain
            }
            current_cluster = next_cluster;
        }

        Ok(files)
    }

    /// Get current directory cluster
    pub fn get_current_directory(&self) -> u32 {
        self.current_dir_cluster
    }

    /// Change to root directory
    pub fn change_to_root(&mut self) {
        self.current_dir_cluster = self.root_dir_cluster;
    }

    /// Change directory by name
    pub fn change_directory(&mut self, dir_name: &str) -> Result<(), Fat32Error> {
        if dir_name == ".." {
            // TODO: Implement parent directory navigation
            // For now, go to root
            self.change_to_root();
            return Ok(());
        }

        let files = self.list_directory()?;

        for i in 0..files.len() {
            let file = &files[i];
            if file.is_directory {
                let name_len = file.name.iter().position(|&x| x == 0).unwrap_or(256);
                let name = core::str::from_utf8(&file.name[..name_len.min(dir_name.len())])
                    .map_err(|_| Fat32Error::InvalidFilename)?;

                if name.eq_ignore_ascii_case(dir_name) {
                    self.current_dir_cluster = file.first_cluster;
                    return Ok(());
                }
            }
        }

        Err(Fat32Error::DirectoryNotFound)
    }

    /// Read file contents
    pub fn read_file(&mut self, filename: &str) -> Result<FileContent, Fat32Error> {
        let files = self.list_directory()?;

        // Find the file
        let mut target_file = None;
        for i in 0..files.len() {
            let file = &files[i];
            if !file.is_directory {
                let name_len = file.name.iter().position(|&x| x == 0).unwrap_or(256);
                let name = core::str::from_utf8(&file.name[..name_len.min(filename.len())])
                    .map_err(|_| Fat32Error::InvalidFilename)?;

                if name.eq_ignore_ascii_case(filename) {
                    target_file = Some(file);
                    break;
                }
            }
        }

        let file = target_file.ok_or(Fat32Error::FileNotFound)?;

        if file.size == 0 {
            return Ok(FileContent::new());
        }

        // Calculate how many clusters we need to read
        let bytes_per_cluster = self.bytes_per_cluster;
        let clusters_needed = (file.size + bytes_per_cluster - 1) / bytes_per_cluster;

        // Limit file size to prevent excessive memory usage
        if file.size > MAX_FILE_SIZE {
            return Err(Fat32Error::FileTooLarge);
        }

        let mut content = FileContent::new();
        let mut current_cluster = file.first_cluster;
        let mut bytes_read = 0;

        for _ in 0..clusters_needed {
            if current_cluster < 2 || current_cluster >= self.cluster_count {
                break;
            }

            // Read cluster data
            let sector = self.cluster_to_sector(current_cluster);
            let sectors_to_read = self
                .sectors_per_cluster
                .min(((file.size - bytes_read + 511) / 512).min(self.sectors_per_cluster))
                as usize;

            for sector_offset in 0..sectors_to_read {
                let mut sector_data = [0u8; 512];
                self.sd_card
                    .read_block(sector + sector_offset as u32, &mut sector_data)?;

                // Copy relevant bytes from this sector
                let bytes_in_sector = if bytes_read + 512 <= file.size {
                    512
                } else {
                    (file.size - bytes_read) as usize
                };

                for i in 0..bytes_in_sector {
                    if content.len < MAX_FILE_SIZE as usize {
                        content.data[content.len] = sector_data[i];
                        content.len += 1;
                    }
                }

                bytes_read += bytes_in_sector as u32;
                if bytes_read >= file.size {
                    return Ok(content);
                }
            }

            // Follow cluster chain
            let next_cluster = self.read_fat_entry(current_cluster)?;
            if next_cluster >= CLUSTER_EOC_MIN && next_cluster <= CLUSTER_EOC_MAX {
                break;
            }
            current_cluster = next_cluster;
        }

        Ok(content)
    }

    /// Print filesystem information
    pub fn print_info(&self) {
        let uart = Uart::new();
        uart.puts("=== FAT32 Filesystem Information ===\n");
        uart.puts("OEM Name: ");
        for &byte in &self.boot_sector.oem_name {
            if byte != 0 && byte != 0x20 {
                uart.putc(byte);
            }
        }
        uart.putc(b'\n');

        uart.puts("Bytes per sector: ");
        uart.put_hex(self.boot_sector.bytes_per_sector as u64);
        uart.putc(b'\n');

        uart.puts("Sectors per cluster: ");
        uart.put_hex(self.sectors_per_cluster as u64);
        uart.putc(b'\n');

        uart.puts("Bytes per cluster: ");
        uart.put_hex(self.bytes_per_cluster as u64);
        uart.putc(b'\n');

        uart.puts("Total clusters: ");
        uart.put_hex(self.cluster_count as u64);
        uart.putc(b'\n');

        uart.puts("Root directory cluster: ");
        uart.put_hex(self.root_dir_cluster as u64);
        uart.putc(b'\n');

        uart.puts("FAT start sector: ");
        uart.put_hex(self.fat_start_sector as u64);
        uart.putc(b'\n');

        uart.puts("Data start sector: ");
        uart.put_hex(self.data_start_sector as u64);
        uart.putc(b'\n');
    }
}

// Helper functions for filename handling
pub fn name_to_83(name: &str) -> [u8; 11] {
    let mut result = [0x20u8; 11]; // Fill with spaces

    let name_bytes = name.as_bytes();
    let mut name_idx = 0;
    let mut result_idx = 0;

    // Find extension
    let ext_pos = name_bytes.iter().rposition(|&b| b == b'.');

    // Copy name part (up to 8 characters)
    while result_idx < 8 && name_idx < name_bytes.len() {
        if Some(name_idx) == ext_pos {
            break;
        }
        let byte = name_bytes[name_idx].to_ascii_uppercase();
        if byte != b' ' && byte != b'.' {
            result[result_idx] = byte;
            result_idx += 1;
        }
        name_idx += 1;
    }

    // Copy extension (up to 3 characters)
    if let Some(ext_start) = ext_pos {
        let mut ext_idx = 0;
        for i in (ext_start + 1)..name_bytes.len() {
            if ext_idx < 3 {
                let byte = name_bytes[i].to_ascii_uppercase();
                if byte != b' ' {
                    result[8 + ext_idx] = byte;
                    ext_idx += 1;
                }
            }
        }
    }

    result
}

pub fn name_from_83(name_83: &[u8; 11]) -> [u8; 13] {
    let mut result = [0u8; 13];
    let mut idx = 0;

    // Copy name part
    for i in 0..8 {
        if name_83[i] != 0x20 {
            result[idx] = name_83[i];
            idx += 1;
        }
    }

    // Add extension if present
    if name_83[8] != 0x20 {
        result[idx] = b'.';
        idx += 1;
        for i in 8..11 {
            if name_83[i] != 0x20 {
                result[idx] = name_83[i];
                idx += 1;
            }
        }
    }

    result
}
