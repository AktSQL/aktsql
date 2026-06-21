# AktSQL Database Management 原型需求切片

本文从 `stitch_aktsql_database_management/` 下的原型反推出产品需求，并把需求切成小块，方便后续进入 OpenSpec change、任务排期和实现验收。

## 读取范围

已完整检查以下原型文件：

- `stitch_aktsql_database_management/aktsql_project_specs.md`
- `stitch_aktsql_database_management/crimson_horizon/DESIGN.md`
- `stitch_aktsql_database_management/aktsql_console_dark_mode/code.html`
- `stitch_aktsql_database_management/aktsql_console_dark_mode/screen.png`
- `stitch_aktsql_database_management/aktsql_console_light_mode/code.html`
- `stitch_aktsql_database_management/aktsql_console_light_mode/screen.png`
- `stitch_aktsql_database_management/aktsql_connection_manager/code.html`
- `stitch_aktsql_database_management/aktsql_connection_manager/screen.png`
- `stitch_aktsql_database_management/aktsql_appearance_settings/code.html`
- `stitch_aktsql_database_management/aktsql_appearance_settings/screen.png`
- `stitch_aktsql_database_management/aktsql_logo/code.html`
- `stitch_aktsql_database_management/aktsql_logo/screen.png`

## 二次补充复核

根据补充要求，本文已额外纳入以下不能遗漏的产品约束：

- i18n 多语言支持必须从产品地基阶段考虑，不能后期硬改。
- 数据库连接参数必须完整，按 Navicat 类专业客户端的完整度拆分通用字段和专属字段。
- SQL 编辑器必须支持按数据库方言自动提示补全。
- SQL 编辑器必须支持表名、字段名、Schema、函数等元数据补全。
- SQL 编辑器必须支持 SQL 美化、格式化和压缩，不只是语法高亮。
- Schema 管理必须覆盖新建表页面、字段 CRUD、索引管理、函数/过程索引和 SQL 导入导出。
- 数据库对象浏览必须覆盖 PostgreSQL 等数据库的特殊对象，例如序列、扩展/插件、多数据库/多 Catalog。
- 数据库和表级危险操作必须明确支持但强确认，例如删除表、清空表。
- 新建数据库需要字符集/排序规则选择，默认按 `utf8mb4-bin` 体验呈现。
- 支持对现有数据库做反向解析/反射，生成表关系图，并可用 Diagram 或 D2 文本形式表达。
- 软件和代码都必须把稳定、健壮作为硬性要求，而不是后期优化项。
- 光暗模式必须可显式切换，并可跟随系统设置。
- 代码架构本身必须符合 Unix 哲学，小核心、清晰接口、可组合、可测试。

## 产品边界

AktSQL Database Management，简称 Akt，是一个面向技术用户的高密度数据库管理工具。

核心不是“好看的数据库客户端”，而是：

- 快速连接多个数据库。
- 快速浏览连接、库、表、函数和历史。
- 快速写 SQL、执行 SQL、查看结果、查看消息和执行计划。
- 用稳定、紧凑、键盘友好的界面承载高频数据库工作。
- 保持红黑品牌、神秘、强势、工程化的视觉识别。

## 需求拆分原则

每个切片应满足：

- 可以独立验收。
- 有清晰输入和输出。
- 不依赖一次性完成整个产品。
- 能映射到 OpenSpec 的一个 capability 或一个 change 内的 task。
- 优先实现真实工作流，再补视觉和高级能力。

## 建议阶段

### P0: 产品地基

P0 解决“这个产品是什么、如何启动、如何保存最小状态、如何显示主界面、代码边界如何保持简单”。

### P1: 第一条真实数据库工作流

P1 解决“完整创建连接、连接数据库、执行查询、展示结果、提供基础补全和多语言地基”。

### P2: 高效工作台

P2 解决“多标签、对象树、表设计器、结果工具、历史、消息、执行计划、快捷键、导入导出”。

### P3: 个性化和多数据库扩展

P3 解决“主题、密度、字体、快捷键档案、更多数据库驱动和兼容层”。

### P4: 专业化能力

P4 解决“事务、安全告警、SSH/SSL、导出、性能指标、云同步等高级能力”。

## 切片总览

