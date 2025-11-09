use anyhow::Result;
use futures::StreamExt;
use std::io::{self, Write};

use crate::api::{ApiClient, StreamChunk, ToolCallAccumulator};
use crate::history::{ChatSession, Message};

/// 发送消息给 AI 并接收响应
pub async fn send_and_receive(
    client: &ApiClient,
    messages: Vec<Message>,
    _session: &ChatSession,
) -> Result<(Message, Option<Vec<crate::history::ToolCall>>, std::collections::HashMap<String, crate::ui::ToolCallDisplay>)> {
    // 使用带重试的流式请求
    let mut stream = client.chat_stream_with_retry(messages).await?;
    
    let mut content = String::new();
    let mut tool_accumulator = ToolCallAccumulator::new();
    let mut has_tool_calls = false;
    
    let mut is_first_reasoning = true;
    let mut has_reasoning = false;
    
    print!("\n\x1b[36m[AI]\x1b[0m ");
    io::stdout().flush()?;

    while let Some(chunk_result) = stream.next().await {
        match chunk_result? {
            StreamChunk::Content(text) => {
                // 如果之前有思考内容，先恢复颜色并换行
                if has_reasoning {
                    print!("\x1b[0m\n\n");  // 重置颜色并换行
                    has_reasoning = false;
                }
                print!("{}", text);
                io::stdout().flush()?;
                content.push_str(&text);
            }
            StreamChunk::Reasoning(text) => {
                if is_first_reasoning {
                    print!("\x1b[90m[THINK] ");  // 深灰色提示
                    is_first_reasoning = false;
                }
                print!("\x1b[90m{}", text);  // 深灰色显示思考过程
                io::stdout().flush()?;
                has_reasoning = true;
            }
            StreamChunk::ToolCall { id, name, arguments } => {
                // 如果之前有思考内容，先恢复颜色并换行
                if has_reasoning {
                    print!("\x1b[0m\n\n");
                    has_reasoning = false;
                }
                if !has_tool_calls {
                    println!();  // 简单换行
                    has_tool_calls = true;
                }
                // 累积工具调用数据（会实时显示）
                tool_accumulator.add_chunk(id, name, arguments);
            }
            StreamChunk::FinishReason(reason) => {
                // 记录完成原因
                tool_accumulator.set_finish_reason(reason);
            }
            StreamChunk::Done => break,
        }
    }
    
    // 确保最后恢复颜色并换行
    if has_reasoning {
        print!("\x1b[0m\n");
    } else if !content.is_empty() {
        // 如果有正常输出，换行
        println!();
    }

    // 获取工具调用和 UI 显示组件
    let displays = tool_accumulator.get_displays().clone();
    let tool_calls = if has_tool_calls {
        let calls = tool_accumulator.into_tool_calls();
        if calls.is_empty() {
            // 关键：检测到有 tool_call 标记但没有成功解析的调用
            // 这意味着 JSON 被截断了，需要告诉 AI
            eprintln!("\n\x1b[31m[✗] Critical:\x1b[0m Tool calls detected but all failed to parse (likely JSON truncation)");
            eprintln!("\x1b[33m[!] Suggestion:\x1b[0m AI should use smaller chunks for file_write\n");
            
            // 在 content 中添加错误提示，让 AI 知道发生了什么
            content.push_str("\n\n[SYSTEM ERROR: Tool call failed due to incomplete JSON in streaming. ");
            content.push_str("This usually means the content parameter was too large (>3000 chars). ");
            content.push_str("Please retry with smaller chunks: use file_write with mode='overwrite' for first ~50 lines, ");
            content.push_str("then mode='append' for additional chunks.]");
            
            None
        } else {
            Some(calls)
        }
    } else {
        None
    };

    let message = Message {
        role: "assistant".to_string(),
        content,
        tool_calls: tool_calls.clone(),
        tool_call_id: None,
        name: None,
    };

    Ok((message, tool_calls, displays))
}
