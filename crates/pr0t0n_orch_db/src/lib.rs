#[macro_use]
extern crate diesel;
extern crate dotenv;

pub use deadpool_diesel::postgres::{Manager, Pool, Runtime};
use diesel::{pg::PgConnection, prelude::*};
use dotenv::dotenv;
use std::env;
use std::marker::Send;

pub mod errors;
pub use errors::Error;
pub mod models;
mod schema;

pub mod testing;

pub fn create_pool(max_size: usize) -> Result<Pool, Error> {
    dotenv()?;
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = Manager::new(database_url, Runtime::Tokio1);
    let result = Pool::builder(manager).max_size(max_size).build();
    match result {
        Ok(pool) => Ok(pool),
        Err(_e) => Err(Error::PoolBuildError),
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub async fn query_async<
    F: FnOnce(&mut PgConnection) -> Result<R, Error> + Send + 'static,
    R: Sized + Send + 'static,
>(
    pool: Pool,
    f: F,
) -> Result<R, Error> {
    let conn = pool.get().await?;
    conn.interact(f).await?
}

#[cfg(test)]
mod tests {
    use diesel::sql_types::Text;

    use super::*;

    #[test]
    fn it_works() {
        let _conn = establish_connection();
    }

    #[tokio::test]
    async fn pool_works() -> Result<(), Error> {
        let pool = create_pool(4)?;

        let string: String = query_async(pool, |conn| {
            let query = diesel::select("Hello world!".into_sql::<Text>());
            Ok(query.get_result::<String>(conn)?)
        })
        .await?;
        println!("Result: {}", string);
        Ok(())
    }
}
