## Context

The prototype console shows a dense SQL editor, active connection metadata, result grid, execution status, and footer metrics. The current app has the surrounding shell and connection profiles, but no query execution state.

## Goals / Non-Goals

**Goals:**

- Keep query state and execution out of the iced UI layer.
- Execute SQLite statements using the active connection draft/profile.
- Show columns, rows, affected rows, elapsed time, and error messages.
- Update global rows/latency status after execution.
- Keep the Query Explorer surface visually aligned with the prototype.
- Browse SQLite schema objects from the Query Explorer sidebar.
- Keep query execution and schema refresh off the immediate UI update path.

**Non-Goals:**

- No SQL syntax highlighting, completion, formatting, multi-tabs, transactions, or execution plans.
- No network database execution for MySQL, PostgreSQL, SQL Server, Oracle, etc.
- No editable data grid.

## Decisions

- **Use a focused `query` module.** It owns SQL draft state, result state, row cap, validation, and SQLite execution.
- **Execute against the active connection form.** Loading a profile already copies it into the active form, so Query Explorer can use the same connection context shown in the sidebar.
- **Stringify values for the first grid.** Typed cells and binary inspectors can be introduced with the future data-grid slice.
- **Cap result rows.** The first implementation limits returned rows to keep the fixed-size UI responsive.
- **Refresh schema explicitly.** The first schema browser is driven by the top/sidebar refresh actions to avoid opening database files on every draft edit.
- **Run database work through iced tasks.** Query execution and schema refresh start a background task and apply results only when the task completes, so UI events remain responsive.

## Risks / Trade-offs

- **Plain SQL editor is not a full IDE editor** -> acceptable for the first working slice; syntax highlighting and completion can layer onto the current text editor later.
- **SQLite-only execution is narrow** -> status messages make this explicit and prevent false expectations for the broad driver list.
- **Concurrent clicks can overload slow database files** -> execution and schema refresh use running-state guards so repeated clicks do not enqueue duplicate jobs.
