use super::interface::user::{FamilyResult, FamilyUser};
use crate::prelude::*;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, Router};
use axum::Json;
use shared::prelude::*;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::{info, trace};

pub const API_VERSION: &str = "v0.0.1";

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &PgPool {
        &self.db
    }

    pub fn db_mut(&mut self) -> &mut PgPool {
        &mut self.db
    }

    pub fn app(&self) -> Router {
        let app_state = Arc::new(self.clone());
        Router::new()
            .route("/health", get(check))
            .route("/api/users", get(get_users).post(create_user))
            .route(
                "/api/users/:id",
                get(get_user).put(update_user).delete(delete_user),
            )
            .route("/api/users/random", get(random_user))
            .nest_service("/", ServeDir::new("static"))
            .with_state(app_state)
    }
}

#[async_trait::async_trait]
impl FamilyUser for AppState {
    async fn get(&self, id: uuid::Uuid) -> FamilyResult<User> {
        trace!("Calling get() for id {}", &id);
        sqlx::query_as::<_, User>(
            r#"
      SELECT id, username, password_hash
      FROM users
      WHERE id = $1
      "#,
            // RETURNING (id, username, password_hash)
        )
        .bind(&id)
        .fetch_one(self.db())
        .await
        .map_err(|e| WiseError::DatabaseError {
            value: e.to_string(),
        })
    }

    async fn get_all(&self) -> FamilyResult<Vec<User>> {
        let req = sqlx::query_as::<_, User>(
            r#"
      SELECT id, username, password_hash
      FROM users
      "#,
        )
        .fetch_all(self.db())
        .await
        .map_err(|e| WiseError::DatabaseError {
            value: e.to_string(),
        })?;
        Ok(req)
    }

    async fn create(&self, user: &User) -> FamilyResult<User> {
        sqlx::query_as::<_, User>(
            r#"
      INSERT INTO users (id, username, password_hash)
      VALUES ($1, $2, $3)
      RETURNING id, username, password_hash
      "#,
        )
        .bind(user.id_ref())
        .bind(user.username_ref())
        .bind(user.password_hash_ref())
        .fetch_one(self.db())
        .await
        .map_err(|e| WiseError::DatabaseError {
            value: e.to_string(),
        })
    }

    async fn update(&self, user: &User) -> FamilyResult<User> {
        trace!("Calling update() for id {}", user.id_ref());
        sqlx::query(
            r#"
      UPDATE users
      SET username = $1, password_hash = $2
      WHERE id = $3
      "#,
        )
        .bind(user.username_ref())
        .bind(user.password_hash_ref())
        .bind(user.id_ref())
        .execute(self.db())
        .await
        .map_err(|e| WiseError::DatabaseError {
            value: e.to_string(),
        })?;

        Ok(user.clone())
    }

    async fn delete(&self, user: &User) -> FamilyResult<()> {
        let req = sqlx::query::<_>(
            r#"
      DELETE from users
      WHERE id = $1
      "#,
        )
        .bind(user.id_ref())
        .execute(self.db())
        .await
        .map_err(|e| WiseError::DatabaseError {
            value: e.to_string(),
        });
        match req {
            Ok(_) => Ok(()),
            Err(e) => Err(WiseError::DatabaseError {
                value: e.to_string(),
            }),
        }
    }
}

async fn get_user(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    info!("Getting user {}", &id);
    let user = data.get(id).await;
    match user {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn get_users(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    info!("Getting all users.");
    let user = data.get_all().await;
    match user {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn create_user(
    State(data): State<Arc<AppState>>,
    Json(user): Json<serde_json::Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    info!("Creating user {}.", &user["username"]);
    let (_, username) = prune_name(&user["username"].to_string()).unwrap();
    let (_, password) = prune_name(&user["password_hash"].to_string()).unwrap();
    let usr = User::new(&username, &password);
    let user = data.create(&usr).await;
    match user {
        Ok(result) => Ok((StatusCode::CREATED, Json(result))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn update_user(
    State(data): State<Arc<AppState>>,
    Json(user): Json<serde_json::Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    info!("Updating user {}.", &user["username"]);
    let (_, username) = prune_name(&user["username"].to_string()).unwrap();
    let (_, password) = prune_name(&user["password_hash"].to_string()).unwrap();
    let mut usr = User::new(&username, &password);
    let id: uuid::Uuid = serde_json::from_str(&user["id"].to_string()).unwrap();
    trace!("ID is: {}.", &id);
    {
        usr.set_id(id);
    }
    let res = data.update(&usr).await;
    match res {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn delete_user(
    State(data): State<Arc<AppState>>,
    Json(user): Json<serde_json::Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    info!("Deleting user {}.", &user["username"]);
    let (_, username) = prune_name(&user["username"].to_string()).unwrap();
    let (_, password) = prune_name(&user["password_hash"].to_string()).unwrap();
    let mut usr = User::new(&username, &password);
    let id: uuid::Uuid = serde_json::from_str(&user["id"].to_string()).unwrap();
    trace!("ID is: {}.", &id);
    {
        usr.set_id(id);
    }
    let res = data.delete(&usr).await;
    match res {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn random_user(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut user = User::new("user", "password");
    {
        let mut gen = RandomUser::new();
        user.set_username(&gen.username());
        user.set_password_hash(&gen.password());
    }
    info!("Creating random user {}.", user.username_ref());
    let user = data.create(&user).await;
    match user {
        Ok(result) => Ok((StatusCode::CREATED, Json(result))),
        Err(WiseError::DatabaseError { value }) => Err((StatusCode::BAD_REQUEST, value)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}
