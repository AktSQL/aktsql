# Cloudflare Pages

AktSQL 官方站点部署为 Cloudflare Pages 项目，默认域名为：

```text
aktsql.pages.dev
```

## GitHub Pages

GitHub Pages 根域名由组织页仓库提供：

```text
AktSQL/aktsql.github.io
```

GitHub Actions 会构建 VuePress，并把静态产物发布到该仓库的 `main` 分支。因此 GitHub Pages 地址是：

```text
https://aktsql.github.io/
```

需要配置的 GitHub Actions secret：

- `PAGES_DEPLOY_TOKEN`

该 token 必须能写入 `AktSQL/aktsql.github.io` 仓库。

## Cloudflare Pages 部署

GitHub Actions 会部署同一份静态产物到 Cloudflare Pages：

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

## 静态产物仓库

`AktSQL/aktsql.github.io` 只保存构建后的静态文件：

- `index.html`
- `assets/`
- `screenshots/`
- VuePress 生成页面

Rust 源码、Cargo workspace、Node 依赖和开发文档不应该进入该仓库。

## 直接构建备选方案

Cloudflare Pages 也可以直接从 `main` 构建：

- Root directory: `docs-site`
- Build command: `npm install && npm run docs:build`
- Build output directory: `docs/.vuepress/dist`

当前仓库采用 GitHub Actions 统一构建并部署，以保证 GitHub Pages 和 Cloudflare Pages 站点产物一致。
