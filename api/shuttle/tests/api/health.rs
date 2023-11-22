use crate::prelude::*;
use axum::http::StatusCode;
use tracing::trace;

pub async fn check(app: &TestApp, host: &str) {
    let client = app.client_ref();
    let url = format!("{}/health", host);
    let response = client.get(&url).send().await.unwrap();

    trace!("{:#?}", &response);
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        api_lib::state::API_VERSION,
        response.headers()["family_server"]
    );
}
