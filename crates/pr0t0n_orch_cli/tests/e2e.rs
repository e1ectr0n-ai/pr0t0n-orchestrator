//! Simple websocket client.
use actix_rt;
use actix_web::{web, App, HttpServer};
use awc::Client;
use futures::StreamExt;

use pr0t0n_orch::websocket::ws_index;
use pr0t0n_orch_db::{PR0T0N_ASSET_GROUP_ID_HEADER, PR0T0N_CLIENT_ADDRESS_HEADER};

#[actix_rt::test]
async fn e2e_test() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let host = "127.0.0.1:8080";

    // Create a server.
    HttpServer::new(|| {
        App::new()
            // websocket route
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
    })
    // start http server on 127.0.0.1:8080
    .bind(host)
    .unwrap()
    .run();

    // Connect a client to it.
    let res = Client::new()
        .ws(format!("http://{}/ws/", host))
        .set_header(PR0T0N_ASSET_GROUP_ID_HEADER, "0")
        .set_header(PR0T0N_CLIENT_ADDRESS_HEADER, "localhost:1234")
        .connect()
        .await
        .map_err(|e| {
            println!("Error: {}", e);
        });
    println!("{:?}", res);
    let (response, framed) = res.unwrap();
    println!("{:?}", response);

    let (_sink, _stream) = framed.split();
    // let addr = ChatClient::create(|ctx| {
    //     ChatClient::add_stream(stream, ctx);
    //     ChatClient {
    //         sink: SinkWrite::new(sink, ctx),
    //     }
    // });

    // addr.send(ClientCommand("Test".to_string())).await.unwrap();
}
