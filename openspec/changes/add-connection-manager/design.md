## Context

The current Rust iced app has a desktop shell, navigation, theme switching, window controls, quick actions, and status feedback. The Databases workspace still shows a placeholder, while the product requirements call for Navicat-like connection parameter completeness and validation before real database workflows are added.

The repository has no database execution layer yet. This change therefore introduces connection profile modeling and UI state only, keeping network I/O and driver integration outside the first slice.

## Goals / Non-Goals

**Goals:**

- Add a typed connection profile model for MySQL, PostgreSQL, and SQLite.
- Expose common connection fields: name, driver, host/path, port, username, password, database, charset, collation, SSL, SSH tunnel, timeout, and notes.
- Provide stable defaults, including `utf8mb4` charset and `utf8mb4_bin` collation for MySQL-compatible database creation flows.
- Add local validation for required fields and numeric ranges.
- Replace the Databases placeholder with an iced connection manager surface.
- Keep UI state changes deterministic and testable through pure Rust data structures where possible.

**Non-Goals:**

- No real database network connection is opened in this change.
- No password persistence, encryption, keychain integration, or config file persistence is added in this change.
- No SQL metadata loading, schema tree refresh, or table autocomplete is added in this change.

## Decisions

- **Use a focused `connection` module.** Connection enums, form state, profile state, validation, and defaulting live outside `app.rs`; `app.rs` only coordinates messages and global status. This keeps the code closer to Unix philosophy: small pieces with narrow responsibilities.
- **Use driver-specific defaults now, not stringly typed fields.** MySQL, PostgreSQL, and SQLite each provide defaults through `DatabaseDriver`. This prevents later UI branching from depending on ad hoc label strings.
- **Model test/save as local validation first.** The UI will validate parameters and update status. Actual driver calls can later replace the placeholder without changing form messages.
- **Keep password visible as an ordinary string field for now.** iced text input wiring is straightforward; secure storage and masking will be handled in a dedicated persistence/security change.

## Risks / Trade-offs

- **Placeholder test action may be mistaken for real connectivity** -> status text will explicitly say the profile is locally valid and driver connection is pending.
- **Connection parameter set can grow quickly** -> this change implements the common base plus the first engine-specific defaults, leaving advanced pages such as SSL certificate files and SSH identities for later slices.
- **No persistence yet** -> saved profiles live only in app memory. This is acceptable for the UI and model slice, but persistence must follow before user-facing release.
