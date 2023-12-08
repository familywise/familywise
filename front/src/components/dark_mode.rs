#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

#[derive(Props)]
pub struct DarkModeProps<'a> {
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn DarkModeButton<'a>(cx: Scope<'a, DarkModeProps<'a>>) -> Element<'a> {
    log::trace!("Dark mode button drawing.");
    let theme = use_shared_state::<Theme>(cx);
    let mut msg = "".to_string();
    match theme {
        Some(val) => msg.push_str(&format!("{}", *val.read())),
        None => msg.push_str("No theme found."),
    }
    cx.render(rsx!(button {
        class: "m-3",
        onclick: move |event| cx.props.on_click.call(event),
        msg
    }))
}
