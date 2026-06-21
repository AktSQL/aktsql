# Getting Started

AktSQL is a desktop database management tool built with Rust and iced.

## Supported database families

- MySQL, MariaDB, and TiDB
- PostgreSQL and CockroachDB
- MongoDB
- SQLite
- SQL Server
- Oracle

## Local build

```sh
cargo run -p aktsql
```

For a fast compile check:

```sh
cargo check -p aktsql --all-targets
```

## Basic workflow

1. Open the connection screen.
2. Choose a driver and enter host, port, username, password, and database name.
3. Use **Connect** for an existing profile or **Save and Connect** for a new one.
4. Browse databases and tables from the workbench tree.
5. Open table data, table structure, database details, or query workspace actions from object menus.

## Schema editing

The table designer separates:

- Columns
- Indexes
- Constraints
- Generated DDL

Changes are submitted through direct actions. Generated SQL remains visible and
copyable where it is useful for review.
