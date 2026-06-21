## Why

The prototype includes an Appearance Settings workspace, but the Rust desktop app still shows a generic placeholder for Settings. The app needs a real settings surface so the shell feels cohesive and can later persist appearance preferences.

## What Changes

- Replace the Settings placeholder with an Appearance Settings workspace.
- Add prototype-aligned panels for theme, density, typography, keybindings, and configuration actions.
- Reuse the existing theme toggle path for the first Apply Configuration interaction.

## Impact

- Affected code: `crates/aktsql_app/src/ui.rs`
- Affected specs: `appearance-settings`