| 编号 | 切片 | 优先级 | 主要来源 |
|---|---|---:|---|
| R01 | 产品命名与品牌基础 | P0 | specs, logo, design |
| R02 | 应用主壳与固定布局 | P0 | console, connection, settings |
| R03 | 全局状态栏 | P0 | console, connection, settings |
| R04 | 导航结构 | P0 | console, connection, settings |
| R05 | 本地配置文件语义 | P0 | README, settings |
| R06 | 连接数据模型 | P1 | connection |
| R07 | 连接列表 | P1 | connection |
| R08 | 新建连接入口 | P1 | connection, console |
| R09 | 连接编辑表单 | P1 | connection |
| R10 | 连接测试 | P1 | connection |
| R11 | 保存并连接 | P1 | connection |
| R12 | 删除连接 | P1 | connection |
| R13 | 连接状态显示 | P1 | console, connection |
| R14 | SQL 编辑器基础 | P1 | console |
| R15 | SQL 执行按钮 | P1 | console |
| R16 | 查询执行反馈 | P1 | console |
| R17 | 结果表格基础 | P1 | console |
| R18 | 结果行数与耗时 | P1 | console |
| R19 | 数据库对象树 | P2 | console |
| R20 | 多 SQL 标签页 | P2 | light console |
| R21 | 结果面板标签 | P2 | light console |
| R22 | 查询消息 | P2 | light console |
| R23 | 执行计划入口 | P2 | light console |
| R24 | 结果搜索 | P2 | console |
| R25 | 结果导出 | P2 | console |
| R26 | 全屏结果视图 | P2 | console |
| R27 | 事务提交入口 | P2 | console |
| R28 | 查询历史 | P2 | console nav |
| R29 | 设置页面入口 | P2 | settings |
| R30 | 主题切换 | P3 | settings |
| R31 | 布局密度设置 | P3 | settings |
| R32 | 侧栏宽度设置 | P3 | settings |
| R33 | 编辑器字体设置 | P3 | settings |
| R34 | 编辑器字号和行高 | P3 | settings |
| R35 | 快捷键展示 | P3 | settings |
| R36 | 快捷键档案 | P3 | settings |
| R37 | 配置应用与放弃 | P3 | settings |
| R38 | 暗色主题设计系统 | P0 | design, dark console |
| R39 | 亮色主题设计系统 | P3 | light console |
| R40 | 高对比红黑主题 | P3 | settings |
| R41 | SQL Server 2000 兼容提示 | P4 | connection |
| R42 | SSL 开关 | P4 | connection |
| R43 | SSH 隧道开关 | P4 | connection |
| R44 | 多数据库驱动策略 | P1-P4 | specs, connection |
| R45 | 搜索对象入口 | P2 | light console |
| R46 | 刷新入口 | P1 | console, connection |
| R47 | 过滤入口 | P2 | console, connection |
| R48 | 支持入口 | P4 | console, settings |
| R49 | 管理员头像与本地用户标识 | P4 | console, settings |
| R50 | 系统日志入口 | P2 | footer |
| R51 | 性能状态指标 | P2 | footer |
| R52 | 加密状态显示 | P4 | light console |
| R53 | 数据库版本显示 | P2 | dark console |
| R54 | 编辑器光标位置显示 | P2 | dark console |
| R55 | 命令面板入口 | P3 | settings keybindings |
| R56 | 移动端降级布局 | P4 | design |
| R57 | 品牌 logo 资产 | P0 | logo |
| R58 | 表格状态徽标 | P1 | console |
| R59 | 微交互规范 | P2 | HTML scripts |
| R60 | 本地持久化开发者偏好 | P3 | settings |
| R61 | i18n 多语言体系 | P1 | user requirement |
| R62 | 完整连接参数分层 | P1 | user requirement, Navicat baseline |
| R63 | 数据库类型专属连接参数 | P1-P4 | user requirement, Navicat baseline |
| R64 | SQL 方言语法提示补全 | P1 | user requirement |
| R65 | 表名和字段名智能补全 | P1 | user requirement |
| R66 | 光暗模式快速切换 | P1 | user requirement, settings |
| R67 | Unix 哲学代码架构约束 | P0 | user requirement |
| R68 | 连接参数校验与缺省策略 | P1 | user requirement |
| R69 | SQL 美化与格式化 | P1 | user requirement |
| R70 | 新建表设计器 | P1 | user requirement, Navicat baseline |
| R71 | 表字段 CRUD | P1 | user requirement, Navicat baseline |
| R72 | 表索引管理 | P1 | user requirement, Navicat baseline |
| R73 | 函数/过程对象索引 | P2 | user requirement, Navicat baseline |
| R74 | 函数/过程查看与编辑 | P2 | user requirement, Navicat baseline |
| R75 | SQL 文件导入执行 | P2 | user requirement, Navicat baseline |
| R76 | SQL/DDL 导出 | P2 | user requirement, Navicat baseline |
| R77 | 表数据 CRUD | P2 | Navicat baseline |
| R78 | DDL 预览与变更确认 | P1 | user requirement, Navicat baseline |
| R79 | Navicat 类能力覆盖基线 | P0 | user requirement |
| R80 | 数据库级对象浏览 | P1 | user requirement, PostgreSQL baseline |
| R81 | 新建数据库 | P1 | user requirement, Navicat baseline |
| R82 | 删除表与清空表 | P1 | user requirement, Navicat baseline |
| R83 | 表分区查看 | P2 | user requirement, Navicat baseline |
| R84 | 整表 CREATE SQL 查看 | P1 | user requirement, Navicat baseline |
| R85 | 数据库关系图与 D2 展示 | P2 | user requirement, Navicat baseline |
| R86 | 稳定性与健壮性质量门禁 | P0 | user requirement |

## 详细需求切片

### R01 产品命名与品牌基础

需求：产品应统一使用 AktSQL Database Management 作为完整名称，使用 Akt 作为短名，主 CLI 命令为 `aktsql`，短别名暂不定义。

验收：

- UI 顶部品牌显示 `Akt`。
- 文档中明确完整名称、短名、命令名和别名。
- 不同页面的标题、页脚和版本标识使用同一命名规则。

### R02 应用主壳与固定布局

需求：应用应采用数据库工作台布局：顶部工具栏、左侧固定导航、中央工作区、底部状态栏。

验收：

- 顶部栏高度固定，包含品牌、一级导航和主要动作。
- 左侧栏默认宽度为 240px。
- 中央工作区根据当前模块切换内容。
- 底部状态栏始终可见。

### R03 全局状态栏

需求：底部状态栏展示版本、系统日志入口、内存、行数、延迟、连接状态等运行信息。

验收：

- 任意主页面都显示状态栏。
- 至少显示版本号和当前连接状态。
- 查询执行后可更新行数和延迟。

### R04 导航结构

需求：应用导航应服务于工具化工作流。左侧以对象树为主，表达 `Connections -> Databases -> Tables/Views/...` 的层级；顶部或主工作区入口区分 Query、History、Settings 等工具。对象树负责定位对象，右侧工作台负责对当前对象执行操作。

验收：

- 左侧默认展示连接、数据库和对象组层级，而不是把 Databases、Tables、Query Explorer 作为彼此重复的平级入口。
- Query、History、Settings 等工具入口与对象树分离。
- 选择连接后进入数据库工作台，并展示当前连接可见的数据库或当前数据库内对象。
- 当前模块有红色高亮或左侧红色激活线。

### R05 本地配置文件语义

需求：支持本地配置文件保存应用偏好和连接元信息。候选文件为 `.aktsql.toml` 和 `aktsql.config.json`。

验收：

- 明确配置文件优先级。
- 明确哪些配置进入项目级配置，哪些进入用户级配置。
- 敏感信息不得明文落入项目文件。

### R06 连接数据模型

需求：连接应至少包含名称、驱动类型、主机、端口、用户名、密码引用、目标数据库/Schema、字符集、连接超时、SSL 设置、SSH 隧道设置、状态和元数据。连接参数不能只停留在演示级字段，必须按专业数据库客户端的完整性设计。

验收：

- 可以表达 SQL Server、MySQL、PostgreSQL、SQLite、Oracle 的连接类型。
- 可以表达旧版 SQL Server 的兼容模式。
- 密码字段以安全引用或密文方式保存，不以普通字符串暴露。
- 通用参数与数据库专属参数分开建模。
- 未填写的可选项必须有明确默认值或空值语义。

### R07 连接列表

需求：连接管理页左侧展示已有连接列表，显示名称、地址、端口、数据库类型、版本和当前活动状态。

验收：

- 连接列表可选中一项。
- 当前选中项有明显红色或左边框高亮。
- 活动连接显示绿色状态点和 `Active Session`。

### R08 新建连接入口

需求：提供明显的 New Connection 入口，用于创建新的数据库连接配置。

验收：

- 连接管理页和工作台侧栏都有新建连接入口。
- 点击后进入空表单或新建流程。
- 不影响已有活动连接。

### R09 连接编辑表单

需求：连接设置页应支持编辑连接名称、驱动类型、主机、端口、认证信息、目标数据库、Schema、字符集、时区、超时、SSL、SSH、代理和高级驱动参数。

验收：

- 表单字段与当前选中连接同步。
- 驱动类型在兼容模式下可以被锁定。
- 表单有基础校验：必填、端口格式、主机不能为空。
- 表单按 General、Authentication、Database、SSL、SSH、Advanced 分组，避免重要参数被隐藏或遗漏。

