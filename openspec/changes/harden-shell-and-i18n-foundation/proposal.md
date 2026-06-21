## Why

The desktop shell must remain stable before larger Navicat-like workflows are added. Recent UI feedback shows that fixed-size window behavior, connection form bounds, persisted configuration, and language-ready copy need a tighter foundation now.

## What Changes

- Harden the fixed-size iced window so standard close/minimize behavior remains available while the content area stays at the prototype-aligned size.
- Add explicit layout constraints for connection manager forms, lists, and status text so controls do not overflow the fixed window.
- Move connection profile persistence toward an application config location instead of relying only on the process working directory.
- Expand the i18n foundation so user-facing shell and connection manager copy can be centralized before Chinese/English switching is implemented.
- No breaking changes.

## Capabilities

### New Capabilities

- `shell-stability-and-i18n`: Desktop shell stability, fixed-window layout constraints, configuration persistence path behavior, and language-ready UI text boundaries.

### Modified Capabilities

- None.

## Impact

- Affected code: `crates/aktsql_app/src/main.rs`, `crates/aktsql_app/src/ui.rs`, `crates/aktsql_app/src/i18n.rs`, `crates/aktsql_app/src/persistence.rs`, and application reducer status messages.
- Affected UI: fixed desktop window, native window controls, connection manager layout, status bar text, and theme/status labels.
- Affected data: connection profiles continue to omit passwords, but config save/load should prefer a stable per-user Akt location when available.
- Dependencies: may add a small cross-platform config directory helper if the standard library alone would produce fragile paths.
