//! Text Editor Shell Command
//!
//! Shell interface for the TinyOS text editor application.

use crate::shell::ShellContext;
use crate::apps::editor::TextEditor;

/// Handle the 'edit' command
pub fn cmd_edit(args: &[&str], context: &mut ShellContext) {
    if args.len() > 1 && (args[1] == "help" || args[1] == "--help") {
        show_editor_help(context);
        return;
    }
    
    let mut editor = if args.len() > 1 {
        // Open specific file
        let filename = args[1];
        context.uart.puts("Opening file: ");
        context.uart.puts(filename);
        context.uart.puts("\n");
        // Create editor and load file manually to avoid lifetime issues
        let mut editor = TextEditor::new();
        if let Err(e) = editor.load_file(filename) {
            context.uart.puts("Warning: Could not load file: ");
            context.uart.puts(e);
            context.uart.puts("\n");
        }
        editor
    } else {
        // Open empty editor
        context.uart.puts("Opening new file\n");
        TextEditor::new()
    };
    
    // Show editor startup message
    context.uart.puts("\n╔══════════════════════════════════════════════════════════════════════════════╗\n");
    context.uart.puts("║                            TinyOS Text Editor                               ║\n");
    #[cfg(feature = "raspi3")]
    context.uart.puts("║                      Optimized for Raspberry Pi 3                          ║\n");
    #[cfg(not(feature = "raspi3"))]
    context.uart.puts("║                      Optimized for Raspberry Pi 4/5                        ║\n");
    context.uart.puts("╚══════════════════════════════════════════════════════════════════════════════╝\n");
    context.uart.puts("\nControls:\n");
    context.uart.puts("  Ctrl+S  - Save file\n");
    context.uart.puts("  Ctrl+Q  - Quit (with save prompt)\n");
    context.uart.puts("  Ctrl+X  - Force quit (without saving)\n");
    context.uart.puts("  Ctrl+A  - Move to beginning of line\n");
    context.uart.puts("  Ctrl+E  - Move to end of line\n");
    context.uart.puts("  Ctrl+D  - Delete character\n");
    context.uart.puts("  Arrow keys - Navigate\n");
    context.uart.puts("  Page Up/Down - Scroll\n");
    context.uart.puts("\nPress any key to continue...\n");
    
    // Wait for user to press a key
    context.uart.getc();
    
    // Run the editor
    match editor.run_with_context(context) {
        Ok(_) => {
            context.uart.puts("\nText editor closed successfully.\n");
        }
        Err(e) => {
            context.uart.puts("\nEditor error: ");
            context.uart.puts(e);
            context.uart.puts("\n");
        }
    }
}

/// Handle the 'editor' command (alias for edit)
pub fn cmd_editor(args: &[&str], context: &mut ShellContext) {
    cmd_edit(args, context);
}

/// Show editor help
fn show_editor_help(context: &mut ShellContext) {
    context.uart.puts("\nTinyOS Text Editor Help\n");
    context.uart.puts("=======================\n\n");
    
    context.uart.puts("Usage:\n");
    context.uart.puts("  edit [filename]     - Open editor with optional file\n");
    context.uart.puts("  editor [filename]   - Alias for edit command\n");
    context.uart.puts("  edit help           - Show this help\n\n");
    
    context.uart.puts("Editor Features:\n");
    context.uart.puts("  • Text editing with cursor navigation\n");
    context.uart.puts("  • File loading and saving\n");
    context.uart.puts("  • Line numbers and status display\n");
    context.uart.puts("  • Optimized for Pi 4/5 performance\n");
    context.uart.puts("  • Terminal-based interface\n\n");
    
    context.uart.puts("Keyboard Shortcuts:\n");
    context.uart.puts("  Ctrl+S  - Save current file\n");
    context.uart.puts("  Ctrl+Q  - Quit with save prompt\n");
    context.uart.puts("  Ctrl+X  - Force quit without saving\n");
    context.uart.puts("  Ctrl+A  - Move to beginning of line\n");
    context.uart.puts("  Ctrl+E  - Move to end of line\n");
    context.uart.puts("  Ctrl+D  - Delete character under cursor\n");
    context.uart.puts("  Arrow keys - Navigate cursor\n");
    context.uart.puts("  Page Up/Down - Scroll viewport\n");
    context.uart.puts("  Home/End - Move to line start/end\n");
    context.uart.puts("  Backspace - Delete previous character\n");
    context.uart.puts("  Delete - Delete current character\n");
    context.uart.puts("  Enter - Insert new line\n");
    context.uart.puts("  Tab - Insert tab character\n\n");
    
    context.uart.puts("File Operations:\n");
    context.uart.puts("  • Automatically detects file existence\n");
    context.uart.puts("  • Creates new files when saving\n");
    context.uart.puts("  • Warns about unsaved changes\n");
    context.uart.puts("  • Works with TinyOS filesystem\n\n");
    
    context.uart.puts("Performance Features:\n");
    context.uart.puts("  • Optimized text buffer for Pi 4/5\n");
    context.uart.puts("  • Efficient screen updates\n");
    context.uart.puts("  • Minimal memory footprint\n");
    context.uart.puts("  • Real-time responsive editing\n\n");
    
    context.uart.puts("Examples:\n");
    context.uart.puts("  edit                - Open new file\n");
    context.uart.puts("  edit hello.txt      - Open hello.txt\n");
    context.uart.puts("  edit readme.md      - Open readme.md\n");
    context.uart.puts("  editor config.txt   - Open config.txt\n\n");
}

// Removed unused quick_edit and benchmark commands for streamlined implementation