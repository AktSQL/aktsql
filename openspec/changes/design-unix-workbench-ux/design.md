# 设计

## 官方模型依据

本 UX 设计以官方文档中的实际数据库模型为约束：

- MySQL：连接、字符集/排序规则、`INFORMATION_SCHEMA` 元数据。MySQL 的 `localhost` 在 Unix 上可能使用 socket，因此连接 UI 必须允许用户明确 TCP/socket 行为。  
  https://dev.mysql.com/doc/refman/8.4/en/connecting.html  
  https://dev.mysql.com/doc/refman/8.4/en/charset-connection.html  
  https://dev.mysql.com/doc/refman/8.4/en/information-schema.html
- MariaDB：MySQL-family 模型，database、字符集/排序规则、`INFORMATION_SCHEMA`，并保留 MariaDB 扩展对象差异。  
  https://mariadb.com/kb/en/create-database/  
  https://mariadb.com/kb/en/information-schema/
- TiDB：MySQL 兼容 SQL 和 `INFORMATION_SCHEMA`，同时可暴露分布式集群、placement、region 等增强元数据。  
  https://docs.pingcap.com/tidb/stable/sql-statement-create-database/  
  https://docs.pingcap.com/tidb/stable/information-schema/
- PostgreSQL：libpq 连接参数、database/schema 层级、system catalogs。一个 PostgreSQL 连接以 database 为目标，schema 是 database 内的命名空间。  
  https://www.postgresql.org/docs/current/libpq-connect.html  
  https://www.postgresql.org/docs/current/ddl-schemas.html  
  https://www.postgresql.org/docs/current/catalogs.html
- MongoDB：连接字符串、database/collection/document 模型、collection 级索引。MongoDB 不能被伪装成纯 SQL schema，但可以映射到同一工作台对象模型。  
  https://www.mongodb.com/docs/manual/reference/connection-string/  
  https://www.mongodb.com/docs/manual/core/databases-and-collections/  
  https://www.mongodb.com/docs/manual/indexes/
- SQL Server：server/instance/database/schema 对象层级、连接属性、加密和认证选项。  
  https://learn.microsoft.com/en-us/sql/relational-databases/databases/databases  
  https://learn.microsoft.com/en-us/sql/relational-databases/security/authentication-access/ownership-and-user-schema-separation  
  https://learn.microsoft.com/en-us/sql/connect/jdbc/setting-the-connection-properties

## 最大公约数对象模型

统一对象模型：

```text
Connection -> Namespace -> Container -> Object Group -> Object
```

含义：

- `Connection`：一个可测试、可保存、可连接的目标。
- `Namespace`：连接下的第一层可浏览范围。不同数据库可映射为 database、instance database、MongoDB database。
- `Container`：对象组织边界。PostgreSQL/SQL Server 通常是 schema；MongoDB 可以是 collection；MySQL 没有显式 schema 时可省略这一层。
- `Object Group`：Tables、Views、Collections、Indexes、Functions、Procedures、Sequences、Extensions 等。
- `Object`：具体表、视图、集合、索引、函数、过程、序列等。

映射：

| 数据库 | Connection | Namespace | Container | Object Group | Object |
|---|---|---|---|---|---|
| MySQL | server profile | database | 省略或 database | Tables/Views/Indexes/Routines/Triggers | table/view/index/routine |
| MariaDB | server profile | database | 省略或 database | Tables/Views/Indexes/Routines/Triggers/Sequences | table/view/index/routine/sequence |
| TiDB | server profile | database | 省略或 database | Tables/Views/Indexes/Placement/Cluster Metadata | table/view/index/placement policy |
| PostgreSQL/CockroachDB | server profile + database | database | schema | Tables/Views/Sequences/Functions/Indexes/Extensions | table/view/sequence/function |
| MongoDB | URI/profile | database | collection | Documents/Indexes | document/index |
| SQL Server | server/instance profile | database | schema | Tables/Views/Stored Procedures/Functions/Indexes | table/view/procedure/function |

