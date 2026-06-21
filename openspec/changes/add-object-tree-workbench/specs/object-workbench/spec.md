# object-workbench Specification

## ADDED Requirements

### Requirement: Object Tree Navigation

The system SHALL use the left database sidebar as an object tree rooted at connections.

#### Scenario: Connected user enters database workbench

- **GIVEN** a saved connection is connected
- **WHEN** the connection succeeds or the user switches to another saved connection
- **THEN** the application opens the database workbench
- **AND** the left object tree shows databases or objects available through the current connection
- **AND** database nodes own database-level actions
- **AND** table and collection nodes own table-level actions

### Requirement: Create Database Direct Execution

The system SHALL create databases from a validated form without routing the user through SQL preview.

#### Scenario: Supported driver creates a database

- **GIVEN** a connected driver supports direct query execution
- **AND** the create database form has a valid database name
- **WHEN** the user submits the form
- **THEN** the system executes the create command directly
- **AND** the query editor content is not replaced with generated SQL
- **AND** the workbench refreshes the database/object tree after success

#### Scenario: Unsupported driver cannot execute create database

- **GIVEN** the selected driver does not yet have a native execution path
- **WHEN** the user submits the create database form
- **THEN** the system shows a clear unsupported message
- **AND** the system does not generate SQL preview as a fallback

### Requirement: Destructive Operation Confirmation

The system SHALL require explicit confirmation before destructive schema or data operations.

#### Scenario: User requests destructive operation

- **GIVEN** a selected database object
- **WHEN** the user requests DROP, TRUNCATE, or destructive ALTER behavior
- **THEN** the system asks for confirmation naming the target object
- **AND** the operation does not execute until confirmation is accepted

### Requirement: Database Object Actions

The system SHALL expose database-level actions from the database node context menu.

#### Scenario: User views database details

- **GIVEN** a selected database node
- **WHEN** the user requests database details
- **THEN** the right workbench shows read-only database details
- **AND** the detail surface does not expose editable controls
- **AND** the detail surface contains only metadata for the selected database
- **AND** table detail and alter-table panels are cleared from the active inspector context
- **AND** table lists and table action panels are not rendered inside the database-detail surface
- **AND** the detail surface loads live metadata when the driver supports metadata queries
- **AND** details are grouped with high-frequency core fields first and lower-frequency storage, object, and runtime fields after them
- **AND** each parent group has a distinct visual marker

#### Scenario: User renames a database

- **GIVEN** a selected database node
- **WHEN** the user requests rename database
- **THEN** the system opens a rename dialog with the current database name and a new name field
- **AND** submitting the dialog uses the driver-specific rename strategy for PostgreSQL/CockroachDB, MySQL-compatible databases, MongoDB collections, SQLite files, SQL Server, and Oracle
- **AND** MySQL-compatible database rename builds its table migration plan from live `information_schema` metadata rather than cached tree rows
- **AND** the schema tree refreshes after success

#### Scenario: User changes database charset

- **GIVEN** a selected database node
- **WHEN** the user requests database charset change
- **THEN** the system opens a charset/collation dialog
- **AND** submitting the dialog executes the charset change directly where the driver supports it
- **AND** the schema tree refreshes after success

#### Scenario: User creates a table from database context

- **GIVEN** a selected database node
- **WHEN** the user requests create table
- **THEN** the system opens a create-table dialog using that database as the target context
- **AND** the dialog exposes a multi-column field grid, index grid, constraint grid, and driver-aware table options such as engine, charset, collation, and comment when applicable
- **AND** submitting the dialog executes the create-table command directly
- **AND** the query editor content is not replaced with generated SQL
- **AND** the schema tree refreshes after success

#### Scenario: User renames a table

- **GIVEN** a selected table or collection node
- **WHEN** the user requests rename table
- **THEN** the system opens a rename dialog with the current table name and a new name field
- **AND** submitting the dialog uses the driver-specific table or collection rename statement
- **AND** the schema tree refreshes after success

#### Scenario: User browses table rows

- **GIVEN** a selected table or collection node
- **WHEN** the user requests row browsing
- **THEN** the system executes a driver-specific row query directly
- **AND** the query editor content is not replaced with generated SQL
- **AND** the result grid shows 100 rows by default
- **AND** the result grid renders all returned columns instead of truncating to a fixed column count
- **AND** the result grid remains horizontally scrollable when returned columns exceed the viewport width
- **AND** pagination controls are shown below the result grid

#### Scenario: User describes a table

- **GIVEN** a selected table or collection node
- **WHEN** the user requests table structure
- **THEN** the right workbench loads read-only table metadata directly
- **AND** the detail surface contains only metadata for the selected table or collection
- **AND** database detail panels are cleared from the active inspector context
- **AND** database lists and database action panels are not rendered inside the table-detail surface
- **AND** the detail surface groups core, storage, object, and runtime metadata consistently with database details
- **AND** the surface includes columns, indexes, and the table CREATE statement when the driver can provide or reconstruct it
- **AND** columns and indexes remain horizontally scrollable when metadata fields exceed the viewport width

#### Scenario: User alters a table

- **GIVEN** a selected table node
- **WHEN** the user requests structure modification
- **THEN** the right workbench opens an alter-table designer panel instead of a modal dialog
- **AND** the designer is organized into concise tabs for columns, indexes, constraints, and DDL
- **AND** column data type uses driver-aware select options instead of an unstructured text field
- **AND** column placement uses explicit FIRST, AFTER, or LAST position controls
- **AND** the query editor content is not replaced with generated SQL
- **AND** the form can submit supported driver-specific operations directly, including rename column, add column, add index, and MySQL-compatible column movement
- **AND** unsupported operations report why they cannot run instead of falling back to SQL preview

#### Scenario: Object workbench copy follows i18n

- **GIVEN** the user changes the application language
- **WHEN** object workbench labels, tabs, action buttons, tree node kind labels, and metadata headers are rendered
- **THEN** the user-facing copy is read from the i18n catalog
- **AND** the default language is Simplified Chinese
