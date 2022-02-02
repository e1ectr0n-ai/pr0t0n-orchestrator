//! Pr0t0n Orchestrator.
pub mod errors;
pub use errors::Error;
pub mod sync;
pub mod websocket;

#[macro_use]
extern crate log;

pub mod testing;

use actix_web::{web, Responder};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)));
    cfg.service(web::resource("/ws/").route(web::get().to(websocket::ws_index)));
    cfg.service(web::resource("/sync/upload/").route(web::post().to(sync::upload)));
    cfg.service(web::resource("/sync/download/").route(web::get().to(sync::download)));
}

pub async fn index(// mut system: web::Json<SystemRepr>,
    // pool: Data<PgPool>,
) -> Result<impl Responder, Error> {
    println!("Log index!");
    Ok("Index")
}
