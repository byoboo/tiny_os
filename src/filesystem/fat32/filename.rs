/// FAT32 Filename Utilities
///
/// This module provides utilities for handling FAT32 filenames including
/// 8.3 name conversion, validation, and formatting.
/// It provides no_std-compliant filename operations for embedded environments.

/// Convert a long filename to 8.3 format
pub fn name_to_83(name: &str) -> [u8; 11] {
    let mut result = [0x20u8; 11]; // Fill with spaces

    let name_bytes = name.as_bytes();
    let mut name_idx = 0;
    let mut result_idx = 0;

    // Find extension position
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

/// Convert 8.3 format to readable filename
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

/// Validate filename for FAT32 compatibility
pub fn validate_filename(name: &str) -> Result<(), FilenameError> {
    if name.is_empty() {
        return Err(FilenameError::Empty);
    }

    if name.len() > 255 {
        return Err(FilenameError::TooLong);
    }

    // Check for invalid characters
    for ch in name.chars() {
        if !is_valid_filename_char(ch) {
            return Err(FilenameError::InvalidCharacter);
        }
    }

    // Check for reserved names
    if is_reserved_name(name) {
        return Err(FilenameError::ReservedName);
    }

    Ok(())
}

/// Check if character is valid in FAT32 filename
fn is_valid_filename_char(ch: char) -> bool {
    match ch {
        // Control characters
        '\x00'..='\x1F' => false,
        // Invalid characters
        '"' | '*' | '/' | ':' | '<' | '>' | '?' | '\\' | '|' => false,
        // Valid characters
        _ => true,
    }
}

/// Check if name is reserved in FAT32 (no_std compatible)
fn is_reserved_name(name: &str) -> bool {
    let name_bytes = name.as_bytes();
    let mut upper_name = [0u8; 32]; // Buffer for uppercase name
    let mut len = 0;

    // Convert to uppercase
    for &byte in name_bytes {
        if len < upper_name.len() {
            upper_name[len] = byte.to_ascii_uppercase();
            len += 1;
        }
    }

    // Find the name part (before any dot)
    let name_part_len = upper_name[..len]
        .iter()
        .position(|&b| b == b'.')
        .unwrap_or(len);

    // Check against reserved names
    matches!(
        &upper_name[..name_part_len],
        b"CON"
            | b"PRN"
            | b"AUX"
            | b"NUL"
            | b"COM1"
            | b"COM2"
            | b"COM3"
            | b"COM4"
            | b"COM5"
            | b"COM6"
            | b"COM7"
            | b"COM8"
            | b"COM9"
            | b"LPT1"
            | b"LPT2"
            | b"LPT3"
            | b"LPT4"
            | b"LPT5"
            | b"LPT6"
            | b"LPT7"
            | b"LPT8"
            | b"LPT9"
    )
}

/// Generate 8.3 short name with numeric suffix if needed
pub fn generate_short_name(long_name: &str) -> [u8; 11] {
    let short_name = name_to_83(long_name);

    // If the name was truncated, we might need to add a numeric suffix
    // For simplicity, we'll just use the basic conversion
    // A full implementation would check for collisions and add ~1, ~2, etc.

    short_name
}

/// Calculate LFN checksum for 8.3 name
pub fn calculate_lfn_checksum(short_name: &[u8; 11]) -> u8 {
    let mut checksum = 0u8;

    for &byte in short_name {
        checksum = ((checksum & 1) << 7) + (checksum >> 1) + byte;
    }

    checksum
}

/// Check if filename needs LFN entries (longer than 8.3 format)
pub fn needs_lfn(filename: &str) -> bool {
    // Check if filename is longer than 8.3 format allows
    if filename.len() > 12 { // "filename.ext" = 12 chars max
        return true;
    }
    
    // Check for invalid 8.3 characters
    for ch in filename.chars() {
        if ch == ' ' || ch.is_lowercase() {
            return true;
        }
    }
    
    // Check name and extension lengths
    if let Some(dot_pos) = filename.find('.') {
        if dot_pos > 8 || filename.len() - dot_pos - 1 > 3 {
            return true;
        }
    } else if filename.len() > 8 {
        return true;
    }
    
    false
}

/// Create LFN entries for a long filename
pub fn create_lfn_entries(filename: &str, short_name: &[u8; 11]) -> Result<([super::directory::Fat32LfnEntry; 4], usize), super::Fat32Error> {
    let checksum = calculate_lfn_checksum(short_name);
    let mut lfn_entries = [super::directory::Fat32LfnEntry {
        ord: 0,
        name1: [0; 5],
        attr: super::ATTR_LONG_NAME,
        entry_type: 0,
        checksum,
        name2: [0; 6],
        first_cluster_low: 0,
        name3: [0; 2],
    }; 4];
    
    // Convert filename to UTF-16
    let mut utf16_name = [0u16; 255];
    let mut utf16_len = 0;
    
    for ch in filename.chars() {
        if utf16_len >= 255 {
            break;
        }
        utf16_name[utf16_len] = ch as u16;
        utf16_len += 1;
    }
    
    // Pad with 0x0000 and 0xFFFF
    if utf16_len < 255 {
        utf16_name[utf16_len] = 0x0000; // Null terminator
        utf16_len += 1;
    }
    
    // Fill remaining with 0xFFFF
    while utf16_len < 255 && utf16_len % 13 != 0 {
        utf16_name[utf16_len] = 0xFFFF;
        utf16_len += 1;
    }
    
    // Calculate number of LFN entries needed
    let num_entries = (utf16_len + 12) / 13; // 13 chars per LFN entry
    if num_entries > 4 {
        return Err(super::Fat32Error::FileTooLarge);
    }
    
    // Create LFN entries
    for i in 0..num_entries {
        let entry_idx = num_entries - 1 - i; // Entries are in reverse order
        let char_start = i * 13;
        
        lfn_entries[entry_idx].ord = (i + 1) as u8;
        if i == num_entries - 1 {
            lfn_entries[entry_idx].ord |= 0x40; // Last entry marker
        }
        
        // Copy 13 characters to LFN entry
        let mut char_idx = 0;
        
        // First 5 characters
        for j in 0..5 {
            if char_start + char_idx < utf16_len {
                lfn_entries[entry_idx].name1[j] = utf16_name[char_start + char_idx];
            } else {
                lfn_entries[entry_idx].name1[j] = 0xFFFF;
            }
            char_idx += 1;
        }
        
        // Next 6 characters
        for j in 0..6 {
            if char_start + char_idx < utf16_len {
                lfn_entries[entry_idx].name2[j] = utf16_name[char_start + char_idx];
            } else {
                lfn_entries[entry_idx].name2[j] = 0xFFFF;
            }
            char_idx += 1;
        }
        
        // Last 2 characters
        for j in 0..2 {
            if char_start + char_idx < utf16_len {
                lfn_entries[entry_idx].name3[j] = utf16_name[char_start + char_idx];
            } else {
                lfn_entries[entry_idx].name3[j] = 0xFFFF;
            }
            char_idx += 1;
        }
    }
    
    Ok((lfn_entries, num_entries))
}

/// Extract long filename from LFN entries
pub fn extract_lfn_name(lfn_entries: &[super::directory::Fat32LfnEntry], num_entries: usize) -> Result<[u8; 256], super::Fat32Error> {
    let mut name = [0u8; 256];
    let mut name_len = 0;
    
    // Process LFN entries in correct order
    for i in 0..num_entries {
        let entry = &lfn_entries[num_entries - 1 - i];
        
        // Extract characters from name1 - copy to avoid alignment issues
        let name1_copy = entry.name1;
        for ch in name1_copy {
            if ch == 0x0000 {
                break; // Null terminator
            }
            if ch != 0xFFFF && name_len < 255 {
                if ch <= 0xFF {
                    name[name_len] = ch as u8;
                    name_len += 1;
                } else {
                    // For simplicity, replace non-ASCII with '?'
                    name[name_len] = b'?';
                    name_len += 1;
                }
            }
        }
        
        // Extract characters from name2 - copy to avoid alignment issues
        let name2_copy = entry.name2;
        for ch in name2_copy {
            if ch == 0x0000 {
                break; // Null terminator
            }
            if ch != 0xFFFF && name_len < 255 {
                if ch <= 0xFF {
                    name[name_len] = ch as u8;
                    name_len += 1;
                } else {
                    name[name_len] = b'?';
                    name_len += 1;
                }
            }
        }
        
        // Extract characters from name3 - copy to avoid alignment issues
        let name3_copy = entry.name3;
        for ch in name3_copy {
            if ch == 0x0000 {
                break; // Null terminator
            }
            if ch != 0xFFFF && name_len < 255 {
                if ch <= 0xFF {
                    name[name_len] = ch as u8;
                    name_len += 1;
                } else {
                    name[name_len] = b'?';
                    name_len += 1;
                }
            }
        }
    }
    
    Ok(name)
}

/// Parse filename into name and extension parts (no_std compatible)
pub fn parse_filename(filename: &str) -> (usize, Option<usize>) {
    if let Some(dot_pos) = filename.rfind('.') {
        let ext_start = dot_pos + 1;
        if ext_start < filename.len() {
            (dot_pos, Some(ext_start))
        } else {
            (dot_pos, None)
        }
    } else {
        (filename.len(), None)
    }
}

/// Format filename for display
pub fn format_filename_for_display(name_83: &[u8; 11]) -> [u8; 13] {
    name_from_83(name_83)
}

/// Compare two filenames case-insensitively
pub fn compare_filenames(name1: &str, name2: &str) -> bool {
    if name1.len() != name2.len() {
        return false;
    }

    for (c1, c2) in name1.chars().zip(name2.chars()) {
        if c1.to_ascii_uppercase() != c2.to_ascii_uppercase() {
            return false;
        }
    }

    true
}

/// Check if filename matches pattern (basic wildcards)
pub fn matches_pattern(filename: &str, pattern: &str) -> bool {
    // Simple implementation - exact match only
    // A full implementation would support * and ? wildcards
    compare_filenames(filename, pattern)
}

/// Normalize filename for FAT32 (no_std compatible)
pub fn normalize_filename(filename: &str, output: &mut [u8]) -> Result<usize, FilenameError> {
    validate_filename(filename)?;

    let mut len = 0;

    for ch in filename.chars() {
        if is_valid_filename_char(ch) && len < output.len() {
            output[len] = ch.to_ascii_uppercase() as u8;
            len += 1;
        }
    }

    Ok(len)
}

/// Filename validation errors
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilenameError {
    Empty,
    TooLong,
    InvalidCharacter,
    ReservedName,
}

impl FilenameError {
    pub fn as_str(&self) -> &'static str {
        match self {
            FilenameError::Empty => "Filename is empty",
            FilenameError::TooLong => "Filename is too long",
            FilenameError::InvalidCharacter => "Filename contains invalid characters",
            FilenameError::ReservedName => "Filename is reserved",
        }
    }
}