### R10 连接测试

需求：用户可以在保存前测试连接。

验收：

- 点击 Test Connection 后显示测试中、成功或失败状态。
- 失败时展示可读错误。
- 测试不自动保存配置。

### R11 保存并连接

需求：用户可以保存连接配置并立即建立会话。

验收：

- Save & Connect 保存配置。
- 成功后状态栏显示当前连接。
- 连接失败时保留表单输入并显示错误。

### R12 删除连接

需求：连接设置页提供删除连接入口。

验收：

- 删除需要确认。
- 删除当前活动连接时应断开或切换连接。
- 删除后连接列表同步更新。

### R13 连接状态显示

需求：工作台应展示当前连接名称、数据库类型、Schema 和连接状态。

验收：

- 侧栏或标题区显示当前连接。
- 编辑器头部可显示 DB 和 Schema。
- 状态栏显示 `Connected: <name>` 或未连接状态。

### R14 SQL 编辑器基础

需求：工作台提供 SQL 编辑器，包含行号、等宽字体、语法高亮、SQL 格式化、方言感知补全和可编辑文本区域。

验收：

- 默认字体为 JetBrains Mono。
- SQL 关键字高亮。
- 支持多行编辑和 Tab 缩进。
- 能根据当前连接的数据库类型识别 SQL 方言。
- 支持调用 SQL 格式化能力。
- 最小版本可先用基础编辑器，后续替换为 Monaco 或 CodeMirror。

### R15 SQL 执行按钮

需求：顶部栏提供 Execute 入口，用于执行当前查询或选中 SQL。

验收：

- Execute 在所有编辑器页面可见。
- 执行期间有 loading 状态。
- 执行完成后更新结果面板和状态栏。

### R16 查询执行反馈

需求：执行查询后，用户应看到成功、失败、耗时和返回行数。

验收：

- 成功时显示 `QUERY SUCCESS` 或等价状态。
- 失败时显示错误状态和错误消息。
- 展示耗时，例如 `4ms`。

### R17 结果表格基础

需求：查询结果以高密度表格展示。表格栏默认最多展示 60 行数据，避免大结果集直接撑爆界面。

验收：

- 表头固定或在滚动时保持可见。
- 单元格使用等宽字体。
- 支持横向和纵向滚动。
- 至少展示列名、行数据和空结果状态。
- 默认每页最多展示 60 行，超出部分通过分页、加载更多或虚拟滚动处理。
- 点击表格头可以切换排序：第一次升序，第二次降序，再次点击可恢复默认排序或清除排序。
- 当前排序列和排序方向必须有明确视觉标记。

### R18 结果行数与耗时

需求：结果区展示返回行数、当前分页范围和执行耗时。

验收：

- 显示 `Results (n rows)` 或 `Showing 1-60 of n rows`。
- 大结果集不一次性渲染全部行。
- 行数和耗时来源于实际执行结果。

### R19 数据库对象树

需求：侧栏支持展开服务器、数据库/Catalog、Schema、表、视图、索引、函数、存储过程、触发器、序列、扩展/插件等对象。

验收：

- 对象树可展开和折叠。
- 表对象有表格图标。
- 点击表对象可进入表数据或生成查询。
- 函数、过程、索引等对象应按类型分组，能快速定位。
- PostgreSQL 类数据库应显示 Sequences、Extensions、Schemas、Databases 等对象组。
- 支持同一连接下查看多个数据库或 Catalog；数据库不支持跨库浏览时应明确提示。

### R20 多 SQL 标签页

需求：支持多个 SQL 标签页，如 `query_main.sql`、`check_indices.sql`。

验收：

- 可以新增标签。
- 可以关闭标签。
- 当前标签高亮。
- 每个标签保留自己的 SQL 内容和执行结果。

### R21 结果面板标签

需求：结果区支持 Results、Messages、Execution Plan 三类标签。

验收：

- 默认显示 Results。
- 可以切换到 Messages。
- 可以切换到 Execution Plan。

### R22 查询消息

需求：Messages 面板展示执行日志、影响行数、错误、警告等信息。

验收：

- 查询完成后有可读消息。
- 错误查询显示错误详情。
- 消息与当前 SQL 标签页绑定。

### R23 执行计划入口

需求：Execution Plan 面板用于展示数据库返回的执行计划。

验收：

- 支持无执行计划状态。
- 支持文本计划作为第一版。
- 后续可扩展为可视化计划树。

### R24 结果搜索

需求：结果区提供搜索入口，用于在当前结果内定位文本。

验收：

- 搜索按钮可打开搜索输入。
- 搜索结果有匹配数量或高亮。
- 搜索不重新执行 SQL。

### R25 结果导出

需求：结果区提供导出入口，原型体现 CSV/下载按钮。

验收：

- 至少支持 CSV 导出。
- 导出当前结果集或当前页的范围需明确。
- 导出失败时有错误提示。

### R26 全屏结果视图

需求：结果区提供全屏或放大查看入口。

验收：

- 点击后结果表格占据主要工作区。
- 可返回编辑器布局。
- 不丢失当前结果。

### R27 事务提交入口

需求：顶部栏提供 Commit 入口，用于提交当前事务。

验收：

- 只有在事务模式或支持事务的连接上启用。
- 提交成功有反馈。
- 提交失败显示错误。

### R28 查询历史

需求：侧栏 History 模块展示历史查询。

验收：

- 记录 SQL、连接、时间、执行状态和耗时。
- 可从历史恢复 SQL 到编辑器。
- 可清理历史。

### R29 设置页面入口

需求：侧栏 Settings 入口进入设置页面。

验收：

- Settings 在侧栏底部固定。
- 当前设置页有激活态。
- 设置页不需要连接数据库也可访问。

### R30 主题切换

需求：外观设置支持 Obsidian(Dark)、Dawn(Light)、Amaterasu 三种主题，并提供光暗模式快速切换能力。

验收：

- 选择主题后界面预览或即时应用。
- 当前主题有选中状态。
- 配置可持久化。
- 支持跟随系统、强制暗色、强制亮色三种模式。

### R31 布局密度设置

需求：支持 Compact 和 Normal 两种布局密度。

验收：

- Compact 对应 4px grid。
- Normal 对应 8px grid。
- 变更后影响表格、侧栏、表单和编辑器周边间距。

### R32 侧栏宽度设置

需求：外观设置支持调整侧栏宽度，默认 240px。

验收：

- 提供滑块。
- 显示当前像素值。
- 应用后侧栏宽度变化并持久化。

### R33 编辑器字体设置

需求：支持选择编辑器字体，默认 JetBrains Mono，候选 Fira Code、Source Code Pro、IBM Plex Mono。

