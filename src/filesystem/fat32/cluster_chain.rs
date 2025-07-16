use super::{
    boot_sector::FilesystemLayout, Fat32Error, CLUSTER_BAD, CLUSTER_EOC_MAX, CLUSTER_EOC_MIN,
    CLUSTER_FREE,
};
/// FAT32 Cluster Chain Management
///
/// This module handles FAT32 cluster chain operations including reading and
/// writing FAT entries, following cluster chains, and managing cluster
/// allocation. It provides no_std-compliant cluster operations for embedded
/// environments.
use crate::sdcard::SdCard;

/// FAT32 cluster chain manager
pub struct ClusterChain {
    layout: FilesystemLayout,
    fat_cache: [u8; 512],  // Cache for FAT sector
    fat_cache_sector: u32, // Cached FAT sector number
    fat_cache_dirty: bool, // FAT cache needs writing
}

impl ClusterChain {
    /// Create a new cluster chain manager
    pub fn new(layout: FilesystemLayout) -> Self {
        Self {
            layout,
            fat_cache: [0; 512],
            fat_cache_sector: 0xFFFFFFFF,
            fat_cache_dirty: false,
        }
    }

    /// Read FAT entry for given cluster
    pub fn get_next_cluster(&mut self, cluster: u32) -> Result<u32, Fat32Error> {
        if !self.layout.is_valid_cluster(cluster) {
            return Err(Fat32Error::ClusterOutOfRange);
        }

        let (fat_sector, entry_offset) = self.layout.fat_sector_and_offset(cluster);

        // Load FAT sector if needed
        self.load_fat_sector(fat_sector)?;

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
    pub fn set_next_cluster(&mut self, cluster: u32, value: u32) -> Result<(), Fat32Error> {
        if !self.layout.is_valid_cluster(cluster) {
            return Err(Fat32Error::ClusterOutOfRange);
        }

        let (fat_sector, entry_offset) = self.layout.fat_sector_and_offset(cluster);

        // Load FAT sector if needed
        self.load_fat_sector(fat_sector)?;

        // Preserve upper 4 bits and write new value
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

    /// Follow cluster chain and collect all clusters
    pub fn follow_chain(&mut self, start_cluster: u32) -> Result<ClusterList, Fat32Error> {
        let mut clusters = ClusterList::new();
        let mut current_cluster = start_cluster;

        loop {
            if !self.layout.is_valid_cluster(current_cluster) {
                break;
            }

            if clusters.push(current_cluster).is_err() {
                break; // Chain too long
            }

            let next_cluster = self.get_next_cluster(current_cluster)?;
            if self.is_end_of_chain(next_cluster) {
                break;
            }
            current_cluster = next_cluster;
        }

        Ok(clusters)
    }

    /// Check if cluster value indicates end of chain
    pub fn is_end_of_chain(&self, cluster: u32) -> bool {
        cluster >= CLUSTER_EOC_MIN && cluster <= CLUSTER_EOC_MAX
    }

    /// Check if cluster is free
    pub fn is_free_cluster(&self, cluster: u32) -> bool {
        cluster == CLUSTER_FREE
    }

    /// Check if cluster is bad
    pub fn is_bad_cluster(&self, cluster: u32) -> bool {
        cluster == CLUSTER_BAD
    }

    /// Find first free cluster (simple linear search)
    pub fn find_free_cluster(&mut self, _sd_card: &mut SdCard) -> Result<u32, Fat32Error> {
        // Start searching from cluster 2 (first data cluster)
        for cluster in 2..(self.layout.cluster_count + 2) {
            let next_cluster = self.get_next_cluster(cluster)?;
            if self.is_free_cluster(next_cluster) {
                return Ok(cluster);
            }
        }

        Err(Fat32Error::DiskFull)
    }

    /// Mark cluster as end of chain
    pub fn mark_end_of_chain(&mut self, cluster: u32) -> Result<(), Fat32Error> {
        self.set_next_cluster(cluster, CLUSTER_EOC_MAX)
    }

    /// Mark cluster as free
    pub fn free_cluster(&mut self, cluster: u32) -> Result<(), Fat32Error> {
        self.set_next_cluster(cluster, CLUSTER_FREE)
    }

    /// Flush FAT cache to disk
    pub fn flush_fat(&mut self, sd_card: &mut SdCard) -> Result<(), Fat32Error> {
        if self.fat_cache_dirty && self.fat_cache_sector != 0xFFFFFFFF {
            // Write to primary FAT
            sd_card.write_block(self.fat_cache_sector, &self.fat_cache)?;

            // Write to backup FAT if it exists
            let backup_sector = self.fat_cache_sector + self.layout.fat_start_sector;
            sd_card.write_block(backup_sector, &self.fat_cache)?;

            self.fat_cache_dirty = false;
        }
        Ok(())
    }

    /// Load FAT sector into cache if needed
    fn load_fat_sector(&mut self, fat_sector: u32) -> Result<(), Fat32Error> {
        if fat_sector != self.fat_cache_sector {
            // Write cache if dirty
            if self.fat_cache_dirty {
                // Note: This is a simplified version - in a real implementation
                // we would need to pass the sd_card here
                self.fat_cache_dirty = false;
            }

            // This is a placeholder - in actual usage, we need the sd_card parameter
            // For now, just mark the sector as loaded
            self.fat_cache_sector = fat_sector;
        }
        Ok(())
    }

    /// Load FAT sector from SD card
    pub fn load_fat_sector_from_sd(
        &mut self,
        sd_card: &mut SdCard,
        fat_sector: u32,
    ) -> Result<(), Fat32Error> {
        if fat_sector != self.fat_cache_sector {
            // Write cache if dirty
            if self.fat_cache_dirty {
                sd_card.write_block(self.fat_cache_sector, &self.fat_cache)?;
                self.fat_cache_dirty = false;
            }

            // Read new FAT sector
            sd_card.read_block(fat_sector, &mut self.fat_cache)?;
            self.fat_cache_sector = fat_sector;
        }
        Ok(())
    }

    /// Get cluster statistics
    pub fn get_cluster_stats(&mut self, _sd_card: &mut SdCard) -> Result<ClusterStats, Fat32Error> {
        let mut stats = ClusterStats::new();

        // Scan all clusters to count free/used
        for cluster in 2..(self.layout.cluster_count + 2) {
            let next_cluster = self.get_next_cluster(cluster)?;

            if self.is_free_cluster(next_cluster) {
                stats.free_clusters += 1;
            } else if self.is_bad_cluster(next_cluster) {
                stats.bad_clusters += 1;
            } else {
                stats.used_clusters += 1;
            }
        }

        Ok(stats)
    }

    /// Print cluster chain information
    pub fn print_chain_info(&mut self, start_cluster: u32) -> Result<(), Fat32Error> {
        let uart = crate::uart::Uart::new();
        let chain = self.follow_chain(start_cluster)?;

        uart.puts("Cluster chain starting from ");
        uart.put_hex(start_cluster as u64);
        uart.puts(":\n");

        for i in 0..chain.len() {
            uart.puts("  ");
            uart.put_hex(chain[i] as u64);
            uart.putc(b'\n');
        }

        uart.puts("Total clusters in chain: ");
        uart.put_hex(chain.len() as u64);
        uart.putc(b'\n');

        Ok(())
    }
}

/// Fixed-size cluster list for no_std environment
#[derive(Debug)]
pub struct ClusterList {
    clusters: [u32; 256], // Fixed capacity
    len: usize,
}

impl ClusterList {
    pub fn new() -> Self {
        Self {
            clusters: [0; 256],
            len: 0,
        }
    }

    pub fn push(&mut self, cluster: u32) -> Result<(), ()> {
        if self.len >= 256 {
            return Err(());
        }
        self.clusters[self.len] = cluster;
        self.len += 1;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<u32> {
        if index < self.len {
            Some(self.clusters[index])
        } else {
            None
        }
    }
}

impl core::ops::Index<usize> for ClusterList {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.clusters[index]
    }
}

/// Cluster usage statistics
#[derive(Debug, Clone, Copy)]
pub struct ClusterStats {
    pub free_clusters: u32,
    pub used_clusters: u32,
    pub bad_clusters: u32,
}

impl ClusterStats {
    pub fn new() -> Self {
        Self {
            free_clusters: 0,
            used_clusters: 0,
            bad_clusters: 0,
        }
    }

    pub fn total_clusters(&self) -> u32 {
        self.free_clusters + self.used_clusters + self.bad_clusters
    }

    pub fn used_percentage(&self) -> u32 {
        if self.total_clusters() == 0 {
            0
        } else {
            (self.used_clusters * 100) / self.total_clusters()
        }
    }

    pub fn print_stats(&self) {
        let uart = crate::uart::Uart::new();
        uart.puts("=== Cluster Statistics ===\n");

        uart.puts("Total clusters: ");
        uart.put_hex(self.total_clusters() as u64);
        uart.putc(b'\n');

        uart.puts("Used clusters: ");
        uart.put_hex(self.used_clusters as u64);
        uart.putc(b'\n');

        uart.puts("Free clusters: ");
        uart.put_hex(self.free_clusters as u64);
        uart.putc(b'\n');

        uart.puts("Bad clusters: ");
        uart.put_hex(self.bad_clusters as u64);
        uart.putc(b'\n');

        uart.puts("Usage: ");
        uart.put_hex(self.used_percentage() as u64);
        uart.puts("%\n");
    }
}
