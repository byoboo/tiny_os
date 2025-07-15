//! Text Buffer Management
//!
//! Efficient text buffer implementation optimized for Raspberry Pi 4/5 performance.
//! Uses static memory allocation for no_std embedded environment.

/// Maximum number of lines in the text buffer (reduced for embedded systems)
const MAX_LINES: usize = 500;
/// Maximum line length (reduced for terminal efficiency)
const MAX_LINE_LENGTH: usize = 128;

/// Cursor direction for movement
#[derive(Debug, Clone, Copy)]
pub enum CursorDirection {
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
}

/// Simple string buffer for no_std environment
#[derive(Debug, Clone, Copy)]
pub struct LineBuffer {
    data: [u8; MAX_LINE_LENGTH],
    length: usize,
}

impl LineBuffer {
    /// Create a new empty line buffer
    pub const fn new() -> Self {
        Self {
            data: [0; MAX_LINE_LENGTH],
            length: 0,
        }
    }
    
    /// Create a line buffer from a string slice
    pub fn from_str(s: &str) -> Self {
        let mut buffer = Self::new();
        buffer.set_content(s);
        buffer
    }
    
    /// Set the content of the line buffer
    pub fn set_content(&mut self, s: &str) {
        self.length = s.len().min(MAX_LINE_LENGTH);
        self.data[..self.length].copy_from_slice(s.as_bytes());
    }
    
    /// Get the content as a string slice
    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.data[..self.length]) }
    }
    
    /// Get the length of the line
    pub fn len(&self) -> usize {
        self.length
    }
    
    /// Check if the line is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
    
    /// Insert a character at the specified position
    pub fn insert(&mut self, pos: usize, ch: char) -> bool {
        if pos > self.length || self.length >= MAX_LINE_LENGTH {
            return false;
        }
        
        // Encode character to UTF-8 bytes
        let mut ch_bytes = [0u8; 4];
        let ch_str = ch.encode_utf8(&mut ch_bytes);
        let ch_len = ch_str.len();
        
        if ch_len + self.length > MAX_LINE_LENGTH {
            return false;
        }
        
        // Shift bytes to the right
        for i in (pos..self.length).rev() {
            self.data[i + ch_len] = self.data[i];
        }
        
        // Insert the character
        self.data[pos..pos + ch_len].copy_from_slice(ch_str.as_bytes());
        self.length += ch_len;
        
        true
    }
    
    /// Remove a character at the specified position
    pub fn remove(&mut self, pos: usize) -> bool {
        if pos >= self.length {
            return false;
        }
        
        // Shift bytes to the left
        for i in pos..self.length - 1 {
            self.data[i] = self.data[i + 1];
        }
        
        self.length -= 1;
        true
    }
    
    /// Clear the line
    pub fn clear(&mut self) {
        self.length = 0;
    }
    
    /// Split the line at the specified position
    pub fn split_at(&mut self, pos: usize) -> LineBuffer {
        if pos >= self.length {
            return LineBuffer::new();
        }
        
        let mut new_line = LineBuffer::new();
        new_line.length = self.length - pos;
        new_line.data[..new_line.length].copy_from_slice(&self.data[pos..self.length]);
        
        self.length = pos;
        new_line
    }
    
    /// Append another line to this one
    pub fn append(&mut self, other: &LineBuffer) -> bool {
        if self.length + other.length > MAX_LINE_LENGTH {
            return false;
        }
        
        self.data[self.length..self.length + other.length].copy_from_slice(&other.data[..other.length]);
        self.length += other.length;
        true
    }
}

/// Text buffer for efficient text editing
pub struct TextBuffer {
    /// Lines of text
    lines: [LineBuffer; MAX_LINES],
    /// Number of lines in use
    line_count: usize,
    /// Current cursor position (row, col)
    cursor: (usize, usize),
    /// Whether the buffer has been modified
    modified: bool,
    /// Scroll position for viewport
    scroll_offset: usize,
    /// Maximum number of lines to display
    viewport_height: usize,
}

impl TextBuffer {
    /// Create a new empty text buffer
    pub fn new() -> Self {
        Self {
            lines: [const { LineBuffer::new() }; MAX_LINES],
            line_count: 1,
            cursor: (0, 0),
            modified: false,
            scroll_offset: 0,
            viewport_height: 20,
        }
    }
    
    /// Initialize the buffer
    pub fn init(&mut self) {
        self.line_count = 1;
        self.lines[0] = LineBuffer::new();
        self.cursor = (0, 0);
        self.modified = false;
        self.scroll_offset = 0;
    }
    
    /// Load content into the buffer
    pub fn load_content(&mut self, content: &str) {
        self.line_count = 0;
        
        for line in content.lines() {
            if self.line_count >= MAX_LINES {
                break;
            }
            self.lines[self.line_count] = LineBuffer::from_str(line);
            self.line_count += 1;
        }
        
        // Ensure we have at least one line
        if self.line_count == 0 {
            self.line_count = 1;
            self.lines[0] = LineBuffer::new();
        }
        
        self.cursor = (0, 0);
        self.modified = false;
        self.scroll_offset = 0;
    }
    
    /// Get the current content as a formatted string
    pub fn get_content_formatted(&self, output: &mut [u8]) -> usize {
        let mut pos = 0;
        
        for i in 0..self.line_count {
            let line = self.lines[i].as_str();
            
            // Copy line content
            for &byte in line.as_bytes() {
                if pos < output.len() {
                    output[pos] = byte;
                    pos += 1;
                }
            }
            
            // Add newline (except for last line)
            if i < self.line_count - 1 && pos < output.len() {
                output[pos] = b'\n';
                pos += 1;
            }
        }
        
        pos
    }
    
