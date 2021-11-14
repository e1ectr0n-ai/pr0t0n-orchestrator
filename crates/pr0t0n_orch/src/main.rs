//! Simple echo websocket server.
use actix::Actor;
use actix_web::{middleware, App, HttpServer};
use pr0t0n_orch::{routes, websocket};
use pr0t0n_orch_db::{create_pool, Pool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Set environment vars...");
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    println!("Init logger...");
    env_logger::init();

    println!("Create DB pool...");
    let pool: Pool = create_pool(4).unwrap();
    println!("Create server...");
    let server = websocket::Server::new(pool.clone()).start();

    println!("Run HTTP Server...");
    HttpServer::new(move || {
        println!("Initialize app...");
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
