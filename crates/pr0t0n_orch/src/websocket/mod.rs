use std::time::{Duration, Instant};

use actix::{
    fut,
    prelude::{Actor, Addr, StreamHandler},
    ActorContext, ActorFuture, AsyncContext, ContextFutureSpawner, Handler, WrapFuture,
};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
mod server;

use crate::Error;

pub use self::server::*;

pub const PR0T0N_ASSET_GROUP_ID_HEADER: &str = "pr0t0n-asset-group-id";
pub const PR0T0N_CLIENT_ADDRESS_HEADER: &str = "pr0t0n-client-address";

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

pub struct WebSocketSession {
    server_addr: Addr<Server>,
    hb: Instant,
    client_addr: String,
    asset_group_id: i32,
}

impl WebSocketSession {
    fn new(server_addr: Addr<Server>, client_addr: String, asset_group_id: i32) -> Self {
        Self {
            server_addr,
            hb: Instant::now(),
            client_addr,
            asset_group_id,
        }
    }

    fn send_heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                info!("Websocket Client heartbeat failed, disconnecting!");
                act.server_addr.do_send(Disconnect {
                    client_addr: act.client_addr.clone(),
                });
                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Send heartbeat...");
        self.send_heartbeat(ctx);

        info!("Get address...");
        let session_addr = ctx.address();

        info!("Send connect...");
        self.server_addr
            .send(Connect {
                addr: session_addr.recipient(),
                client_addr: self.client_addr.clone(),
                asset_group_id: self.asset_group_id,
            })
            .into_actor(self)
            .then(|res, _act, ctx| {
                match res {
                    Ok(_res) => {}
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
}

impl Handler<Message> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                // let message = text.trim();
                match text {
                    _ => ctx.text("Respond to client."),
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                info!("closed ws session");
                self.server_addr.do_send(Disconnect {
                    client_addr: self.client_addr.clone(),
                });
                ctx.close(reason);
                ctx.stop();
            }
            Err(err) => {
                warn!("Error handling msg: {:?}", err);
                ctx.stop()
            }
            _ => ctx.stop(),
        }
    }
}
fn get_header_str<'a>(req: &'a HttpRequest, key: &str) -> Option<&'a str> {
    req.headers().get(key)?.to_str().ok()
}

fn get_conn_headers<'a>(req: &'a HttpRequest) -> Result<(i32, &'a str), Error> {
    match (
        get_header_str(&req, PR0T0N_ASSET_GROUP_ID_HEADER),
        get_header_str(&req, PR0T0N_ASSET_GROUP_ID_HEADER),
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
    println!("Start websocket...");
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
