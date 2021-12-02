use std::time::Duration;

use actix::clock::delay_for;
use actix_web::client::Client;
use actix_web_actors::ws;
use futures::{SinkExt, StreamExt};
use pr0t0n_orch_db::{
    get_conn,
    models::{AssetGroup, DbDelete, DbInsert, HealthStatus, NewAssetGroup, Service},
    new_pool, PR0T0N_ASSET_GROUP_ID_HEADER, PR0T0N_CLIENT_ADDRESS_HEADER,
};

use pr0t0n_orch::{
    testing::{get_test_server, get_websocket_frame_data},
    Error,
};

#[actix_rt::test]
async fn test_ws() -> Result<(), Error> {
    // Setup database connection and server.
    let pool = new_pool();
    let conn = get_conn(&pool)?;
    let server = get_test_server();

    // Create a new adgroup for testing.
    let asset_group = NewAssetGroup {
        name: "temp_asset_group",
        description: "A test asset group",
    }
    .insert(&conn)?;
    let asset_group_id_string: String = asset_group.asset_group_id.to_string();

    // Attach a client.
    let addr = "localhost:1235";
    let client = Client::default();
    let (_response, mut sock) = client
        .ws(server.url("/ws/"))
        .set_header(PR0T0N_ASSET_GROUP_ID_HEADER, asset_group_id_string.clone())
        .set_header(PR0T0N_CLIENT_ADDRESS_HEADER, addr)
        .connect()
        .await
        .unwrap();

    // Send a message after connecting and get the response.
    sock.send(ws::Message::Text("Connected".to_string()))
        .await
        .unwrap();

    let mut stream = sock.take(1);
    let msg = stream.next().await;
    let data = get_websocket_frame_data(msg.unwrap().unwrap());
    assert_eq!(data, Some("Registered".to_string()));

    // After connecting, make sure we added an entry to the database.
    {
        let service = Service::find_by_addr(&conn, addr)?;
        assert_eq!(service.health_status, HealthStatus::Healthy);
    }

    // After disconnecting, make sure we removed the entry from the database.
    stream.close().await.unwrap();
    delay_for(Duration::from_secs_f32(0.2)).await; // Let the socket time out
    {
        let service = Service::find_by_addr(&conn, addr)?;
        assert_eq!(service.health_status, HealthStatus::Disconnected);
    }

    // Clean up.
    server.stop().await;
    AssetGroup::delete(&conn, asset_group.asset_group_id)?;
    Ok(())
}
