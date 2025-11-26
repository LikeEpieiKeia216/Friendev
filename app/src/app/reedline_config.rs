use reedline::{
    default_emacs_keybindings, ColumnarMenu, DefaultCompleter, EditCommand, Keybindings,
    KeyCode, KeyModifiers, Reedline, ReedlineEvent, ReedlineMenu, Signal,
};
use std::io;

use super::reedline_prompt::FriendevPrompt;

/// Initialize reedline with custom configuration
pub fn create_reedline() -> io::Result<Reedline> {
    // Create key bindings (Emacs-style with custom additions)
    let mut keybindings = default_emacs_keybindings();
    
    // Custom key bindings
    add_custom_keybindings(&mut keybindings);
    
    // Create completer (can be extended later)
    let completer = Box::new(DefaultCompleter::default());
    
    // Create completion menu
    let completion_menu = Box::new(
        ColumnarMenu::default()
            .with_name("completion_menu")
            .with_columns(1)
            .with_column_width(None)
            .with_column_padding(2),
    );
    
    // Build reedline instance
    let line_editor = Reedline::create()
        .with_edit_mode(Box::new(reedline::Emacs::new(keybindings)))
        .with_completer(completer)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu));
    
    Ok(line_editor)
}

/// Add custom key bindings for Friendev
fn add_custom_keybindings(keybindings: &mut Keybindings) {
    // Enter: Submit (send the input) - Default behavior like most chat apps
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Enter,
        ReedlineEvent::Submit,
    );
    
    // Alt+Enter: Insert newline (multi-line editing)
    keybindings.add_binding(
        KeyModifiers::ALT,
        KeyCode::Enter,
        ReedlineEvent::Edit(vec![EditCommand::InsertNewline]),
    );
    
    // Shift+Enter: Insert newline (alternative for terminals that don't support Alt+Enter)
    keybindings.add_binding(
        KeyModifiers::SHIFT,
        KeyCode::Enter,
        ReedlineEvent::Edit(vec![EditCommand::InsertNewline]),
    );
    
    // Ctrl+Enter: Insert newline (another alternative)
    keybindings.add_binding(
        KeyModifiers::CONTROL,
        KeyCode::Enter,
        ReedlineEvent::Edit(vec![EditCommand::InsertNewline]),
    );
    
    // Ctrl+D on empty line: Exit
    keybindings.add_binding(
        KeyModifiers::CONTROL,
        KeyCode::Char('d'),
        ReedlineEvent::Multiple(vec![
            ReedlineEvent::Edit(vec![EditCommand::Clear]),
            ReedlineEvent::CtrlD,
        ]),
    );
}

/// Create the prompt
pub fn create_prompt() -> FriendevPrompt {
    FriendevPrompt::new(">>".to_string())
}

/// Process reedline signal
pub enum InputResult {
    Input(String),
    CtrlC,
    CtrlD,
    Error(String),
}

pub fn process_signal(signal: Signal) -> InputResult {
    match signal {
        Signal::Success(buffer) => {
            let trimmed = buffer.trim();
            if trimmed.is_empty() {
                InputResult::Input(String::new())
            } else {
                InputResult::Input(buffer)
            }
        }
        Signal::CtrlC => InputResult::CtrlC,
        Signal::CtrlD => InputResult::CtrlD,
    }
}
