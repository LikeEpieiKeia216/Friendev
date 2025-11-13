# Changelog

all friendev update changelog on here.
---
## [0.1.3] - 2025-11-14
新增了/agents.md命令，可以让AI生成AGENTS.md
新增了AGENTS.md处理逻辑，自动会将AGENTS.md的内容加入到上下文
Approval Required增加了`[i]nfo`，用于查看AI的详细编码。（By Friendev）
优化了`file_replace`工具的字符串匹配逻辑，新增`normalize`参数支持宽松匹配，并增强了失败时的诊断信息以解决换行符/空格差异问题

---
## [0.1.2] - 2025-11-13

### new

- `network_search_auto` Tool
- `network_search_duckduckgo` Tool
- `network_search_bing` Tool

- Add timeout settings
- SSE Json
- Refactored some features
---
## [0.1.1] - 2025-11-09

### new

- `file_replace` Tool

### edit

- System Prompt to English
- Optimize JSON validation and string processing