use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileListArgs {
    pub path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FileReadArgs {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct FileWriteArgs {
    pub path: String,
    pub content: String,
    #[serde(default = "default_write_mode")]
    pub mode: String,  // "overwrite" 或 "append"
}

pub fn default_write_mode() -> String {
    "overwrite".to_string()
}

#[derive(Debug, Deserialize)]
pub struct Edit {
    pub old: String,
    pub new: String,
    #[serde(default)]
    pub replace_all: bool,
    #[serde(default)]
    pub normalize: bool,  // 是否启用宽松匹配（忽略多余空格/换行符差异）
    #[serde(default)]
    pub regex: bool,  // 是否使用正则表达式匹配
}

#[derive(Debug, Deserialize)]
pub struct FileReplaceArgs {
    pub path: String,
    pub edits: Vec<Edit>,
}

#[derive(Debug, Deserialize)]
pub struct SearchArgs {
    pub keywords: String,
    #[serde(default = "default_max_results")]
    pub max_results: usize,
}

pub fn default_max_results() -> usize {
    5
}