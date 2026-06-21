# Design

## Navigation Model

The left side is an object locator, not a duplicate app menu. It presents saved connections, connected databases, and object groups such as tables, views, functions, indexes, and collections. Selecting an object updates the workbench context.

Top-level tools remain separate:

- Query: ad hoc SQL/command execution.
- History: execution and mutation history.
- Settings: application preferences.

## Create Operations

Create operations use forms and execute directly:

- The form owns required and optional parameters.
- Submit validates parameters before issuing the database command.
- Success keeps the user in the database workbench, selects the newly created object when possible, and refreshes the tree.
- Failure keeps the form open and shows the database error.

SQL preview is not the primary create workflow. Generated SQL can be recorded in logs/details later for auditability.

## Confirmation Boundary

Non-destructive creation can execute directly after validation. Destructive or hard-to-reverse operations require confirmation:

- DROP DATABASE and DROP TABLE.
- TRUNCATE TABLE with database-native semantics where supported.
- DROP COLUMN, DROP INDEX, and DROP CONSTRAINT.
- Replace or alter functions/procedures.

Confirmations must name the target object and avoid generic "Are you sure?" prompts.

## Driver Boundary

Direct create database execution is available only where a native driver path exists. Unsupported drivers must return a clear unavailable message instead of generating SQL preview or pretending to execute.
