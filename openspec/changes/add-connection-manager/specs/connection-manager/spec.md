## ADDED Requirements

### Requirement: Connection Profile Model

The system SHALL model database connection profiles with typed driver selection and explicit parameter fields instead of opaque connection strings.

#### Scenario: Supported initial drivers are available

- **WHEN** the connection manager is shown
- **THEN** the user can choose MySQL, MariaDB, TiDB, OceanBase, PostgreSQL, CockroachDB, Greenplum, SQLite, DuckDB, SQL Server, SQL Server 2000, Oracle, Db2, Informix, Sybase ASE, Firebird, ClickHouse, Redis, MongoDB, Cassandra, Elasticsearch, Snowflake, BigQuery, Redshift, Trino, Hive, or Databricks as the connection driver

#### Scenario: Driver defaults are applied

- **WHEN** the user switches the connection driver
- **THEN** the form updates port, host/path, charset, and collation defaults for that driver

### Requirement: Connection Parameter Completeness

The system SHALL expose the baseline connection parameters required for professional database clients: profile name, host or file path, port, username, password, database name, charset, collation, SSL toggle, SSH tunnel toggle, timeout, and notes.

#### Scenario: MySQL default charset and collation

- **WHEN** a MySQL connection profile is created
- **THEN** the default charset is `utf8mb4`
- **THEN** the default collation is `utf8mb4_bin`

#### Scenario: SQLite uses a file path

- **WHEN** SQLite is selected as the driver
- **THEN** the form treats the location field as a database file path instead of a network host

### Requirement: Local Connection Validation

The system SHALL validate connection form input before reporting a profile as locally valid or saving it.

#### Scenario: Missing required values are rejected

- **WHEN** the user tests or saves a connection profile with a missing profile name or missing host/path
- **THEN** the system reports validation errors and does not mark the profile as valid

#### Scenario: Invalid port is rejected

- **WHEN** the user enters a non-numeric or out-of-range port for a network database driver
- **THEN** the system reports a port validation error

### Requirement: Connection Manager UI

The system SHALL provide an iced desktop connection manager view in the Databases workspace.

#### Scenario: User opens Databases workspace

- **WHEN** the Databases section is selected
- **THEN** the workspace shows a connection list and editable connection form

#### Scenario: User tests a local profile

- **WHEN** the user clicks Test
- **THEN** the app validates the current form and reports whether the profile is locally valid

#### Scenario: User tests a SQLite profile

- **WHEN** the user clicks Test for a valid SQLite profile
- **THEN** the app opens the SQLite database file and reports the real connection result

#### Scenario: User saves a profile

- **WHEN** the user clicks Save
- **THEN** the app validates the current form and stores the profile in memory if validation succeeds
