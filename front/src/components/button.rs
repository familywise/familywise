#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;
use std::fmt;

pub enum ButtonType {
    Primary,
    Secondary,
}

impl fmt::Display for ButtonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ButtonType::Primary => write!(f, "bg-blue-700 hover:bg-blue-800 active:bg-blue-900"),
            ButtonType::Secondary => write!(f, "bg-rose-700 hover:bg-rose-800 active:bg-rose-900"),
        }
    }
}

#[inline_props]
pub fn Button<'a>(
    cx: Scope<'a>,
    button_type: ButtonType,
    onclick: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
) -> Element {
    cx.render(rsx!(button {
        class: "text-slate-200 inline-flex items-center border-0 py-1 px-3 focus:outline-none rounded mt-4 md:mt-0 {button_type.to_string()}",
        onclick: move |event| onclick.call(event),
        children
    }))
}

#[derive(Props)]
struct InputButtonProps<'a> {
    #[props(into)]
    #[props(default="p-3 flex flex-row justify-center".to_string())]
    button: String,
    #[props(into)]
    username: String,
    on_click: EventHandler<'a, MouseEvent>,
}

fn InputButtonInner<'a>(cx: Scope<'a, InputButtonProps<'a>>) -> Element<'a> {
    log::trace!("Input button drawing.");
    cx.render(rsx!(button {
        class: cx.props.button.as_str(),
        onclick: move |event| cx.props.on_click.call(event),
        "Submit"
    }))
}

#[derive(Props, PartialEq)]
pub struct InputButtonParams {
    #[props(into)]
    username: String,
    #[props(into)]
    password: String,
}

pub fn InputButton(cx: Scope<InputButtonParams>) -> Element {
    let theme = use_shared_state::<Theme>(cx);
    let button = Theme::get(&theme, "button");
    cx.render(rsx!(InputButtonInner {
        button: button,
        username: &cx.props.username,
        on_click: move |evt| {
            log::trace!("{:#?}", &evt);
            log::info!("Username: {}", &cx.props.username);
            log::info!("Password: {}", &cx.props.password);
        },
    }))
}