验收：

- 下拉框列出候选字体。
- 选择后预览区和编辑器更新。
- 字体不可用时回退到等宽字体。

### R34 编辑器字号和行高

需求：支持设置编辑器字号和行高。

验收：

- 字号默认 13px。
- 行高默认 1.5em。
- 输入值有范围校验。

### R35 快捷键展示

需求：设置页展示核心快捷键。

验收：

- 展示执行当前查询：Ctrl+Enter。
- 展示保存当前编辑内容或结构变更：Ctrl+S。
- 展示刷新当前上下文数据：F5。
- 展示切换侧栏：Ctrl+B。
- 展示打开命令面板：Ctrl+Shift+P。
- 展示提交事务：F9。

### R36 快捷键档案

需求：支持 Standard、Vim、Emacs 三种快捷键人格档案。

验收：

- 当前档案有选中状态。
- 切换档案影响实际快捷键绑定。
- 冲突快捷键有提示。

### R37 配置应用与放弃

需求：设置页提供 Apply Configuration 和 Discard Changes。

验收：

- 未应用的修改可被放弃。
- 应用后写入本地配置。
- 应用失败时保留未提交状态并提示错误。

### R38 暗色主题设计系统

需求：默认暗色主题采用深黑、深灰、赤红、浅红文本的高对比风格。

验收：

- 主背景接近 `#0A0A0A` 或 `#121414`。
- 主强调色使用赤红。
- UI 通过边框和色调层级表达深度，不依赖柔和阴影。

### R39 亮色主题设计系统

需求：亮色主题使用白色/浅灰底和赤红强调色。

验收：

- 亮色模式下文本对比度可读。
- 编辑器、表格、侧栏和底栏都跟随主题切换。
- 品牌红在亮色下仍作为主要强调色。

### R40 高对比红黑主题

需求：Amaterasu 主题提供更强烈的黑红高对比视觉。

验收：

- 主题卡片可选中。
- 黑色背景和高饱和红色是主视觉。
- 不影响表格可读性。

### R41 SQL Server 2000 兼容提示

需求：当连接目标为 SQL Server 2000 或使用旧认证协议时，展示兼容和安全风险提示。

验收：

- 连接设置页显示 Legacy Warning。
- 提示说明旧认证和兼容层风险。
- 不阻止连接，但应要求用户明确知道风险。

### R42 SSL 开关

需求：连接设置支持 SSL/加密连接开关。

验收：

- 可打开和关闭。
- 打开时可配置证书路径或证书校验策略。
- 缺失证书时给出校验提示。

### R43 SSH 隧道开关

需求：连接设置支持 SSH tunneling。

验收：

- 可打开和关闭。
- 打开后显示代理主机、端口、用户名、认证方式等配置。
- 连接测试时走隧道。

### R44 多数据库驱动策略

需求：产品目标支持 SQL Server 2000+、MySQL、PostgreSQL、SQLite、Oracle。多数据库支持必须同时覆盖连接、元数据读取、SQL 方言、自动补全、执行结果、Schema DDL、数据库级对象、导入导出和错误映射。

验收：

- 先定义统一驱动接口。
- 第一版至少实现一个真实驱动。
- 其他驱动可以作为明确的后续切片，不在同一实现任务内混做。
- 每个驱动必须声明支持能力矩阵，例如事务、执行计划、Schema、存储过程、SSL、SSH、批量执行。
- 每个驱动必须声明 DDL 能力矩阵，例如新建表、修改字段、索引、外键、触发器、函数/过程、导出 DDL。
- 每个驱动必须声明数据库级对象能力矩阵，例如 database/catalog、schema、sequence、extension/plugin、tablespace、partition。

### R45 搜索对象入口

需求：顶部提供对象搜索，用于快速查找数据库对象。

验收：

- 搜索框支持输入对象名。
- 结果可跳转到对象树中的目标。
- 无连接时显示空状态。

### R46 刷新入口

需求：提供刷新入口，用于刷新对象树、结果、表数据、元数据或连接状态。F5 是默认刷新快捷键。

验收：

- 当前上下文决定刷新目标。
- 按 F5 刷新当前上下文，例如表数据页刷新数据，结果区刷新当前查询结果，对象树刷新元数据。
- 刷新期间有 loading 状态。
- 刷新失败显示错误。

### R47 过滤入口

需求：提供过滤入口，用于过滤连接列表、对象树或结果集。

验收：

- 连接管理页可过滤连接列表。
- 工作台可过滤对象树或结果。
- 过滤条件可清除。

### R48 支持入口

需求：侧栏提供 Support 入口。

验收：

- 入口固定在侧栏底部。
- 第一版可打开帮助文档或本地说明页。
- 不阻塞核心数据库工作流。

### R49 管理员头像与本地用户标识

需求：顶部栏展示本地用户或管理员头像。

验收：

- 可显示默认头像。
- 不依赖云账号即可运行。
- 后续可扩展为用户资料设置。

### R50 系统日志入口

需求：状态栏提供 System Logs 入口。

验收：

- 点击可打开日志面板或日志文件。
- 查询、连接、配置保存等关键动作写入日志。
- 日志不包含明文密码。

### R51 性能状态指标

需求：状态栏展示内存、行数、延迟等运行指标。

验收：

- 查询完成后更新行数和延迟。
- 内存指标可以先显示进程内存。
- 指标异常不影响主流程。

### R52 加密状态显示

需求：状态栏在连接场景展示加密状态，如 `Encrypted: AES-256`。

验收：

- 加密状态来自当前连接配置或实际连接能力。
- 未加密时不能误报已加密。
- 不支持加密时显示明确状态。

### R53 数据库版本显示

需求：状态栏展示当前数据库类型和版本，如 PostgreSQL 15.2。

验收：

- 连接成功后读取并展示版本。
- 读取失败时显示未知版本。
- 断开连接后清空或显示未连接。

### R54 编辑器光标位置显示

需求：状态栏展示当前编辑器光标位置，如 `Ln 4, Col 22`。

验收：

- 光标移动后更新行列。
- 多标签页切换时显示当前标签的位置。
- 非编辑器页面不显示或显示空状态。

### R55 命令面板入口

需求：支持通过 Ctrl+Shift+P 打开命令面板。

验收：

- 快捷键可触发命令面板。
- 命令面板至少支持搜索命令名称。
- 可执行常用动作：执行查询、切换主题、打开连接管理。

### R56 移动端降级布局

需求：设计系统要求移动端侧栏收进抽屉，表格横向滚动或卡片化。

验收：

- 小屏不出现严重重叠。
- 侧栏可打开和关闭。
- 表格至少支持横向滚动。

### R57 品牌 logo 资产

需求：使用红云 logo 作为主品牌资产。

验收：

