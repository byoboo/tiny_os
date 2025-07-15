//! Terminal UI Management
//!
//! Efficient terminal-based user interface for the text editor,
//! optimized for Raspberry Pi 4/5 performance.

use crate::shell::ShellContext;
use crate::apps::editor::buffer::TextBuffer;

/// Terminal UI controller for the text editor
pub struct EditorUI {
    /// Terminal width
    width: usize,
    /// Terminal height
    height: usize,
    /// Status line enabled
    show_status: bool,
    /// Current message to display
    message: Option<&'static str>,
    /// Message timeout (simple counter)
    message_timeout: u32,
}

impl EditorUI {
    /// Create a new editor UI
    pub fn new() -> Self {
        Self {
            width: 80,
            height: 24,
            show_status: true,
            message: None,
            message_timeout: 0,
        }
    }
    
    /// Initialize the UI
    pub fn init(&mut self, context: &mut ShellContext) -> Result<(), &'static str> {
        // Clear screen
        self.clear_screen(context);
        
        // Set up terminal (basic setup)
        self.setup_terminal(context);
        
        Ok(())
    }
    
    /// Clean up the UI
    pub fn cleanup(&mut self, context: &mut ShellContext) {
        // Clear screen and reset
        self.clear_screen(context);
        context.uart.puts("Text editor closed.\n");
    }
    
    /// Clear the screen
    fn clear_screen(&self, context: &mut ShellContext) {
        // Simple clear screen using ANSI escape codes
        context.uart.puts("\x1b[2J\x1b[H");
    }
    
    /// Set up terminal for editor mode
    fn setup_terminal(&self, context: &mut ShellContext) {
        // Move cursor to top-left
        context.uart.puts("\x1b[H");
        
        // Hide cursor initially
        context.uart.puts("\x1b[?25l");
    }
    
    /// Draw the complete editor interface
    pub fn draw_editor(&mut self, buffer: &TextBuffer, context: &mut ShellContext) {
        // Clear screen
        self.clear_screen(context);
        
        // Draw header
        self.draw_header(buffer, context);
        
        // Draw content area
        self.draw_content(buffer, context);
        
        // Draw status line
        if self.show_status {
            self.draw_status_line(buffer, context);
        }
        
        // Draw message if present
        if let Some(msg) = self.message {
            if self.message_timeout > 0 {
                self.draw_message(msg, context);
                self.message_timeout -= 1;
            } else {
                self.message = None;
            }
        }
        
        // Position cursor
        self.position_cursor(buffer, context);
        
        // Show cursor
        context.uart.puts("\x1b[?25h");
    }
    
    /// Draw the editor header
    fn draw_header(&self, buffer: &TextBuffer, context: &mut ShellContext) {
        context.uart.puts("\x1b[1;1H"); // Move to top-left
        context.uart.puts("TinyOS Text Editor");
        
        if buffer.is_modified() {
            context.uart.puts(" [Modified]");
        }
        
        context.uart.puts("\n");
        
        // Draw separator line
        for _ in 0..self.width {
            context.uart.puts("-");
        }
        context.uart.puts("\n");
    }
    
    /// Draw the content area
    fn draw_content(&self, buffer: &TextBuffer, context: &mut ShellContext) {
        let visible_lines = buffer.get_visible_lines();
        let scroll_offset = buffer.get_scroll_offset();
        
        // Calculate available height (minus header, status, etc.)
        let content_height = self.height.saturating_sub(4); // Header + separator + status
        
        for (display_row, line) in visible_lines.iter().enumerate() {
            if display_row >= content_height {
                break;
            }
            
            // Move to correct position
            let screen_row = display_row + 3; // After header and separator
            context.uart.puts(&format!("\x1b[{};1H", screen_row));
            
            // Draw line number
            let line_num = scroll_offset + display_row + 1;
            context.uart.puts(&format!("{:4} ", line_num));
            
            // Draw line content (truncate if too long)
            let display_content = if line.len() > self.width - 6 {
                &line[..self.width - 6]
            } else {
                line
            };
            
            context.uart.puts(display_content);
            
            // Clear to end of line
            context.uart.puts("\x1b[K");
        }
        
        // Clear remaining lines in content area
        for display_row in visible_lines.len()..content_height {
            let screen_row = display_row + 3;
            context.uart.puts(&format!("\x1b[{};1H", screen_row));
            context.uart.puts("~");
            context.uart.puts("\x1b[K");
        }
    }
    
    /// Draw the status line
    fn draw_status_line(&self, buffer: &TextBuffer, context: &mut ShellContext) {
        let status_row = self.height - 1;
        context.uart.puts(&format!("\x1b[{};1H", status_row));
        
        // Draw separator
        for _ in 0..self.width {
            context.uart.puts("-");
        }
        
        // Move to status line
        context.uart.puts(&format!("\x1b[{};1H", self.height));
        
        // Show cursor position
        let (row, col) = buffer.get_cursor();
        context.uart.puts(&format!("Line {}, Col {} | ", row + 1, col + 1));
        
        // Show total lines
        context.uart.puts(&format!("{} lines", buffer.line_count()));
        
        // Show modification status
        if buffer.is_modified() {
            context.uart.puts(" | Modified");
        }
        
        // Show help
        context.uart.puts(" | Ctrl+S: Save, Ctrl+Q: Quit");
        
        // Clear to end of line
        context.uart.puts("\x1b[K");
    }
    
    /// Position the cursor at the correct location
    fn position_cursor(&self, buffer: &TextBuffer, context: &mut ShellContext) {
        let (buffer_row, buffer_col) = buffer.get_cursor();
        let scroll_offset = buffer.get_scroll_offset();
        
        // Calculate screen position
        let screen_row = buffer_row.saturating_sub(scroll_offset) + 3; // After header
        let screen_col = buffer_col + 6; // After line number
        
        // Ensure cursor is within bounds
        if screen_row < self.height - 2 { // Before status area
            context.uart.puts(&format!("\x1b[{};{}H", screen_row, screen_col));
        }
    }
    
    /// Show a message to the user
    pub fn show_message(&mut self, message: &'static str, context: &mut ShellContext) {
        self.message = Some(message);
        self.message_timeout = 100; // Display for ~100 refresh cycles
        
        // Immediately draw the message
        self.draw_message(message, context);
    }
    
    /// Draw a message at the bottom of the screen
    fn draw_message(&self, message: &str, context: &mut ShellContext) {
        let message_row = self.height - 2;
        context.uart.puts(&format!("\x1b[{};1H", message_row));
        context.uart.puts("MSG: ");
        context.uart.puts(message);
        context.uart.puts("\x1b[K");
    }
    
    /// Update terminal dimensions
    pub fn set_dimensions(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }
    
    /// Toggle status line visibility
    pub fn toggle_status(&mut self) {
        self.show_status = !self.show_status;
    }
}

/// Format a string for safe terminal output
fn format_for_terminal(s: &str, max_len: usize) -> String {
    let mut result = String::with_capacity(max_len);
    
    for ch in s.chars().take(max_len) {
        if ch.is_control() {
            result.push('?'); // Replace control characters
        } else {
            result.push(ch);
        }
    }
    
    result
}

/// ANSI escape codes for terminal control
#[allow(dead_code)]
pub mod ansi {
    pub const CLEAR_SCREEN: &str = "\x1b[2J";
    pub const CLEAR_LINE: &str = "\x1b[K";
    pub const CURSOR_HOME: &str = "\x1b[H";
    pub const CURSOR_HIDE: &str = "\x1b[?25l";
    pub const CURSOR_SHOW: &str = "\x1b[?25h";
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const REVERSE: &str = "\x1b[7m";
}

/// Helper function to create cursor position escape sequence
pub fn cursor_position(row: usize, col: usize) -> String {
    format!("\x1b[{};{}H", row, col)
}