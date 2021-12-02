#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
extern crate dotenv;

use r2d2::{Pool, PooledConnection};

use diesel::{pg::PgConnection, prelude::*, r2d2::ConnectionManager};
use dotenv::dotenv;
use std::env;

pub mod errors;
pub use errors::Error;
pub mod models;
mod schema;

pub mod testing;
// The Postgres-specific connection pool managing all database connections.
pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub const PR0T0N_ASSET_GROUP_ID_HEADER: &str = "pr0t0n-asset-group-id";
pub const PR0T0N_CLIENT_ADDRESS_HEADER: &str = "pr0t0n-client-address";

pub fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, r2d2::Error> {
    pool.get().map_err(|err| {
        error!("Failed to get connection - {}", err.to_string());
        err.into()
    })
}

pub fn new_pool() -> PgPool {
    // TODO: pass the connection URL into this function rather than extracting
    // it from the environment within this function
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL was not set");
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .max_size(1) // Error with libpq requires this to be 1 for now: https://github.com/diesel-rs/diesel/discussions/2947.
        .build(manager)
        .expect("Failed to build connection pool")
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pool = new_pool();
        let _conn = pool.get().unwrap();
    }
}
