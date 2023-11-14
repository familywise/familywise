use crate::prelude::*;
use axum::body::Body;
use axum::http::{Request, StatusCode};
// use shared::prelude::*;
use tower::ServiceExt;
use tracing::{info, trace};

pub async fn check(app: TestApp) {
    trace!("Calling health check.");
    let url = format!("{}/health", HOST);

    let request = Request::builder()
        .method("GET")
        // .header("Content-Type", "application/json")
        .uri(&url)
        .body(Body::empty())
        .unwrap();

    let response = app.router().oneshot(request).await.unwrap();
    info!("{:#?}", &response);
    assert!(&response.status().is_success());
    assert_eq!(&response.status(), &StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: String = serde_json::from_slice(&body).unwrap();
    info!("Body: {:#?}", &body);
    // assert!(res.status().is_success());
    // assert_eq!(api_lib::state::API_VERSION, res.headers()["family_server"]);
}
