## 1. Connection Model

- [x] 1.1 Add a focused connection module with driver enum, form state, saved profile, defaults, and validation.
- [x] 1.2 Wire connection state into the application state without mixing UI rendering into the model.

## 2. Application Messages

- [x] 2.1 Add messages for editing connection fields, selecting drivers, testing, saving, creating, selecting, and deleting profiles.
- [x] 2.2 Update the app reducer so connection actions update form/list/status deterministically.

## 3. Desktop UI

- [x] 3.1 Replace the Databases placeholder with a connection manager surface.
- [x] 3.2 Add connection list, driver selector, parameter fields, toggles, validation feedback, and action buttons.

## 4. Verification

- [x] 4.1 Run Rust formatting and compiler checks.
- [x] 4.2 Confirm OpenSpec reports the change artifacts and task list as complete.

## 5. Connector Interface

- [x] 5.1 Add a focused connector interface and local tester for connection test flow handoff.
- [x] 5.2 Add real SQLite connection testing through rusqlite.

## 6. Product Polish

- [x] 6.1 Fix fixed-size window behavior and remove maximize from the custom title bar.
- [x] 6.2 Restore the prototype logo and normalize title/sidebar copy for later i18n.
- [x] 6.3 Expand the connection driver list to include broad market coverage including SQL Server 2000.
- [x] 6.4 Recalibrate Connections Settings against the prototype with fixed logo spacing, narrower form width, underline-style inputs, switch-like connection options, and a delete action.
- [x] 6.5 Reduce Connections layout drift by centering the bounded settings form in the right work area, converting profile cards to single-click rows, trimming card height, and fixing the app window to its prototype size.
- [x] 6.6 Align the Databases sidebar/header with the connection-manager prototype by removing duplicate shell controls and using the prototype-style connection list header.
- [x] 6.7 Tighten the Connections screen against `stitch_aktsql_database_management` by matching the 347px list column, visible form fields, bordered inputs, pill toggles, action row spacing, legacy warning block, and logo asset proportions.
- [x] 6.8 Recalibrate the desktop shell to the prototype screenshot scale by using a 1600x1264 fixed window, 60px top bar, 30px footer, 300px sidebar, 434px connection list, seeded prototype profiles, and scaled form/list typography.
- [x] 6.9 Return the app to the original 1280x800 fixed window and scale the prototype-aligned shell back down proportionally while retaining seeded prototype profiles and layout structure.
- [x] 6.10 Compress the 1280x800 Connections layout for Windows-targeted iced rendering by reducing vertical form density, forcing prototype profiles ahead of persisted profiles, clipping the title-bar logo, and screenshot-checking the result.
- [x] 6.11 Add a working connection list filter cycle and soften border treatments by reducing default outline opacity while preserving active/hover contrast.
- [x] 6.12 Continue Connections functionality by adding list search, persisted-profile reload, context-aware top-bar actions, visible advanced connection parameters, and softer underline-style form inputs.
- [x] 6.13 Convert charset from free text to a driver-aware dropdown with per-database option sets and model-side normalization.
- [x] 6.14 Link charset and collation dropdowns so collation options update by selected charset, including expanded MySQL utf8mb4 collations such as `utf8mb4_0900_ai_ci`, `utf8mb4_general_ci`, and `utf8mb4_unicode_ci`.
- [x] 6.15 Expand MySQL-family charset coverage and add regression checks for charset/collation linkage and normalization.
- [x] 6.16 Polish the custom title bar with real minimize/close commands, i18n switching, normal Akt casing, aligned branding, MySQL default draft state, pick-list menu padding, and a black/red app frame.
- [x] 6.17 Add bundled CJK font rendering, expand the language cycle to EN/ZH/JA/KO, localize the main visible UI surfaces, and recenter the red-cloud logo asset itself.
- [x] 6.18 Replace language cycling with an explicit EN/FR/DE/RU/AR/ZH/JA/KO dropdown, remove global i18n state, bundle Latin/Arabic/CJK font coverage, make the logo transparent red-cloud only, and replace text radio glyphs with fixed aligned controls.
- [x] 6.19 Trim database choices to mainstream engines plus SQLite file profiles, replace the crashing language pick-list overlay with an in-app language tray, limit i18n choices to EN/FR/RU/ZH/JA, and restyle placeholder workspaces to match the connection surface.
- [x] 6.20 Restore the language selector to a compact dropdown shape without using the crashing native pick-list overlay, and rebuild the brand mark as one container with red cloud, fixed spacing, and Akt text.
- [x] 6.21 Start structural cleanup by splitting database driver metadata, seeded connection profiles, language identity, shell/title-bar UI, and generic placeholder workspace code into focused Rust submodules.