- 应用顶部展示红云 logo。
- favicon 或应用图标使用红云变体。
- 暗色和亮色背景下均可辨识。

### R58 表格状态徽标

需求：结果表格中的状态字段可以显示徽标，如 Active、Offline、Pending。

验收：

- 状态字段可按值渲染 badge。
- badge 样式与主题一致。
- 未识别状态回退为普通文本。

### R59 微交互规范

需求：按钮、卡片、主题选择、连接选择等交互应有轻量反馈。

验收：

- 点击 Execute 有执行中状态。
- 连接卡片点击更新选中态。
- 主题卡片点击更新选中态。
- 成功动作可显示 toast。

### R60 本地持久化开发者偏好

需求：主题、密度、字体、快捷键档案和侧栏宽度应持久化在本地开发者配置中。

验收：

- 重启应用后偏好仍生效。
- 配置损坏时回退默认值。
- 配置 schema 可版本化。

### R61 i18n 多语言体系

需求：产品必须支持 i18n 多语言，界面文案、菜单、按钮、错误提示、连接表单标签、状态栏、设置项和日志展示文案都不能硬编码为单一语言。

验收：

- 第一版至少支持 `zh-CN` 和 `en-US`。
- 语言资源集中管理，业务代码不得散落硬编码 UI 文案。
- 支持运行时切换语言，切换后主要界面无需重启即可更新。
- 日期、时间、数字、货币、行数、延迟等格式按当前 locale 展示。
- 数据库原始错误保留原文，同时可提供本地化解释。

### R62 完整连接参数分层

需求：连接配置必须按专业数据库客户端完整建模，不能只有 host/port/user/password。参数应分为通用参数、认证参数、数据库目标参数、安全参数、网络隧道参数和高级驱动参数。

验收：

- General：连接名、颜色/分组、数据库类型、驱动版本、备注。
- Network：主机、端口、Unix socket/pipe、连接超时、读取超时、写入超时、keepalive。
- Authentication：用户名、密码引用、认证方式、保存密码策略、操作系统集成认证。
- Database Target：默认数据库、Schema、Catalog、服务名、实例名、文件路径等。
- Encoding/Locale：字符集、排序规则、时区、日期时间格式偏好。
- Security：SSL/TLS、证书、CA、客户端证书、私钥、证书校验策略。
- Tunnel/Proxy：SSH 隧道、HTTP/SOCKS 代理、跳板机、代理认证。
- Advanced：驱动自定义参数、连接字符串追加项、只读模式、自动重连、连接池限制。

### R63 数据库类型专属连接参数

需求：不同数据库类型必须有专属连接参数页或动态字段，参考 Navicat 类连接管理器的完整度设计。

验收：

- SQL Server：host、port、instance、database、domain、SQL Server Auth、Windows/Integrated Auth、encrypt、trust server certificate、TDS/legacy compatibility、application name。
- MySQL/MariaDB：host、port、socket、database、charset、SSL mode、allow public key retrieval、server timezone、init SQL。
- PostgreSQL：host、port、database、schema search path、SSL mode、service file、application name、connect timeout、options。
- SQLite：database file path、只读模式、create if missing、busy timeout、journal mode、foreign keys pragma。
- Oracle：host、port、service name、SID、TNS alias、role、wallet、NLS language、thick/thin driver mode。
- 每种数据库的默认端口、必填字段和不可用字段应自动变化。

### R64 SQL 方言语法提示补全

需求：SQL 编辑器必须根据当前连接的数据库类型提供对应 SQL 方言的关键字、函数、操作符和语法片段补全。

验收：

- PostgreSQL、MySQL、SQL Server、SQLite、Oracle 至少能加载各自关键字集合。
- 补全列表根据当前连接动态切换。
- 支持常见片段：SELECT、INSERT、UPDATE、DELETE、JOIN、CREATE TABLE、ALTER TABLE、事务语句。
- 不同方言的分页、字符串函数、日期函数、引号规则不能混淆。
- 无连接时使用通用 SQL 补全。

### R65 表名和字段名智能补全

需求：SQL 编辑器必须基于当前连接元数据补全库名、Schema、表名、视图名、字段名、函数名，并尽量理解别名和当前 SQL 上下文。

验收：

- 连接成功后可刷新元数据缓存。
- 输入 `schema.` 后补全该 Schema 下对象。
- 输入 `table.` 或表别名后补全字段。
- JOIN、WHERE、ORDER BY、GROUP BY 场景能优先提示相关字段。
- 元数据缓存过期、刷新失败或权限不足时有清晰状态，不阻塞手写 SQL。
- 补全结果显示对象类型、来源 Schema 和字段类型。

### R66 光暗模式快速切换

需求：除设置页主题选择外，产品还应提供显式光暗模式快速切换，满足日常使用中的即时切换。

验收：

- 顶部栏或命令面板可快速切换 Light/Dark。
- 支持跟随系统模式。
- 切换不导致当前 SQL、连接、结果丢失。
- 切换后的编辑器、结果表格、弹窗、状态栏和连接表单全部同步。

### R67 Unix 哲学代码架构约束

需求：代码本身必须符合 Unix 哲学：小核心、清晰边界、组合式能力、文本友好、配置透明、错误明确。不要把数据库驱动、UI、配置、日志、SQL 编辑、补全和连接安全揉成一个不可测试的大模块。该约束同时服务于稳定性和可维护性。

验收：

- 每个模块只负责一类事情，例如 connection、driver、metadata、query、completion、settings、i18n、ui-shell。
- 核心业务逻辑不依赖具体 UI 框架。
- 数据库驱动通过窄接口接入，新增驱动不需要改动编辑器和设置页核心逻辑。
- 配置可导入、导出、 diff，并避免不可读的私有二进制格式。
- 日志和错误面向排障，错误包含上下文但不泄露密码。
- CLI 或内部命令层可复用核心能力，便于脚本化和自动化。
- 测试以小模块为单位，避免只能通过端到端 UI 测试验证核心逻辑。
- 任一模块失败不能拖垮整个应用；驱动、补全、图表、导入导出等能力应有可降级边界。

### R68 连接参数校验与缺省策略

需求：连接参数必须有统一校验、默认值和错误解释机制，避免用户因为缺字段、错字段或数据库类型切换导致配置不可用。

验收：

- 必填字段按数据库类型动态变化。
- 端口、路径、超时、SSL 文件路径、SSH 主机和代理参数都有格式校验。
- 数据库类型切换时保留可复用字段，并提示会丢弃的专属字段。
- 测试连接前先做本地参数校验，再发起网络连接。
- 错误提示应区分参数错误、网络错误、认证错误、权限错误和驱动不支持。

### R69 SQL 美化与格式化

需求：SQL 编辑器必须支持 SQL 美化、格式化和压缩，不能只提供语法高亮。格式化应尽量按当前数据库方言处理关键字大小写、缩进、换行、逗号位置和子查询层级。

