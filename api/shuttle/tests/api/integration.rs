use crate::prelude::*;
use tracing::{info, trace};

#[tokio::test]
async fn integration() {
    let mut app = TestApp::new().await;

    info!("Local testing disabled.");
    // info!("Checking local health.");
    // check_local(&app).await;
    // info!("Testing local user lifecycle.");
    // local_user_lifecycle(&app).await;
}
