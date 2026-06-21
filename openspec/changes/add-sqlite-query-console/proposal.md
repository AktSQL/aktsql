## 为什么

Akt 已有连接管理器和真实 SQLite 连接测试，但 Query Explorer 仍像占位界面。下一个产品切片应提供第一条端到端查询工作流：编辑 SQL、执行 SQL、查看结果。

## 变更内容

- 添加查询控制台能力，包含 SQL 草稿状态、执行反馈和结果显示。
- 使用现有 `rusqlite` 依赖，对当前 SQLite 连接配置执行 SQL。
- 用按原型建模的桌面控制台界面替换 Query Explorer 占位。
- 在添加驱动专属执行层前，明确不支持非 SQLite 执行。

## 能力

### 新增能力

- `query-console`：SQL 编辑、SQLite 执行，以及结果/消息显示。

### 修改能力

- 无。

## 影响

- 受影响代码：`crates/aktsql_app/src/app.rs`、`crates/aktsql_app/src/main.rs`、`crates/aktsql_app/src/ui.rs`，以及新的聚焦 query 模块。
- 受影响 UI：Query Explorer 变为可工作的 SQL 控制台界面。
- 依赖：使用现有 `rusqlite`。
