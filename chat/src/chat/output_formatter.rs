use std::io::{self, Write};
use ui::{enhanced_output, get_i18n};

/// Handle content output
pub fn print_content(text: &str, has_reasoning: &mut bool) -> std::io::Result<()> {
    // If there was reasoning before, reset and add spacing
    if *has_reasoning {
        print!("\n\n");
        io::stdout().flush()?;
        *has_reasoning = false;
    }
    enhanced_output::print_content(text)
}

/// Handle reasoning output
pub fn print_reasoning(
    text: &str,
    is_first_reasoning: &mut bool,
    has_reasoning: &mut bool,
) -> std::io::Result<()> {
    if *is_first_reasoning {
        enhanced_output::print_reasoning_prefix()?;
        *is_first_reasoning = false;
    }
    enhanced_output::print_reasoning_text(text)?;
    *has_reasoning = true;
    Ok(())
}

/// Print AI prefix
pub fn print_ai_prefix() -> std::io::Result<()> {
    enhanced_output::print_ai_prefix()
}

/// Print tool call separator
pub fn print_tool_call_separator() -> std::io::Result<()> {
    println!();
    Ok(())
}

/// Finalize output formatting
pub fn finalize_output(has_reasoning: bool, content_empty: bool) -> std::io::Result<()> {
    // Ensure proper newline at the end
    if has_reasoning {
        println!();
    } else if !content_empty {
        println!();
    }
    Ok(())
}

/// Print tool parsing error
pub fn print_tool_parse_error() {
    let i18n = get_i18n();
    let error_msg = format!(
        "{}: {}",
        i18n.get("error"),
        i18n.get("chat_tool_parse_error")
    );
    let _ = enhanced_output::print_error(&error_msg);
    
    let debug_msg = format!(
        "{}: {}",
        i18n.get("chat_debug_info_label"),
        i18n.get("chat_tool_parse_debug")
    );
    let _ = enhanced_output::print_warning(&debug_msg);
}
