# 示例脚本和使用指南

本目录包含订阅额度段的示例脚本和配置参考。

## 示例脚本

### 1. 检查 Claude API 配额 (纯文本格式)

**脚本文件**: `check_claude_quota.sh`

```bash
#!/bin/bash
# 检查 Claude API 剩余配额百分比
# 返回格式: 纯文本数字 (0-100)

# 从 ~/.claude/settings.json 获取 API 密钥
API_KEY=$(grep -o '"claude_api_key": "[^"]*"' ~/.claude/settings.json | cut -d'"' -f4)

# 调用 Anthropic API 获取使用情况
# 注意：这里只是一个示例，实际 API 端点可能不同
RESPONSE=$(curl -s -H "Authorization: Bearer $API_KEY" \
  "https://api.anthropic.com/v1/organizations/me/usage" 2>/dev/null)

if [ $? -eq 0 ]; then
  # 解析 JSON 获取配额百分比
  # 假设返回格式: {"credits_used": 750, "credits_total": 1000}
  PERCENTAGE=$(echo "$RESPONSE" | grep -o '"credits_used":[0-9]*' | cut -d':' -f2)
  TOTAL=$(echo "$RESPONSE" | grep -o '"credits_total":[0-9]*' | cut -d':' -f2)

  if [ -n "$PERCENTAGE" ] && [ -n "$TOTAL" ] && [ "$TOTAL" -gt 0 ]; then
    # 计算已使用百分比
    USED=$((PERCENTAGE * 100 / TOTAL))
    # 计算剩余百分比
    REMAINING=$((100 - USED))
    echo "$REMAINING"
  else
    echo "Error: Unable to parse API response"
    exit 1
  fi
else
  echo "Error: API request failed"
  exit 1
fi
```

**配置设置**:
```toml
# 在配置文件中添加
[[segments]]
id = "subscription_quota"
enabled = true
[segments.options]
custom_script_path = "/path/to/check_claude_quota.sh"
output_format = "text"
cache_duration = 300  # 5分钟
timeout = 10
```

### 2. 检查 AWS 账户余额 (JSON 格式)

**脚本文件**: `check_aws_billing.sh`

```bash
#!/bin/bash
# 检查 AWS 账户余额百分比
# 返回格式: JSON

# 使用 AWS CLI 获取计费信息
aws ce GetCostAndUsage \
  --time-period Start=2024-01-01,End=2024-12-31 \
  --granularity MONTHLY \
  --metrics BlendedCost \
  --group-by Type=DIMENSION,Key=Service \
  --profile default \
  --query 'ResultsByTime[0].Groups[?Metrics.BlendedCost.Amount != `0`].[Metrics.BlendedCost.Amount]' \
  --output text 2>/dev/null | awk '{sum += $1} END {
    # 假设预算为 $100
    budget = 100
    used = sum
    remaining_percent = ((budget - used) / budget) * 100
    if (remaining_percent < 0) remaining_percent = 0
    printf "{\"percentage\": %.1f, \"remaining\": \"$%.2f\", \"total\": \"$%.2f\"}\n", remaining_percent, budget - used, budget
  }'
```

**配置设置**:
```toml
[[segments]]
id = "subscription_quota"
enabled = true
[segments.options]
custom_script_path = "/path/to/check_aws_billing.sh"
output_format = "json"
cache_duration = 3600  # 1小时
timeout = 30
```

### 3. 检查 GitHub Actions 使用量 (键值对格式)

**脚本文件**: `check_github_actions.sh`

```bash
#!/bin/bash
# 检查 GitHub Actions 使用百分比
# 返回格式: 键值对

# 从环境变量或配置文件获取 GitHub Token
GITHUB_TOKEN="${GITHUB_TOKEN}"

if [ -z "$GITHUB_TOKEN" ]; then
  echo "Error: GITHUB_TOKEN not set"
  exit 1
fi

# 获取使用统计
USAGE=$(curl -s -H "Authorization: token $GITHUB_TOKEN" \
  "https://api.github.com/user" 2>/dev/null)

if [ $? -eq 0 ]; then
  # 解析响应（示例逻辑）
  # 实际 GitHub API 可能需要不同的端点
  echo "percentage=65"
else
  echo "Error: Unable to fetch usage"
  exit 1
fi
```

**配置设置**:
```toml
[[segments]]
id = "subscription_quota"
enabled = true
[segments.options]
custom_script_path = "/path/to/check_github_actions.sh"
output_format = "key_value"
cache_duration = 1800  # 30分钟
timeout = 15
```

## 高级配置

### 自定义解析规则

对于更复杂的输出格式，你可以在配置中定义解析规则：

```toml
[[segments]]
id = "subscription_quota"
enabled = true
[segments.options]
custom_script_path = "/path/to/custom_script.sh"
output_format = "json"
parse_rules = '''
{
  "percentage_field": "data.quota.remaining_percent",
  "regex_pattern": "剩余量[:：]\\s*(\\d+)",
  "default_percentage": 100
}
'''
cache_duration = 300
timeout = 10
```

### 安全注意事项

1. **脚本权限**: 确保脚本文件有执行权限 (`chmod +x script.sh`)
2. **脚本路径**: 建议使用绝对路径
3. **敏感信息**: 不要在脚本中硬编码 API 密钥，使用环境变量或密钥管理服务
4. **超时设置**: 为长时间运行的脚本设置合理的超时时间
5. **错误处理**: 确保脚本有适当的错误处理和日志记录

### 故障排除

**问题**: 脚本执行但返回空值
- **解决方案**: 检查脚本权限和路径是否正确

**问题**: 脚本超时
- **解决方案**: 增加 `timeout` 配置值或优化脚本性能

**问题**: 解析错误
- **解决方案**: 验证输出格式是否与配置的 `output_format` 匹配

**问题**: 频繁执行脚本
- **解决方案**: 增加 `cache_duration` 值

## 在 TUI 配置中设置

1. 运行 `ccline --config` 打开 TUI 配置界面
2. 导航到"段列表"(Segment List)
3. 选择"Subscription Quota"段
4. 按 `Enter` 进入段配置
5. 启用段 (Enabled: Yes)
6. 在"自定义选项"中设置：
   - Script Path: 你的脚本路径
   - Output Format: text/json/key_value
   - Cache Duration: 缓存时间（秒）
   - Timeout: 超时时间（秒）
7. 保存并退出

## 注意事项

- 该段默认是关闭的，需要手动启用
- 确保脚本返回的百分比在 0-100 范围内
- 脚本会在每次状态行更新时检查缓存是否过期
- 如果脚本执行失败，段将显示 "-" 并使用缓存数据（如果可用）
