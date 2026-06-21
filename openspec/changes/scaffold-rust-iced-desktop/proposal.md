## Why

Akt needs its first committed application stack so the product can move from prototype documents to an actual desktop database manager. Rust with iced gives the project a native, robust foundation aligned with the requirement for stability, clear module boundaries, and a compact workbench UI.

## What Changes

- Add a Rust workspace and initial `aktsql` desktop application crate.
- Use iced as the desktop UI framework.
- Implement the first application shell: top bar, sidebar navigation, main workspace placeholder, and status bar.
- Add theme scaffolding for Akt's dark red/black identity and a light-mode placeholder.
- Add module boundaries for app state, theme, UI shell, and future database capabilities.
- Add basic verification commands for formatting, checking, and running the desktop app.

## Capabilities

### New Capabilities

- `desktop-app-shell`: Rust/iced desktop application shell, navigation layout, status bar, theme foundation, and stable module boundaries.

### Modified Capabilities

- None.

## Impact

- Adds Rust/Cargo project files and source code.
- Adds iced as the first application framework dependency.
- Updates repository documentation with build, run, and verification commands.
- Establishes the initial architecture that later changes will extend for connections, query console, table designer, and database metadata.
