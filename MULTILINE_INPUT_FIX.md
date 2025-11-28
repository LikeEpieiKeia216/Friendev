# 多行输入显示覆盖问题修复

## 问题描述

在使用 Friendev 时，当输入区被自动填充多行内容时，会出现显示覆盖或被覆盖的问题。具体场景包括：

1. **优化提示词功能**：使用 `!` 前缀或 `Shift+Enter` 优化提示词后，优化结果会自动填充到输入框，如果内容是多行的，显示会出现问题
2. **历史记录选择**：使用上下方向键选择多行的历史记录时，也会出现覆盖问题

### 问题原因

当 reedline 渲染多行输入时，需要占用多行终端空间。如果当前光标位置接近终端底部，没有足够的垂直空间来显示完整的多行内容，就会导致内容与其他 UI 元素（如提示符、之前的输出等）发生视觉覆盖。

## 解决方案

在 `app/src/app/repl.rs` 的 `prefill_input` 函数中添加了智能空间管理：

### 主要改进

1. **检测多行内容**：通过统计换行符数量判断是否为多行内容
2. **计算所需空间**：计算显示多行内容需要的终端行数（内容行数 + 提示符 + 缓冲空间）
3. **检查可用空间**：获取当前光标位置和终端高度，计算可用空间
4. **动态创建空间**：如果空间不足，通过打印换行符来滚动终端，创建足够的显示空间

### 代码逻辑

```rust
fn prefill_input(line_editor: &mut Reedline, text: &str) -> Result<()> {
    // 统计换行符数量
    let newline_count = text.matches('\n').count();
    
    // 如果是多行内容
    if newline_count > 0 {
        // 获取终端大小
        let (_, height) = terminal::size().unwrap_or((80, 24));
        
        // 获取当前光标位置
        if let Ok((_, current_y)) = cursor::position() {
            // 计算需要的行数（内容 + 提示符 + 缓冲）
            let needed_lines = newline_count + 3;
            let available_lines = height.saturating_sub(current_y);
            
            // 如果空间不足，打印换行符来滚动创建空间
            if available_lines < needed_lines as u16 {
                let extra_lines = needed_lines as u16 - available_lines + 1;
                for _ in 0..extra_lines {
                    println!();
                }
            }
        } else {
            // 降级方案：对多行内容打印一些换行符
            println!("\n");
        }
        
        io::stdout().flush().ok();
    }
    
    // 填充内容到输入框
    line_editor.run_edit_commands(&[
        EditCommand::Clear,
        EditCommand::InsertString(text.to_string()),
    ]);
    
    Ok(())
}
```

## 优势

1. **智能检测**：只对多行内容进行处理，单行输入不受影响
2. **精确计算**：根据实际需要的空间来决定是否滚动终端
3. **无副作用**：只在必要时添加换行，不影响正常使用体验
4. **兼容性好**：包含降级方案，确保在各种终端环境下都能工作

## 测试场景

修复后，以下场景应该都能正常显示：

### 场景 1：优化提示词
```bash
>> !写个排序算法

⚙ 正在优化提示词...
✓ 优化完成

原始: 写个排序算法
优化: 请帮我实现一个排序算法，具体要求如下：
1. 算法类型：请使用快速排序（QuickSort）
2. 编程语言：Rust
3. 功能要求：
   - 支持泛型，可以对任意实现了 Ord trait 的类型进行排序
   - 使用原地排序，减少内存开销

>> [多行内容正确显示在输入框中，不会覆盖上面的输出]
```

### 场景 2：历史记录选择
```bash
>> [按上方向键]
[从历史记录加载多行命令，正确显示，不会出现覆盖]
```

### 场景 3：接近终端底部
即使光标位置已经接近终端底部，填充多行内容时也会自动滚动创建空间，确保完整显示。

## 相关文件

- `app/src/app/repl.rs` - REPL 主循环和输入预填充逻辑
- `app/src/app/reedline_prompt.rs` - 多行提示符渲染
- `app/src/app/reedline_config.rs` - Reedline 配置和快捷键绑定

## 其他改进建议

如果问题仍然存在，可以考虑：

1. **调整缓冲行数**：在 `needed_lines` 计算中增加更多缓冲空间（目前是 +3）
2. **清屏方案**：对于非常长的多行内容，可以考虑清屏后再显示
3. **分页显示**：对超长内容提供分页或滚动查看功能

## 版本信息

- 修复版本：v0.1.3+
- Reedline 版本：0.28
- Crossterm 版本：0.27
