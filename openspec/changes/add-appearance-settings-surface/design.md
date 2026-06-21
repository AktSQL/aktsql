## Context

The appearance prototype is a dense workbench page with persistent shell navigation, a large title, and compact panels for theme, density, typography, and keybindings.

## Decisions

- Keep this first slice UI-only except for reusing the existing theme toggle.
- Keep configuration controls lightweight until persisted settings are introduced.
- Preserve the existing fixed-size desktop window and nonblocking update path.

## Non-Goals

- No settings persistence in this slice.
- No custom font discovery or file-system theme import.
- No keybinding remapping engine.
