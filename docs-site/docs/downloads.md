# 下载

稳定下载链接指向 GitHub 最新 Release 的安装包。推送版本标签后，CI 会自动构建并附加这些文件。

```sh
git tag v0.1.0
git push origin v0.1.0
```

<section class="download-grid">
  <a class="download-card" href="https://github.com/AktSQL/aktsql/releases/latest/download/AktSQL-windows-x64.exe">
    <h3>Windows EXE</h3>
    <p><code>AktSQL-windows-x64.exe</code></p>
  </a>
  <a class="download-card" href="https://github.com/AktSQL/aktsql/releases/latest/download/AktSQL-windows-x64.msi">
    <h3>Windows MSI</h3>
    <p><code>AktSQL-windows-x64.msi</code></p>
  </a>
  <a class="download-card" href="https://github.com/AktSQL/aktsql/releases/latest/download/AktSQL-macos.dmg">
    <h3>macOS DMG</h3>
    <p><code>AktSQL-macos.dmg</code></p>
  </a>
  <a class="download-card" href="https://github.com/AktSQL/aktsql/releases/latest/download/AktSQL-linux-x86_64.AppImage">
    <h3>Linux AppImage</h3>
    <p><code>AktSQL-linux-x86_64.AppImage</code></p>
  </a>
</section>

## CI 产物

Release workflow 会在三个平台构建：

- Windows: `.exe` 与 `.msi`
- macOS: `.dmg`
- Linux: `.AppImage`

带版本号的副本也会附加到同一个 GitHub Release，便于归档和校验。
