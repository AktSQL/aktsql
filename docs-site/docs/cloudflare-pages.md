# Cloudflare Deployment

The repository keeps a dedicated `gh-pages` branch for static site artifacts and
also uploads Cloudflare Worker versions with Wrangler.

## Wrangler versions

GitHub Actions builds VuePress, publishes the static output to `gh-pages`, then
runs:

```sh
npx wrangler versions upload \
  --config wrangler.jsonc \
  --tag docs-<git-sha> \
  --message "Upload AktSQL official site"
```

Manual workflow runs can also deploy the uploaded version to all traffic:

```sh
npx wrangler versions deploy \
  --config wrangler.jsonc \
  --version-tag docs-<git-sha>@100% \
  --yes
```

Required GitHub secrets:

- `CLOUDFLARE_API_TOKEN`
- `CLOUDFLARE_ACCOUNT_ID`

## Static artifact branch

The `gh-pages` branch is an orphan branch containing only built static files:

- `index.html`
- `assets/`
- `screenshots/`
- generated VuePress pages

No Rust source code or application workspace files should live in `gh-pages`.

## Cloudflare Pages fallback

If Cloudflare Pages is used instead of Workers versions, connect Pages to the
`gh-pages` branch:

- Production branch: `gh-pages`
- Build command: leave empty
- Build output directory: `/`
- Root directory: leave empty

## Alternative direct build

Cloudflare can also build from `main` directly:

- Root directory: `docs-site`
- Build command: `npm install && npm run docs:build`
- Build output directory: `docs/.vuepress/dist`

Use the branch-based mode for deterministic releases.
