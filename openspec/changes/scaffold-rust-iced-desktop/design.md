## Context

The repository currently contains product notes, OpenSpec configuration, prototype assets, and requirement slicing documentation, but no application source code. The first implementation decision is now explicit: Akt will start as a Rust desktop application using iced.

The scaffold must establish a stable base without prematurely implementing all database-management features. The app should visibly match the Akt workbench direction: top navigation, left database/navigation rail, central workspace, and bottom status bar.

## Goals / Non-Goals

**Goals:**

- Create a Rust workspace with a desktop app crate.
- Use iced for the native desktop UI.
- Implement a first visible desktop shell aligned with the prototype.
- Keep module boundaries small and explicit.
- Add basic theme support for dark and light modes.
- Add commands for running, formatting, and checking the app.

**Non-Goals:**

- No real database connections in this change.
- No table designer, query execution, SQL formatting, i18n, or ER diagram generation yet.
- No secure credential storage yet.
- No plugin or extension system yet.

## Decisions

### Rust workspace with one app crate

Use a Cargo workspace with `crates/aktsql_app` as the first member.

Rationale: this keeps the repository ready for future crates such as `aktsql_core`, `aktsql_drivers`, or `aktsql_sql`, while avoiding extra abstraction before there is real code.

Alternative considered: a single root crate. That is simpler today, but it makes later separation of UI/core/driver logic more disruptive.

### iced for the desktop UI

Use iced as the first UI dependency.

Rationale: the user explicitly selected Rust iced. iced also fits a message-driven architecture, which keeps state updates explicit and testable.

Alternative considered: Tauri or egui. Tauri adds a web stack, and egui has a different immediate-mode model. Both can work, but they do not match the requested stack.

### Message-driven app shell

Keep initial state and events in an `App` type with a `Message` enum. UI functions render from immutable state and emit messages.

Rationale: this makes state transitions clear and keeps future behavior such as theme toggling, navigation, and refresh actions predictable.

### Module boundaries

Initial modules:

- `app`: application state, messages, update flow.
- `ui`: visual layout and widgets.
- `theme`: Akt palette and theme selection.

Future modules should stay narrow:

- `connection`: saved connection metadata and validation.
- `drivers`: database-specific access behind traits.
- `metadata`: object tree and schema introspection.
- `query`: query execution state.
- `sql`: formatting, dialect metadata, and completion.

## Risks / Trade-offs

- iced APIs may differ by version -> Pin a current version and keep wrapper functions local to `ui`.
- First scaffold has placeholders, not real DB behavior -> Label placeholder areas clearly in code and docs, and keep future tasks explicit.
- Native GUI dependencies may need system libraries -> Document verification commands and surface any build failures directly.
- Theme styling can sprawl -> Centralize palette values in `theme`.

## Migration Plan

1. Add Cargo workspace files and `crates/aktsql_app`.
2. Add iced app shell source modules.
3. Update repository docs with Rust/iced commands.
4. Run formatting and compile checks.

Rollback is straightforward at this stage: remove the new Cargo workspace files and app crate.

## Open Questions

- Whether the final distribution format will be raw binaries, platform installers, or a package manager target.
- Whether database drivers will live in the same workspace or separate optional crates.
- Whether SQL editor functionality will use iced-native widgets only or integrate a specialized editor component later.
