//! File Operations
//!
//! Efficient file I/O operations for the text editor, leveraging TinyOS
//! filesystem with no_std compatibility.

use crate::filesystem::fat32::interface::Fat32FileSystem;

/// Maximum file content size (64KB - optimized for embedded systems)
const MAX_FILE_SIZE: usize = 64 * 1024;
/// Maximum filename length
const MAX_FILENAME_LEN: usize = 255;

/// File operations handler
pub struct FileOperations {
    /// Current working directory path
    current_dir: [u8; 256],
    /// Current directory length
    current_dir_len: usize,
    /// File system interface
    fs: Option<Fat32FileSystem>,
    /// Buffer for file content
    file_buffer: [u8; MAX_FILE_SIZE],
}

impl FileOperations {
    /// Create a new file operations handler
    pub fn new() -> Self {
        let mut ops = Self {
            current_dir: [0; 256],
            current_dir_len: 1,
            fs: None,
            file_buffer: [0; MAX_FILE_SIZE],
        };

        // Initialize with root directory
        ops.current_dir[0] = b'/';
        ops
    }

    /// Initialize file system access
    pub fn init(&mut self) -> Result<(), &'static str> {
        // Try to initialize the FAT32 file system
        // Note: Fat32FileSystem::new() requires an SdCard parameter
        // For now, we'll use mock operations until SD card integration is ready
        self.fs = None;
        Ok(())
    }

    /// Read a file and return its content in the provided buffer
    pub fn read_file(&mut self, filename: &str, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if filename.len() > MAX_FILENAME_LEN {
            return Err("Filename too long");
        }

        if let Some(ref mut fs) = self.fs {
            // Try to read from actual filesystem
            match fs.read_file(filename) {
                Ok(data) => {
                    if data.len() > buffer.len() {
                        return Err("File too large for buffer");
                    }
                    buffer[..data.len()].copy_from_slice(data.as_slice());
                    Ok(data.len())
                }
                Err(_) => Err("Failed to read file"),
            }
        } else {
            // Mock file operations for testing
            self.mock_read_file(filename, buffer)
        }
    }

    /// Write content to a file (not supported in read-only FAT32)
    pub fn write_file(&mut self, filename: &str, content: &[u8]) -> Result<(), &'static str> {
        if filename.len() > MAX_FILENAME_LEN {
            return Err("Filename too long");
        }

        if content.len() > MAX_FILE_SIZE {
            return Err("File too large");
        }

        if let Some(ref mut _fs) = self.fs {
            // FAT32 implementation doesn't support file writing yet
            Err("File writing not supported")
        } else {
            // Mock file operations for testing
            self.mock_write_file(filename, content)
        }
    }

    /// Create a new file (not supported in read-only FAT32)
    pub fn create_file(&mut self, filename: &str) -> Result<(), &'static str> {
        if filename.len() > MAX_FILENAME_LEN {
            return Err("Filename too long");
        }

        if let Some(ref mut _fs) = self.fs {
            // FAT32 implementation doesn't support file creation yet
            Err("File creation not supported")
        } else {
            // Mock operation
            Ok(())
        }
    }

    /// Delete a file (not supported in read-only FAT32)
    pub fn delete_file(&mut self, filename: &str) -> Result<(), &'static str> {
        if filename.len() > MAX_FILENAME_LEN {
            return Err("Filename too long");
        }

        if let Some(ref mut _fs) = self.fs {
            // FAT32 implementation doesn't support file deletion yet
            Err("File deletion not supported")
        } else {
            // Mock operation
            Ok(())
        }
    }

    /// Check if a file exists
    pub fn file_exists(&mut self, filename: &str) -> bool {
        if filename.len() > MAX_FILENAME_LEN {
            return false;
        }

        if let Some(ref mut fs) = self.fs {
            // Try to find the file
            match fs.find_file(filename) {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            // Mock: assume common files exist
            matches!(filename, "test.txt" | "readme.txt" | "hello.txt")
        }
    }

    /// List files in current directory
    pub fn list_files(
        &mut self,
        file_list: &mut [FileInfo],
        max_files: usize,
    ) -> Result<usize, &'static str> {
        if let Some(ref mut fs) = self.fs {
            let _current_dir_str = unsafe {
                core::str::from_utf8_unchecked(&self.current_dir[..self.current_dir_len])
            };

            match fs.list_directory() {
                Ok(entries) => {
                    let mut count = 0;
                    for i in 0..entries.len().min(max_files) {
                        if count >= file_list.len() {
                            break;
                        }

                        if let Some(entry) = entries.get(i) {
                            // Convert name from [u8; 256] to &str
                            let name_len = entry
                                .name
                                .iter()
                                .position(|&x| x == 0)
                                .unwrap_or(entry.name.len());
                            let name_str =
                                unsafe { core::str::from_utf8_unchecked(&entry.name[..name_len]) };
                            file_list[count] = FileInfo::from_name(name_str);
                            count += 1;
                        }
                    }
                    Ok(count)
                }
                Err(_) => Err("Failed to list directory"),
            }
        } else {
            // Mock file listing
            if max_files > 0 && !file_list.is_empty() {
                file_list[0] = FileInfo::from_name("test.txt");
                if max_files > 1 && file_list.len() > 1 {
                    file_list[1] = FileInfo::from_name("readme.txt");
                }
                if max_files > 2 && file_list.len() > 2 {
                    file_list[2] = FileInfo::from_name("hello.txt");
                }
                Ok(3.min(max_files).min(file_list.len()))
            } else {
                Ok(0)
            }
        }
    }

    /// Get file size
    pub fn get_file_size(&mut self, filename: &str) -> Result<usize, &'static str> {
        if filename.len() > MAX_FILENAME_LEN {
            return Err("Filename too long");
        }

        if let Some(ref mut fs) = self.fs {
            // Note: This method may not exist in Fat32FileSystem
            // For now, try to read the file to get its size
            match fs.read_file(filename) {
                Ok(data) => Ok(data.len()),
                Err(_) => Err("Failed to get file size"),
            }
        } else {
            // Mock file size
            Ok(filename.len() * 10) // Simple mock based on filename length
        }
    }

    /// Change current directory
    pub fn change_directory(&mut self, dir: &str) -> Result<(), &'static str> {
        if dir.len() > self.current_dir.len() - 1 {
            return Err("Directory path too long");
        }

        if let Some(ref mut _fs) = self.fs {
            // Note: This method may not exist in Fat32FileSystem
            // For now, just update the current directory
            self.current_dir_len = dir.len();
            self.current_dir[..dir.len()].copy_from_slice(dir.as_bytes());
            Ok(())
        } else {
            // Mock: always succeed
            self.current_dir_len = dir.len();
            self.current_dir[..dir.len()].copy_from_slice(dir.as_bytes());
            Ok(())
        }
    }

    /// Get current directory
    pub fn get_current_directory(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.current_dir[..self.current_dir_len]) }
    }

    /// Mock file reading for testing without filesystem
    fn mock_read_file(&self, filename: &str, buffer: &mut [u8]) -> Result<usize, &'static str> {
        let content = match filename {
            "test.txt" => "Hello, TinyOS!\nThis is a test file.\nEditing with TinyOS Text Editor.",
            "readme.txt" => "TinyOS Text Editor\n=================\n\nA lightweight text editor for TinyOS.\n\nFeatures:\n- Basic text editing\n- File operations\n- Terminal interface\n- Optimized for Pi 4/5",
            "hello.txt" => "Hello, World!\n\nThis is a simple text file.\nYou can edit it with the TinyOS text editor.",
            _ => return Err("File not found")
        };

        let content_bytes = content.as_bytes();
        if content_bytes.len() > buffer.len() {
            return Err("File too large for buffer");
        }

        buffer[..content_bytes.len()].copy_from_slice(content_bytes);
        Ok(content_bytes.len())
    }

    /// Mock file writing for testing without filesystem
    fn mock_write_file(&self, filename: &str, content: &[u8]) -> Result<(), &'static str> {
        // In a real implementation, this would write to storage
        // For now, we'll just simulate success
        if filename.is_empty() {
            Err("Invalid filename")
        } else if content.len() > MAX_FILE_SIZE {
            Err("File too large")
        } else {
            Ok(())
        }
    }

    // Backup/restore functionality removed for streamlined implementation
    // Can be re-added if needed for production use

    /// Check if filesystem is available
    pub fn is_filesystem_available(&self) -> bool {
        self.fs.is_some()
    }

    /// Get file extension
    pub fn get_file_extension<'a>(&self, filename: &'a str) -> Option<&'a str> {
        filename.rfind('.').map(|i| &filename[i + 1..])
    }

    /// Validate filename
    pub fn is_valid_filename(&self, filename: &str) -> bool {
        !filename.is_empty()
            && !filename.contains('/')
            && !filename.contains('\\')
            && !filename.contains('\0')
            && filename.len() <= MAX_FILENAME_LEN
    }

    /// Get safe filename (remove invalid characters)
    pub fn sanitize_filename(&self, filename: &str, output: &mut [u8]) -> usize {
        let mut pos = 0;
        for ch in filename.chars() {
            if pos >= output.len() {
                break;
            }

            if ch.is_alphanumeric() || ch == '.' || ch == '_' || ch == '-' {
                let mut char_buf = [0u8; 4];
                let char_str = ch.encode_utf8(&mut char_buf);
                let char_bytes = char_str.as_bytes();

                if pos + char_bytes.len() <= output.len() {
                    output[pos..pos + char_bytes.len()].copy_from_slice(char_bytes);
                    pos += char_bytes.len();
                }
            }
        }
        pos
    }
}

