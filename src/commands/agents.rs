use anyhow::Result;

use crate::history::ChatSession;
use crate::i18n::I18n;

/// Handle /agents.md command - returns prompt (auto-sent to AI by main.rs)
pub async fn handle_agents_md_command(
    session: &ChatSession,
    _i18n: &I18n,
) -> Result<String> {
    println!("\n\x1b[33m[*] Analyzing project structure...\x1b[0m");

    // Generate analysis prompt
    let analysis_prompt = crate::agents::generate_agents_analysis_prompt(&session.working_directory)?;

    println!("\x1b[32m[OK]\x1b[0m Sending to AI for AGENTS.md generation...\n");

    Ok(analysis_prompt)
}
