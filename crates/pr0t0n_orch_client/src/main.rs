use std::{io, thread};

use actix::io::SinkWrite;
use actix::*;
use awc::Client;
use futures::StreamExt;

use pr0t0n_orch::websocket::{PR0T0N_ASSET_GROUP_ID_HEADER, PR0T0N_CLIENT_ADDRESS_HEADER};
use pr0t0n_orch_client::{ChatClient, ClientCommand};
use pr0t0n_orch_db::{
    establish_connection,
    models::{DbInsert, NewAssetGroup},
};

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let sys = System::new("websocket-client");

    // Create a new adgroup for testing.
    let conn = establish_connection();
    let asset_group = NewAssetGroup {
        name: "temp_asset_group",
        description: "A test asset group",
    }
    .insert(&conn)
    .unwrap();

    let asset_group_id_string: String = asset_group.asset_group_id.to_string();
    Arbiter::spawn(async move {
        let (response, framed) = Client::new()
            .ws("http://127.0.0.1:8080/ws/")
            .set_header(PR0T0N_ASSET_GROUP_ID_HEADER, asset_group_id_string.clone())
            .set_header(PR0T0N_CLIENT_ADDRESS_HEADER, "localhost:1234")
            .connect()
            .await
            .map_err(|e| {
                println!("Error: {}", e);
            })
            .unwrap();

        println!("{:?}", response);
        let (sink, stream) = framed.split();
        let addr = ChatClient::create(|ctx| {
            ChatClient::add_stream(stream, ctx);
            ChatClient {
                sink: SinkWrite::new(sink, ctx),
            }
        });

        // start console loop
        thread::spawn(move || loop {
            let mut cmd = String::new();
            if io::stdin().read_line(&mut cmd).is_err() {
                println!("error");
                return;
            }
            addr.do_send(ClientCommand(cmd));
        });
    });
    sys.run().unwrap();
}
