use super::command_handler;
use super::reedline_config::{create_prompt, create_reedline, process_signal, InputResult};
use super::startup::AppState;
use super::terminal_ui::TerminalUI;
use anyhow::Result;
use ui::get_i18n;

/// Run the REPL loop with reedline
pub async fn run_repl(mut state: AppState) -> Result<()> {
    let mut line_editor = create_reedline()?;
    let prompt = create_prompt();

    loop {
        // Display hint before each input
        let _ = TerminalUI::print_simple_hint();
        
        let sig = line_editor.read_line(&prompt);

        match sig {
            Ok(signal) => match process_signal(signal) {
                InputResult::Input(buffer) => {
                    if buffer.is_empty() {
                        continue;
                    }

                    // Handle user input and commands
                    if let Err(e) = command_handler::handle_user_input(&buffer, &mut state).await {
                        let i18n = get_i18n();
                        eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("error"), e);
                    }
                }
                InputResult::CtrlC => {
                    println!("\n\x1b[33m^C\x1b[0m");
                    continue;
                }
                InputResult::CtrlD => {
                    let i18n = get_i18n();
                    println!("\n\x1b[36m{}\x1b[0m\n", i18n.get("goodbye"));
                    break;
                }
                InputResult::Error(err) => {
                    let i18n = get_i18n();
                    eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("error"), err);
                }
            },
            Err(err) => {
                let i18n = get_i18n();
                eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("error"), err);
                break;
            }
        }
    }

    Ok(())
}
