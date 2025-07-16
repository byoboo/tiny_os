//! Terminal UI Management
//!
//! Efficient terminal-based user interface for the text editor,
//! optimized for Raspberry Pi 4/5 performance with no_std compatibility.

use crate::{
    apps::editor::buffer::TextBuffer, shell::ShellContext,
    utils::formatting::write_number_to_buffer,
};

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
    /// Buffer for formatting numbers
    format_buffer: [u8; 64],
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
            format_buffer: [0; 64],
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

    /// Draw the complete editor interface (initial render)
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

        // Position cursor
        self.position_cursor(buffer, context);

        // Show cursor
        context.uart.puts("\x1b[?25h");
    }

    /// Update only the cursor position (efficient for simple moves)
    pub fn update_cursor_only(&mut self, buffer: &TextBuffer, context: &mut ShellContext) {
        self.position_cursor(buffer, context);
    }

    /// Update status line only (efficient for status changes)
    pub fn update_status_only(&mut self, buffer: &TextBuffer, context: &mut ShellContext) {
        if self.show_status {
            self.draw_status_line(buffer, context);
        }
    }

    /// Update content area only (efficient for text changes)
    pub fn update_content_only(&mut self, buffer: &TextBuffer, context: &mut ShellContext) {
        self.draw_content(buffer, context);
        self.position_cursor(buffer, context);
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
    fn draw_content(&mut self, buffer: &TextBuffer, context: &mut ShellContext) {
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
            self.move_cursor_to(screen_row, 1, context);

            // Draw line number
            let line_num = scroll_offset + display_row + 1;
            self.write_line_number(line_num, context);
            context.uart.puts(" ");

            // Draw line content (truncate if too long)
            let line_content = line.as_str();
            let display_content = if line_content.len() > self.width - 6 {
                &line_content[..self.width - 6]
            } else {
                line_content
            };

            context.uart.puts(display_content);

            // Clear to end of line
            context.uart.puts("\x1b[K");
        }

        // Clear remaining lines in content area
        for display_row in visible_lines.len()..content_height {
            let screen_row = display_row + 3;
            self.move_cursor_to(screen_row, 1, context);
            context.uart.puts("~");
            context.uart.puts("\x1b[K");
        }
    }

    /// Draw the status line
    fn draw_status_line(&mut self, buffer: &TextBuffer, context: &mut ShellContext) {
        let status_row = self.height - 1;
        self.move_cursor_to(status_row, 1, context);

        // Draw separator
        for _ in 0..self.width {
            context.uart.puts("-");
        }

        // Move to status line
        self.move_cursor_to(self.height, 1, context);

        // Show cursor position
        let (row, col) = buffer.get_cursor();
        context.uart.puts("Line ");
        self.write_number(row + 1, context);
        context.uart.puts(", Col ");
        self.write_number(col + 1, context);
        context.uart.puts(" | ");

        // Show total lines
        self.write_number(buffer.line_count(), context);
        context.uart.puts(" lines");

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
    fn position_cursor(&mut self, buffer: &TextBuffer, context: &mut ShellContext) {
        let (buffer_row, buffer_col) = buffer.get_cursor();
        let scroll_offset = buffer.get_scroll_offset();

        // Calculate screen position
        let screen_row = buffer_row.saturating_sub(scroll_offset) + 3; // After header
        let screen_col = buffer_col + 6; // After line number

        // Ensure cursor is within bounds
        if screen_row < self.height - 2 {
            // Before status area
            self.move_cursor_to(screen_row, screen_col, context);
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
    fn draw_message(&mut self, message: &str, context: &mut ShellContext) {
        let message_row = self.height - 2;
        self.move_cursor_to(message_row, 1, context);
        context.uart.puts("MSG: ");
        context.uart.puts(message);
        context.uart.puts("\x1b[K");
    }

    /// Move cursor to specific position
    fn move_cursor_to(&mut self, row: usize, col: usize, context: &mut ShellContext) {
        context.uart.puts("\x1b[");
        self.write_number(row, context);
        context.uart.puts(";");
        self.write_number(col, context);
        context.uart.puts("H");
    }

    /// Write a number to the terminal
    fn write_number(&mut self, num: usize, context: &mut ShellContext) {
        let len = write_number_to_buffer(num as u64, &mut self.format_buffer);
        let num_str = unsafe { core::str::from_utf8_unchecked(&self.format_buffer[..len]) };
        context.uart.puts(num_str);
    }

    /// Write a line number with padding
    fn write_line_number(&mut self, num: usize, context: &mut ShellContext) {
        let len = write_number_to_buffer(num as u64, &mut self.format_buffer);
        let num_str = unsafe { core::str::from_utf8_unchecked(&self.format_buffer[..len]) };

        // Right-align the number in a 4-character field
        let padding = 4_usize.saturating_sub(len);
        for _ in 0..padding {
            context.uart.puts(" ");
        }
        context.uart.puts(num_str);
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
/// This is a simplified version that doesn't use format!
pub fn write_cursor_position(row: usize, col: usize, buffer: &mut [u8]) -> usize {
    let mut pos = 0;

    // Add escape sequence start
    buffer[pos] = b'\x1b';
    pos += 1;
    buffer[pos] = b'[';
    pos += 1;

    // Add row number
    let row_len = write_number_to_buffer(row as u64, &mut buffer[pos..]);
    pos += row_len;

    // Add separator
    buffer[pos] = b';';
    pos += 1;

    // Add column number
    let col_len = write_number_to_buffer(col as u64, &mut buffer[pos..]);
    pos += col_len;

    // Add terminator
    buffer[pos] = b'H';
    pos += 1;

    pos
}
