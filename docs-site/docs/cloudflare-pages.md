# Cloudflare Pages

AktSQL publishes the official site as a Cloudflare Pages project. This creates a
Cloudflare Pages domain such as:

```text
aktsql.pages.dev
```

## Pages deployment

GitHub Actions builds VuePress, publishes the static output to `gh-pages` for a
plain static artifact branch, then deploys the same output to Cloudflare Pages:

```sh
npx wrangler pages deploy docs/.vuepress/dist \
  --project-name aktsql \
  --branch main
```

The Pages project name is:

```text
aktsql
```

Keep this project name unchanged so the default Pages domain remains
`aktsql.pages.dev`.

Required GitHub secrets:

- `CLOUDFLARE_API_TOKEN`
- `CLOUDFLARE_ACCOUNT_ID`

After the first successful deployment, Cloudflare will show the project under
**Workers & Pages -> Pages**, and the default domain will be available from the
project overview.

## Static artifact branch

The `gh-pages` branch is an orphan branch containing only built static files:

- `index.html`
- `assets/`
- `screenshots/`
- generated VuePress pages

No Rust source code or application workspace files should live in `gh-pages`.

## Alternative direct build

Cloudflare Pages can also build from `main` directly:

- Root directory: `docs-site`
- Build command: `npm install && npm run docs:build`
- Build output directory: `docs/.vuepress/dist`

Use the branch-based mode for deterministic releases.
