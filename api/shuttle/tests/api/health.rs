use crate::prelude::WildHost;
use polite::Polite;
// use axum::http::StatusCode;
use axum::http::{self, Request, StatusCode};
use axum::{body::Body, Json};
use http_body_util::BodyExt;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;
use serde_json::Value;
use tracing::{info, trace};

pub async fn health(host: &str, client: Client<HttpConnector, Body>) -> Polite<()> {
    let url = format!("{}/health", host);
    let request = Request::builder()
        .uri(&url)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())?;

    let response = client.request(request).await?;

    trace!("{:#?}", &response);
    assert_eq!(response.status(), 200);
    let body = response.into_body().collect().await?.to_bytes();
    assert!(body.is_empty());
    Ok(())
}

pub async fn book(host: &str, client: Client<HttpConnector, Body>) -> Polite<()> {
    let url = format!("{}/book", host);
    let request = Request::builder()
        .uri(&url)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::empty())?;

    let response = client.request(request).await?;

    assert_eq!(response.status(), 200);
    let body = response.into_body().collect().await?.to_bytes();
    let body = std::str::from_utf8(&body)?;
    info!("{:#?}", &body);

    // assert_eq!(response.status(), StatusCode::OK);
    // assert_eq!(
    //     api_lib::state::API_VERSION,
    //     response.headers()["family_server"]
    // );
    Ok(())
}
