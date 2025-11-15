use anyhow::Result;

use crate::i18n::I18n;

/// Handle /runcommand command
pub fn handle_run_command_command(parts: &[&str], _i18n: &I18n) -> Result<()> {
    match parts.get(1) {
        Some(&"list") => {
            match crate::tools::command_manager::CommandConfig::load() {
                Ok(config) => {
                    let commands = config.list_always_approve_commands();

                    if commands.is_empty() {
                        println!("\n\x1b[90m[i] No commands require approval\x1b[0m\n");
                    } else {
                        println!("\n\x1b[1;33mCommands requiring approval:\x1b[0m");
                        for (i, cmd) in commands.iter().enumerate() {
                            println!("  \x1b[32m[{}]\x1b[0m {}", i + 1, cmd);
                        }
                        println!();
                    }
                }
                Err(e) => eprintln!("\n\x1b[31m[X] Failed to load command config:\x1b[0m {}\n", e),
            }
        }
        Some(&"add") => {
            if let Some(cmd) = parts.get(2) {
                let mut config = match crate::tools::command_manager::CommandConfig::load() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("\n\x1b[31m[X] Failed to load command config:\x1b[0m {}\n", e);
                        return Ok(());
                    }
                };

                if config.add_always_approve_command(cmd) {
                    config.save()?;
                    println!("\n\x1b[32m[OK]\x1b[0m Added '{}' to approval list\n", cmd);
                } else {
                    println!("\n\x1b[33m[!] '{}' is already in approval list\n", cmd);
                }
            } else {
                println!("\n\x1b[33m[!] Usage:\x1b[0m /runcommand add <command>\n");
            }
        }
        Some(&"del") | Some(&"remove") => {
            if let Some(cmd) = parts.get(2) {
                let mut config = match crate::tools::command_manager::CommandConfig::load() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("\n\x1b[31m[X] Failed to load command config:\x1b[0m {}\n", e);
                        return Ok(());
                    }
                };

                if config.remove_always_approve_command(cmd) {
                    config.save()?;
                    println!("\n\x1b[32m[OK]\x1b[0m Removed '{}' from approval list\n", cmd);
                } else {
                    println!("\n\x1b[33m[!] '{}' is not in approval list\n", cmd);
                }
            } else {
                println!("\n\x1b[33m[!] Usage:\x1b[0m /runcommand del <command>\n");
            }
        }
        Some(&"info") => {
            if let Some(id_str) = parts.get(2) {
                match crate::tools::command_manager::CommandConfig::load() {
                    Ok(config) => {
                        if let Some(cmd) = config.get_background_command(id_str) {
                            println!("\n\x1b[1;33mBackground Command Info:\x1b[0m");
                            println!("  ID: {}", cmd.id);
                            println!("  Command: {}", cmd.command);
                            println!("  Status: {}", cmd.status);
                            println!("  Started: {}", cmd.start_time.format("%Y-%m-%d %H:%M:%S UTC"));

                            if let Some(code) = cmd.exit_code {
                                println!("  Exit Code: {}", code);
                            }

                            if let Some(output) = &cmd.output {
                                println!("\n\x1b[1;33mOutput:\x1b[0m");
                                println!("  {}", output.replace("\n", "\n  "));
                            }

                            println!();
                        } else {
                            println!("\n\x1b[31m[X] Command with ID '{}' not found\x1b[0m\n", id_str);
                        }
                    }
                    Err(e) => eprintln!("\n\x1b[31m[X] Failed to load command config:\x1b[0m {}\n", e),
                }
            } else {
                println!("\n\x1b[33m[!] Usage:\x1b[0m /runcommand info <id>\n");
            }
        }
        _ => {
            println!("\n\x1b[33m[?] Help for /runcommand:\x1b[0m");
            println!("    \x1b[36m/runcommand\x1b[0m list        List commands requiring approval");
            println!("    \x1b[36m/runcommand\x1b[0m add <cmd>   Add command to approval list");
            println!("    \x1b[36m/runcommand\x1b[0m del <cmd>   Remove command from approval list");
            println!("    \x1b[36m/runcommand\x1b[0m info <id>   Show background command details\n");
        }
    }
    Ok(())
}
