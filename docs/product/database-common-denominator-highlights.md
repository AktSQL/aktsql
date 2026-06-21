# 数据库最大公约数与 Akt 亮点功能

本文整理 MySQL、MariaDB、TiDB、PostgreSQL、MongoDB、SQL Server 的共同模型，将其转化为 AktSQL Database Management 的产品亮点和附加能力。

## 官方依据

- MySQL：连接、字符集/排序规则、`INFORMATION_SCHEMA`。  
  https://dev.mysql.com/doc/refman/8.4/en/connecting.html  
  https://dev.mysql.com/doc/refman/8.4/en/charset-connection.html  
  https://dev.mysql.com/doc/refman/8.4/en/information-schema.html
- MariaDB：`CREATE DATABASE`、字符集/排序规则、`INFORMATION_SCHEMA`。  
  https://mariadb.com/kb/en/create-database/  
  https://mariadb.com/kb/en/information-schema/
- TiDB：MySQL 兼容 SQL、`CREATE DATABASE`、`INFORMATION_SCHEMA`。  
  https://docs.pingcap.com/tidb/stable/sql-statement-create-database/  
  https://docs.pingcap.com/tidb/stable/information-schema/
- PostgreSQL：libpq 连接参数、database/schema、system catalogs。  
  https://www.postgresql.org/docs/current/libpq-connect.html  
  https://www.postgresql.org/docs/current/ddl-schemas.html  
  https://www.postgresql.org/docs/current/catalogs.html
- MongoDB：connection string、database/collection/document、indexes。  
  https://www.mongodb.com/docs/manual/reference/connection-string/  
  https://www.mongodb.com/docs/manual/core/databases-and-collections/  
  https://www.mongodb.com/docs/manual/indexes/
- SQL Server：database、schema、连接属性、加密和认证。  
  https://learn.microsoft.com/en-us/sql/relational-databases/databases/databases  
  https://learn.microsoft.com/en-us/sql/relational-databases/security/authentication-access/ownership-and-user-schema-separation  
  https://learn.microsoft.com/en-us/sql/connect/jdbc/setting-the-connection-properties

## 最大公约数模型

所有目标数据库都可以抽象为：

```text
Connection -> Namespace -> Container -> Object Group -> Object
```

映射关系：

| 数据库 | Namespace | Container | Object Group | Object |
|---|---|---|---|---|
| MySQL | database | 省略 | Tables / Views / Indexes / Routines / Triggers | table、view、index、routine |
| MariaDB | database | 省略 | Tables / Views / Indexes / Routines / Triggers / Sequences | table、view、index、routine、sequence |
| TiDB | database | 省略 | Tables / Views / Indexes / Placement / Cluster Metadata | table、view、index、placement policy |
| PostgreSQL | database | schema | Tables / Views / Sequences / Functions / Indexes / Extensions | table、view、sequence、function |
| MongoDB | database | collection | Documents / Indexes | document、index |
| SQL Server | database | schema | Tables / Views / Procedures / Functions / Indexes / Sequences | table、view、procedure、function |

产品结论：

- Akt 不把所有数据库硬塞成“表”。MongoDB 保留 collection/document 语义。
- Akt 明确区分 PostgreSQL/SQL Server 的 database 与 schema。
- Akt 将 MySQL/MariaDB/TiDB 归入 MySQL-family，但保留各自特性入口。
- Akt 对所有对象生成统一路径，用于导航、日志、确认和复制。

## 亮点一：统一对象路径

对象路径是 Akt 的核心导航资产：

```text
connection://namespace/container/group/object
```

示例：

- `prod-mysql://app_db/Tables/orders`
- `prod-pg://app_db/public/Tables/orders`
- `analytics-mongo://events/clicks/Documents`
- `corp-sqlserver://Sales/dbo/Tables/Orders`

价值：

- 任何对象都能复制路径。
- 日志和错误能精确指向对象。
- 危险操作确认不再只写“确定吗”，而是写完整目标。
- 跨数据库 UI 保持一致，但不抹平真实数据库概念。

## 亮点二：驱动感知对象树

左侧对象树不使用固定死菜单，而由驱动能力矩阵生成。

能力矩阵至少包含：

- 是否支持多 database。
- 是否支持 schema。
- 是否支持 table/view/index。
- 是否支持 sequence、extension、procedure、function。
- 是否支持 collection/document。
- 是否支持 database charset/collation。
- 是否支持 table options、partition、tablespace。

价值：

- MySQL 用户看到 database/table/view/routine。
- PostgreSQL 用户看到 database/schema/sequence/extension。
- MongoDB 用户看到 database/collection/document/index。
- SQL Server 用户看到 database/schema/procedure/function。
- 不支持的功能不伪装成按钮。

