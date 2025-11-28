# Subscription Quota Text 格式说明

## 概述

`subscription_quota` 的 text 格式提供**完全自定义**的文本显示功能，用户可以自由输出任何格式的文本内容，系统会直接显示而不做任何解析或修改。

## 功能特性

### 1. 直接文本输出

Text 格式会**原样显示**用户脚本的输出内容，不做任何解析：

```bash
echo "██████ 45% | 2000/4500 | reset 4h50m"
```

显示效果：
```
██████ 45% | 2000/4500 | reset 4h50m
```

### 2. 任意格式支持

用户可以输出任何自定义格式：

#### 进度条格式
```bash
echo "██████ 45% | 2000/4500 tokens"
```

#### 带颜色代码
```bash
echo -e "\033[32m✓ 正常\033[0m - 45% used"
```

#### 详细信息
```bash
echo "MiniMax-M2: 2000/4500 (45%) | Reset in 4h50m"
```

#### 纯文本
```bash
echo "No quota limit"
```

### 3. 无限制自定义

- 支持任意字符和符号
- 支持 ANSI 颜色代码
- 支持特殊 Unicode 字符（如图标、进度条等）
- 支持多行内容（会显示为一行）
- 脚本输出什么就显示什么

## 配置方法

### 1. 启用 Subscription Quota 段

在配置界面中启用 "Subscription Quota" 段。

### 2. 设置输出格式为 "text"

在段配置选项中设置：
- `output_format`: `text`

### 3. 配置自定义脚本路径

设置：
- `custom_script_path`: `/path/to/your/script.sh`

### 4. 其他可选设置

- `cache_duration`: 缓存时间（秒，默认 300）
- `timeout`: 脚本超时时间（秒，默认 5）

## 示例脚本

参考 `openspec/changes/add-subscription-quota-segment/example-scripts/query_minmax_remains.sh`

这个脚本展示了如何：
1. 查询 MiniMax API
2. 计算使用百分比
3. 生成带进度条的彩色输出
4. 输出自定义格式化文本

脚本输出示例：
```bash
#!/usr/bin/env bash
# ... 计算逻辑 ...
echo "██████ 45% | 2000/4500 | reset 4h50m"
```

## 对比其他格式

| 格式 | 说明 | 显示方式 |
|------|------|----------|
| **text** | 自定义文本 | 直接显示脚本输出 |
| **json** | 结构化数据 | 解析 JSON，显示百分比 + 环形图标 |
| **key_value** | 键值对 | 解析 percentage 字段，显示百分比 + 环形图标 |

## 技术实现

修改文件：`src/core/segments/subscription_quota.rs`

主要变更：
1. Text 格式不再进行任何解析
2. 直接将脚本输出作为 `primary` 文本显示
3. 不显示环形图标（保持显示简洁）

## 注意事项

1. **无需百分比**: 不需要脚本输出包含百分比数字
2. **完全自定义**: 可以输出任何文本内容
3. **缓存有效**: 缓存机制仍然工作，避免频繁执行脚本
4. **颜色支持**: 支持 ANSI 颜色码（如果终端支持）
5. **长度注意**: 过长的文本可能会被状态栏截断
