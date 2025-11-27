## Why
用户需要一个通用的订阅额度显示功能，能够监控各种订阅服务的剩余额度（如 Claude API 配额、云服务额度、SaaS 订阅等）。现有工具缺乏这种通用性，通常只针对特定服务。通过支持外部脚本执行，用户可以灵活地获取任何订阅服务的额度信息，并以直观的百分比+圆环形式显示。

## What Changes
- **新增 SubscriptionQuotaSegment**：一个新的段（segment），专门用于显示订阅额度信息
- **圆环视觉设计**：使用 Nerd Font 圆环图标，8个分段显示不同百分比区间（0-12%, 13-25%, 26-37%, 38-50%, 51-62%, 63-75%, 76-87%, 88-100%）
- **外部脚本支持**：通过配置 `custom_script_path` 选项，允许用户提供脚本路径，程序执行脚本获取额度数据
- **灵活脚本输出格式**：支持多种脚本输出格式（纯文本数字、JSON、键值对），用户可在配置中定义解析规则
- **默认关闭**：新段默认不启用，需用户在 TUI 配置中手动开启
- **TUI 配置界面**：在段配置中添加自定义选项，支持设置脚本路径、输出格式、解析规则等
- **缓存机制**：实现缓存机制避免频繁执行脚本，支持 `cache_duration` 配置
- **错误处理**：脚本执行失败或格式错误时优雅降级，显示 "-" 或使用缓存数据

## Impact
- **Affected Specs**:
  - `specs/segments/spec.md` - 添加新段类型定义和配置选项
- **Affected Code**:
  - `src/core/segments/` - 新增 `subscription_quota.rs`
  - `src/core/segments/mod.rs` - 注册新段类型
  - `src/config/types.rs` - 在 `SegmentId` 枚举中添加 `SubscriptionQuota`
  - `src/ui/components/` - 扩展段配置组件支持新选项
- **Breaking Changes**: 无破坏性变更
- **Dependencies**:
  - 需要 Nerd Font 支持圆环图标显示
  - 依赖外部脚本执行，可能需要考虑安全性（脚本路径验证）

## Migration Plan
1. 用户需要手动在 TUI 配置中启用 `SubscriptionQuota` 段
2. 用户需要提供自定义脚本路径和配置输出格式解析规则
3. 向后兼容：现有配置不受影响