验收：

- 提供 Format SQL 入口，可作用于全文或选中 SQL。
- 支持 Beautify 和 Minify 两类动作。
- 支持配置关键字大小写：UPPER、lower、Preserve。
- 支持配置缩进宽度、逗号前置/后置、JOIN/WHERE/GROUP BY/ORDER BY 换行策略。
- 格式化失败时不破坏原 SQL，并显示错误原因。
- 不同数据库方言的引号、变量、批处理分隔符和函数语法不得被错误改写。

### R70 新建表设计器

需求：必须提供工具化的新建表页面。用户可以通过图形界面创建表，配置字段、主键、索引、外键、约束、默认值、注释、引擎/表空间等参数，并直接执行建表操作。常见建表功能要覆盖高频数据库客户端工作流，但主路径不把用户送去 SQL 预览。

验收：

- 对象树或 Tables 模块提供 New Table 入口。
- 新建表页面至少包含 Fields、Indexes、Constraints、Options 分区。
- 可配置表名、Schema、表注释。
- Fields 区必须支持字段名称、数据类型、长度、精度、小数位、nullable、默认值、主键、自增、唯一、字符集/排序规则、注释。
- Options 区按数据库能力展示 engine、tablespace、partition、charset、collation、row format 等常见选项。
- 保存直接执行数据库方言对应的建表操作；生成语句可作为后续日志或审计详情，不作为主交互路径。
- 保存成功后刷新对象树并能打开新表。
- 保存失败时保留设计器内容并显示数据库返回错误。

### R71 表字段 CRUD

需求：表设计器必须支持字段级 CRUD，包括新增字段、编辑字段、删除字段、调整顺序和设置字段属性。

验收：

- 字段支持名称、数据类型、长度、精度、小数位、nullable、默认值、主键、自增、唯一、字符集、排序规则、注释。
- 字段类型列表根据数据库类型动态变化。
- 常见类型需要按数据库分组展示，例如整数、浮点、定点、字符串、文本、日期时间、布尔、JSON、二进制、UUID、枚举或数据库等价类型。
- 字段支持上移和下移，用于调整字段顺序；数据库不支持物理重排时应生成兼容方案或明确提示。
- 修改已有表字段时能生成 ALTER TABLE DDL。
- 删除字段需要确认，并提示可能造成数据丢失。
- 字段改名、类型变更、nullable 变更应在保存前的变更摘要或后续日志中明确展示。
- 字段新增和字段修改可以先进入未保存状态，按 Ctrl+S 保存并执行对应 DDL。
- Ctrl+S 保存前必须保留可确认的结构化变更状态，并直接执行对应数据库操作。

### R72 表索引管理

需求：表设计器必须支持索引管理，包括新增、编辑、删除普通索引、唯一索引、主键索引和数据库支持的特殊索引。

验收：

- 索引可配置名称、类型、字段列表、字段顺序、排序方向。
- 支持唯一索引和普通索引。
- 支持多字段组合索引。
- 数据库支持时可配置 fulltext、spatial、hash、btree、partial index、included columns 等能力。
- 删除索引需要确认。
- 索引变更通过结构化参数直接执行，失败时保留编辑状态并显示数据库返回错误。

### R73 函数/过程对象索引

需求：数据库对象树必须对函数、存储过程、触发器等可编程对象建立索引式浏览能力，便于快速定位和打开对象。

验收：

- 对象树按 Functions、Procedures、Triggers 分组展示。
- 支持按名称搜索函数/过程。
- 展示对象所属 Schema、参数摘要、返回类型或对象类型。
- 元数据刷新后函数/过程列表同步更新。
- 权限不足时显示空状态或权限提示，不误报为不存在。

### R74 函数/过程查看与编辑

需求：用户可以查看函数、存储过程、触发器的定义，支持编辑并生成对应 ALTER/CREATE 脚本。

验收：

- 可打开对象定义到 SQL 编辑器或专用对象编辑器。
- 支持只读查看和可编辑模式。
- 保存前展示变更脚本。
- 支持另存为 SQL 文件或复制 DDL。
- 不同数据库的函数/过程语法由驱动适配。

### R75 SQL 文件导入执行

需求：支持导入 `.sql` 文件并在当前连接中执行，覆盖常见初始化、迁移和数据导入场景。

验收：

- 可选择 SQL 文件并预览内容。
- 可配置编码、语句分隔符、是否在事务中执行、遇错继续/停止。
- 执行时显示进度、当前语句、成功/失败数量。
- 大文件不能一次性阻塞 UI。
- 执行日志可保存或复制。

### R76 SQL/DDL 导出

需求：支持导出 SQL，包括表结构 DDL、选中对象 DDL、查询结果导出为 INSERT 脚本，以及数据库/Schema 的结构导出。

验收：

- 表对象可导出 CREATE TABLE DDL。
- 可选择导出范围：单表、多表、Schema、查询结果。
- 查询结果可导出为 INSERT SQL。
- 支持只导出结构、只导出数据、结构和数据同时导出。
- 可配置是否包含 DROP、IF EXISTS、注释、索引、外键、触发器。

### R77 表数据 CRUD

需求：除了 SQL 查询结果，产品也应提供表数据浏览和行级 CRUD 能力，满足 Navicat 类数据表编辑工作流。

验收：

- 打开表后显示数据网格。
- 表数据网格默认最多展示 60 行。
- 支持点击表头按字段升序/降序排序。
- 支持直接在表格单元格中修改数据。
- 支持新增行、编辑单元格、删除行、按 Ctrl+S 提交变更、回滚未提交变更。
- 主键缺失时明确提示编辑能力受限。
- 支持 NULL、默认值、日期时间、布尔、二进制等类型的基础编辑。
- 修改数据前后有状态标记，提交失败可定位到失败行。

### R78 Schema 变更执行与确认

需求：Schema 变更类操作以参数表单为主。创建类操作在校验通过后直接执行，不再把 SQL 预览作为主路径；破坏性或高风险操作必须在执行前确认，避免图形界面操作导致不可见的破坏性变更。

验收：

- 新建数据库、新建表等创建类操作由表单收集必要和可选参数，提交后直接执行并反馈成功或失败。
- 修改表、删除字段、删除索引、修改函数/过程等高风险变更必须在执行前确认。
- 危险操作必须有明确警告，例如删除字段、截断表、删除表。
- 系统日志应记录执行的变更摘要、目标对象、耗时和失败原因；必要时可在日志详情中提供可复制 SQL，但不作为创建流程的主入口。
- 执行成功和失败都记录到系统日志。

### R79 Navicat 类能力覆盖基线

