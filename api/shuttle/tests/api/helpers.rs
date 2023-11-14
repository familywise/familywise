use api_lib::prelude::*;
use axum::routing::{get, post, Router};
use fake::{Fake, Faker};
use shared::prelude::*;
// use once_cell::sync::Lazy;

pub fn new_user() -> User {
    let username = Faker.fake::<String>();
    let password_hash = Faker.fake::<String>();
    User::new(&username, &password_hash)
}

pub const HOST: &str = "https://familyserver.shuttleapp.rs";
pub const LOCAL: &str = "http://localhost:8000";

pub struct TestClient {
    client: reqwest::Client,
}

impl TestClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .cookie_store(true)
                .build()
                .unwrap(),
        }
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn client_mut(&mut self) -> &mut reqwest::Client {
        &mut self.client
    }
}

/// The `TestApp` struct
pub struct TestApp {
    router: Router,
    client: TestClient,
}

impl TestApp {
    pub async fn new() -> Self {
        let settings = DatabaseSettings::from_env().unwrap();
        settings.create_db().await;
        // settings.configure_database().await;
        let db_pool = settings.get_connection_pool();
        let router = AppState::new(db_pool).app();
        let client = TestClient::new();
        TestApp { router, client }
    }

    pub fn router(&self) -> Router {
        self.router.clone()
    }

    pub fn client_ref(&self) -> &TestClient {
        &self.client
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
