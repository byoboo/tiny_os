//! Input Processing
//!
//! Keyboard input handling for the text editor, optimized for responsive
//! editing.

use crate::apps::editor::buffer::CursorDirection;

/// Input actions that can be performed
#[derive(Debug, Clone, Copy)]
pub enum InputAction {
    /// Insert a character
    Insert(char),
    /// Backspace (delete previous character)
    Backspace,
    /// Delete (delete current character)
    Delete,
    /// Move cursor
    MoveCursor(CursorDirection),
    /// Save file
    Save,
    /// Quit editor
    Quit,
    /// Force quit (without saving)
    ForceQuit,
    /// No action
    None,
}

/// Input handler for processing keyboard input
pub struct InputHandler {
    /// State for escape sequence processing
    escape_state: EscapeState,
    /// Buffer for multi-byte sequences
    sequence_buffer: [u8; 8],
    /// Current position in sequence buffer
    sequence_pos: usize,
}

/// State machine for processing escape sequences
#[derive(Debug, Clone, Copy, PartialEq)]
enum EscapeState {
    Normal,
    Escape,
    CSI,
    Sequence,
}

impl InputHandler {
    /// Create a new input handler
    pub fn new() -> Self {
        Self {
            escape_state: EscapeState::Normal,
            sequence_buffer: [0; 8],
            sequence_pos: 0,
        }
    }

    /// Process a single input byte and return the corresponding action
    pub fn process_input(&mut self, byte: u8) -> InputAction {
        match self.escape_state {
            EscapeState::Normal => self.process_normal_input(byte),
            EscapeState::Escape => self.process_escape_input(byte),
            EscapeState::CSI => self.process_csi_input(byte),
            EscapeState::Sequence => self.process_sequence_input(byte),
        }
    }

    /// Process normal character input
    fn process_normal_input(&mut self, byte: u8) -> InputAction {
        match byte {
            // Control characters
            0x1B => {
                // ESC - start escape sequence
                self.escape_state = EscapeState::Escape;
                self.sequence_pos = 0;
                InputAction::None
            }
            0x08 | 0x7F => {
                // Backspace or DEL
                InputAction::Backspace
            }
            0x0A | 0x0D => {
                // LF or CR - insert newline
                InputAction::Insert('\n')
            }
            0x09 => {
                // Tab
                InputAction::Insert('\t')
            }
            0x13 => {
                // Ctrl+S - Save
                InputAction::Save
            }
            0x11 => {
                // Ctrl+Q - Quit
                InputAction::Quit
            }
            0x18 => {
                // Ctrl+X - Force quit
                InputAction::ForceQuit
            }
            0x01 => {
                // Ctrl+A - Home
                InputAction::MoveCursor(CursorDirection::Home)
            }
            0x05 => {
                // Ctrl+E - End
                InputAction::MoveCursor(CursorDirection::End)
            }
            0x04 => {
                // Ctrl+D - Delete
                InputAction::Delete
            }
            0x0C => {
                // Ctrl+L - Refresh (no action for now)
                InputAction::None
            }
            // Regular printable characters
            0x20..=0x7E => InputAction::Insert(byte as char),
            _ => {
                // Other control characters or invalid bytes
                InputAction::None
            }
        }
    }

    /// Process escape sequence start
    fn process_escape_input(&mut self, byte: u8) -> InputAction {
        match byte {
            b'[' => {
                // CSI (Control Sequence Introducer)
                self.escape_state = EscapeState::CSI;
                self.sequence_buffer[0] = byte;
                self.sequence_pos = 1;
                InputAction::None
            }
            b'O' => {
                // SS3 (Single Shift Three) - function keys
                self.escape_state = EscapeState::Sequence;
                self.sequence_buffer[0] = byte;
                self.sequence_pos = 1;
                InputAction::None
            }
            _ => {
                // Reset to normal state
                self.escape_state = EscapeState::Normal;
                self.sequence_pos = 0;
                InputAction::None
            }
        }
    }

    /// Process CSI sequence
    fn process_csi_input(&mut self, byte: u8) -> InputAction {
        if self.sequence_pos < self.sequence_buffer.len() - 1 {
            self.sequence_buffer[self.sequence_pos] = byte;
            self.sequence_pos += 1;
        }

        // Check if this is the final byte of the sequence
        if byte >= 0x40 && byte <= 0x7E {
            let action = self.parse_csi_sequence();
            self.escape_state = EscapeState::Normal;
            self.sequence_pos = 0;
            action
        } else {
            InputAction::None
        }
    }

    /// Process SS3 sequence
    fn process_sequence_input(&mut self, byte: u8) -> InputAction {
        if self.sequence_pos < self.sequence_buffer.len() - 1 {
            self.sequence_buffer[self.sequence_pos] = byte;
            self.sequence_pos += 1;
        }

        let action = self.parse_ss3_sequence(byte);
        self.escape_state = EscapeState::Normal;
        self.sequence_pos = 0;
        action
    }

