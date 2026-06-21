# Downloads

Release artifacts are produced by GitHub Actions when a version tag is pushed.

```sh
git tag v0.1.0
git push origin v0.1.0
```

The release workflow builds:

<section class="download-grid">
  <div class="download-card">
    <h3>Windows</h3>
    <p><code>aktsql.exe</code> and <code>AktSQL-*.msi</code></p>
  </div>
  <div class="download-card">
    <h3>macOS</h3>
    <p><code>AktSQL.app</code> archive and <code>AktSQL-*.dmg</code></p>
  </div>
  <div class="download-card">
    <h3>Linux</h3>
    <p><code>AktSQL-*.AppImage</code></p>
  </div>
</section>

Artifacts are attached to the GitHub Release for the tag. Cloudflare Pages can
link to the release page until signed auto-update channels are defined.
