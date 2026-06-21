# 任务

## 1. UX 设计

- [x] 1.1 定义新建数据库交互流程和像素级原型。
- [x] 1.2 定义修改数据库交互流程和像素级原型。
- [x] 1.3 定义新建数据表交互流程和像素级原型。
- [x] 1.4 定义修改数据表交互流程和像素级原型。
- [x] 1.5 定义统一动作条、变更计划、执行反馈和失败反馈。
- [x] 1.6 读取全量 OpenSpec 并对齐 schema 操作粒度。
- [x] 1.7 固化 1280x800 基准下的像素级工作台规格。

## 2. 后续实现拆分

- [ ] 2.1 抽象 `SchemaDesignerState`，区分 database designer 和 table designer。
- [x] 2.2 抽象 `ChangePlan`，记录新增、修改、高风险、阻塞和元数据刷新项。
- [ ] 2.3 扩展创建数据库表单，加入驱动感知字段分组、校验和执行结果。
- [ ] 2.4 扩展数据库修改设计器，加入 General、Encoding、Objects、Change Plan、History。
- [ ] 2.5 扩展创建表设计器，加入 Fields、Indexes、Constraints、Options、Change Plan、Validation。
- [x] 2.6 扩展修改表设计器，补齐变更标记、变更计划、风险确认和局部回退。
  - [x] 2.6.1 修改表设计器显示 app 层生成的变更计划条目。
  - [x] 2.6.2 阻塞项禁止执行，高风险项进入确认弹窗后再执行。
  - [x] 2.6.3 支持回退选中变更和回退全部变更。
- [ ] 2.7 将执行结果统一接入 status/log surface，并记录 elapsed/metadata refresh latency。

## 3. 验证

- [x] 3.1 运行 `openspec validate refine-schema-operations-navicat-ux --strict`。
- [x] 3.2 检查与 `object-workbench`、`unix-workbench-ux` 的规格关系。
