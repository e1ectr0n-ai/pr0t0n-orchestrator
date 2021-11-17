//! Simple echo websocket server.
use actix::Actor;
use actix_web::{middleware, App, HttpServer};
use pr0t0n_orch::{routes, websocket};
use pr0t0n_orch_db::{new_pool, PgPool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let pool: PgPool = new_pool();
    let server = websocket::Server::new(pool.clone()).start();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(server.clone())
            .wrap(middleware::Logger::default())
            .configure(routes)
    })
    // start http server on 127.0.0.1:8080
    .workers(4)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
