# 任务

## 1. 规格与原型

- [x] 1.1 建立基于官方文档的最大公约数 UX 设计。
- [x] 1.2 定义统一对象模型和四类数据库映射。
- [x] 1.3 定义自然操作流、危险操作确认和右侧工作台规则。
- [x] 1.4 定义系统字体发现、theme 字体差异和不打包字体策略。
- [x] 1.5 定义真实连接延迟指标和 UI 呈现位置。

## 2. 后续实现准备

- [ ] 2.1 设计系统字体发现模块接口，并移除内置字体依赖计划。
- [ ] 2.2 扩展连接测试结果模型，包含 connect/roundtrip/metadata latency。
- [ ] 2.3 扩展连接参数模型，覆盖 MySQL、PostgreSQL、MongoDB、SQL Server 的基础与专属字段。
- [ ] 2.4 设计对象路径和对象树节点类型，覆盖 SQL 与 MongoDB。
- [ ] 2.5 设计右侧工作台布局组件，统一详情、表单、查询和日志区域。
- [ ] 2.6 设计状态栏真实数据绑定，显示 driver、server version、latency、encoding、rows。

## 3. 验证

- [x] 3.1 运行 `openspec validate design-unix-workbench-ux --strict`。
- [x] 3.2 与现有 `connection-manager`、`object-workbench`、`query-console`、`appearance-settings` 规格核对冲突。
