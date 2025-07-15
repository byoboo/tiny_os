//! TinyOS Built-in Text Editor
//!
//! A high-performance text editor optimized for Raspberry Pi 4/5 hardware,
//! demonstrating efficient application development on TinyOS.

pub mod buffer;
pub mod ui;
pub mod input;
pub mod file_ops;

use crate::apps::Application;
use crate::shell::ShellContext;
use buffer::TextBuffer;
use ui::EditorUI;
use input::InputHandler;
use file_ops::FileOperations;

/// Text editor application
pub struct TextEditor {
    buffer: TextBuffer,
    ui: EditorUI,
    input_handler: InputHandler,
    file_ops: FileOperations,
    current_file: [u8; 256],
    current_file_len: usize,
    running: bool,
}

impl TextEditor {
    /// Create a new text editor instance
    pub fn new() -> Self {
        Self {
            buffer: TextBuffer::new(),
            ui: EditorUI::new(),
            input_handler: InputHandler::new(),
            file_ops: FileOperations::new(),
            current_file: [0; 256],
            current_file_len: 0,
            running: false,
        }
    }
    
    /// Create a text editor with a file to open
    pub fn with_file(filename: &str) -> Self {
        let mut editor = Self::new();
        editor.set_current_file(filename);
        editor
    }
    
    /// Set the current file name
    fn set_current_file(&mut self, filename: &str) {
        self.current_file_len = filename.len().min(self.current_file.len());
        self.current_file[..self.current_file_len].copy_from_slice(filename.as_bytes());
    }
    
    /// Get the current file name
    fn get_current_file(&self) -> Option<&str> {
        if self.current_file_len > 0 {
            Some(unsafe { 
                core::str::from_utf8_unchecked(&self.current_file[..self.current_file_len]) 
            })
        } else {
            None
        }
    }
    
    /// Load a file into the editor
    pub fn load_file(&mut self, filename: &str) -> Result<(), &'static str> {
        let mut content_buffer = [0u8; 64 * 1024]; // 64KB buffer
        let content_size = self.file_ops.read_file(filename, &mut content_buffer)?;
        
        // Convert bytes to string and load into buffer
        let content_str = unsafe { 
            core::str::from_utf8_unchecked(&content_buffer[..content_size]) 
        };
        self.buffer.load_content(content_str);
        self.set_current_file(filename);
        Ok(())
    }
    
    /// Save the current buffer to file
    pub fn save_file(&mut self) -> Result<(), &'static str> {
        if let Some(filename) = self.get_current_file() {
            // Create a copy of the filename to avoid borrowing issues
            let mut filename_buffer = [0u8; 256];
            let filename_len = filename.len().min(filename_buffer.len());
            filename_buffer[..filename_len].copy_from_slice(filename.as_bytes());
            let filename_str = unsafe { 
                core::str::from_utf8_unchecked(&filename_buffer[..filename_len]) 
            };
            
            let mut content_buffer = [0u8; 64 * 1024]; // 64KB buffer
            let content_size = self.buffer.get_content_formatted(&mut content_buffer);
            self.file_ops.write_file(filename_str, &content_buffer[..content_size])?;
            self.buffer.mark_saved();
            Ok(())
        } else {
            Err("No file to save")
        }
    }
    
    /// Save the current buffer to a new file
    pub fn save_file_as(&mut self, filename: &str) -> Result<(), &'static str> {
        let mut content_buffer = [0u8; 64 * 1024]; // 64KB buffer
        let content_size = self.buffer.get_content_formatted(&mut content_buffer);
        self.file_ops.write_file(filename, &content_buffer[..content_size])?;
        self.set_current_file(filename);
        self.buffer.mark_saved();
        Ok(())
    }
    
    /// Run the editor with a shell context
    pub fn run_with_context(&mut self, context: &mut ShellContext) -> Result<(), &'static str> {
        self.running = true;
        
        // Load file if specified
        if let Some(filename) = self.get_current_file() {
            // Create a temporary buffer to store the filename
            let mut filename_buffer = [0u8; 256];
            let filename_len = filename.len();
            filename_buffer[..filename_len].copy_from_slice(filename.as_bytes());
            let filename_str = unsafe { 
                core::str::from_utf8_unchecked(&filename_buffer[..filename_len]) 
            };
            
            if let Err(e) = self.load_file(filename_str) {
                context.uart.puts("Warning: Could not load file: ");
                context.uart.puts(e);
                context.uart.puts("\n");
            }
        }
        
        // Initialize UI
        self.ui.init(context)?;
        self.ui.draw_editor(&self.buffer, context);
        
        // Main editor loop
        while self.running {
            if let Some(input) = context.uart.getc() {
                match self.input_handler.process_input(input) {
                    input::InputAction::Insert(ch) => {
                        self.buffer.insert_char(ch);
                        self.ui.update_content_only(&self.buffer, context);
                    }
                    input::InputAction::Backspace => {
                        self.buffer.backspace();
                        self.ui.update_content_only(&self.buffer, context);
                    }
                    input::InputAction::Delete => {
                        self.buffer.delete();
                        self.ui.update_content_only(&self.buffer, context);
                    }
                    input::InputAction::MoveCursor(dir) => {
                        self.buffer.move_cursor(dir);
                        self.ui.update_cursor_only(&self.buffer, context);
                    }
                    input::InputAction::Save => {
                        match self.save_file() {
                            Ok(_) => {
                                self.ui.show_message("File saved", context);
                                self.ui.update_status_only(&self.buffer, context);
                            }
                            Err(e) => {
                                self.ui.show_message(e, context);
                            }
                        }
                    }
                    input::InputAction::Quit => {
                        if self.buffer.is_modified() {
                            self.ui.show_message("File modified. Save first (Ctrl+S) or force quit (Ctrl+Q)", context);
                        } else {
                            self.running = false;
                        }
                    }
                    input::InputAction::ForceQuit => {
                        self.running = false;
                    }
                    input::InputAction::None => {
                        // No action needed
                    }
                }
            }
        }
        
        // Cleanup UI
        self.ui.cleanup(context);
        
        Ok(())
    }
}

impl Application for TextEditor {
    fn init(&mut self) -> Result<(), &'static str> {
        // Application initialization
        self.buffer.init();
        Ok(())
    }
    
    fn run(&mut self) -> Result<(), &'static str> {
        // This would need a ShellContext to run properly
        // For now, just indicate that the editor is ready
        self.running = true;
        Ok(())
    }
    
    fn cleanup(&mut self) {
        self.running = false;
        self.buffer.clear();
    }
    
    fn name(&self) -> &'static str {
        "TinyOS Text Editor"
    }
}

/// Create a new text editor command for the shell
pub fn create_editor_command() -> impl Fn(&[&str], &mut ShellContext) {
    |args: &[&str], context: &mut ShellContext| {
        let mut editor = if args.len() > 1 {
            TextEditor::with_file(args[1])
        } else {
            TextEditor::new()
        };
        
        context.uart.puts("Starting TinyOS Text Editor...\n");
        context.uart.puts("Commands: Ctrl+S (Save), Ctrl+Q (Quit), Ctrl+X (Force Quit)\n");
        context.uart.puts("Use arrow keys to navigate, type to edit.\n\n");
        
        match editor.run_with_context(context) {
            Ok(_) => {
                context.uart.puts("Editor closed.\n");
            }
            Err(e) => {
                context.uart.puts("Editor error: ");
                context.uart.puts(e);
                context.uart.puts("\n");
            }
        }
    }
}