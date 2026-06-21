## ADDED Requirements

### Requirement: SQL Query State

The system SHALL model SQL console state separately from iced rendering.

#### Scenario: SQL draft is edited

- **WHEN** the user changes the SQL input
- **THEN** the application stores the SQL draft without changing connection profile fields

### Requirement: SQLite Query Execution

The system SHALL execute SQL against the current SQLite connection profile.

#### Scenario: User executes a SQLite query returning rows

- **WHEN** the current connection driver is SQLite
- **AND** the SQL statement returns rows
- **THEN** the system opens the SQLite database file and displays result columns, rows, elapsed time, and row count

#### Scenario: User executes a SQLite statement without result rows

- **WHEN** the current connection driver is SQLite
- **AND** the SQL statement is DDL or DML without result rows
- **THEN** the system reports affected rows and elapsed time

#### Scenario: User executes empty SQL

- **WHEN** the SQL draft is empty or whitespace
- **THEN** the system reports a validation error and does not execute a database statement

#### Scenario: User executes a non-SQLite profile

- **WHEN** the current connection driver is not SQLite
- **THEN** the system reports that query execution is not yet wired for that driver

### Requirement: Query Console UI

The system SHALL provide an iced query console view in the Query Explorer workspace.

#### Scenario: User opens Query Explorer

- **WHEN** Query Explorer is selected
- **THEN** the workspace shows active connection context, SQL input, execute action, result preview, and execution messages

#### Scenario: Query result updates global status

- **WHEN** a query execution finishes successfully
- **THEN** the status bar shows the latest row count and elapsed time

### Requirement: SQLite Schema Browser

The system SHALL load SQLite schema objects for the active connection on demand.

#### Scenario: User refreshes Query Explorer schema

- **WHEN** the current connection driver is SQLite
- **AND** the user refreshes the Query Explorer schema
- **THEN** the system lists tables, views, and indexes from `sqlite_master`

#### Scenario: User selects a schema object

- **WHEN** the user selects a table or view from the schema browser
- **THEN** the SQL editor is populated with a preview query for that object

#### Scenario: User refreshes schema for unsupported driver

- **WHEN** the current connection driver is not SQLite
- **THEN** the system reports that schema browsing is not yet wired for that driver

### Requirement: Responsive Query Operations

The system SHALL keep Query Explorer interactions responsive while database work is in progress.

#### Scenario: Query execution is started

- **WHEN** the user starts query execution
- **THEN** the app returns immediately to the event loop and applies the result after the task finishes

#### Scenario: Schema refresh is started

- **WHEN** the user starts schema refresh
- **THEN** the app returns immediately to the event loop and applies the schema after the task finishes

#### Scenario: Operation is already running

- **WHEN** a query execution or schema refresh is already running
- **THEN** duplicate clicks do not enqueue another identical operation
