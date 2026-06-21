# Cloudflare Pages

AktSQL 官方站点部署为 Cloudflare Pages 项目，默认域名为：

```text
aktsql.pages.dev
```

## Pages 部署

GitHub Actions 会构建 VuePress，把静态产物同步到 `gh-pages` 分支，并部署同一份产物到 Cloudflare Pages：

```sh
npx wrangler pages deploy docs/.vuepress/dist \
  --project-name aktsql \
  --branch main
```

Pages 项目名固定为：

```text
aktsql
```

不要修改这个项目名，否则默认 Pages 域名就不再是 `aktsql.pages.dev`。

需要配置的 GitHub Actions secrets：

- `CLOUDFLARE_API_TOKEN`
- `CLOUDFLARE_ACCOUNT_ID`

## 静态产物分支

`gh-pages` 是 orphan 分支，只保存构建后的静态文件：

- `index.html`
- `assets/`
- `screenshots/`
- VuePress 生成页面

Rust 源码、Cargo workspace、Node 依赖和开发文档不应该进入 `gh-pages`。

## 直接构建备选方案

Cloudflare Pages 也可以直接从 `main` 构建：

- Root directory: `docs-site`
- Build command: `npm install && npm run docs:build`
- Build output directory: `docs/.vuepress/dist`

当前仓库采用 GitHub Actions 统一构建并部署，以保证 `gh-pages` 和 Pages 站点产物一致。