/// Filename utilities for shell commands
pub mod shell {
    use super::*;

    /// Print filename validation result
    pub fn print_filename_validation(filename: &str) {
        let uart = crate::uart::Uart::new();
        uart.puts("Validating filename: ");
        uart.puts(filename);
        uart.putc(b'\n');

        match validate_filename(filename) {
            Ok(()) => {
                uart.puts("Filename is valid\n");

                // Show 8.3 conversion
                let short_name = name_to_83(filename);
                uart.puts("8.3 name: ");
                for &byte in &short_name {
                    if byte != 0x20 {
                        uart.putc(byte);
                    }
                }
                uart.putc(b'\n');
            }
            Err(error) => {
                uart.puts("Filename is invalid: ");
                uart.puts(error.as_str());
                uart.putc(b'\n');
            }
        }
    }

    /// Print filename conversion examples
    pub fn print_filename_examples() {
        let uart = crate::uart::Uart::new();
        uart.puts("=== Filename Conversion Examples ===\n");

        let examples = [
            "readme.txt",
            "LONGFILENAME.DOC",
            "file",
            "document.html",
            "test.c",
        ];

        for example in &examples {
            uart.puts("Long name: ");
            uart.puts(example);
            uart.puts(" -> 8.3: ");

            let short_name = name_to_83(example);
            for &byte in &short_name {
                if byte != 0x20 {
                    uart.putc(byte);
                }
            }
            uart.putc(b'\n');
        }
    }
}

// For no_std compatibility
impl core::fmt::Display for FilenameError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// no_std compatible string comparison
pub fn compare_names_no_std(name1: &[u8], name2: &str) -> bool {
    if name1.len() != name2.len() {
        return false;
    }

    let name2_bytes = name2.as_bytes();
    for (i, &byte1) in name1.iter().enumerate() {
        if i >= name2_bytes.len() {
            return false;
        }

        let byte2 = name2_bytes[i];
        if byte1.to_ascii_uppercase() != byte2.to_ascii_uppercase() {
            return false;
        }
    }

    true
}
