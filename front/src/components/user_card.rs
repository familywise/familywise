use crate::prelude::*;
// use axum::Json;
use dioxus::prelude::*;
use shared::prelude::*;
// use shared::models::{user::User, ButtonType};

#[inline_props]
pub fn UserCard(
    cx: Scope,
    // pub fn UserCard<'a>(
    //     cx: Scope<'a>,
    // user: &'a User,
    // on_edit: EventHandler<'a, MouseEvent>,
    // on_delete: EventHandler<'a, MouseEvent>,
) -> Element {
    // let env = use_shared_state::<Env>(cx).unwrap();
    // let future = use_future(cx, (env,), |(env,)| async move {
    //     reqwest::get(format!("{}/api/users/random", *env.read()))
    let future = use_future(cx, (), |()| async move {
        reqwest::get("https://familywise.shuttleapp.rs/api/users/random")
            .await
            .unwrap()
            .json::<User>()
            .await
    });
    log::info!("User card drawing.");
    cx.render(match future.value() {
        Some(Ok(user)) => rsx! {
            div {
                p { format!("Username: {}", user.username_ref()) }
                p { format!("Password: {}", user.password_hash_ref()) }
            }
        },
        Some(Err(e)) => rsx! {
            div {
                e.to_string()
            }
        },
        None => rsx! {
            div {
                "Fetch of user failed."
            }
        },
    })
}
