use std::collections::HashMap;

use actix::prelude::{Actor, Context, Handler, Message, Recipient};
use pr0t0n_orch_db::{get_conn, models::Service, Error, PgPool};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct TextMessage(pub String);

struct Session {
    addr: Recipient<TextMessage>,
}
impl Session {
    fn new(addr: Recipient<TextMessage>) -> Self {
        Session { addr }
    }
}

/// Server for managing websockets.
pub struct Server {
    pool: PgPool,
    sessions: HashMap<String, Session>,
}
impl Server {
    pub fn new(pool: PgPool) -> Self {
        Server {
            pool,
            sessions: HashMap::new(),
        }
    }

    fn send_to_client(&self, addr: &str, data: TextMessage) {
        info!("Sending to client: '{}'", data.0);
        if let Some(session) = self.sessions.get(addr) {
            match session.addr.do_send(data) {
                Err(err) => {
                    error!("Error sending client message: {:?}", err);
                }
                _ => {}
            }
        } else {
            warn!("Could not find session by client addr: {}", addr);
        }
    }
}
impl Actor for Server {
    type Context = Context<Self>;
}

#[derive(Message, Debug)]
#[rtype(result = "Result<(), Error>")]
pub struct ConnectMessage {
    pub addr: Recipient<TextMessage>,
    pub asset_group_id: i32,
    pub client_addr: String,
}
impl Handler<ConnectMessage> for Server {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: ConnectMessage, _: &mut Context<Self>) -> Result<(), Error> {
        info!("Receieved {:?}", msg);
        self.sessions
            .insert(msg.client_addr.clone(), Session::new(msg.addr));

        let conn = get_conn(&self.pool)?;
        Service::upsert_healthy_address(&conn, msg.asset_group_id, &msg.client_addr)?;
        self.send_to_client(&msg.client_addr, TextMessage("Registered".to_string()));
        Ok(())
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), Error>")]
pub struct DisconnectMessage {
    pub client_addr: String,
}
impl Handler<DisconnectMessage> for Server {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: DisconnectMessage, _: &mut Context<Self>) -> Result<(), Error> {
        self.sessions.remove(&msg.client_addr);

        let conn = get_conn(&self.pool)?;
        Service::disconnect_address(&conn, &msg.client_addr)?;
        info!("Service {} was disconnected.", &msg.client_addr);
        Ok(())
    }
}

/// Message sent back to clients for config updates.
#[derive(Message, Deserialize, Serialize, Debug)]
#[rtype(result = "()")]
pub struct MessageToClient {
    pub addr: String,
    pub data: Value,
}
impl MessageToClient {
    pub fn new(addr: &str, data: Value) -> Self {
        Self {
            addr: addr.to_string(),
            data,
        }
    }
}
impl Handler<MessageToClient> for Server {
    type Result = ();

    fn handle(&mut self, msg: MessageToClient, _: &mut Context<Self>) -> Self::Result {
        self.send_to_client(&msg.addr, TextMessage(msg.data.to_string()));
    }
}
