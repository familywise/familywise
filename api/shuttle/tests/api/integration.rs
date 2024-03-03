use crate::prelude::*;
use cordial::prelude::Polite;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::test]
async fn integration() -> Polite<()> {
    match tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cordial=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
    {
        Ok(_) => {}
        Err(_) => {}
    };
    let app = WildHost::from_env().await?;
    let mut labels = std::collections::HashMap::new();
    labels.insert(LOCAL, "local");
    labels.insert(HOST, "remote");
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let uri = format!("http://{}", addr);
    labels.insert(&uri, "local");
    let envs = vec![uri.as_str()];

    tokio::spawn(async move {
        axum::serve(listener, app.host.bearing()).await.unwrap();
    });

    for env in envs {
        info!("Checking {} health.", labels[env]);
        health(env, app.voice.clone()).await?;
        info!("{} health check successful.", labels[env]);

        info!("Checking {} book.", labels[env]);
        book(env, app.voice.clone()).await?;
        info!("{} book check successful.", labels[env]);
    }

    // info!("Testing {} user lifecycle.", labels[env]);
    // user_lifecycle(&app, env).await;
    // info!("Test of {} user lifecycle successful.", labels[env]);
    // }
    Ok(())
}