需求：“类似 Navicat”作为产品覆盖基线，应被拆分成明确模块：连接管理、对象树、表设计器、数据表编辑、SQL 编辑器、查询结果、函数/过程、索引、导入导出、用户偏好。不能用一句“类似 Navicat”替代可验收需求。

验收：

- 需求文档中每个 Navicat 类能力都有对应切片。
- 实现计划不得把所有 Navicat 类能力塞进单个任务。
- 每个数据库驱动声明哪些 Navicat 类能力已支持、部分支持或暂不支持。
- UI 上未实现的能力应显示不可用状态，而不是隐藏到用户无法判断。

### R80 数据库级对象浏览

需求：对象树必须支持数据库级对象浏览，覆盖 PostgreSQL 等数据库中的特殊对象。不能只显示表和视图。

验收：

- PostgreSQL 连接应显示 Databases、Schemas、Tables、Views、Sequences、Functions、Procedures、Extensions、Indexes、Triggers。
- MySQL/MariaDB 连接应显示 Databases、Tables、Views、Functions、Procedures、Events、Triggers。
- SQL Server 连接应显示 Databases、Schemas、Tables、Views、Stored Procedures、Functions、Sequences、Indexes。
- Oracle 连接应显示 Schemas/Users、Tables、Views、Sequences、Packages、Procedures、Functions、Indexes。
- 对象树支持刷新局部节点，不必每次刷新整棵树。
- 权限不足或数据库不支持的对象组应显示明确状态。

### R81 新建数据库

需求：支持新建数据库，并允许选择字符集和排序规则。默认选项按产品体验显示为 `utf8mb4-bin`；在 MySQL/MariaDB 中应映射为 `utf8mb4` 字符集和 `utf8mb4_bin` 排序规则。

验收：

- 连接或服务器节点提供 New Database 入口。
- 表单支持数据库名、字符集/编码、排序规则，以及数据库支持的 owner、template、tablespace、initial collection 等必要或可选字段。
- MySQL/MariaDB 默认字符集/排序规则为 `utf8mb4` + `utf8mb4_bin`，界面可显示 `utf8mb4-bin`。
- PostgreSQL/SQL Server/Oracle 使用各自支持的 locale、encoding、collation、tablespace 或 owner 字段。
- 创建操作提交后直接执行，不跳转到 SQL 预览或查询编辑器。
- 创建成功后选中新数据库并刷新数据库/对象树列表；失败时保留表单并显示数据库返回错误。
- 当前尚未接入 native driver 的数据库类型必须明确显示“暂不可直接执行”，不能伪装成功。
- 数据库节点右键菜单提供新建数据表、查看数据库详情、重命名数据库、修改字符集/排序规则、删除数据库。
- 查看数据库详情只在右侧工作台展示只读信息，不提供编辑控件；必须从当前数据库加载真实 metadata，按 Core、Storage/Encoding、Objects、Runtime 等父类分组，高频信息在上，低频信息在下，不同父类用颜色区分。
- 重命名数据库和修改字符集/排序规则使用弹窗表单，提交后按数据库类型选择对应实现并直接执行；不跳转到 SQL 预览。MySQL/MariaDB/TiDB 的数据库重命名必须执行时读取 `information_schema` 生成迁移计划，不能依赖左侧树是否已展开。
- 数据表或集合节点提供重命名操作，提交后按数据库类型执行 `ALTER TABLE`、`RENAME TABLE`、`renameCollection` 或等价语义。
- 数据表或集合节点的 `SELECT ROWS` 直接查询结果，不把语句写入查询编辑器；默认每页 100 行，并在结果表格底部提供上一页/下一页。
- 数据表或集合节点的 `DESCRIBE TABLE` 直接展示只读表结构详情，按 Core、Storage/Encoding、Objects、Runtime 等父类分组，并展示字段、索引和本表 CREATE 语句。
- 新增数据表使用弹窗表单提交并直接执行，不进入 SQL 编辑器；高级建表器后续扩展多字段、索引、约束和引擎选项。
- 数据表或集合节点的 `ALTER TABLE` 使用右侧工作台内的表设计器提交并直接执行，不使用弹窗；当前至少覆盖字段重命名、新增字段、新增索引、MySQL 兼容数据库字段移动。数据库不支持的物理字段移动必须直接显示不可用原因，不能退回 SQL 预览。

### R82 删除表与清空表

需求：表对象必须支持删除表和清空表，但这两类危险操作必须有强确认。非危险查看和编辑操作不得以 SQL 预览作为主路径。

验收：

- 表对象菜单提供 `DROP TABLE`、`TRUNCATE TABLE`、`ALTER TABLE`、`DESCRIBE TABLE`、`SELECT ROWS` 等数据库术语，不再使用泛化文案。
- 删除表执行前显示确认弹窗，确认目标对象后才执行 DROP TABLE。
- 清空表执行前显示确认弹窗，确认目标对象后才执行 TRUNCATE TABLE 或兼容语义。
- 危险操作确认文案必须包含表名，必要时要求二次输入表名。
- 执行结果写入系统日志。
- 失败时展示数据库返回错误并保留对象树状态。

### R83 表分区查看

需求：表详情页支持查看表分区信息，包括分区策略、分区字段、分区列表和分区 DDL。

验收：

- 支持在表详情中打开 Partitions 标签。
- 展示分区类型，例如 range、list、hash、composite 或数据库等价概念。
- 展示分区名、边界/表达式、行数估计、存储位置等可用信息。
- 支持复制分区相关 DDL。
- 数据库或表不支持分区时显示空状态。

### R84 整表 CREATE SQL 查看

需求：表详情页必须支持查看整表 CREATE SQL，包括字段、主键、索引、外键、注释、分区和数据库支持的表选项。

验收：

- 表对象菜单或详情页提供 View/Create SQL。
- 生成的 SQL 能复制、保存为 `.sql` 文件、发送到 SQL 编辑器。
- SQL 应包含字段定义、主键、索引、外键、默认值、注释。
- 数据库支持时包含 engine、tablespace、partition、collation、storage options。
- 与 R76 导出能力复用同一 DDL 生成逻辑，避免两个地方输出不一致。

### R85 数据库关系图与 D2 展示

需求：支持对现有数据库进行反向解析/反射，读取表、字段、主键、外键和可推断关系，并以关系图展示表与表之间的关联。图形视图应支持 Diagram 交互展示，也应支持导出或查看 D2 文本表示。

验收：

- 可从数据库、Schema 或选中表集合生成关系图。
- 关系来源优先使用数据库外键；没有外键时可按命名规则提供可选的推断关系，但必须标记为 inferred。
- 图中节点展示表名、主键、关键字段；边展示关系字段和方向。
- 支持缩放、拖拽、自动布局、按表名搜索定位。
- 支持导出 D2 文本，且 D2 内容可重新生成同等关系图。
- 支持导出图片或可复制图文本。
- 大库生成关系图时必须可取消、可分批加载，不能阻塞应用。

