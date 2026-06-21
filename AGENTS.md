# Akt Agent Guide

## 1. Project Snapshot

This project is intended to become **AktSQL Database Management**.
The product short name is `Akt`.

Current repository state:
- The repo contains project notes, OpenSpec configuration, Cursor helper assets, CodeGraph metadata, and the initial Rust desktop application scaffold.
- The committed implementation stack is Rust with iced for the desktop UI.
- The first app crate is `crates/aktsql_app`, with the Cargo package and binary named `aktsql`.

## 2. Product Notes

Known project intent from `README.md`:
- Primary CLI command: `aktsql`
- Short CLI alias: not defined yet.
- Candidate config files:
  - `.aktsql.toml`
  - `aktsql.config.json`
- Branding:
  - Main visual: red cloud
  - Colors: black and red
  - Tone: mysterious and powerful
- Candidate slogans: `Dawn of Database Management`, `Powerful as the Organization`

## 3. Repository Map

Important paths:
- `README.md` - current product notes.
- `Cargo.toml` - Rust workspace configuration.
- `crates/aktsql_app/` - initial iced desktop application.
- `docs/product/aktsql-requirement-slices.md` - product requirement slices derived from prototypes.
- `openspec/config.yaml` - OpenSpec configuration.
- `.cursor/commands/` and `.cursor/skills/` - Cursor and OpenSpec helpers.
- `.codegraph/` - local CodeGraph index files.
- `AGENTS.md` - this guide.

## 4. CodeGraph

This project has a CodeGraph MCP server configured.
Use CodeGraph for structural code questions once source code exists.

Prefer these tools:
- `codegraph_files` for project file structure.
- `codegraph_context` for broad task context.
- `codegraph_search` for symbol definitions.
- `codegraph_node` for one symbol's signature or source.
- `codegraph_callers`, `codegraph_callees`, `codegraph_trace`, and `codegraph_impact` for relationships and change impact.

Use native shell search only for literal text queries, generated files, or files not represented in CodeGraph.

## 5. OpenSpec Workflow

The repository is configured for spec-driven work via `openspec/config.yaml`.
For substantial features, clarify or create an OpenSpec change before implementing source code.

Use OpenSpec-style work when:
- Adding the first application scaffold.
- Choosing the implementation stack.
- Defining CLI commands and flags.
- Defining database connection behavior.
- Adding config file semantics.
- Introducing persisted state, plugins, or migrations.

Small documentation edits can be made directly.

## 6. Implementation Rules

Implementation stack:
- Rust workspace.
- iced desktop UI.
- Follow Cargo workspace conventions.
- Keep UI, application state, theme, database drivers, metadata, query execution, SQL tooling, and configuration in separate modules or crates as they grow.
- Preserve Unix-style boundaries: small modules, explicit state transitions, text-friendly configuration, and testable core logic that is not tied to the UI framework.

## 7. Editing Rules

Follow the existing repo shape and keep changes scoped.

- Prefer small, direct edits.
- Avoid unrelated formatting churn.
- Preserve user changes in the working tree.
- Use structured parsers or standard tools when modifying structured files.
- Keep comments useful and sparse.
- Use ASCII unless a file already needs non-ASCII text.

## 8. Verification

Current useful checks are:

```sh
cargo fmt --check
cargo check
rg --files -uu
```

Run the desktop app:

```sh
cargo run -p aktsql
```

## 9. Git State

`.git/` exists. Preserve user changes and avoid reverting unrelated work.

## 10. First Scaffold Checklist

The first scaffold has selected Rust + iced. Before implementing database features, decide:
- Supported database engines for the first version.
- Driver crate strategy and optional feature layout.
- Configuration file precedence.
- Secure credential storage approach.
- Test strategy for database integrations.