设计约束：

- UI 不把 MongoDB 强行叫做 table。
- UI 不把 PostgreSQL schema 和 database 混用。
- UI 不把 MySQL database rename 伪装成普通 `ALTER DATABASE RENAME`，后续实现必须按驱动能力处理。
- 对象路径必须可显示、可复制、可用于日志，例如 `prod.pg/public/orders`。

## 最大公约数亮点功能

从六类数据库模型中抽出的 Akt 产品亮点：

- **统一对象路径**：所有对象都能表达为 `connection://namespace/container/group/object`，用于导航、日志、确认和复制。
- **驱动感知对象树**：对象组由驱动能力矩阵生成，不把 MongoDB collection 叫成 table，也不混淆 PostgreSQL/SQL Server 的 database 和 schema。
- **真实连接画像**：连接结果包含认证延迟、往返延迟、元数据刷新延迟、服务端版本、编码/排序规则和驱动能力摘要。
- **Schema 变更计划**：创建和修改数据库/数据表前生成 Change Plan，标记新增、修改、删除、高风险和不支持项。
- **数据库族策略**：MySQL/MariaDB/TiDB 归为 MySQL Family，PostgreSQL/CockroachDB/Greenplum 归为 PostgreSQL Family，MongoDB 保留 Document Family，SQL Server 保留实例/database/schema 模型。
- **上下文动作系统**：对象树右键、详情页动作条、顶部 New 菜单使用同一动作定义，避免菜单堆叠。
- **可审计执行日志**：每次 schema 操作记录目标路径、SQL/命令、耗时、元数据刷新、错误阶段和是否部分执行。

## 自然操作流

左侧只做定位，右侧只做动作。

用户路径：

1. 在连接列表选择或新建连接。
2. 点击“测试连接”，得到真实连接结果和延迟分解。
3. 成功后进入数据库工作台，左侧显示对象树。
4. 单击对象：右侧显示详情。
5. 双击对象：执行默认打开动作。
6. 右键对象：显示该对象可执行动作。
7. 创建类动作打开右侧表单或轻量弹窗，校验后直接执行。
8. 危险动作必须显示目标完整路径、动作类型和不可逆提示，确认后执行。

默认动作：

| 对象 | 默认动作 |
|---|---|
| database/schema | 查看详情并展开 |
| table/view | 打开数据浏览 |
| collection | 打开 documents 浏览 |
| index | 查看索引详情 |
| function/procedure | 查看定义 |
| query tool | 打开 SQL/command 编辑器 |

危险动作：

- 删除 database/schema/table/collection。
- 清空 table/collection。
- 删除字段、索引、约束。
- 执行批量结构变更或不可逆数据变更。

确认文案必须命名目标，例如“确认删除 `prod.public.orders`”，不能只写“确定吗？”。

## 连接与延迟指标

连接测试不是装饰状态，必须测量并保存最近一次结果。

指标：

- `connect_latency_ms`：从发起连接到认证完成。
- `roundtrip_latency_ms`：认证完成后执行最小 ping/query 的耗时。
- `metadata_latency_ms`：刷新顶层对象树或元数据入口的耗时。
- `tested_at`：最近测试时间。
- `server_version`：可用时显示服务端版本。
- `connection_encoding`：可用时显示连接字符集/编码。

每类数据库的最小往返动作：

| 数据库 | 最小动作 |
|---|---|
| MySQL | `SELECT 1`，并读取连接字符集/排序规则 |
| PostgreSQL | `SELECT 1`，并读取 server version/current schema |
| MongoDB | `ping` command |
| SQL Server | `SELECT 1`，并读取 server/database 上下文 |

UI 呈现：

- 连接卡片显示：状态、最近连接延迟、最近测试时间。
- 顶栏显示：当前连接、database/schema、延迟摘要。
- 状态栏显示：rows、latency、encoding、driver、server version。
- 测试失败显示：错误阶段、错误码/错误类、可读消息。

## 连接参数 UX

表单分层：

