use crate::prelude::*;
// use axum::Json;
use cordial_guest::Guest;
use dioxus::prelude::*;
// use shared::prelude::*;

#[derive(Props)]
pub struct GuestContent<'a> {
    pub children: Element<'a>,
}

impl<'a> GuestContent<'a> {
    pub fn new(cx: Scope<'a>, guest: &Guest) -> Self {
        let children = cx.render(rsx!(
        body {
            class: "prose prose-gray prose-lg",
            div {
                p { format!("Username: {}", guest.name) }
                p { format!("Password: {}", guest.hash) }
            }
        }
              ));
        Self { children }
    }

    pub fn children_ref(&self) -> &Element<'a> {
        &self.children
    }
}

#[component]
pub fn GuestCard(
    cx: Scope,
    // on_edit: EventHandler<'a, MouseEvent>,
    // on_delete: EventHandler<'a, MouseEvent>,
) -> Element {
    let future = use_future(cx, (), |()| async move {
        let name = reqwest::get(format!("{}/improv/name/num", HOST))
            .await
            .unwrap()
            .json::<String>()
            .await
            .unwrap();
        let pass = reqwest::get(format!("{}/improv/pass", HOST))
            .await
            .unwrap()
            .json::<String>()
            .await
            .unwrap();
        Guest::new(&name, &pass)
    });
    log::info!("Guest card drawing.");
    cx.render(match future.value() {
        Some(guest) => rsx! {
            div {
                p { format!("Username: {}", guest.name) }
                p { format!("Password: {}", guest.hash) }
            }
        },
        None => rsx! {
            div {
                "Fetch of user failed."
            }
        },
    })
}
