/// TinyOS Filesystem Module
///
/// This module provides filesystem support for TinyOS in a modular, no_std
/// environment. It includes support for FAT32 filesystem operations with
/// embedded-focused design.
///
/// # Architecture
///
/// The filesystem module is organized into specialized submodules:
/// - `fat32/` - FAT32 filesystem implementation
/// - `vfs` - Virtual filesystem layer (future expansion)
///
/// # Design Principles
///
/// - **no_std compliance**: All operations use stack allocation and direct
///   hardware access
/// - **Memory efficiency**: Fixed-size buffers and zero-copy operations where
///   possible
/// - **Hardware focus**: Designed for SD card and embedded storage devices
/// - **Modular structure**: Each component has clear responsibilities
/// - **Shell integration**: All operations testable via shell commands
pub mod fat32;

// Re-export main types for backward compatibility
pub use fat32::{
    Fat32Error, Fat32FileSystem, FileContent, FileInfo, FileList, ATTR_ARCHIVE, ATTR_DIRECTORY,
    ATTR_HIDDEN, ATTR_LONG_NAME, ATTR_READ_ONLY, ATTR_SYSTEM, ATTR_VOLUME_ID, CLUSTER_BAD,
    CLUSTER_EOC_MAX, CLUSTER_EOC_MIN, CLUSTER_FREE, CLUSTER_RESERVED_MIN, MAX_FILE_SIZE,
};

// For backward compatibility, expose the main filesystem interface
pub type FileSystem = Fat32FileSystem;

/// Initialize filesystem with SD card
pub fn init_filesystem(sd_card: crate::sdcard::SdCard) -> Result<Fat32FileSystem, Fat32Error> {
    Fat32FileSystem::new(sd_card)
}

/// Filename conversion utilities
pub mod filename {
    pub use super::fat32::filename::{name_from_83, name_to_83};
}
