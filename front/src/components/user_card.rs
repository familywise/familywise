use crate::components::Button;
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
    let future = use_future(cx, (), |_| async move {
        reqwest::get("http://localhost:8000/health")
            .await
            .unwrap()
            .json::<User>()
            .await
    });
    log::info!("User card drawing.");
    cx.render(match future.value() {
        // Some(Ok(Json(code))) => rsx! {
        //     div {
        //         format!("{:#?}", code)
        //     }
        // },
        Some(Ok(res)) => rsx! {
            div {
                // format!("Response: {:#?}", res.status())
                format!("Unrecognized body: {:#?}", res)
            }
        },
        Some(Err(e)) => rsx! {
            div {
                e.to_string()
            }
        },
        None => rsx! {
            div {
                "No user found."
            }
        },
    })
}
