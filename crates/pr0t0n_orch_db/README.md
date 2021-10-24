# Pr0t0n Orchestrator Database

PostgreSQL database for Pr0t0n Orchestrator.

## Notes

- `.env` contains config for the database connection used by Diesel.
- `./migrations/` contains SQL scripts for migrating the database.
- `./src/models.rs/` contains Rust structs for interacting with the models.
- `./src/schema.rs` contains Rust macros for the defining the table schema.

## Windows Setup

1. Install PostgreSQL from https://www.enterprisedb.com/downloads/postgres-postgresql-downloads. Use the default install options.
1. Add `C:\Program Files\PostgreSQL\14\lib` and `C:\Program Files\PostgreSQL\14\bin` to your `PATH`.
1. Install Diesel CLI.
   ```
   cargo install diesel_cli --no-default-features --features postgres
   ```

---

1. Install MySQL.
   1. Go to https://dev.mysql.com/downloads/installer/ to get the community build for server and client.
   1. Run `ALTER USER 'root'@'localhost' IDENTIFIED WITH mysql_native_password BY '<PASSWORD>'` to enable access by VS Code.
1. Use `vcpkg` to install `libmysql` as instructed at https://github.com/sgrif/mysqlclient-sys.
   1. Clone the vcpkg repo.
      ```
      git clone https://github.com/Microsoft/vcpkg.git
      ```
      Make sure you are in the directory you want the tool installed to before doing this.
   1. Run the bootstrap script to build `vcpkg`.
      ```
      .\vcpkg\bootstrap-vcpkg.bat
      ```
   1. Install `libmysql`.
      ```
      vcpkg install libmysql:x64-windows
      ```
1. Install Diesel CLI by running `cargo install diesel_cli --no-default-features --features mysql`.
