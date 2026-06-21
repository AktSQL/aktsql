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

GitHub Actions publishes `docs/.vuepress/dist` to the organization Pages
repository `AktSQL/aktsql.github.io`, so the GitHub Pages URL is:

```text
https://aktsql.github.io/
```

The same static output is also deployed to the Cloudflare Pages project
`aktsql`, available at:

```text
https://aktsql.pages.dev/
```
