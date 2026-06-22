项目名：Akt
全称：AktSQL Database Management
桌面应用显示名：Akt
缩写：Akt

Logo 设计：
- 主图案：红云（晓组织标志）
- 颜色：黑色+红色
- 风格：神秘、强大

Slogan：
- "Dawn of Database Management"（数据库管理的黎明）
- "Powerful as the Organization"（如组织般强大）

命令行工具：
$ aktsql
暂不定义短别名

配置文件：
.aktsql.toml
aktsql.config.json

需求文档：
- [原型需求切片](docs/product/aktsql-requirement-slices.md)
- [数据库最大公约数与产品亮点](docs/product/database-common-denominator-highlights.md)
- [新增表与修改表设计计划](docs/product/table-create-alter-plan.md)

技术栈：
- Rust workspace
- iced desktop UI
- `docs-site/` 中的 VuePress 官方站点

启动桌面应用：
```sh
cargo run -p aktsql
```

如果只想确认能否编译：
```sh
cargo check --workspace --all-targets
```

验证：
```sh
cargo fmt --check
cargo check --workspace --all-targets
cargo test --workspace
```

官网：
```sh
cd docs-site
npm install
npm run docs:dev
```

发布：
- 官网由 GitHub Actions 构建后部署到 Cloudflare Pages 项目 `aktsql`，默认域名为 `https://aktsql.pages.dev/`。
- 桌面版本在推送 `v*` tag 后构建 Windows `.exe/.msi`、macOS `.dmg`、Linux `.AppImage`。
