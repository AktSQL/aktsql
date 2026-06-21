# Add Object Tree Workbench

## Why

The database workspace should behave like a focused Unix-style tool: the left side locates database objects, and the right side performs explicit actions on the selected object. The current flat workspace split between Databases, Query Explorer, and Tables creates redundant navigation and makes object operations feel indirect.

Create flows should also stop using SQL preview as the primary path. Users should fill a complete parameter form and execute directly, while destructive operations keep strong confirmation.

## What Changes

- Replace the flat database navigation model with an object tree: Connections -> Databases -> object groups.
- Keep Query, History, and Settings as tools outside the object tree.
- Treat the database workbench as the default landing page after connecting or switching connections.
- Convert create database flows from SQL-preview-first to direct execution from a validated form.
- Keep confirmations for destructive schema/data operations.
- Record schema mutation results in status/log surfaces without forcing the SQL editor to change.

## Impact

- Affected specs: object-workbench
- Affected code: `crates/aktsql_app/src/app.rs`, `crates/aktsql_app/src/ui.rs`, `crates/aktsql_app/src/query.rs`, `docs/product/aktsql-requirement-slices.md`
