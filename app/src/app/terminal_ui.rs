use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

use ui::get_i18n;

/// Terminal UI helper for displaying persistent hints
pub struct TerminalUI;

impl TerminalUI {
    /// Print the bottom hint bar
    pub fn print_hint_bar() -> io::Result<()> {
        let i18n = get_i18n();
        let hint_text = format!(
            " {} Enter {}  |  {} Alt+Enter {} ",
            "📤",
            i18n.get("hint_send"),
            "💬",
            i18n.get("hint_newline")
        );

        // Get terminal size
        let (width, _) = terminal::size().unwrap_or((80, 24));

        // Calculate padding to center or fill the bar
        let hint_len = strip_ansi_len(&hint_text);
        let padding = if hint_len < width as usize {
            (width as usize - hint_len) / 2
        } else {
            0
        };

        // Print the hint bar
        execute!(
            io::stdout(),
            SetBackgroundColor(Color::DarkGrey),
            SetForegroundColor(Color::White),
            Print(" ".repeat(padding)),
            Print(&hint_text),
            Print(" ".repeat(width as usize - hint_len - padding)),
            ResetColor,
            Print("\n")
        )?;

        io::stdout().flush()
    }

    /// Clear the screen and prepare for new input
    pub fn prepare_screen() -> io::Result<()> {
        execute!(
            io::stdout(),
            Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        io::stdout().flush()
    }

    /// Print hint bar at a specific position (bottom of screen)
    pub fn print_hint_bar_at_bottom() -> io::Result<()> {
        let (width, height) = terminal::size().unwrap_or((80, 24));
        let i18n = get_i18n();

        let hint_text = format!(
            " {} Enter {}  |  {} Alt+Enter {} ",
            "📤",
            i18n.get("hint_send"),
            "💬",
            i18n.get("hint_newline")
        );

        // Save cursor position
        execute!(io::stdout(), cursor::SavePosition)?;

        // Move to bottom line
        execute!(
            io::stdout(),
            cursor::MoveTo(0, height - 1),
            Clear(ClearType::CurrentLine)
        )?;

        // Calculate padding
        let hint_len = strip_ansi_len(&hint_text);
        let padding = if hint_len < width as usize {
            (width as usize - hint_len) / 2
        } else {
            0
        };

        // Print the hint bar
        execute!(
            io::stdout(),
            SetBackgroundColor(Color::DarkGrey),
            SetForegroundColor(Color::White),
            Print(" ".repeat(padding)),
            Print(&hint_text),
            Print(" ".repeat(width as usize - hint_len - padding)),
            ResetColor
        )?;

        // Restore cursor position
        execute!(io::stdout(), cursor::RestorePosition)?;
        io::stdout().flush()
    }

    /// Simple version: just print the hint before prompt
    pub fn print_simple_hint() -> io::Result<()> {
        let i18n = get_i18n();
        execute!(
            io::stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print("💡 "),
            Print(i18n.get("hint_short")),
            ResetColor,
            Print("\n")
        )?;
        io::stdout().flush()
    }
}

/// Strip ANSI codes and calculate actual display length
fn strip_ansi_len(s: &str) -> usize {
    // Simple implementation: count visible characters
    // This is a rough approximation
    let mut count = 0;
    let mut in_escape = false;

    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape && c == 'm' {
            in_escape = false;
        } else if !in_escape {
            // Count emojis as 2 width (approximation)
            if c as u32 > 0x1F000 {
                count += 2;
            } else {
                count += 1;
            }
        }
    }

    count
}
