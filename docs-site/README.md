# AktSQL Official Site

This directory contains the VuePress site for AktSQL Database Management.

```sh
cd docs-site
npm install
npm run docs:dev
```

Build static files:

```sh
npm run docs:build
```

GitHub Actions publishes `docs/.vuepress/dist` to the `gh-pages` branch. Connect
Cloudflare Pages to that branch with an empty build command and `/` as output.
