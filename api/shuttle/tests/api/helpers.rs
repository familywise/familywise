use axum::body::Body;
use cordial::prelude::{Host, Polite};
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;

pub const HOST: &str = "https://familywise.shuttleapp.rs";
pub const LOCAL: &str = "http://localhost:8000";

#[derive(Clone)]
pub struct Voice {
    pub client: reqwest::Client,
}

impl Voice {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .cookie_store(true)
                .build()
                .unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct WildHost {
    pub host: Host,
    pub voice: Client<HttpConnector, Body>,
}

impl WildHost {
    pub async fn from_env() -> Polite<Self> {
        let host = Host::from_env().await?;
        let voice = Client::builder(hyper_util::rt::TokioExecutor::new()).build_http();
        Ok(Self { host, voice })
    }
}