- 基础：连接名、驱动、主机/URI/文件、端口、目标 database、用户名、密码引用、连接超时。
- 认证：密码、系统认证、domain、auth database、auth mechanism。
- 命名空间：schema/search path、catalog、instance、service name。
- 安全：SSL/TLS、加密、trust certificate、证书文件、证书校验策略。
- 网络：socket、SSH tunnel、proxy、keepalive。
- 高级：驱动参数、只读模式、init SQL、application name。

规则：

- 字段应按驱动动态显示，不适用字段不占主要视觉空间。
- 专属字段仍要可发现，避免高级功能藏死。
- 默认值应来自驱动模型，而不是 UI 文案硬编码。
- 密码不进入普通配置持久化。

## UI 结构计划

### 顶栏

- 左侧：Akt 品牌、当前工作区。
- 中间：当前连接路径，例如 `local-mysql / app_db`。
- 右侧：执行、刷新、新建、导出、设置等上下文动作。

### 左侧对象区

- 上半部分：对象树。
- 下半部分：工具入口，固定为 Query、History、Settings。
- 对象树保持层级稳定，展开不改变用户当前工作区。

### 右侧工作台

- 标题区：对象名、类型、路径、状态。
- 动作区：当前对象可执行动作。
- 内容区：详情、数据、表单、查询编辑器或结果。
- 日志区：最近动作、错误、延迟。

### 状态栏

固定显示：

- app version
- connection state
- 驱动
- 服务端版本
- 行数
- 延迟
- 编码 / 排序规则
- 光标上下文
- 日志入口

## 主题与字体设计

目标：不打包字体，降低发布包体积和内存占用；优先使用本地系统字体。

系统字体选择：

| 平台 | UI 字体优先级 | 等宽字体优先级 |
|---|---|---|
| macOS | PingFang SC -> Apple system UI fallback | Menlo -> SF Mono -> system monospace |
| Windows | Microsoft YaHei -> Segoe UI fallback | Cascadia Mono -> Consolas -> system monospace |
| Linux | Noto Sans CJK -> WenQuanYi Micro Hei -> DejaVu Sans -> system sans | Noto Sans Mono CJK -> DejaVu Sans Mono -> system monospace |

实现约束：

- 应用启动时发现可用字体族。
- 设置页显示“当前实际 UI 字体”和“当前实际等宽字体”。
- 字体不存在时回退，不阻塞启动。
- 不再把 CJK/Latin/Arabic 字体作为二进制资产打包。

不同 theme 的字体/密度策略：

| Theme | UI 字重 | 行高 | 表格密度 | 等宽字体使用 |
|---|---|---|---|---|
| 暗色 | 中等字重，避免细字发虚 | 紧凑 | Compact | SQL、数据、元数据标签优先等宽 |
| 亮色 | 常规字重 | 标准 | Normal | SQL 和数据使用等宽，导航少用等宽 |
| 高对比 | 较强字重 | 标准 | Comfortable | 状态、错误、SQL 使用等宽强化识别 |

## 分步落地策略

1. 先补规格，不动代码。
2. 实现系统字体发现，移除内置字体依赖。
3. 扩展连接测试结果模型，记录真实延迟。
4. 重构连接表单字段分层和驱动专属字段。
5. 统一对象路径模型和对象树节点类型。
6. 调整右侧工作台，让详情、表单、查询、日志共享一致布局。
7. 把状态栏接入真实 driver/server/latency/encoding 数据。
8. 按 MySQL、PostgreSQL、MongoDB、SQL Server 分别补元数据适配器。

## 设计取舍

- 先做最大公约数，不追求四类数据库所有高级功能一次完成。
- MongoDB 保持 document/collection 语义，不强行 SQL 化。
- SQL 数据库保持 database/schema/table 语义，不用泛化词遮蔽真实模型。
- 字体使用系统发现，牺牲完全一致的视觉，换取更小包体、更低内存和更自然的平台体验。
- 延迟指标真实测量，避免“连接成功”只是 UI 状态。
