use crate::prelude::*;
use axum::http::StatusCode;
use tracing::trace;

pub async fn check_local(app: &TestApp) {
    let response = app
        .client_ref()
        .get(format!("{}/health", LOCAL))
        .send()
        .await
        .unwrap();

    trace!("{:#?}", &response);
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        api_lib::state::API_VERSION,
        response.headers()["family_server"]
    );
}
