## 1. Core Implementation
- [x] 1.1 在 SegmentId 枚举中添加 SubscriptionQuota 变体
- [x] 1.2 创建 SubscriptionQuotaSegment 结构体
- [x] 1.3 实现 Segment trait 的 collect 方法
- [x] 1.4 实现外部脚本执行逻辑
- [x] 1.5 实现多格式输出解析器（纯文本、JSON、键值对）
- [x] 1.6 实现缓存机制（参考 UsageSegment）
- [x] 1.7 实现圆环图标选择逻辑

## 2. Configuration Integration
- [x] 2.1 在 segments/mod.rs 中注册新段类型
- [x] 2.2 创建默认配置（默认关闭）
- [x] 2.3 定义段选项结构（script_path、output_format、parse_rules 等）

## 3. TUI Configuration Interface
- [x] 3.1 扩展 segment_list 组件支持新段
- [x] 3.2 在段编辑器中添加自定义选项编辑
- [x] 3.3 创建脚本路径输入组件
- [x] 3.4 创建输出格式选择器
- [x] 3.5 创建解析规则编辑器

## 4. Theme Integration
- [x] 4.1 在默认主题中添加订阅额度段配置
- [x] 4.2 为所有预设主题添加该段（默认关闭）
- [x] 4.3 测试主题预设兼容性

## 5. Testing & Documentation
- [x] 5.3 编写文档说明自定义脚本的编写规范
- [x] 5.4 创建示例脚本和配置

## 6. Example Scripts & Validation
- [x] 6.1 创建示例脚本（检查 Claude API 配额）
- [x] 6.2 创建示例脚本（检查云服务额度）
- [x] 6.3 验证脚本执行的错误处理
- [x] 6.4 验证配置界面的易用性
