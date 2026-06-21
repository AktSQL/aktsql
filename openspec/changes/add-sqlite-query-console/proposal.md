## Why

Akt has a connection manager and real SQLite connection testing, but Query Explorer still behaves like a placeholder. The next product slice should provide the first end-to-end query workflow: edit SQL, execute it, and inspect results.

## What Changes

- Add a query console capability with SQL draft state, execution feedback, and result display.
- Execute SQL against the current SQLite connection profile using the existing `rusqlite` dependency.
- Replace the Query Explorer placeholder with a desktop console surface modeled after the prototype.
- Keep non-SQLite execution explicitly unsupported until driver-specific execution layers are added.

## Capabilities

### New Capabilities

- `query-console`: SQL editing, SQLite execution, and result/message display.

### Modified Capabilities

- None.

## Impact

- Affected code: `crates/aktsql_app/src/app.rs`, `crates/aktsql_app/src/main.rs`, `crates/aktsql_app/src/ui.rs`, plus a new focused query module.
- Affected UI: Query Explorer becomes a working SQL console surface.
- Dependencies: Uses existing `rusqlite`.
