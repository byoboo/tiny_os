/// FAT32 Boot Sector Handling
///
/// This module handles parsing and validation of FAT32 boot sectors.
/// It provides no_std-compliant boot sector operations for embedded environments.

use crate::sdcard::SdCard;
use super::Fat32Error;

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

impl Fat32BootSector {
    /// Read and parse boot sector from SD card
    pub fn read_from_sd(sd_card: &mut SdCard) -> Result<Self, Fat32Error> {
        // Read boot sector (sector 0)
        let mut boot_sector_data = [0u8; 512];
        sd_card.read_block(0, &mut boot_sector_data)?;

        // Parse boot sector
        let boot_sector = unsafe { 
            core::mem::transmute::<[u8; 512], Fat32BootSector>(boot_sector_data) 
        };

        // Validate boot sector
        boot_sector.validate()?;

        Ok(boot_sector)
    }

    /// Validate boot sector structure and content
    pub fn validate(&self) -> Result<(), Fat32Error> {
        // Check boot sector signature
        if self.signature != 0xAA55 {
            return Err(Fat32Error::InvalidSignature);
        }

        // Check sector size (must be 512 bytes)
        if self.bytes_per_sector != 512 {
            return Err(Fat32Error::UnsupportedSectorSize);
        }

        // Check cluster size (must be power of 2)
        if self.sectors_per_cluster == 0
            || (self.sectors_per_cluster & (self.sectors_per_cluster - 1)) != 0
        {
            return Err(Fat32Error::UnsupportedClusterSize);
        }

        // Validate FAT32 specific fields
        if self.fat_size_32 == 0 || self.root_cluster < 2 {
            return Err(Fat32Error::InvalidBootSector);
        }

        Ok(())
    }

    /// Calculate filesystem layout parameters
    pub fn calculate_layout(&self) -> Result<FilesystemLayout, Fat32Error> {
        let fat_start_sector = self.reserved_sector_count as u32;
        let fat_size = self.fat_size_32;
        let data_start_sector = fat_start_sector + (self.num_fats as u32 * fat_size);

        let total_sectors = if self.total_sectors_16 != 0 {
            self.total_sectors_16 as u32
        } else {
            self.total_sectors_32
        };

        let data_sectors = total_sectors - data_start_sector;
        let cluster_count = data_sectors / self.sectors_per_cluster as u32;

        // Verify this is actually FAT32 (cluster count must be >= 65525)
        if cluster_count < 65525 {
            return Err(Fat32Error::InvalidBootSector);
        }

        Ok(FilesystemLayout {
            fat_start_sector,
            data_start_sector,
            sectors_per_cluster: self.sectors_per_cluster as u32,
            bytes_per_cluster: self.sectors_per_cluster as u32 * 512,
            cluster_count,
            root_dir_cluster: self.root_cluster,
        })
    }

    /// Get volume label as a string (removing padding spaces)
    pub fn get_volume_label(&self) -> [u8; 12] {
        let mut label = [0u8; 12];
        let mut len = 0;
        
        for &byte in self.volume_label.iter() {
            if byte != 0x20 && byte != 0x00 && len < 11 {
                label[len] = byte;
                len += 1;
            }
        }
        
        label
    }

    /// Print boot sector information via UART
    pub fn print_info(&self) {
        let uart = crate::uart::Uart::new();
        uart.puts("=== FAT32 Boot Sector Information ===\n");
        
        uart.puts("OEM Name: ");
        for &byte in &self.oem_name {
            if byte != 0 && byte != 0x20 {
                uart.putc(byte);
            }
        }
        uart.putc(b'\n');

        uart.puts("Bytes per sector: ");
        uart.put_hex(self.bytes_per_sector as u64);
        uart.putc(b'\n');

        uart.puts("Sectors per cluster: ");
        uart.put_hex(self.sectors_per_cluster as u64);
        uart.putc(b'\n');

        uart.puts("Reserved sectors: ");
        uart.put_hex(self.reserved_sector_count as u64);
        uart.putc(b'\n');

        uart.puts("Number of FATs: ");
        uart.put_hex(self.num_fats as u64);
        uart.putc(b'\n');

        uart.puts("FAT size (sectors): ");
        uart.put_hex(self.fat_size_32 as u64);
        uart.putc(b'\n');

        uart.puts("Root directory cluster: ");
        uart.put_hex(self.root_cluster as u64);
        uart.putc(b'\n');

        uart.puts("Total sectors: ");
        if self.total_sectors_16 != 0 {
            uart.put_hex(self.total_sectors_16 as u64);
        } else {
            uart.put_hex(self.total_sectors_32 as u64);
        }
        uart.putc(b'\n');

        uart.puts("Volume label: ");
        let label = self.get_volume_label();
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
    }
}

/// Filesystem layout parameters calculated from boot sector
#[derive(Debug, Clone, Copy)]
pub struct FilesystemLayout {
    pub fat_start_sector: u32,
    pub data_start_sector: u32,
    pub sectors_per_cluster: u32,
    pub bytes_per_cluster: u32,
    pub cluster_count: u32,
    pub root_dir_cluster: u32,
}

impl FilesystemLayout {
    /// Convert cluster number to sector number
    pub fn cluster_to_sector(&self, cluster: u32) -> u32 {
        if cluster < 2 {
            return 0; // Invalid cluster
        }
        self.data_start_sector + (cluster - 2) * self.sectors_per_cluster
    }

    /// Calculate FAT sector and offset for a given cluster
    pub fn fat_sector_and_offset(&self, cluster: u32) -> (u32, usize) {
        let fat_offset = cluster * 4; // FAT32 uses 4 bytes per entry
        let fat_sector = self.fat_start_sector + (fat_offset / 512);
        let entry_offset = (fat_offset % 512) as usize;
        (fat_sector, entry_offset)
    }

    /// Validate cluster number
    pub fn is_valid_cluster(&self, cluster: u32) -> bool {
        cluster >= 2 && cluster < self.cluster_count + 2
    }

    /// Print layout information
    pub fn print_info(&self) {
        let uart = crate::uart::Uart::new();
        uart.puts("=== Filesystem Layout ===\n");
        
        uart.puts("FAT start sector: ");
        uart.put_hex(self.fat_start_sector as u64);
        uart.putc(b'\n');

        uart.puts("Data start sector: ");
        uart.put_hex(self.data_start_sector as u64);
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
    }
}
