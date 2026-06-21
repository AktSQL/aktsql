## Context

The iced shell already has a prototype-aligned desktop layout, native theme switching, a connection manager surface, and local connection profile state. Recent feedback exposed two stability issues: form controls can exceed the fixed working area, and minimize behavior must remain standard while maximize/resizing stays unavailable in practice.

The next database features will add denser forms, SQL editors, data grids, import/export dialogs, and i18n. The shell therefore needs predictable layout limits, centralized text, and stable config persistence before more UI is layered on top.

## Goals / Non-Goals

**Goals:**

- Keep the application at a fixed 1280x800 content target without relying on fragile custom window controls.
- Preserve standard close and minimize behavior through native window decorations.
- Prevent the connection manager form and profile list from overflowing the fixed window.
- Centralize more shell and connection-manager copy in the i18n module.
- Save connection profiles in a stable Akt config location when the platform provides one, while keeping a working-directory fallback for development.

**Non-Goals:**

- No runtime language switcher yet.
- No password persistence or keychain integration.
- No SQL editor, autocomplete, metadata tree, table designer, or data grid implementation in this slice.
- No redesign of the prototype visual direction.

## Decisions

- **Use native decorations for window controls.** Native close/minimize is more reliable than self-drawn buttons across Linux window managers. Fixed size is enforced with equal min/max window dimensions rather than a custom maximize button.
- **Keep layout constraints local to UI construction.** Width constants for sidebar, connection list, and form bounds live near the view functions. This keeps styling and layout concerns out of connection state.
- **Continue the Unix-style module split.** `main.rs` owns window bootstrapping, `ui.rs` owns layout, `theme.rs` owns colors/styles, `i18n.rs` owns text, and `persistence.rs` owns config file I/O.
- **Do not persist secrets.** Passwords remain skipped during serialization. A later security slice can choose keychain or encrypted storage.
- **Prefer a platform config directory with fallback.** The persistence module should pick a per-user config path when available; otherwise it should keep using `aktsql.config.json` in the working directory so development remains simple.

## Risks / Trade-offs

- **Some window managers may still show a maximize button when min/max sizes are equal** -> the fixed size constraints prevent effective resize; if a target platform still exposes an active maximize affordance, revisit with platform-specific window attributes.
- **Centralizing every string at once can create churn** -> only move active shell and connection-manager copy in this slice, then continue as new surfaces are built.
- **Config path migration can strand existing local files** -> load from the stable config path first, then fall back to the working-directory file if present.
