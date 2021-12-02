use actix::prelude::Addr;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::Error;

mod server;
pub use server::*;
mod session;
use session::WebSocketSession;

use pr0t0n_orch_db::{PR0T0N_ASSET_GROUP_ID_HEADER, PR0T0N_CLIENT_ADDRESS_HEADER};

fn get_header_str<'a>(req: &'a HttpRequest, key: &str) -> Option<&'a str> {
    req.headers().get(key)?.to_str().ok()
}

fn get_conn_headers<'a>(req: &'a HttpRequest) -> Result<(i32, &'a str), Error> {
    match (
        get_header_str(&req, PR0T0N_ASSET_GROUP_ID_HEADER),
        get_header_str(&req, PR0T0N_CLIENT_ADDRESS_HEADER),
    ) {
        (Some(asset_group_id_str), Some(client_addr)) => {
            if let Some(asset_group_id) = asset_group_id_str.parse::<i32>().ok() {
                Ok((asset_group_id, client_addr))
            } else {
                return Err(Error::BadRequest(format!(
                    "Missing required headers {} and {}.",
                    PR0T0N_ASSET_GROUP_ID_HEADER, PR0T0N_CLIENT_ADDRESS_HEADER
                )));
            }
        }
        _ => {
            return Err(Error::BadRequest(format!(
                "Missing required headers {} and {}.",
                PR0T0N_ASSET_GROUP_ID_HEADER, PR0T0N_CLIENT_ADDRESS_HEADER
            )))
        }
    }
}

pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    server_addr: web::Data<Addr<Server>>,
) -> Result<HttpResponse, actix_web::Error> {
    let (asset_group_id, client_addr) = get_conn_headers(&req)?;
    let res = ws::start(
        WebSocketSession::new(
            server_addr.get_ref().clone(),
            client_addr.to_string(),
            asset_group_id,
        ),
        &req,
        stream,
    )?;

    Ok(res)
}