    /// Clear the buffer
    pub fn clear(&mut self) {
        self.line_count = 1;
        self.lines[0] = LineBuffer::new();
        self.cursor = (0, 0);
        self.modified = false;
        self.scroll_offset = 0;
    }
    
    /// Insert a character at the current cursor position
    pub fn insert_char(&mut self, ch: char) {
        let (row, col) = self.cursor;
        
        if ch == '\n' {
            // Split line at cursor position
            if self.line_count < MAX_LINES {
                let new_line = self.lines[row].split_at(col);
                
                // Shift lines down
                for i in (row + 1..self.line_count).rev() {
                    self.lines[i + 1] = self.lines[i].clone();
                }
                
                self.lines[row + 1] = new_line;
                self.line_count += 1;
                self.cursor = (row + 1, 0);
            }
        } else {
            // Insert character into current line
            if self.lines[row].insert(col, ch) {
                self.cursor.1 += 1;
            }
        }
        
        self.modified = true;
        self.adjust_viewport();
    }
    
    /// Delete character at cursor (backspace)
    pub fn backspace(&mut self) {
        let (row, col) = self.cursor;
        
        if col > 0 {
            // Delete character in current line
            self.lines[row].remove(col - 1);
            self.cursor.1 -= 1;
            self.modified = true;
        } else if row > 0 {
            // Merge with previous line
            let new_col = self.lines[row - 1].len();
            let current_line = self.lines[row].clone();
            if self.lines[row - 1].append(&current_line) {
                // Shift lines up
                for i in row..self.line_count - 1 {
                    self.lines[i] = self.lines[i + 1].clone();
                }
                self.line_count -= 1;
                self.cursor = (row - 1, new_col);
                self.modified = true;
            }
        }
        
        self.adjust_viewport();
    }
    
    /// Delete character at cursor (delete key)
    pub fn delete(&mut self) {
        let (row, col) = self.cursor;
        
        if col < self.lines[row].len() {
            // Delete character in current line
            self.lines[row].remove(col);
            self.modified = true;
        } else if row < self.line_count - 1 {
            // Merge with next line
            let next_line = self.lines[row + 1].clone();
            if self.lines[row].append(&next_line) {
                // Shift lines up
                for i in row + 1..self.line_count - 1 {
                    self.lines[i] = self.lines[i + 1].clone();
                }
                self.line_count -= 1;
                self.modified = true;
            }
        }
    }
    
    /// Move cursor in the specified direction
    pub fn move_cursor(&mut self, direction: CursorDirection) {
        let (row, col) = self.cursor;
        
        match direction {
            CursorDirection::Up => {
                if row > 0 {
                    let new_row = row - 1;
                    let new_col = col.min(self.lines[new_row].len());
                    self.cursor = (new_row, new_col);
                }
            }
            CursorDirection::Down => {
                if row < self.line_count - 1 {
                    let new_row = row + 1;
                    let new_col = col.min(self.lines[new_row].len());
                    self.cursor = (new_row, new_col);
                }
            }
            CursorDirection::Left => {
                if col > 0 {
                    self.cursor.1 -= 1;
                } else if row > 0 {
                    self.cursor = (row - 1, self.lines[row - 1].len());
                }
            }
            CursorDirection::Right => {
                if col < self.lines[row].len() {
                    self.cursor.1 += 1;
                } else if row < self.line_count - 1 {
                    self.cursor = (row + 1, 0);
                }
            }
            CursorDirection::Home => {
                self.cursor.1 = 0;
            }
            CursorDirection::End => {
                self.cursor.1 = self.lines[row].len();
            }
            CursorDirection::PageUp => {
                let new_row = row.saturating_sub(self.viewport_height);
                let new_col = col.min(self.lines[new_row].len());
                self.cursor = (new_row, new_col);
            }
            CursorDirection::PageDown => {
                let new_row = (row + self.viewport_height).min(self.line_count - 1);
                let new_col = col.min(self.lines[new_row].len());
                self.cursor = (new_row, new_col);
            }
        }
        
        self.adjust_viewport();
    }
    
    /// Adjust viewport to ensure cursor is visible
    fn adjust_viewport(&mut self) {
        let cursor_row = self.cursor.0;
        
        // Adjust scroll offset to keep cursor visible
        if cursor_row < self.scroll_offset {
            self.scroll_offset = cursor_row;
        } else if cursor_row >= self.scroll_offset + self.viewport_height {
            self.scroll_offset = cursor_row - self.viewport_height + 1;
        }
    }
    
    /// Get the current cursor position
    pub fn get_cursor(&self) -> (usize, usize) {
        self.cursor
    }
    
    /// Get a specific line
    pub fn get_line(&self, index: usize) -> Option<&str> {
        if index < self.line_count {
            Some(self.lines[index].as_str())
        } else {
            None
        }
    }
    
    /// Get the total number of lines
    pub fn line_count(&self) -> usize {
        self.line_count
    }
    
    /// Check if the buffer has been modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }
    
    /// Mark the buffer as saved
    pub fn mark_saved(&mut self) {
        self.modified = false;
    }
    
    /// Get the scroll offset
    pub fn get_scroll_offset(&self) -> usize {
        self.scroll_offset
    }
    
    /// Set the viewport height
    pub fn set_viewport_height(&mut self, height: usize) {
        self.viewport_height = height;
        self.adjust_viewport();
    }
    
    /// Get visible lines for the current viewport
    pub fn get_visible_lines(&self) -> &[LineBuffer] {
        let start = self.scroll_offset;
        let end = (start + self.viewport_height).min(self.line_count);
        &self.lines[start..end]
    }
    
    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.line_count == 1 && self.lines[0].is_empty()
    }
}