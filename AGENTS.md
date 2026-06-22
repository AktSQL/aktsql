# Akt Agent 指南

## 1. 项目快照

本项目目标是成为 **AktSQL Database Management**。
产品短名为 `Akt`。

当前仓库状态：
- 仓库包含项目说明、OpenSpec 配置、Cursor 辅助资产、CodeGraph 元数据，以及初始 Rust 桌面应用脚手架。
- 已提交的实现技术栈是 Rust，桌面 UI 使用 iced。
- Workspace 已拆分为多个 crate：`crates/app` 是 iced 桌面入口，Cargo package 和二进制名称仍为 `aktsql`；`crates/engine` 承载共享模型、消息和应用引擎类型；`crates/sql` 承载 SQL/DDL 生成；`crates/db` 承载数据库驱动与元数据读取；`crates/ui` 承载 iced 视图、主题和 i18n re-export。

## 2. 产品说明

从 `README.md` 可确认的项目意图：
- 主 CLI 命令：`aktsql`
- 短 CLI 别名：暂未定义。
- 候选配置文件：
  - `.aktsql.toml`
  - `aktsql.config.json`
- 品牌：
  - 主视觉：红云
  - 配色：黑色与红色
  - 调性：神秘、强大
- 候选标语：`Dawn of Database Management`（数据库管理的黎明）、`Powerful as the Organization`（如组织般强大）

## 3. 仓库地图

重要路径：
- `README.md` - 当前产品说明。
- `Cargo.toml` - Rust workspace 配置。
- `crates/app/` - iced 桌面入口、应用状态机和副作用编排。
- `crates/engine/` - 连接配置、schema/query/workbench 领域模型、消息和导航枚举。
- `crates/sql/` - SQL、DDL 与数据库命令生成。
- `crates/db/` - 数据库连接测试、查询执行和元数据读取。
- `crates/ui/` - iced 视图、主题样式与多语言文案 re-export。
- `docs/product/aktsql-requirement-slices.md` - 从原型拆出的产品需求切片。
- `openspec/config.yaml` - OpenSpec 配置。
- `.cursor/commands/` 和 `.cursor/skills/` - Cursor 与 OpenSpec 辅助文件。
- `.codegraph/` - 本地 CodeGraph 索引文件。
- `AGENTS.md` - 本指南。

## 4. CodeGraph

本项目已配置 CodeGraph MCP server。
源代码存在后，结构性代码问题优先使用 CodeGraph。

优先使用这些工具：
- `codegraph_files` 查看项目文件结构。
- `codegraph_context` 获取宽范围任务上下文。
- `codegraph_search` 查找符号定义。
- `codegraph_node` 查看单个符号的签名或源码。
- `codegraph_callers`、`codegraph_callees`、`codegraph_trace` 和 `codegraph_impact` 分析调用关系与变更影响。

只有在查询字面文本、生成文件或 CodeGraph 未覆盖的文件时，才使用原生 shell 搜索。

## 5. OpenSpec 工作流

仓库通过 `openspec/config.yaml` 配置了规格驱动工作流。
实现重要功能前，应先澄清或创建 OpenSpec 变更。

以下场景使用 OpenSpec 风格工作：
- 添加第一个应用脚手架。
- 选择实现技术栈。
- 定义 CLI 命令和参数。
- 定义数据库连接行为。
- 添加配置文件语义。
- 引入持久化状态、插件或迁移。

小型文档编辑可以直接修改。

## 6. 实现规则

实现技术栈：
- Rust workspace。
- iced 桌面 UI。
- 遵循 Cargo workspace 约定。
- 随着项目增长，将 UI、应用状态、主题、数据库驱动、元数据、查询执行、SQL 工具和配置拆分到独立模块或 crate。
- 保持 Unix 风格边界：小模块、显式状态转换、文本友好配置，以及不绑定 UI 框架的可测试核心逻辑。

## 7. 编辑规则

遵循现有仓库形态，保持变更范围收敛。

- 优先做小而直接的编辑。
- 避免无关格式化 churn。
- 保留工作区中的用户改动。
- 修改结构化文件时使用结构化解析器或标准工具。
- 注释保持有用且克制。
- 除非文件本身需要非 ASCII 文本，否则使用 ASCII。

## 8. 验证

当前有用的检查命令：

```sh
cargo fmt --check
cargo check --workspace --all-targets
rg --files -uu
```

运行桌面应用：

```sh
cargo run -p aktsql
```

## 9. Git 状态

`.git/` 存在。保留用户改动，避免回退无关工作。

## 10. 首个脚手架检查清单

首个脚手架已选择 Rust + iced。实现数据库功能前，需要决定：
- 第一版支持的数据库引擎。
- 驱动 crate 策略与可选 feature 布局。
- 配置文件优先级。
- 安全凭据存储方案。
- 数据库集成测试策略。