## 亮点三：真实连接画像

每个连接不只是“成功/失败”，而是有连接画像：

- `connect_latency_ms`：认证连接耗时。
- `roundtrip_latency_ms`：最小 ping/query 往返耗时。
- `metadata_latency_ms`：对象树刷新耗时。
- `server_version`：服务端版本。
- `encoding/collation`：连接字符集或排序规则。
- `capability_summary`：驱动能力摘要。

价值：

- 用户能快速判断连接慢在认证、网络还是元数据。
- 状态栏能显示真实延迟，不是装饰数字。
- 连接列表能按健康度排序。
- 未来可做“连接体检”功能。

## 亮点四：Schema 变更计划

数据库和表结构修改不会直接黑箱执行，而是生成 `Change Plan`：

```text
1. 修改 public.orders.status 的类型：varchar(64)
2. 新增 public.orders.source 字段：varchar(32)
3. 刷新 public.orders 元数据
```

每条变更带状态：

| 标记 | 含义 |
|---|---|
| `+` | 新增 |
| `~` | 修改 |
| `-` | 删除 |
| `!` | 高风险或驱动不完全支持 |
| `=` | 未变化 |

价值：

- 对齐 Navicat 类结构化设计体验。
- 保存前知道要执行什么。
- 高风险操作强确认。
- 失败时能定位到具体阶段和语句。

## 亮点五：数据库族策略

Akt 使用“数据库族 + 驱动特性”的产品策略。

### MySQL 系列

覆盖：

- MySQL
- MariaDB
- TiDB

共同能力：

- database 作为 namespace。
- `INFORMATION_SCHEMA` 作为基础元数据入口。
- charset/collation 作为数据库与连接重要属性。
- table/view/index/routine 作为基础对象。

差异能力：

- MariaDB 可出现 sequence 等扩展对象。
- TiDB 可暴露分布式集群、placement policy、region/cluster 状态等增强信息。
- MySQL 的 socket/TCP 行为需要在连接参数中明确。

### PostgreSQL 系列

覆盖：

- PostgreSQL
- CockroachDB
- Greenplum

共同能力：

- database 与 schema 分层。
- system catalogs 作为元数据核心。
- sequence、extension、function 是一等对象。
- search path 影响 SQL 上下文。

### 文档数据库系列

覆盖：

- MongoDB

共同能力：

- database/collection/document。
- index 是 collection 级对象。
- 查询不是 SQL 表格模型，但结果可以表格化展示。

### SQL Server 系列

覆盖：

- SQL Server
- SQL Server 2000 legacy mode

共同能力：

- server/instance -> database -> schema。
- stored procedure、function、sequence 是一等对象。
- 连接加密、trust certificate、认证方式是连接表单核心字段。

## 亮点六：上下文动作而非菜单堆叠

所有对象动作由当前对象类型决定：

| 对象 | 主要动作 |
|---|---|
| Connection | 测试连接、连接、新建数据库、刷新元数据 |
| Database | 详情、新建表、设计数据库、重命名、删除 |
| Schema | 新建表、新建函数、刷新 |
| Table | 浏览行、设计表、查看结构、导出结构报告、删除、截断 |
| Collection | 浏览文档、索引、重命名、删除 |
| Index | 详情、重建、删除 |
| Function/Procedure | 查看定义、编辑、删除 |

价值：

- 用户不会在无关菜单里找功能。
- 对象树右键、详情页动作条、顶部 New 菜单保持一致。
- 不支持动作不会出现在主要路径上。

## 亮点七：可审计执行日志

每次 schema 操作都产生执行记录：

- 目标对象路径。
- 执行动作。
- 生成结构化变更计划和驱动执行策略。
- 开始时间、结束时间、耗时。
- 元数据刷新耗时。
- 成功/失败。
- 错误码、错误阶段、是否部分执行。

价值：

- 方便排障。
- 支持复制结构化摘要和故障上下文。
- 支持后续做历史、回放、审计和变更报告。

## 产品表达

可以在官网或 README 中表达为：

> Akt 不是把数据库粗暴地压平成一个“表列表”。它先理解 MySQL、MariaDB、TiDB、PostgreSQL、MongoDB、SQL Server 的真实对象模型，再用统一工作台组织连接、对象、变更计划和执行反馈。用户获得一致的操作方式，同时保留每种数据库自己的语义。

短亮点：

- 跨数据库统一对象路径。
- 驱动感知对象树。
- 真实连接延迟画像。
- Schema 变更计划。
- 数据库族能力矩阵。
- 上下文动作系统。
- 可审计执行日志。
