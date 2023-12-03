use crate::prelude::*;
// use axum::Json;
use dioxus::prelude::*;
use shared::prelude::*;
// use shared::models::{user::User, ButtonType};

#[derive(Props)]
pub struct UserContent<'a> {
    pub children: Element<'a>,
}

impl<'a> UserContent<'a> {
    pub fn new(cx: Scope<'a>, user: &User) -> Self {
        let children = cx.render(rsx!(
        body {
            class: "prose prose-gray prose-lg",
            div {
                p { format!("Username: {}", user.username_ref()) }
                p { format!("Password: {}", user.password_hash_ref()) }
            }
        }
              ));
        Self { children }
    }

    pub fn children_ref(&self) -> &Element<'a> {
        &self.children
    }
}

#[inline_props]
pub fn UserCard(
    cx: Scope,
    // on_edit: EventHandler<'a, MouseEvent>,
    // on_delete: EventHandler<'a, MouseEvent>,
) -> Element {
    let future = use_future(cx, (), |()| async move {
        reqwest::get(format!("{}/api/users/random", HOST))
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
