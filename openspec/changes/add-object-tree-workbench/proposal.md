# 添加对象树工作台

## 为什么

数据库工作区应像专注的 Unix 风格工具：左侧定位数据库对象，右侧对选中对象执行明确操作。当前在 Databases、Query Explorer 和 Tables 之间平铺拆分工作区，会产生重复导航，并让对象操作变得间接。

创建流程应以完整参数表单、校验、变更摘要和执行反馈作为主路径；破坏性操作则保留强确认。

## 变更内容

- 将平铺数据库导航模型替换为对象树：Connections -> Databases -> 对象组。
- 将 Query、History 和 Settings 作为对象树之外的工具保留。
- 连接或切换连接后，默认进入数据库工作台。
- 将创建数据库流程固定为通过已校验表单直接执行。
- 对破坏性 schema/数据操作保留确认。
- 将 schema 变更结果记录到状态/日志界面，而不是强制修改查询编辑器。

## 影响

- 受影响 specs：object-workbench
- 受影响代码：`crates/aktsql_app/src/app.rs`、`crates/aktsql_app/src/ui.rs`、`crates/aktsql_app/src/query.rs`、`docs/product/aktsql-requirement-slices.md`
