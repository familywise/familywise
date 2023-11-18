use api_lib::prelude::*;
use axum::body::Body;
use axum::http::{Request, Response};
use axum::routing::{get, post, Router};
use fake::{Fake, Faker};
use reqwest::Client;
use secrecy::ExposeSecret;
use shared::prelude::*;
use tower::ServiceExt;
use tracing::info;
// use once_cell::sync::Lazy;

pub fn new_user() -> User {
    let username = Faker.fake::<String>();
    let password_hash = Faker.fake::<String>();
    User::new(&username, &password_hash)
}

pub const HOST: &str = "https://familywise.shuttleapp.rs";
pub const LOCAL: &str = "http://localhost:8000";

#[derive(Clone)]
pub struct TestClient {
    client: reqwest::Client,
}

impl TestClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .cookie_store(true)
                .build()
                .unwrap(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn client_mut(&mut self) -> &mut Client {
        &mut self.client
    }
}

/// The `TestApp` struct
#[derive(Clone)]
pub struct TestApp {
    router: Router,
    pub client: TestClient,
}

impl TestApp {
    pub async fn new() -> Self {
        if let Ok(()) = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init()
        {};
        let settings = DatabaseSettings::from_env().unwrap();
        info!(
            "Connection: {}",
            &settings.connection_string().expose_secret()
        );
        settings.try_delete_db().await;
        settings.create_db().await;
        let db_pool = settings.get_connection_pool();
        let router = AppState::new(db_pool).app();
        let client = TestClient::new();
        TestApp { router, client }
    }

    pub fn router(&self) -> Router {
        self.router.clone()
    }

    pub fn client_ref(&self) -> &Client {
        self.client.client()
    }

    pub async fn create_db(&self) {
        let settings = DatabaseSettings::from_env().unwrap();
        settings.create_db().await;
    }

    pub async fn delete_db(&self) {
        let settings = DatabaseSettings::from_env().unwrap();
        settings.delete_db().await;
    }
}
