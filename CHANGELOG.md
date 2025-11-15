# Changelog

all friendev update changelog on here.
---
## [0.1.4] - 2025-11-15
新增了file_diff_edit工具，`file_diff_edit` 是一个用于精确编辑文件内容的工具，使用 diff 风格的 hunks 来进行批量行级编辑。
修改了 AGENTS.md 的上下文加入位置
新增了run_command工具，run_command为LLM赋予了执行命令的能力(by Friendev)
新增了--ally参数，自动同意全部的Approval Required
将会自动移除没有对应工具响应的孤立工具调用
调整了启动时的代码顺序
重构了部分逻辑
修改了语言ID。
为setup添加了i18n。
更改了setup流程，新增语言设置。
UI语言限制输入内容。
新增了--setup参数，当带有此参数将会强制进入初次启动流程。
充写了Changelog
---
## [0.1.3] - 2025-11-14

### Added
- Added the `/agents.md` command to allow the AI to generate an `AGENTS.md` file.
- Implemented automatic context integration for `AGENTS.md`: its content is now automatically included in the conversation context.

### Enhanced
- **Expanded Approval Required functionality**: Added an `[i]nfo` option to view detailed encoding information of AI-generated content. (By Friendev)
- **Improved the `file_replace` tool**:
  - Added `normalize` (fuzzy matching) and `regex` (regular expression) parameters for greater flexibility;
  - Standardized line endings across files to ensure consistent cross-platform handling;
  - Addressed various edge cases to improve robustness.

### Fixed
- Resolved issues related to invalid tool invocations and parameter handling:
  - Strengthened validation to skip malformed tool calls and invalid JSON parameters;
  - Fixed errors caused by missing `tool_response` messages for `tool_call_id`s (e.g., `invalid_parameter_error`: "An assistant message with 'tool_calls' must be followed by tool messages..."). (By Friendev)

### UX Improvements
- The `/history list` command now automatically filters out sessions with zero messages. (By Friendev)
- On startup, the system now automatically deletes any history files containing zero messages, keeping the session list clean. (By Friendev)
---
## [0.1.2] - 2025-11-13
新增了`network_search_auto`、`network_search_duckduckgo`与`network_search_bing`工具，现在AI可以搜索互联网内容了。
- `network_search_auto`默认使用duckduckgo，如果duckduckgo无法使用则会降级为bing。
为 AI 请求增加了超时重试机制。
更改了流式传输JSON的逻辑，引入了SSE
重构了部分功能
---
## [0.1.1] - 2025-11-09

新增了`file_replace`工具，现在AI可以局部替换了
系统提示词被重写为英文
Optimize JSON validation and string processing