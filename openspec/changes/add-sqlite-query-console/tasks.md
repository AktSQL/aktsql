## 1. Query Model

- [x] 1.1 Add a focused query module with SQL draft state, result model, validation, and row cap.
- [x] 1.2 Execute SQLite SQL through `rusqlite` and report rows, affected count, elapsed time, and errors.

## 2. Application Wiring

- [x] 2.1 Add messages for SQL draft editing and query execution.
- [x] 2.2 Wire Query Explorer navigation, top-bar Execute, global row count, and latency to query state.

## 3. Desktop UI

- [x] 3.1 Replace the Query Explorer placeholder with a query console surface.
- [x] 3.2 Add active connection context, SQL input, execution messages, and result grid preview.

## 4. Verification

- [x] 4.1 Run Rust formatting and compiler checks.
- [x] 4.2 Confirm OpenSpec artifacts and task list are complete.

## 5. Schema Browser

- [x] 5.1 Load SQLite schema objects from `sqlite_master`.
- [x] 5.2 Wire Query Explorer refresh to schema loading.
- [x] 5.3 Populate SQL editor from selected schema objects.

## 6. Responsiveness

- [x] 6.1 Move query execution off the synchronous update path.
- [x] 6.2 Move schema refresh off the synchronous update path.
- [x] 6.3 Guard running query/schema operations from duplicate enqueue.
- [x] 6.4 Avoid rebuilding SQL text on non-edit editor actions.

## 7. Prototype Calibration

- [x] 7.1 Merge Query Explorer schema browsing into the primary 240px sidebar to match the console prototype.
- [x] 7.2 Tighten Query Console content margins so editor and results use the prototype-style workbench canvas.
- [x] 7.3 Update the status bar to expose version, logs, rows, latency, encoding, driver, and cursor context.
- [x] 7.4 Convert top refresh/download/filter controls to compact icon-like buttons.
- [x] 7.5 Add command/ctrl-enter query execution to match the settings prototype keybinding.
- [x] 7.6 Add prototype-style result row striping and status chips.
- [x] 7.7 Add nonblocking Commit feedback and F9 shortcut placeholder to match prototype controls.
- [x] 7.8 Wire result toolbar actions with search feedback, CSV export, and a result-grid focus mode for smoother large-result inspection.
