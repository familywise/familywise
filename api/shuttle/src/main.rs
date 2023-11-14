use anyhow::anyhow;
use api_lib::prelude::*;
use shuttle_runtime::CustomError;
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};
use tracing::info;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    let secret = if let Some(pass) = secret_store.get("ADMIN_PASSWORD") {
        pass
    } else {
        return Err(anyhow!("Password was not found").into());
    };
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;
    let router = AppState::new(pool).app();

    info!("🚀 Server started successfully");
    Ok(router.into())
}
