/// FAT32 Filesystem Implementation Module
///
/// This module implements a full-featured FAT32 filesystem for TinyOS with complete
/// write support, memory safety, and no_std compliance. The implementation provides
/// a production-ready filesystem with proper directory entry management.
///
/// # Recent Improvements
///
/// - **Complete Write Support**: Full file creation, modification, and deletion
/// - **Directory Entry Management**: Safe directory entry creation and deletion
/// - **Memory Safety**: Eliminated all unsafe operations with safe byte-by-byte parsing
/// - **Cluster Chain Safety**: Cycle detection prevents infinite loops
/// - **Error Recovery**: Comprehensive error handling and filesystem validation
///
/// # Architecture
///
/// - `boot_sector` - Safe boot sector parsing with field-by-field validation
/// - `directory` - Directory entry management with create/delete operations
/// - `file_operations` - Complete file I/O with cluster chain management
/// - `cluster_chain` - Safe FAT manipulation with cycle detection
/// - `interface` - High-level filesystem API with write operations
/// - `filename` - 8.3 filename conversion and validation utilities
///
/// # Design Principles
///
/// - **Memory Safety**: All operations use safe Rust with no unsafe code
/// - **Complete Write Support**: Full file and directory operations
/// - **Fixed-size buffers**: All structures use compile-time known sizes
/// - **Direct hardware access**: No heap allocation, direct SD card I/O
/// - **Error handling**: Comprehensive error types for embedded environments
/// - **Standard Compatibility**: FAT32 format compatible with all operating systems
/// - **Performance**: Efficient cluster operations with minimal overhead
pub mod boot_sector;
pub mod cluster_chain;
pub mod directory;
pub mod file_operations;
pub mod filename;
pub mod interface;

// Re-export main types
pub use boot_sector::*;
pub use cluster_chain::*;
pub use directory::*;
pub use file_operations::*;
pub use filename::*;
pub use interface::Fat32FileSystem;

// Constants and types that are used across modules
pub const MAX_FILE_SIZE: u32 = 1024 * 1024; // 1MB max file size

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
    SdCardError(crate::sdcard::SdError),
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
    FileAlreadyExists,
    WriteProtected,
}

impl From<crate::sdcard::SdError> for Fat32Error {
    fn from(err: crate::sdcard::SdError) -> Self {
        Fat32Error::SdCardError(err)
    }
}

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

    pub(crate) fn push_byte(&mut self, byte: u8) -> Result<(), Fat32Error> {
        if self.len >= MAX_FILE_SIZE as usize {
            return Err(Fat32Error::FileTooLarge);
        }
        self.data[self.len] = byte;
        self.len += 1;
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data[..self.len]
    }

    #[allow(dead_code)]
    pub(crate) fn set_len(&mut self, len: usize) {
        self.len = len.min(MAX_FILE_SIZE as usize);
    }

    #[allow(dead_code)]
    pub(crate) fn data_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
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
