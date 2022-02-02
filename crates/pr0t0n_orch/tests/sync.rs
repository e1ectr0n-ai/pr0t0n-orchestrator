use actix_web::{http::StatusCode, test};
use assert_json_diff::assert_json_eq;
use pr0t0n_orch::{testing::get_service, Error};
use pr0t0n_orch_db::{
    get_conn,
    models::{
        AssetGroup, ConfigRepr, DbDelete, DbInsert, GetGroupRequest, NewAssetGroup, ServiceRepr,
        ServiceType, SystemRepr,
    },
    new_pool,
};

#[actix_rt::test]
async fn test_response() -> Result<(), Error> {
    let mut app = get_service().await;

    // Setup database connection and server.
    let pool = new_pool();
    let conn = get_conn(&pool)?;

    // Create a new adgroup for testing.
    let asset_group = NewAssetGroup {
        name: "temp_asset_group",
        description: "A test asset group",
    }
    .insert(&conn)?;
    let asset_group_id: i32 = asset_group.asset_group_id;

    let system_repr: SystemRepr = SystemRepr {
        asset_group_id,
        services: vec![
            ServiceRepr {
                address: "localhost:123".to_string(),
                service_type: ServiceType::Input,
                name: "localhost:123".to_string(),
                output_addresses: vec!["localhost:234".to_string()],
                config_name: Some("TestConfig".to_string()),
                ..Default::default()
            },
            ServiceRepr {
                address: "localhost:234".to_string(),
                service_type: ServiceType::Input,
                name: "localhost:234".to_string(),
                output_addresses: vec![],
                config_name: Some("TestConfig".to_string()),
                ..Default::default()
            },
        ],
        configs: vec![ConfigRepr {
            name: "TestConfig".to_string(),
            description: "A test config".to_string(),
            json_config: serde_json::from_str(r#"{ "key": "value" }"#)?,
            ..Default::default()
        }],
    };

    // Test upload
    {
        let request = test::TestRequest::post()
            .uri("/sync/upload/")
            .set_json(&system_repr)
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    // Test download
    {
        let get_group_request = GetGroupRequest { asset_group_id };
        let request = test::TestRequest::get()
            .uri("/sync/download/")
            .set_json(&get_group_request)
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body = test::read_body(response).await;
        let response_system_repr: SystemRepr = serde_json::from_slice(&body)?;
        assert_json_eq!(system_repr, response_system_repr);
    }

    AssetGroup::delete(&conn, asset_group.asset_group_id)?;
    println!("Deleted asset group");
    Ok(())
}
