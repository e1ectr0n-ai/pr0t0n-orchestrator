use actix::*;

use pr0t0n_orch_db::{
    establish_connection,
    models::{DbInsert, NewAssetGroup},
};

fn main() {
    ::std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let sys = System::new("websocket-client");

    // Create a new adgroup for testing.
    let conn = establish_connection();
    let _asset_group = NewAssetGroup {
        name: "temp_asset_group",
        description: "A test asset group",
    }
    .insert(&conn)
    .unwrap();

    sys.run().unwrap();
}
