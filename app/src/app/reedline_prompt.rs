use reedline::{Prompt, PromptHistorySearch, PromptHistorySearchStatus};
use std::borrow::Cow;

/// Custom prompt for Friendev
pub struct FriendevPrompt {
    pub prefix: String,
}

impl FriendevPrompt {
    pub fn new(prefix: String) -> Self {
        Self { prefix }
    }
}

impl Prompt for FriendevPrompt {
    fn render_prompt_left(&self) -> Cow<str> {
        Cow::Borrowed("")
    }

    fn render_prompt_right(&self) -> Cow<str> {
        Cow::Borrowed("")
    }

    fn render_prompt_indicator(&self, _prompt_mode: reedline::PromptEditMode) -> Cow<str> {
        Cow::Owned(format!("\x1b[36m{}\x1b[0m ", self.prefix))
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        // Multi-line continuation indicator
        Cow::Borrowed("\x1b[90m...\x1b[0m ")
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: PromptHistorySearch,
    ) -> Cow<str> {
        let prefix = match history_search.status {
            PromptHistorySearchStatus::Passing => "",
            PromptHistorySearchStatus::Failing => "failing ",
        };

        Cow::Owned(format!(
            "({}reverse-search: {}) ",
            prefix, history_search.term
        ))
    }
}
