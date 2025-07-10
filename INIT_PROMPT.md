We're going to define this project. As we go add updates to README.md. Be concise. Ask clarifying questions.

Do not implement anything. Only add to README.md

Schema is defined in `SCHEMA.md`. You may suggest schema changes to optimize the library.

Source data is in `weapons.json`.

This project exposes a rust library with an optional binary.

The library has methods for querying the weapons database via sqlx these methods are exposed as MCP Tool definitions.

The library also exposes methods to populate the database from `weapons.json`.

It also exposes a binary which applies the database schema to a postgres database.

The binary may also use the population methods exposed by the library.

Always recreate the entire database on schema or data change.

Expect to be running on top of a postgres docker image https://hub.docker.com/_/postgres

The library and main method accept connection parameters. Accept connection params as individual args e.g. (--host --port).

Create custom error types for Error Handling.
