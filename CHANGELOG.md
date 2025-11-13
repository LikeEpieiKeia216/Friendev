# Changelog

all friendev update changelog on here.
---
## [0.1.3] - 2025-11-14
新增了/agents.md命令，可以让AI生成AGENTS.md
新增了AGENTS.md处理逻辑，自动会将AGENTS.md的内容加入到上下文
Approval Required增加了`[i]nfo`，用于查看AI的详细编码。（By Friendev）
优化了`file_replace`工具：新增`normalize`(宽松匹配)和`regex`(正则)参数、规范化文件换行符，确保一致性处理、其他边界问题
修复了调用工具时可能出现的边界情况(增强工具调用执行的有效性检查，跳过无效的工具调用和无效的JSON参数)，例如：`{"error":{"code":"invalid_parameter_error","message":"\u003c400\u003e InternalError.Algo.InvalidParameter: An assistant message with \"tool_calls\" must be followed by tool messages responding to each \"tool_call_id\". The following tool_call_ids did not have response messages: message[41].role","param":null,"type":"invalid_request_error"},"request_id":"183ac8b3-df24-4f44-b02b-14c505e8c939"}`（By Friendev）
"/history list"命令将会自动过滤掉消息数量为零的会话。
每次启动的时候，都会自动删除0 msg的history文件
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