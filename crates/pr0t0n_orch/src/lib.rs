//! Pr0t0n Orchestrator.
pub mod errors;
pub use errors::Error;
pub mod sync;
pub mod websocket;

#[macro_use]
extern crate log;

pub mod testing;

use actix_web::web::{self, Data};
use pr0t0n_orch_db::PgPool;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ws/").route(web::get().to(websocket::ws_index)));
    cfg.service(web::resource("/config/sync").route(web::post().to(sync::config_sync)));
}
