## ADDED Requirements

### Requirement: 最大公约数对象模型
系统 SHALL 使用统一对象模型表达 MySQL、PostgreSQL、MongoDB 和 SQL Server 的可浏览对象层级。

#### Scenario: 渲染数据库对象树
- **WHEN** 用户连接到受支持数据库
- **THEN** 对象树使用 `Connection -> Namespace -> Container -> Object Group -> Object` 的逻辑模型
- **AND** MySQL 类数据库将 database 映射为 namespace
- **AND** PostgreSQL 类数据库将 database 映射为 namespace，并将 schema 映射为 container
- **AND** MongoDB 将 database 映射为 namespace，并将 collection 映射为 container
- **AND** SQL Server 将 database 映射为 namespace，并将 schema 映射为 container

### Requirement: 自然工作台操作流
系统 SHALL 让左侧对象树负责定位对象，右侧工作台负责对当前对象执行上下文操作。

#### Scenario: 用户选择对象
- **WHEN** 用户单击对象树中的对象
- **THEN** 右侧工作台显示该对象的详情、路径、状态和可用动作
- **AND** 左侧对象树不切换成无关工具页面

#### Scenario: 用户打开对象默认动作
- **WHEN** 用户双击 table、view、collection、index、function 或 procedure
- **THEN** 系统执行该对象类型的默认打开动作
- **AND** 默认动作不修改查询编辑器内容，除非用户明确选择“生成查询”类动作

#### Scenario: 用户执行危险操作
- **WHEN** 用户请求删除、清空或破坏性结构变更
- **THEN** 系统显示强确认
- **AND** 确认内容包含目标对象完整路径和具体动作
- **AND** 用户确认前不得执行该操作

### Requirement: 驱动感知连接表单
系统 SHALL 以通用字段加驱动专属字段的方式呈现连接参数。

#### Scenario: 用户切换数据库驱动
- **WHEN** 用户在连接表单中切换 MySQL、PostgreSQL、MongoDB 或 SQL Server
- **THEN** 表单保留通用字段
- **AND** 表单显示该驱动适用的专属字段
- **AND** 不适用字段不占据主要视觉空间
- **AND** 默认值来自驱动模型而不是 UI 文案硬编码

#### Scenario: 用户保存连接配置
- **WHEN** 用户保存连接配置
- **THEN** 系统保存非敏感连接元数据
- **AND** 密码、token、私钥不得进入普通配置文件

### Requirement: 真实连接延迟指标
系统 SHALL 在连接测试和状态显示中使用真实测量的延迟指标。

#### Scenario: 用户测试连接
- **WHEN** 用户点击测试连接
- **THEN** 系统测量连接认证延迟
- **AND** 系统测量最小 ping/query 往返延迟
- **AND** 支持时系统测量元数据刷新延迟
- **AND** 测试结果记录最近测试时间和失败阶段

#### Scenario: 用户查看连接状态
- **WHEN** 连接测试成功或工作台连接处于活动状态
- **THEN** 连接卡片、顶栏或状态栏显示真实延迟摘要
- **AND** 状态栏可显示 driver、server version、encoding/collation、rows 和 latency

### Requirement: 系统字体发现
系统 SHALL 从本地系统选择 UI 字体和等宽字体，而不是打包字体资产。

#### Scenario: 应用启动并选择字体
- **WHEN** 应用在 macOS 启动
- **THEN** UI 字体优先使用 PingFang SC，等宽字体优先使用 Menlo 或 SF Mono
- **WHEN** 应用在 Windows 启动
- **THEN** UI 字体优先使用 Microsoft YaHei，等宽字体优先使用 Cascadia Mono 或 Consolas
- **WHEN** 应用在 Linux 启动
- **THEN** UI 字体优先使用 Noto Sans CJK、WenQuanYi Micro Hei、DejaVu Sans 或系统 sans，等宽字体优先使用 Noto Sans Mono CJK、DejaVu Sans Mono 或系统 monospace

#### Scenario: 首选字体不存在
- **WHEN** 系统缺少首选字体
- **THEN** 应用回退到下一个可用字体或系统默认字体
- **AND** 应用不得因字体缺失而启动失败

### Requirement: 主题字体与密度差异
系统 SHALL 允许不同 theme 使用不同字体、字重、行高和密度策略。

#### Scenario: 用户切换主题
- **WHEN** 用户切换暗色、亮色或高对比 theme
- **THEN** UI 使用该 theme 的字体策略、表格密度和行高策略
- **AND** SQL 编辑器、数据表格和元数据标签可优先使用等宽字体
- **AND** 导航和表单文案保持系统 UI 字体优先

### Requirement: 官方模型约束
系统 SHALL 避免用泛化 UI 术语掩盖数据库真实模型。

#### Scenario: 显示不同数据库对象
- **WHEN** 当前驱动是 MongoDB
- **THEN** UI 使用 database、collection、document、index 等 MongoDB 语义
- **WHEN** 当前驱动是 PostgreSQL 或 SQL Server
- **THEN** UI 明确区分 database 与 schema
- **WHEN** 当前驱动是 MySQL
- **THEN** UI 明确区分 database、table、view、routine、trigger 等对象