    /// Parse CSI sequence and return appropriate action
    fn parse_csi_sequence(&self) -> InputAction {
        if self.sequence_pos < 2 {
            return InputAction::None;
        }

        let final_byte = self.sequence_buffer[self.sequence_pos - 1];

        match final_byte {
            b'A' => InputAction::MoveCursor(CursorDirection::Up),
            b'B' => InputAction::MoveCursor(CursorDirection::Down),
            b'C' => InputAction::MoveCursor(CursorDirection::Right),
            b'D' => InputAction::MoveCursor(CursorDirection::Left),
            b'H' => InputAction::MoveCursor(CursorDirection::Home),
            b'F' => InputAction::MoveCursor(CursorDirection::End),
            b'~' => {
                // Extended sequences like Page Up/Down, Delete, etc.
                self.parse_extended_sequence()
            }
            _ => InputAction::None,
        }
    }

    /// Parse SS3 sequence (function keys)
    fn parse_ss3_sequence(&self, byte: u8) -> InputAction {
        match byte {
            b'H' => InputAction::MoveCursor(CursorDirection::Home),
            b'F' => InputAction::MoveCursor(CursorDirection::End),
            _ => InputAction::None,
        }
    }

    /// Parse extended sequences (those ending with ~)
    fn parse_extended_sequence(&self) -> InputAction {
        if self.sequence_pos < 3 {
            return InputAction::None;
        }

        // Look for numeric parameter
        let mut param = 0u8;
        for i in 1..self.sequence_pos - 1 {
            let byte = self.sequence_buffer[i];
            if byte >= b'0' && byte <= b'9' {
                param = param * 10 + (byte - b'0');
            } else {
                break;
            }
        }

        match param {
            1 => InputAction::MoveCursor(CursorDirection::Home),
            2 => InputAction::None, // Insert key
            3 => InputAction::Delete,
            4 => InputAction::MoveCursor(CursorDirection::End),
            5 => InputAction::MoveCursor(CursorDirection::PageUp),
            6 => InputAction::MoveCursor(CursorDirection::PageDown),
            _ => InputAction::None,
        }
    }

    /// Reset the input handler state
    pub fn reset(&mut self) {
        self.escape_state = EscapeState::Normal;
        self.sequence_pos = 0;
        for i in 0..self.sequence_buffer.len() {
            self.sequence_buffer[i] = 0;
        }
    }

    /// Check if currently processing an escape sequence
    pub fn is_in_sequence(&self) -> bool {
        self.escape_state != EscapeState::Normal
    }
}

/// Helper functions for input processing
impl InputHandler {
    /// Process a string of input bytes
    pub fn process_string(&mut self, bytes: &[u8], actions: &mut [InputAction]) -> usize {
        let mut count = 0;

        for &byte in bytes {
            if count >= actions.len() {
                break;
            }

            let action = self.process_input(byte);
            if !matches!(action, InputAction::None) {
                actions[count] = action;
                count += 1;
            }
        }

        count
    }

    /// Check if a byte is a printable ASCII character
    pub fn is_printable(byte: u8) -> bool {
        byte >= 0x20 && byte <= 0x7E
    }

    /// Check if a byte is a control character
    pub fn is_control(byte: u8) -> bool {
        byte < 0x20 || byte == 0x7F
    }
}

/// Common key combinations and their byte values
#[allow(dead_code)]
pub mod keys {
    pub const CTRL_A: u8 = 0x01;
    pub const CTRL_B: u8 = 0x02;
    pub const CTRL_C: u8 = 0x03;
    pub const CTRL_D: u8 = 0x04;
    pub const CTRL_E: u8 = 0x05;
    pub const CTRL_F: u8 = 0x06;
    pub const CTRL_G: u8 = 0x07;
    pub const CTRL_H: u8 = 0x08;
    pub const CTRL_I: u8 = 0x09; // Tab
    pub const CTRL_J: u8 = 0x0A; // LF
    pub const CTRL_K: u8 = 0x0B;
    pub const CTRL_L: u8 = 0x0C;
    pub const CTRL_M: u8 = 0x0D; // CR
    pub const CTRL_N: u8 = 0x0E;
    pub const CTRL_O: u8 = 0x0F;
    pub const CTRL_P: u8 = 0x10;
    pub const CTRL_Q: u8 = 0x11;
    pub const CTRL_R: u8 = 0x12;
    pub const CTRL_S: u8 = 0x13;
    pub const CTRL_T: u8 = 0x14;
    pub const CTRL_U: u8 = 0x15;
    pub const CTRL_V: u8 = 0x16;
    pub const CTRL_W: u8 = 0x17;
    pub const CTRL_X: u8 = 0x18;
    pub const CTRL_Y: u8 = 0x19;
    pub const CTRL_Z: u8 = 0x1A;

    pub const ESC: u8 = 0x1B;
    pub const BACKSPACE: u8 = 0x08;
    pub const DELETE: u8 = 0x7F;
    pub const TAB: u8 = 0x09;
    pub const SPACE: u8 = 0x20;
}
