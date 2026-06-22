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

GitHub Actions 会将 `docs/.vuepress/dist` 部署到 Cloudflare Pages 项目
`aktsql`，访问地址为：

```text
https://aktsql.pages.dev/
```