### R86 稳定性与健壮性质量门禁

需求：无论是代码本身还是软件运行体验，都必须以稳定、健壮为硬性标准。数据库管理工具面对生产库时不能因为单个异常、网络波动、大结果集、驱动错误或 UI 渲染失败导致数据丢失、误操作或应用崩溃。

验收：

- 所有数据库写操作必须有明确的成功/失败状态，不允许静默失败。
- 破坏性操作必须强确认、可预览 SQL、写入日志。
- 网络断开、超时、权限不足、驱动不兼容、大结果集、元数据读取失败都必须有可恢复错误状态。
- UI 长任务必须可取消或至少显示进度，不能长期无响应。
- 配置损坏、缓存损坏、语言资源缺失、主题资源缺失时应用必须回退默认值。
- 核心模块需要单元测试，驱动适配需要集成测试，危险 SQL 生成需要快照或等价测试。
- 发布前必须有基础 smoke test：启动、连接测试、执行查询、新建表 DDL 预览、SQL 格式化、对象树刷新。
- 日志要足够排障，但不得记录明文密码、token、私钥。

## 首个 MVP 建议

不要第一步就实现全部多数据库和所有设置项。建议首个 MVP 控制在一条闭环：

1. 应用主壳：顶部栏、侧栏、工作区、状态栏。
2. 暗色主题和光暗快速切换：红黑品牌、基础组件、Light/Dark 切换不丢状态。
3. 连接管理：完整连接模型、列表、表单、保存、测试连接、参数校验。
4. 一个真实数据库驱动：建议从 SQLite 或 PostgreSQL 二选一；如果必须验证原型中的专业性，再做 SQL Server。
5. SQL 编辑器：基础编辑、执行按钮、执行反馈、当前方言关键字补全。
6. SQL 格式化：至少支持当前编辑器全文格式化。
7. 新建表页面：Fields、Indexes、DDL Preview 三个核心区先跑通。
8. 表操作：支持查看整表 CREATE SQL，支持删除表、清空表的强确认流程。
9. 结果表格：展示列、行、耗时、行数。
10. 元数据补全：至少补全当前连接的表名和字段名。
11. 数据库对象树：至少覆盖当前驱动的数据库、Schema、表、视图、序列或等价对象。
12. 稳定性地基：错误状态、强确认、日志脱敏、长任务进度和可恢复失败。
13. 本地配置：保存连接元信息、外观偏好和语言偏好。
14. i18n 地基：至少支持 `zh-CN` 和 `en-US` 的界面资源结构。

这样 MVP 能证明产品核心价值：用户能保存一个连接、执行一条 SQL、看到结果，并能用图形界面创建一张表。

## OpenSpec 拆分建议

后续可以按下面顺序创建 OpenSpec changes：

### change 1: `establish-app-shell-and-brand`

包含：

- R01 产品命名与品牌基础
- R79 Navicat 类能力覆盖基线
- R02 应用主壳与固定布局
- R03 全局状态栏
- R04 导航结构
- R38 暗色主题设计系统
- R57 品牌 logo 资产
- R66 光暗模式快速切换
- R67 Unix 哲学代码架构约束
- R86 稳定性与健壮性质量门禁

### change 2: `add-i18n-foundation`

包含：

- R61 i18n 多语言体系

### change 3: `add-local-configuration`

包含：

- R05 本地配置文件语义
- R60 本地持久化开发者偏好

### change 4: `add-connection-management`

包含：

- R06 连接数据模型
- R07 连接列表
- R08 新建连接入口
- R81 新建数据库
- R09 连接编辑表单
- R10 连接测试
- R11 保存并连接
- R12 删除连接
- R13 连接状态显示
- R44 多数据库驱动策略中的第一版驱动
- R62 完整连接参数分层
- R63 数据库类型专属连接参数
- R68 连接参数校验与缺省策略

### change 5: `add-query-console`

包含：

- R14 SQL 编辑器基础
- R15 SQL 执行按钮
- R16 查询执行反馈
- R17 结果表格基础
- R18 结果行数与耗时
- R46 刷新入口
- R50 系统日志入口
- R51 性能状态指标
- R64 SQL 方言语法提示补全
- R65 表名和字段名智能补全
- R69 SQL 美化与格式化

### change 6: `add-table-designer`

包含：

- R70 新建表设计器
- R71 表字段 CRUD
- R72 表索引管理
- R82 删除表与清空表
- R84 整表 CREATE SQL 查看
- R78 DDL 预览与变更确认

### change 7: `improve-workbench-productivity`

包含：

- R19 数据库对象树
- R80 数据库级对象浏览
- R20 多 SQL 标签页
- R21 结果面板标签
- R22 查询消息
- R23 执行计划入口
- R24 结果搜索
- R25 结果导出
- R26 全屏结果视图
- R27 事务提交入口
- R28 查询历史
- R45 搜索对象入口
- R47 过滤入口
- R53 数据库版本显示
- R54 编辑器光标位置显示
- R55 命令面板入口
- R85 数据库关系图与 D2 展示
- R73 函数/过程对象索引
- R74 函数/过程查看与编辑
- R77 表数据 CRUD
- R83 表分区查看

### change 8: `add-sql-import-export`

包含：

- R75 SQL 文件导入执行
- R76 SQL/DDL 导出

### change 9: `add-appearance-settings`

包含：

- R29 设置页面入口
- R30 主题切换
- R31 布局密度设置
- R32 侧栏宽度设置
- R33 编辑器字体设置
- R34 编辑器字号和行高
- R35 快捷键展示
- R36 快捷键档案
- R37 配置应用与放弃
- R39 亮色主题设计系统
- R40 高对比红黑主题

### change 10: `add-secure-and-legacy-connectivity`

包含：

- R41 SQL Server 2000 兼容提示
- R42 SSL 开关
- R43 SSH 隧道开关
- R52 加密状态显示

## 待确认问题

这些点原型有暗示，但不足以直接定死：

- 首个真实驱动选 SQLite、PostgreSQL 还是 SQL Server。
- 产品是桌面 GUI、Web 本地应用、TUI，还是 CLI 加 GUI 混合。
- 连接密码使用系统钥匙串、加密文件还是外部 secret provider。
- Query Console 是否需要一开始支持事务模式。
- 执行计划第一版只做文本，还是直接做可视化树。
- SQL 补全底层使用 Monaco、CodeMirror、Tree-sitter 还是自研轻量解析层。
- 除 `zh-CN` 和 `en-US` 外，第二批语言是否需要 `ja-JP`、`ko-KR` 或其他语言。
- Support 和 Cloud Sync 是否属于首个商业化版本。
- `aktsql`/`aktsql` CLI 是否只是启动应用，还是也要提供查询和连接管理命令。
