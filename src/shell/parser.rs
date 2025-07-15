//! Command Line Parser
//!
//! This module provides command line parsing functionality for the TinyOS shell,
//! converting traditional command-line input into structured commands and arguments.

use crate::shell::ShellContext;

/// Maximum command line length
const MAX_COMMAND_LENGTH: usize = 256;
/// Maximum number of arguments per command
const MAX_ARGS: usize = 16;

/// Parsed command structure
#[derive(Debug)]
pub struct Command {
    pub name: [u8; 64],
    pub name_len: usize,
    pub args: [[u8; 64]; MAX_ARGS],
    pub arg_lens: [usize; MAX_ARGS],
    pub arg_count: usize,
}

impl Command {
    /// Create a new empty command
    pub fn new() -> Self {
        Self {
            name: [0; 64],
            name_len: 0,
            args: [[0; 64]; MAX_ARGS],
            arg_lens: [0; MAX_ARGS],
            arg_count: 0,
        }
    }

    /// Get the command name as a string slice
    pub fn name(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.name[..self.name_len]) }
    }

    /// Get an argument as a string slice
    pub fn arg(&self, index: usize) -> Option<&str> {
        if index < self.arg_count {
            Some(unsafe { 
                core::str::from_utf8_unchecked(&self.args[index][..self.arg_lens[index]]) 
            })
        } else {
            None
        }
    }

    /// Get all arguments as string slices
    pub fn args(&self) -> impl Iterator<Item = &str> {
        (0..self.arg_count).map(move |i| unsafe { 
            core::str::from_utf8_unchecked(&self.args[i][..self.arg_lens[i]]) 
        })
    }

    /// Check if command has no arguments
    pub fn is_empty(&self) -> bool {
        self.name_len == 0
    }
}

/// Command line input handler
pub struct CommandInput {
    /// Current input buffer
    buffer: [u8; MAX_COMMAND_LENGTH],
    /// Current position in buffer
    position: usize,
    /// Command history (simple ring buffer)
    history: [[u8; MAX_COMMAND_LENGTH]; 8],
    /// History lengths
    history_lens: [usize; 8],
    /// Current history position
    history_pos: usize,
    /// History count
    history_count: usize,
}

impl CommandInput {
    /// Create a new command input handler
    pub fn new() -> Self {
        Self {
            buffer: [0; MAX_COMMAND_LENGTH],
            position: 0,
            history: [[0; MAX_COMMAND_LENGTH]; 8],
            history_lens: [0; 8],
            history_pos: 0,
            history_count: 0,
        }
    }

    /// Process a single input character
    /// Returns Some(Command) if a complete command was entered
    pub fn process_char(&mut self, ch: u8, context: &mut ShellContext) -> Option<Command> {
        match ch {
            b'\n' | b'\r' => {
                // End of line - process command
                if self.position > 0 {
                    let command_str = unsafe { 
                        core::str::from_utf8_unchecked(&self.buffer[..self.position]) 
                    };
                    
                    // Parse command first
                    let command = self.parse_command(command_str);
                    
                    // Add to history
                    self.add_to_history();
                    
                    // Clear buffer
                    self.clear_buffer();
                    
                    // Echo newline
                    context.uart.puts("\r\n");
                    
                    Some(command)
                } else {
                    // Empty line
                    context.uart.puts("\r\n");
                    None
                }
            }
            8 | 127 => {
                // Backspace
                if self.position > 0 {
                    self.position -= 1;
                    self.buffer[self.position] = 0;
                    context.uart.puts("\x08 \x08"); // Backspace, space, backspace
                }
                None
            }
            21 => {
                // Ctrl+U - clear line
                while self.position > 0 {
                    self.position -= 1;
                    context.uart.puts("\x08 \x08");
                }
                None
            }
            3 => {
                // Ctrl+C - interrupt/cancel
                context.uart.puts("^C\r\n");
                self.clear_buffer();
                None
            }
            _ => {
                // Regular character
                if ch >= 32 && ch <= 126 && self.position < MAX_COMMAND_LENGTH - 1 {
                    self.buffer[self.position] = ch;
                    self.position += 1;
                    context.uart.putc(ch); // Echo character
                }
                None
            }
        }
    }

    /// Clear the input buffer
    fn clear_buffer(&mut self) {
        self.buffer.fill(0);
        self.position = 0;
    }

    /// Add current buffer to history
    fn add_to_history(&mut self) {
        if self.position > 0 {
            let hist_index = self.history_pos % self.history.len();
            self.history[hist_index][..self.position].copy_from_slice(&self.buffer[..self.position]);
            self.history_lens[hist_index] = self.position;
            self.history_pos = (self.history_pos + 1) % self.history.len();
            if self.history_count < self.history.len() {
                self.history_count += 1;
            }
        }
    }

    /// Parse a command string into a Command structure
    fn parse_command(&self, input: &str) -> Command {
        let mut command = Command::new();
        let mut tokens = input.trim().split_whitespace();
        
        // First token is the command name
        if let Some(name) = tokens.next() {
            command.name_len = name.len().min(command.name.len());
            command.name[..command.name_len].copy_from_slice(name.as_bytes());
            
            // Remaining tokens are arguments
            for (i, arg) in tokens.enumerate() {
                if i >= MAX_ARGS {
                    break;
                }
                
                command.arg_lens[i] = arg.len().min(command.args[i].len());
                command.args[i][..command.arg_lens[i]].copy_from_slice(arg.as_bytes());
                command.arg_count += 1;
            }
        }
        
        command
    }

    /// Get current input as string (for display)
    pub fn current_input(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.buffer[..self.position]) }
    }
}

/// Command completion helper
pub struct CommandCompletion {
    /// List of available commands
    commands: [&'static str; 32],
    /// Number of commands in the list
    command_count: usize,
}

impl CommandCompletion {
    /// Create a new command completion helper
    pub fn new() -> Self {
        let mut completion = Self {
            commands: [""; 32],
            command_count: 0,
        };
        
        // Add standard commands
        completion.add_command("help");
        completion.add_command("ls");
        completion.add_command("cd");
        completion.add_command("pwd");
        completion.add_command("cat");
        completion.add_command("edit");
        completion.add_command("mkdir");
        completion.add_command("rmdir");
        completion.add_command("rm");
        completion.add_command("cp");
        completion.add_command("mv");
        completion.add_command("clear");
        completion.add_command("echo");
        completion.add_command("date");
        completion.add_command("uptime");
        completion.add_command("ps");
        completion.add_command("kill");
        completion.add_command("mount");
        completion.add_command("umount");
        completion.add_command("df");
        completion.add_command("free");
        completion.add_command("test");
        completion.add_command("benchmark");
        completion.add_command("reboot");
        completion.add_command("halt");
        
        completion
    }

    /// Add a command to the completion list
    fn add_command(&mut self, cmd: &'static str) {
        if self.command_count < self.commands.len() {
            self.commands[self.command_count] = cmd;
            self.command_count += 1;
        }
    }

    /// Find completions for a partial command
    pub fn complete<'a>(&'a self, partial: &'a str) -> impl Iterator<Item = &'static str> + '_ {
        self.commands[..self.command_count]
            .iter()
            .filter(move |&&cmd| cmd.starts_with(partial))
            .copied()
    }
}