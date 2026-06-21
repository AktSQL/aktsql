# 添加官网和发布管线

## 为什么

AktSQL 需要一个官网产品页，用于展示截图、安装指引和发布下载。桌面应用也需要可重复生成的跨平台产物，覆盖 Windows、macOS 和 Linux。

## 变更内容

- 在 `docs-site/` 下添加 VuePress 官网。
- 将构建后的静态站点发布到 `gh-pages` 分支，供 Cloudflare Pages 使用。
- 添加 release CI，生成 Windows `.exe` 和 `.msi`、macOS `.app` 和 `.dmg`、Linux `.AppImage`。
- 将打包脚本保留在 Rust 应用 crate 之外。

## 影响

- 添加仅用于文档的 Node/VuePress 工具链。
- 添加 GitHub Actions workflows。
- 添加打包脚本和安装器元数据。