/// File metadata information
#[derive(Debug, Clone, Copy)]
pub struct FileInfo {
    pub name: [u8; 64],
    pub name_len: usize,
    pub size: usize,
    pub is_directory: bool,
    pub modified_time: u64, // Timestamp
}

impl FileInfo {
    pub fn new() -> Self {
        Self {
            name: [0; 64],
            name_len: 0,
            size: 0,
            is_directory: false,
            modified_time: 0,
        }
    }

    pub fn from_name(name: &str) -> Self {
        let mut info = Self::new();
        info.name_len = name.len().min(info.name.len());
        info.name[..info.name_len].copy_from_slice(name.as_bytes());
        info.size = name.len() * 10; // Mock size
        info
    }

    pub fn get_name(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.name[..self.name_len]) }
    }
}

/// File operation result type
pub type FileResult<T> = Result<T, FileError>;

/// File operation errors
#[derive(Debug, Clone, Copy)]
pub enum FileError {
    NotFound,
    PermissionDenied,
    InvalidName,
    DiskFull,
    IoError,
    InvalidFormat,
}

impl FileError {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileError::NotFound => "File not found",
            FileError::PermissionDenied => "Permission denied",
            FileError::InvalidName => "Invalid filename",
            FileError::DiskFull => "Disk full",
            FileError::IoError => "I/O error",
            FileError::InvalidFormat => "Invalid file format",
        }
    }
}
