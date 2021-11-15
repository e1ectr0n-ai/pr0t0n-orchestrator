use std::time::SystemTime;

use actix::Actor;
use actix_http::Request;
use actix_service::Service;
use actix_web::{body::Body, dev::ServiceResponse, error::Error, test, App};
use actix_web_actors::ws;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;

use crate::{routes, websocket::Server};

#[derive(Deserialize, Serialize, Debug)]
struct CookieValue {
    identity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_timestamp: Option<SystemTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visit_timestamp: Option<SystemTime>,
}

pub async fn get_service(
) -> impl Service<Request = Request, Response = ServiceResponse<Body>, Error = Error> {
    test::init_service(
        App::new()
            .data(pr0t0n_orch_db::new_pool())
            .data(Server::new(pr0t0n_orch_db::new_pool()).start())
            .configure(routes),
    )
    .await
}

pub fn get_test_server() -> test::TestServer {
    test::start(|| {
        App::new()
            .data(pr0t0n_orch_db::new_pool())
            .data(Server::new(pr0t0n_orch_db::new_pool()).start())
            .configure(routes)
    })
}

/// Helper for HTTP GET integration tests
pub async fn test_get<R>(route: &str, token: Option<String>) -> (u16, R)
where
    R: DeserializeOwned,
{
    let mut app = get_service().await;
    let mut req = test::TestRequest::get().uri(route);
    if let Some(token) = token {
        req = req.header("Authorization", token);
    }

    let res = test::call_service(&mut app, req.to_request()).await;

    let status = res.status().as_u16();
    let body = test::read_body(res).await;
    let json_body = serde_json::from_slice(&body).unwrap_or_else(|_| {
        panic!(
            "read_response_json failed during deserialization. response: {} status: {}",
            String::from_utf8(body.to_vec())
                .unwrap_or_else(|_| "Could not convert Bytes -> String".to_string()),
            status
        )
    });

    (status, json_body)
}

/// Helper for HTTP POST integration tests
pub async fn test_post<T: Serialize, R>(route: &str, params: T, token: Option<String>) -> (u16, R)
where
    R: DeserializeOwned,
{
    let mut app = get_service().await;

    let mut req = test::TestRequest::post().set_json(&params).uri(route);
    if let Some(token) = token {
        req = req.header("Authorization", token);
    }

    let res = test::call_service(&mut app, req.to_request()).await;

    let status = res.status().as_u16();
    let body = test::read_body(res).await;
    let json_body = serde_json::from_slice(&body).unwrap_or_else(|_| {
        panic!(
            "read_response_json failed during deserialization. response: {} status: {}",
            String::from_utf8(body.to_vec())
                .unwrap_or_else(|_| "Could not convert Bytes -> String".to_string()),
            status
        )
    });

    (status, json_body)
}

pub fn get_websocket_frame_data(frame: ws::Frame) -> Option<String> {
    match frame {
        ws::Frame::Text(t) => {
            let bytes = t.as_ref();
            let data = String::from_utf8(bytes.to_vec()).unwrap();
            return Some(data);
        }
        _ => {}
    }
    None
}
