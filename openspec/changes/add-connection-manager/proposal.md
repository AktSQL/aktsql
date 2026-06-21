## Why

Akt needs a real connection management foundation before database browsing, SQL completion, import/export, and table design can become meaningful. The first implementation should make connection parameters explicit and validated instead of hiding them behind loose strings.

## What Changes

- Add a connection manager capability for creating, editing, selecting, testing, and deleting saved database connection profiles.
- Introduce a typed connection parameter model covering common Navicat-style fields and engine-specific defaults for MySQL, PostgreSQL, and SQLite.
- Add an iced connection manager view with profile list, driver selector, editable parameter form, and status feedback.
- Keep actual network/database driver connection execution out of this change; test/save actions are local placeholders until the database execution layer is introduced.
- No breaking changes.

## Capabilities

### New Capabilities

- `connection-manager`: Database connection profile modeling, validation, and desktop UI workflow.

### Modified Capabilities

- None.

## Impact

- Affected code: `crates/aktsql_app/src/app.rs`, `crates/aktsql_app/src/ui.rs`, plus new focused modules for connection state and model definitions.
- Affected UI: Databases workspace becomes a connection manager surface rather than a placeholder.
- Affected docs/specs: OpenSpec change artifacts define the connection profile contract.
- Dependencies: No new runtime dependency in this change.
