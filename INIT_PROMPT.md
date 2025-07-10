We're going to define this project. As we go add updates to README.md. Be concise. Ask clarifying questions.

Do not implement anything. Only add to README.md

DO NOT READ README_old.md

# Database

Schema is defined in `SCHEMA.md`. You may suggest schema changes to optimize the library.

Source data is in `weapons.json`.

Always recreate the entire database on schema or data change.

Database is Postgres running in a container. https://hub.docker.com/_/postgres

# Rust Library

This project exposes a rust library with an optional binary.
The library and binary read connection parameters from environment variables.
Create custom error types for error handling. Connection Failures, Query Errors
Include dependencies in project definition.

## Library

### Client

The library has methods for querying the weapons database via sqlx.

Users instantiate one shared client for the application which is threadsafe and contains the sqlx connection pool.

Query methods are accessed through the shared client.

Query methods return streams immediately without blocking, deferring database execution until the stream is consumed.

### Database Management

The library also exposes methods to populate the database from `weapons.json`.

It handles migration by dropping the current database and repopulating.

To populate, iterate over `weapons.json` performing insert operations as new data is discovered.

## Rust Binary

Include a binary CLI which uses the Database Management portion of the library to:

- Clear existing data
- Apply db schema
- Populate data
