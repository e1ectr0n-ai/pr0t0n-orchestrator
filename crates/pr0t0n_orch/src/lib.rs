//! Pr0t0n Orchestrator.
pub mod errors;
pub use errors::Error;
pub mod websocket;
pub use websocket::WebSocketSession;
pub mod routes;

#[macro_use]
extern crate log;

#[cfg(test)]
pub mod testing;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ws/").route(web::get().to(websocket::ws_index)));
}
