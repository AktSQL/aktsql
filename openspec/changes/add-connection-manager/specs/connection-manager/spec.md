## ADDED Requirements

### Requirement: 连接配置模型

系统 SHALL 使用类型化驱动选择和显式参数字段建模数据库连接配置，而不是使用不透明连接字符串。

#### Scenario: 初始支持驱动可用

- **WHEN** 显示连接管理器
- **THEN** 用户可以选择 MySQL、MariaDB、TiDB、OceanBase、PostgreSQL、CockroachDB、Greenplum、SQLite、DuckDB、SQL Server、SQL Server 2000、Oracle、Db2、Informix、Sybase ASE、Firebird、ClickHouse、Redis、MongoDB、Cassandra、Elasticsearch、Snowflake、BigQuery、Redshift、Trino、Hive 或 Databricks 作为连接驱动

#### Scenario: 应用驱动默认值

- **WHEN** 用户切换连接驱动
- **THEN** 表单更新该驱动的端口、主机/路径、字符集和排序规则默认值

### Requirement: 连接参数完整性

系统 SHALL 暴露专业数据库客户端所需的基线连接参数：配置名称、主机或文件路径、端口、用户名、密码、数据库名、字符集、排序规则、SSL 开关、SSH 隧道开关、超时和备注。

#### Scenario: MySQL 默认字符集和排序规则

- **WHEN** 创建 MySQL 连接配置
- **THEN** 默认字符集为 `utf8mb4`
- **THEN** 默认排序规则为 `utf8mb4_bin`

#### Scenario: SQLite 使用文件路径

- **WHEN** 选择 SQLite 作为驱动
- **THEN** 表单将位置字段视为数据库文件路径，而不是网络主机

### Requirement: 本地连接校验

系统 SHALL 在报告配置本地有效或保存前校验连接表单输入。

#### Scenario: 拒绝缺失必填值

- **WHEN** 用户测试或保存缺少配置名称或主机/路径的连接配置
- **THEN** 系统报告校验错误，并且不将配置标记为有效

#### Scenario: 拒绝无效端口

- **WHEN** 用户为网络数据库驱动输入非数字或超出范围的端口
- **THEN** 系统报告端口校验错误

### Requirement: 连接管理器 UI

系统 SHALL 在 Databases 工作区提供 iced 桌面连接管理器视图。

#### Scenario: 用户打开 Databases 工作区

- **WHEN** 选择 Databases section
- **THEN** 工作区显示连接列表和可编辑连接表单

#### Scenario: 用户测试本地配置

- **WHEN** 用户点击 Test
- **THEN** 应用校验当前表单，并报告配置是否本地有效

#### Scenario: 用户测试 SQLite 配置

- **WHEN** 用户对有效 SQLite 配置点击 Test
- **THEN** 应用打开 SQLite 数据库文件，并报告真实连接结果

#### Scenario: 用户保存配置

- **WHEN** 用户点击 Save
- **THEN** 应用校验当前表单，并在校验成功时将配置保存到内存
