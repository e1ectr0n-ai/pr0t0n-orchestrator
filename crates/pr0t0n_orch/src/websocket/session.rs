use std::time::{Duration, Instant};

use actix::{
    fut,
    prelude::{Actor, Addr, StreamHandler},
    ActorContext, ActorFuture, AsyncContext, ContextFutureSpawner, Handler, Running, WrapFuture,
};
use actix_web_actors::ws;

use crate::websocket::{ConnectMessage, DisconnectMessage};

use super::{Server, TextMessage};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(20);

pub struct WebSocketSession {
    server_addr: Addr<Server>,
    hb: Instant,
    client_addr: String,
    asset_group_id: i32,
}

impl WebSocketSession {
    pub fn new(server_addr: Addr<Server>, client_addr: String, asset_group_id: i32) -> Self {
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
                println!("Websocket Client heartbeat failed, disconnecting!");
                act.server_addr.do_send(DisconnectMessage {
                    client_addr: act.client_addr.clone(),
                });
                ctx.stop();
                return; // Don't send another ping if timed out.
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Started connection for {}", self.client_addr);
        self.send_heartbeat(ctx);

        let session_addr = ctx.address();

        self.server_addr
            .send(ConnectMessage {
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

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        self.server_addr.do_send(DisconnectMessage {
            client_addr: self.client_addr.clone(),
        });
        Running::Stop
    }
}

impl Handler<TextMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        debug!("Message: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => match text {
                _ => ctx.text("Received message."),
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Err(err) => {
                error!("Error handling msg: {:?}", err);
                ctx.stop()
            }
            _ => ctx.stop(),
        }
    }
}
