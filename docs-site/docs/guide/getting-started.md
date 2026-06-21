# 快速开始

AktSQL 是使用 Rust 和 iced 构建的桌面数据库管理工具。

## 支持的数据库

- MySQL、MariaDB、TiDB
- PostgreSQL、CockroachDB
- MongoDB
- SQLite
- SQL Server
- Oracle

## 本地运行

```sh
cargo run -p aktsql
```

快速检查编译：

```sh
cargo check -p aktsql --all-targets
```

## 基本流程

1. 打开连接页面。
2. 选择数据库驱动，填写主机、端口、用户名、密码和数据库名。
3. 已有连接使用“连接”，新连接使用“保存并连接”。
4. 在数据库工作台展开数据库、数据表和字段。
5. 通过对象菜单打开表数据、表结构、数据库详情或查询工作台。

## 表结构设计

表结构编辑器分为：

- 字段
- 索引
- 约束
- 建表 SQL

结构变更通过直接操作提交。需要审阅或复用时，生成 SQL 保持可复制。
