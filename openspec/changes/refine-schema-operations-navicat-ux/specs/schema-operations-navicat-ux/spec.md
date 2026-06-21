## ADDED Requirements

### Requirement: Schema 操作入口统一
系统 SHALL 为新建数据库、修改数据库、新建数据表、修改数据表提供一致且可发现的操作入口。

#### Scenario: 用户从对象树进入 schema 操作
- **WHEN** 用户右键连接、数据库、schema、Tables group 或 table 节点
- **THEN** 上下文菜单显示该对象支持的新建、设计、重命名、修改或查看动作
- **AND** 不支持的动作不会伪装成可执行动作

#### Scenario: 用户从工作台动作条进入 schema 操作
- **WHEN** 用户正在查看数据库或数据表详情
- **THEN** 右侧工作台动作条提供当前对象适用的修改、设计、结构摘要、变更计划或更多动作

### Requirement: Schema 设计器像素级布局
系统 SHALL 在固定 1280x800 内容窗口内使用稳定的 schema 设计器布局，避免后续实现漂移。

#### Scenario: 打开 schema 设计器
- **WHEN** 用户打开新建数据库、修改数据库、新建数据表或修改数据表界面
- **THEN** 顶栏高度为 52px
- **AND** 状态栏高度为 30px
- **AND** 左侧对象树宽度为 240px
- **AND** 右侧工作台宽度为 1040px
- **AND** 工作台头部高度为 56px
- **AND** 动作条高度为 44px
- **AND** tab 条高度为 36px
- **AND** 内容区和变更计划区使用固定分割，不因动态文案改变整体布局

#### Scenario: 显示 schema 设计器表格
- **WHEN** 字段、索引或约束表格列数超过可用宽度
- **THEN** 表格保持横向滚动
- **AND** 列宽不得被压缩到低于设计规格
- **AND** 表头高度为 30px
- **AND** 表格行高为 28px

### Requirement: 新建数据库 UX
系统 SHALL 通过驱动感知表单创建数据库，并在成功后保留用户上下文。

#### Scenario: 用户创建数据库
- **WHEN** 用户触发新建数据库
- **THEN** 系统打开包含 General、Encoding 和 Options 分区的表单
- **AND** 表单带入目标连接和驱动默认值
- **AND** 表单提供校验、变更摘要和执行结果作为辅助审阅
- **AND** 用户点击 Create 后系统直接执行创建操作
- **AND** 成功后对象树刷新并选中新数据库

### Requirement: 修改数据库 UX
系统 SHALL 通过数据库设计器修改数据库属性，并在执行前展示变更计划。

#### Scenario: 用户修改数据库属性
- **WHEN** 用户打开数据库设计器
- **THEN** 系统加载实时数据库元数据
- **AND** 设计器提供 General、Encoding、Objects、Change Plan 和 History 分区
- **AND** 用户修改名称、字符集、排序规则或支持的数据库属性时，系统生成 Change Plan
- **AND** Apply Changes 前系统展示变更摘要
- **AND** 高风险或不可逆变更需要额外确认

### Requirement: 新建数据表 UX
系统 SHALL 通过结构化表设计器创建数据表，而不是要求用户先写 SQL。

#### Scenario: 用户创建数据表
- **WHEN** 用户从 database、schema 或 Tables group 触发新建数据表
- **THEN** 系统打开表设计器并带入目标上下文
- **AND** 设计器提供 Fields、Indexes、Constraints、Options、Change Plan 和 Validation 分区
- **AND** 字段类型使用驱动感知选项
- **AND** 用户只审阅结构化变更计划、风险、校验和执行结果
- **AND** 设计器不得展示、复制或要求用户审阅底层执行文本
- **AND** 创建成功后对象树刷新并选中新表

### Requirement: 修改数据表 UX
系统 SHALL 通过内嵌表设计器修改表结构，并清晰标记每个变更。

#### Scenario: 用户修改表结构
- **WHEN** 用户打开表设计器修改已有数据表
- **THEN** 系统加载实时字段、索引、约束和表选项
- **AND** 每个结构变化标记为新增、修改、删除、未变化、高风险或不支持
- **AND** Change Plan 按执行顺序展示将执行的操作
- **AND** 用户可以回退全部变更或回退选中变更
- **AND** 设计器不得展示、复制或要求用户审阅底层执行文本
- **AND** Apply Changes 前系统要求确认高风险操作

### Requirement: Schema 操作执行反馈
系统 SHALL 为 schema 操作提供统一执行反馈和错误信息。

#### Scenario: Schema 操作执行成功
- **WHEN** 新建或修改数据库/数据表成功
- **THEN** 系统显示执行状态、结构动作数量、耗时、元数据刷新耗时和目标对象路径
- **AND** 系统提供查看日志、刷新元数据、打开对象的后续动作

#### Scenario: Schema 操作执行失败
- **WHEN** 新建或修改数据库/数据表失败
- **THEN** 系统显示失败阶段、数据库错误码或错误类、可读错误消息
- **AND** 系统说明是否存在部分执行
- **AND** 系统提供刷新元数据、回退草稿或重试入口
