use std::time::Duration;

use actix::clock::delay_for;
use actix_web::client::Client;
// use actix_web_actors::ws;
use futures::{SinkExt, StreamExt};
use pr0t0n_orch_db::{
    get_conn,
    models::{AssetGroup, DbDelete, DbInsert, HealthStatus, NewAssetGroup, Service},
    new_pool,
};

use pr0t0n_orch::{
    testing::{get_test_server, get_websocket_frame_data},
    Error,
};
use pr0t0n_orch_db::{PR0T0N_ASSET_GROUP_ID_HEADER, PR0T0N_CLIENT_ADDRESS_HEADER};

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
    let addresses: Vec<String> = (1..1000).map(|i| format!("localhost:{}", i)).collect();
    let mut futures = Vec::with_capacity(addresses.len());
    for addr in &addresses {
        let client = Client::default();
        futures.push(
            client
                .ws(server.url("/ws/"))
                .set_header(PR0T0N_ASSET_GROUP_ID_HEADER, asset_group_id_string.clone())
                .set_header(PR0T0N_CLIENT_ADDRESS_HEADER, addr.clone())
                .connect(),
        );
    }

    let mut streams = Vec::with_capacity(addresses.len());
    for future in futures {
        let (_response, sock) = future.await.unwrap();
        let mut stream = sock.take(1);
        let msg = stream.next().await;
        let data = get_websocket_frame_data(msg.unwrap().unwrap());
        assert_eq!(data, Some("Registered".to_string()));
        streams.push(stream);
    }

    // After connecting, make sure we added an entry to the database.
    for addr in &addresses {
        let service = Service::find_by_addr(&conn, addr)?;
        assert_eq!(service.health_status, HealthStatus::Healthy);
    }

    let mut close_futures = Vec::with_capacity(addresses.len());
    for stream in &mut streams {
        // After disconnecting, make sure we removed the entry from the database.
        close_futures.push(stream.close())
    }

    for future in close_futures {
        future.await.unwrap();
    }

    delay_for(Duration::from_secs_f32(2.)).await; // Let the sockets time out
    for addr in &addresses {
        let service = Service::find_by_addr(&conn, addr)?;
        assert_eq!(service.health_status, HealthStatus::Disconnected);
    }

    // Clean up.
    server.stop().await;
    AssetGroup::delete(&conn, asset_group.asset_group_id)?;
    Ok(())
}
