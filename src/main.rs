mod api;
mod chat;
mod commands;
mod config;
mod history;
mod i18n;
mod prompts;
mod security;
mod tools;
mod ui;

use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::env;

use api::ApiClient;
use config::Config;
use history::{ChatSession, Message};
use i18n::I18n;

#[tokio::main]
async fn main() -> Result<()> {
    // 加载或初始化配置
    let mut config = match Config::load()? {
        Some(c) => c,
        None => Config::initialize()?,
    };
    
    // 创建 i18n 实例（用于启动消息）
    let i18n = I18n::new(&config.ui_language);
    
    println!("\x1b[32m[OK]\x1b[0m \x1b[2m{}\x1b[0m\n", i18n.get("config_loaded"));

    // 获取当前工作目录
    let working_dir = env::current_dir()?;
    println!("\x1b[36m[DIR]\x1b[0m \x1b[2m{}\x1b[0m\n", working_dir.display());

    // 创建或加载聊天会话
    let mut session = ChatSession::new(working_dir.clone());
    session.save()?;
    println!("\x1b[32m[OK]\x1b[0m \x1b[2m{}:\x1b[0m \x1b[90m{}\x1b[0m\n", i18n.get("new_session"), session.id);

    // 创建 API 客户端
    let mut api_client = ApiClient::new(config.clone());

    // 创建 REPL
    let mut rl = DefaultEditor::new()?;
    
    // 打印欢迎信息
    prompts::print_welcome(&config, &i18n);

    loop {
        let readline = rl.readline(">> ");
        
        match readline {
            Ok(line) => {
                let line = line.trim();
                
                if line.is_empty() {
                    continue;
                }

                rl.add_history_entry(line)?;

                // 处理命令
                if line.starts_with('/') {
                    if let Err(e) = commands::handle_command(line, &mut config, &mut session, &mut api_client).await {
                        eprintln!("\n\x1b[31m[X] Error:\x1b[0m {}\n", e);
                    }
                    continue;
                }
                
                // 安全检查：拦截特殊标记
                if security::is_input_suspicious(line) {
                    eprintln!("\n\x1b[31m[X] Security Warning:\x1b[0m Input contains forbidden control tokens\n");
                    continue;
                }

                // 用户消息
                let user_message = Message {
                    role: "user".to_string(),
                    content: line.to_string(),
                    tool_calls: None,
                    tool_call_id: None,
                    name: None,
                };
                session.add_message(user_message);

                // 准备消息，添加系统提示词
                let mut messages = vec![
                    Message {
                        role: "system".to_string(),
                        content: prompts::get_system_prompt(&config.ai_language, &config.current_model),
                        tool_calls: None,
                        tool_call_id: None,
                        name: None,
                    }
                ];
                messages.extend(session.messages.clone());
                
                loop {
                    match chat::send_and_receive(&api_client, messages.clone(), &session).await {
                        Ok((response_msg, tool_calls, mut displays)) => {
                            session.add_message(response_msg);
                            
                            if let Some(calls) = tool_calls {
                                // 执行工具调用（启用审批）
                                let tool_results = api::execute_tool_calls(
                                    &calls, 
                                    &session.working_directory,
                                    &mut displays,
                                    true  // 需要用户审批
                                ).await;
                                
                                for result in tool_results {
                                    session.add_message(result);
                                }
                                
                                // 继续循环，发送工具结果给 AI
                                messages = vec![
                                    Message {
                                        role: "system".to_string(),
                                        content: prompts::get_system_prompt(&config.ai_language, &config.current_model),
                                        tool_calls: None,
                                        tool_call_id: None,
                                        name: None,
                                    }
                                ];
                                messages.extend(session.messages.clone());
                                continue;
                            }
                            
                            break;
                        }
                        Err(e) => {
                            eprintln!("\n\x1b[31m[X] API Error:\x1b[0m {}\n", e);
                            // 删除最后一条用户消息，因为没有得到有效响应
                            if !session.messages.is_empty() {
                                session.messages.pop();
                            }
                            break;
                        }
                    }
                }

                session.save()?;
            }
            Err(ReadlineError::Interrupted) => {
                println!("\n\x1b[33m^C\x1b[0m");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("\n\x1b[36mGoodbye!\x1b[0m\n");
                break;
            }
            Err(err) => {
                eprintln!("\n\x1b[31m[X] Error:\x1b[0m {}\n", err);
                break;
            }
        }
    }

    Ok(())
}

// 功能模块：
// - chat.rs: 流式响应处理 (send_and_receive)
// - commands.rs: 命令处理 (handle_command, print_help)
// - prompts.rs: 提示与欢迎信息 (get_system_prompt, print_welcome, print_help)
// - security.rs: 安全检查 (is_input_suspicious)
