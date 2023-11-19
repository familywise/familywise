use crate::prelude::*;
use tracing::{info, trace};

#[tokio::test]
async fn integration() {
    let mut app = TestApp::new().await;
    info!("Checking local health.");
    check_local(&app).await;
    info!("Testing local user lifecycle.");
    user_lifecycle(&app, LOCAL).await;
    info!("Test of local user lifecycle successful.");
    info!("Testing remote user lifecycle.");
    user_lifecycle(&app, HOST).await;
    info!("Test of remote user lifecycle successful.");
    // local_user_lifecycle(&app).await;
}
