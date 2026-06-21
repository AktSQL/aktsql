# Cloudflare Pages

The repository publishes the static site to a dedicated `gh-pages` branch.
Cloudflare Pages should be connected to that branch.

## Recommended Cloudflare settings

- Production branch: `gh-pages`
- Build command: leave empty
- Build output directory: `/`
- Root directory: leave empty

The `gh-pages` branch contains already-built VuePress static files. This keeps
Cloudflare Pages simple and makes GitHub Actions the single build point.

## Alternative direct build

Cloudflare Pages can also build from `main` directly:

- Root directory: `docs-site`
- Build command: `npm install && npm run docs:build`
- Build output directory: `docs/.vuepress/dist`

Use the branch-based mode for deterministic releases.
