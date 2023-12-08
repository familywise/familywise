#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn MenuItems(cx: Scope<MenuProps>) -> Element {
    log::trace!("Menu item drawing.");
    cx.render(rsx!(
            menu {
                class: "flex flex-row justify-between p-3",
                HomeTile {}
                div {
                    class: cx.props.center.as_str(),
                    Link { class: "self-center m-3", to: Route::Home {}, "Home" }
                    Link { class: "self-center m-3", to: Route::Login {}, "Login" }
                }
                DarkModeButton {
                    on_click: move |event| {
                        log::info!("{:?}", &event);
                        let theme = use_shared_state::<Theme>(cx);
                        let mut msg = "".to_string();
                        match theme {
                            Some(value) => {
                                let t = value.read().clone();
                                *value.write() = t.next();
                            }
                            None => msg.push_str("No theme found."),
                        }
                    },
                }
    }))
}

pub fn Menu(cx: Scope) -> Element {
    log::trace!("Menu item drawing.");
    let theme = use_shared_state::<Theme>(cx);
    let center = Theme::get(&theme, "center");
    let pad = Theme::get(&theme, "p3");
    cx.render(rsx!(MenuItems {
        center: center,
        pad: pad
    }))
}

#[derive(Props, PartialEq)]
pub struct MenuProps {
    #[props(into)]
    #[props(default="flex flex-row justify-center".to_string())]
    center: String,
    #[props(into)]
    #[props(default="p-3".to_string())]
    pad: String,
}
