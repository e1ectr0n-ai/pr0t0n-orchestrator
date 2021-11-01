# Pr0t0n Orchestrator Database

PostgreSQL database for Pr0t0n Orchestrator.

## Notes

- `.env` contains config for the database connection used by Diesel.
- `./migrations/` contains SQL scripts for migrating the database.
- `./src/models.rs/` contains Rust structs for interacting with the models.
- `./src/schema.rs` contains Rust macros for the defining the table schema.

## Development

To run unit tests, run:

```
cargo test -- --test-threads 1
```

We force a single thread to prevent locking in the database.

## Common flows

The system needs to be able to execute the following flows:

- Basic CRUD on all entities.
- Query the entire setup for an asset group.
- Config sync:
  - For each input device, query all services that it needs to upload to. Update it's config based on those connections.
  - For each service, query all services and output devices it needs to forward to.

Should we keep input devices, output devices, and services separate?
They all share common fields such as their address and need similar configs regarding health monitoring.

## Windows Setup

1. Install PostgreSQL from https://www.enterprisedb.com/downloads/postgres-postgresql-downloads. Use the default install options.
1. Add `C:\Program Files\PostgreSQL\14\lib` and `C:\Program Files\PostgreSQL\14\bin` to your `PATH`.
1. Install Diesel CLI.
   ```
   cargo install diesel_cli --no-default-features --features postgres
   ```
