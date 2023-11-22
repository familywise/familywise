use crate::prelude::*;
use tracing::info;

#[tokio::test]
async fn integration() {
    let mut app = TestApp::new().await;
    let envs = vec![LOCAL, HOST];
    let mut labels = std::collections::HashMap::new();
    labels.insert(LOCAL, "local");
    labels.insert(HOST, "remote");

    for env in envs {
        info!("Checking {} health.", labels[env]);
        check(&app, env).await;
        info!("{} health check successful.", labels[env]);

        info!("Testing {} user lifecycle.", labels[env]);
        user_lifecycle(&app, env).await;
        info!("Test of {} user lifecycle successful.", labels[env]);
    }
}
