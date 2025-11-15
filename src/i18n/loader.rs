use std::collections::HashMap;
use super::{en_us, zh_cn};

/// Load messages for the specified language
pub fn load_messages(language: &str) -> HashMap<String, String> {
    match language {
        "zh" | "zh-CN" => zh_cn::get_messages(),
        _ => en_us::get_messages(),
    }
}
