# AktSQL 官方站点

本目录包含 AktSQL Database Management 的 VuePress 官网。

```sh
cd docs-site
npm install
npm run docs:dev
```

构建静态文件：

```sh
npm run docs:build
```

GitHub Actions 会将 `docs/.vuepress/dist` 发布到组织 Pages 仓库
`AktSQL/aktsql.github.io`，因此 GitHub Pages 地址为：

```text
https://aktsql.github.io/
```

同一份静态产物也会部署到 Cloudflare Pages 项目 `aktsql`，访问地址为：

```text
https://aktsql.pages.dev/
```
