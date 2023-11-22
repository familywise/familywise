use crate::prelude::*;
use api_lib::prelude::*;
use axum::http::{
    header::{HeaderMap, CONTENT_TYPE},
    StatusCode,
};
use shared::prelude::*;
use tracing::{info, trace};

pub async fn user_lifecycle(app: &TestApp, host: &str) {
    let client = app.client_ref();
    info!("Testing create user.");
    let mut gen = RandomUser::new();
    let user = gen.user();
    trace!(
        "Sending local request to create user {}.",
        &user.username_ref()
    );
    trace!("{:#?}", serde_json::json!(&user));

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let response = client
        .post(format!("{}/api/users", host))
        .headers(headers.clone())
        .json(&user)
        .send()
        .await
        .unwrap();

    trace!("{:#?}", &response);
    assert_eq!(&response.status(), &StatusCode::CREATED);

    let body = response.json::<User>().await.unwrap();

    assert_eq!(user.username_ref(), body.username_ref());
    assert_eq!(user.password_hash_ref(), body.password_hash_ref());
    let id: uuid::Uuid = body.id();
    info!("Create user successful.");

    info!("Testing get users.");
    let response = client
        .get(format!("{}/api/users", host))
        .headers(headers.clone())
        .send()
        .await
        .unwrap();

    trace!("{:#?}", &response);
    assert_eq!(&response.status(), &StatusCode::OK);

    let body = response.json::<Vec<User>>().await.unwrap();
    trace!("Body: {:#?}", &body);
    if !body.is_empty() {
        let usr = body[body.len() - 1].clone();
        assert_eq!(user.username_ref(), usr.username_ref());
        assert_eq!(user.password_hash_ref(), usr.password_hash_ref());
    }
    info!("Get users successful.");

    info!("Testing get user.");
    info!(
        "Sending local request to get user {}.",
        &user.username_ref()
    );
    let url = format!("{}/api/users/{}", host, &id);
    info!("Url is {}", &url);

    let response = client
        .get(&url)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();

    trace!("{:#?}", &response);
    assert_eq!(&response.status(), &StatusCode::OK);

    let body = response.json::<User>().await.unwrap();
    trace!("Body: {:#?}", &body);

    assert_eq!(user.username_ref(), body.username_ref());
    assert_eq!(user.password_hash_ref(), body.password_hash_ref());
    info!("Get user successful.");

    info!("Testing update user.");
    let mut user = gen.user();
    {
        user.set_id(id.clone());
    }
    info!(
        "Sending local request to update user {}.",
        &user.username_ref()
    );
    trace!("{:#?}", &user);

    let response = client
        .put(&url)
        .headers(headers.clone())
        .json(&user)
        .send()
        .await
        .unwrap();

    trace!("{:#?}", &response);
    assert_eq!(&response.status(), &StatusCode::OK);
    let body = response.json::<User>().await.unwrap();
    assert_eq!(user.username_ref(), body.username_ref());
    assert_eq!(user.password_hash_ref(), body.password_hash_ref());

    info!("Calling get by id to compare database value for id to new user values.");
    let response = client
        .get(&url)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();

    let body = response.json::<User>().await.unwrap();
    assert_eq!(user.username_ref(), body.username_ref());
    assert_eq!(user.password_hash_ref(), body.password_hash_ref());
    info!("Update user successful.");

    info!("Testing delete user.");
    info!(
        "Sending local request to delete user {}.",
        &user.username_ref()
    );

    let response = client
        .delete(&url)
        .headers(headers.clone())
        .json(&user)
        .send()
        .await
        .unwrap();

    trace!("{:#?}", &response);
    assert_eq!(&response.status(), &StatusCode::OK);
    info!("Delete user successful.");
}
