## ADDED Requirements

### Requirement: 固定原生窗口主壳
系统 SHALL 在固定尺寸桌面窗口中呈现 Akt，同时保留标准原生关闭和最小化行为。

#### Scenario: 应用窗口打开
- **WHEN** 桌面应用启动
- **THEN** 窗口使用 Akt 应用标题
- **THEN** 窗口以固定产品尺寸打开
- **THEN** 窗口保持原生装饰启用

#### Scenario: 用户最小化窗口
- **WHEN** 用户触发原生最小化控件
- **THEN** 操作系统收到标准最小化请求

### Requirement: 有界连接管理器布局
系统 SHALL 约束连接管理器列表、表单、校验消息和状态文本，使其适配固定应用窗口。

#### Scenario: 用户打开连接管理器
- **WHEN** 选择 Databases 工作区
- **THEN** 配置列表有明确固定宽度
- **THEN** 可编辑表单有明确最大宽度
- **THEN** 垂直溢出的表单内容仍可通过滚动访问

#### Scenario: 显示长配置或驱动标签
- **WHEN** 配置名称、目标、驱动标签、校验消息或状态消息长于可用容器
- **THEN** UI 显示有界短标签，而不是拉伸容器

### Requirement: 语言就绪文本边界
系统 SHALL 在引入运行时语言切换前，将活跃 shell 和连接管理器的面向用户文案保存在集中化文本目录中。

#### Scenario: 渲染 shell 文本
- **WHEN** 渲染顶部导航、侧边导航、状态文本、连接表单标签或连接操作
- **THEN** 文本从 i18n 文本目录或类型化领域标签读取

### Requirement: 稳定连接配置持久化
系统 SHALL 将已保存连接配置持久化到稳定 Akt 配置文件，且不存储密码。

#### Scenario: 配置被保存
- **WHEN** 用户保存或删除连接配置
- **THEN** 应用将连接配置列表写入首选 Akt 配置路径
- **THEN** 已保存配置数据不包含密码字段

#### Scenario: 存在既有开发配置
- **WHEN** 首选 Akt 配置路径为空，且工作目录中存在 `aktsql.config.json`
- **THEN** 应用将工作目录配置作为 fallback 加载
