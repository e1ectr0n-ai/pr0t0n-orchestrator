use std::collections::HashMap;

use actix::prelude::{Actor, Context, Handler, Message as ActixMessage, Recipient};
use pr0t0n_orch_db::{get_conn, models::Service, Error, PgPool};
use serde::{Deserialize, Serialize};
use serde_json::{error::Result as SerdeResult, to_string, Value};

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message sent back to clients for config updates.
#[derive(ActixMessage, Deserialize, Serialize)]
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

struct Session {
    addr: Recipient<Message>,
}
impl Session {
    fn new(addr: Recipient<Message>) -> Self {
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

    fn send_to_client_addr(&self, addr: &str, data: SerdeResult<String>) {
        if let Some(session) = self.sessions.get(addr) {
            if let Ok(ref data) = data {
                match session.addr.do_send(Message(data.clone())) {
                    Err(err) => {
                        error!("Error sending client message: {:?}", err);
                    }
                    _ => {}
                }
            }
        } else {
            warn!("Could not find session by client addr: {}", addr);
        }
    }
}
impl Actor for Server {
    type Context = Context<Self>;
}

#[derive(ActixMessage, Debug)]
#[rtype(result = "Result<(), Error>")]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub asset_group_id: i32,
    pub client_addr: String,
}
impl Handler<Connect> for Server {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Result<(), Error> {
        println!("Got message {:?}", msg);
        self.sessions
            .insert(msg.client_addr.clone(), Session::new(msg.addr));

        let conn = get_conn(&self.pool)?;
        info!("Inserting...");
        Service::upsert_healthy_address(&conn, msg.asset_group_id, &msg.client_addr)?;
        info!("Inserted.");
        Ok(())
    }
}

#[derive(ActixMessage)]
#[rtype(result = "Result<(), Error>")]
pub struct Disconnect {
    pub client_addr: String,
}
impl Handler<Disconnect> for Server {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Result<(), Error> {
        self.sessions.remove(&msg.client_addr);

        let conn = get_conn(&self.pool)?;

        info!("Disconnecting...");
        Service::disconnect_address(&conn, &msg.client_addr)?;
        info!("Disconnected.");
        Ok(())
    }
}

impl Handler<MessageToClient> for Server {
    type Result = ();

    fn handle(&mut self, msg: MessageToClient, _: &mut Context<Self>) -> Self::Result {
        self.send_to_client_addr(&msg.addr, to_string(&msg));
    }
}

// #[cfg(test)]
// mod tests {
//     use actix_web::client::Client;
//     use actix_web_actors::ws;
//     use futures::{SinkExt, StreamExt};

//     // use pr0t0n_orch_db::{get_conn, new_pool};

//     use crate::testing::{get_test_server, get_websocket_frame_data};

//     #[actix_rt::test]
//     async fn test_ws_auth_broadcasts_users() {
//         // let pool = new_pool();
//         // let conn = get_conn(&pool).unwrap();

//         // let game: Game = diesel::insert_into(games::table)
//         //     .values(NewGame {
//         //         slug: "abc123".to_string(),
//         //     })
//         //     .get_result(&conn)
//         //     .unwrap();

//         let srv = get_test_server();

//         let client = Client::default();
//         let mut ws_conn = client.ws(srv.url("/ws/")).connect().await.unwrap();

//         // let user: User = diesel::insert_into(users::table)
//         //     .values(NewUser {
//         //         game_id: game.id,
//         //         user_name: "agmcleod".to_string(),
//         //     })
//         //     .get_result(&conn)
//         //     .unwrap();

//         ws_conn
//             .1
//             .send(ws::Message::Text(format!("/auth {{\"token\":\"\"}}")))
//             .await
//             .unwrap();

//         let mut stream = ws_conn.1.take(1);

//         let msg = stream.next().await;
//         let data = get_websocket_frame_data(msg.unwrap().unwrap());
//         if data.is_some() {
//             // let msg = data.unwrap();
//             // assert_eq!(msg.path, "/players");
//             // assert_eq!(msg.game_id, game.id);
//             // let players = msg.data.as_array().unwrap();
//             // assert_eq!(players.len(), 1);
//             // let player = players.get(0).unwrap().to_owned();
//             // let player: UserDetails = serde_json::from_value(player).unwrap();
//             // assert_eq!(player.user_name, "agmcleod");
//             // assert_eq!(player.game_id, game.id);
//         } else {
//             assert!(false, "Message was not a string");
//         }

//         drop(stream);

//         srv.stop().await;
//         // Clean up
//         // diesel::delete(users::table).execute(&conn).unwrap();
//         // diesel::delete(games::table).execute(&conn).unwrap();
//     }
// }
