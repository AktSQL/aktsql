## ADDED Requirements

### Requirement: 官网静态站点
系统 SHALL 提供 AktSQL 官方静态站点，用于产品介绍、截图、安装指引和下载入口。

#### Scenario: 用户访问官网
- **WHEN** 用户打开官网首页
- **THEN** 页面展示 AktSQL 产品定位、工作台截图和下载入口
- **AND** 官网内容由 `docs-site/` 下的 VuePress 项目构建

### Requirement: 官网部署管线
系统 SHALL 通过 CI 构建官网并发布到托管站点。

#### Scenario: 官网构建发布
- **WHEN** 官网发布 workflow 运行
- **THEN** CI 构建 VuePress 静态产物
- **AND** 静态产物可发布到 GitHub Pages 或 Cloudflare Pages

### Requirement: 跨平台发布产物
系统 SHALL 为桌面应用生成可下载的跨平台安装产物。

#### Scenario: 创建 release tag
- **WHEN** 仓库推送版本 tag
- **THEN** release workflow 构建 Windows、macOS 和 Linux 产物
- **AND** 构建产物上传到对应 GitHub Release
