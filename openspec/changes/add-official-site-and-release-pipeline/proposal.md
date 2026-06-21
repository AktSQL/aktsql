# Add Official Site and Release Pipeline

## Why

AktSQL needs an official product page for screenshots, installation guidance,
and release downloads. The desktop app also needs repeatable cross-platform
artifacts for Windows, macOS, and Linux.

## What Changes

- Add a VuePress official site under `docs-site/`.
- Publish the built static site to a `gh-pages` branch for Cloudflare Pages.
- Add release CI for Windows `.exe` and `.msi`, macOS `.app` and `.dmg`, and Linux `.AppImage`.
- Keep packaging scripts outside the Rust app crate.

## Impact

- Adds Node/VuePress tooling for documentation only.
- Adds GitHub Actions workflows.
- Adds packaging scripts and installer metadata.
