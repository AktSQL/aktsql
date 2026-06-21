# 下载 AktSQL

稳定下载链接指向 GitHub 最新 Release 的安装包。当前版本重新发布时会覆盖同名资产，因此官网链接始终保持不变。

```sh
git tag v0.1.0
git push origin v0.1.0
```

<section class="download-section">
  <div class="download-group">
    <div>
      <h2>Windows</h2>
      <p>x64 与 ARM64 都提供 EXE 和 MSI 安装包。</p>
    </div>
    <div class="download-grid">
      <a class="download-card" href="https://github.com/AktSQL/aktsql/releases/latest/download/AktSQL-windows-x64.exe">
        <span>Windows x64</span>
        <strong>EXE 安装包</strong>
        <code>AktSQL-windows-x64.exe</code>
      </a>
      <a class="download-card" href="https://github.com/AktSQL/aktsql/releases/latest/download/AktSQL-windows-x64.msi">
        <span>Windows x64</span>
        <strong>MSI 安装包</strong>
        <code>AktSQL-windows-x64.msi</code>
      </a>
      <a class="download-card" href="https://github.com/AktSQL/aktsql/releases/latest/download/AktSQL-windows-arm64.exe">
        <span>Windows ARM64</span>
        <strong>EXE 安装包</strong>
        <code>AktSQL-windows-arm64.exe</code>
      </a>
      <a class="download-card" href="https://github.com/AktSQL/aktsql/releases/latest/download/AktSQL-windows-arm64.msi">
        <span>Windows ARM64</span>
        <strong>MSI 安装包</strong>
        <code>AktSQL-windows-arm64.msi</code>
      </a>
    </div>
  </div>

  <div class="download-group">
    <div>
      <h2>Linux</h2>
      <p>x86_64 与 aarch64 AppImage。aarch64 面向 64 位 ARM Linux，包括树莓派 64 位系统。</p>
    </div>
    <div class="download-grid">
      <a class="download-card" href="https://github.com/AktSQL/aktsql/releases/latest/download/AktSQL-linux-x86_64.AppImage">
        <span>Linux x86_64</span>
        <strong>AppImage</strong>
        <code>AktSQL-linux-x86_64.AppImage</code>
      </a>
      <a class="download-card" href="https://github.com/AktSQL/aktsql/releases/latest/download/AktSQL-linux-aarch64.AppImage">
        <span>Linux aarch64</span>
        <strong>AppImage</strong>
        <code>AktSQL-linux-aarch64.AppImage</code>
      </a>
    </div>
  </div>

  <div class="download-group">
    <div>
      <h2>macOS</h2>
      <p>Apple Silicon 版本，适用于 M 系列与 ARM 架构 macOS 设备。</p>
    </div>
    <div class="download-grid">
      <a class="download-card" href="https://github.com/AktSQL/aktsql/releases/latest/download/AktSQL-macos-arm64.dmg">
        <span>macOS arm64</span>
        <strong>DMG 安装包</strong>
        <code>AktSQL-macos-arm64.dmg</code>
      </a>
    </div>
  </div>
</section>

## CI 产物

Release workflow 会在三个平台构建并上传：

- Windows x64: `.exe`、`.msi`
- Windows ARM64: `.exe`、`.msi`
- Linux x86_64: `.AppImage`
- Linux aarch64: `.AppImage`
- macOS arm64: `.dmg`

带版本号的副本也会附加到同一个 GitHub Release，便于归档和校验。
