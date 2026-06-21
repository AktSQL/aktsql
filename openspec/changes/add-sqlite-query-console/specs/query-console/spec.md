## ADDED Requirements

### Requirement: SQL 查询状态

系统 SHALL 将 SQL 控制台状态与 iced 渲染分离建模。

#### Scenario: SQL 草稿被编辑

- **WHEN** 用户修改 SQL 输入
- **THEN** 应用保存 SQL 草稿，且不改变连接配置字段

### Requirement: SQLite 查询执行

系统 SHALL 针对当前 SQLite 连接配置执行 SQL。

#### Scenario: 用户执行返回行的 SQLite 查询

- **WHEN** 当前连接驱动是 SQLite
- **AND** SQL 语句返回行
- **THEN** 系统打开 SQLite 数据库文件，并显示结果列、结果行、耗时和行数

#### Scenario: 用户执行不返回结果行的 SQLite 语句

- **WHEN** 当前连接驱动是 SQLite
- **AND** SQL 语句是不返回结果行的 DDL 或 DML
- **THEN** 系统报告影响行数和耗时

#### Scenario: 用户执行空 SQL

- **WHEN** SQL 草稿为空或只有空白
- **THEN** 系统报告校验错误，且不执行数据库语句

#### Scenario: 用户执行非 SQLite 配置

- **WHEN** 当前连接驱动不是 SQLite
- **THEN** 系统报告该驱动尚未接入查询执行

### Requirement: 查询控制台 UI

系统 SHALL 在 Query Explorer 工作区提供 iced 查询控制台视图。

#### Scenario: 用户打开 Query Explorer

- **WHEN** 选择 Query Explorer
- **THEN** 工作区显示活动连接上下文、SQL 输入、执行操作、结果预览和执行消息

#### Scenario: 查询结果更新全局状态

- **WHEN** 查询执行成功完成
- **THEN** 状态栏显示最新行数和耗时

### Requirement: SQLite 结构浏览器

系统 SHALL 按需为活动连接加载 SQLite schema 对象。

#### Scenario: 用户刷新 Query Explorer schema

- **WHEN** 当前连接驱动是 SQLite
- **AND** 用户刷新 Query Explorer schema
- **THEN** 系统从 `sqlite_master` 列出 tables、views 和 indexes

#### Scenario: 用户选择 schema 对象

- **WHEN** 用户从 schema browser 选择 table 或 view
- **THEN** SQL 编辑器填入该对象的 preview query

#### Scenario: 用户为不支持驱动刷新 schema

- **WHEN** 当前连接驱动不是 SQLite
- **THEN** 系统报告该驱动尚未接入 schema browsing

### Requirement: 响应式查询操作

系统 SHALL 在数据库工作进行时保持 Query Explorer 交互响应。

#### Scenario: 查询执行启动

- **WHEN** 用户启动查询执行
- **THEN** 应用立即回到事件循环，并在 task 完成后应用结果

#### Scenario: Schema 刷新启动

- **WHEN** 用户启动 schema 刷新
- **THEN** 应用立即回到事件循环，并在 task 完成后应用 schema

#### Scenario: 操作已在运行

- **WHEN** 查询执行或 schema 刷新已在运行
- **THEN** 重复点击不会再次入队相同操作
