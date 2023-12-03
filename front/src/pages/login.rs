#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

pub fn Login(cx: Scope) -> Element {
    let theme = use_shared_state::<Theme>(cx);
    let class = Theme::get(&theme, "background");
    cx.render(rsx!(LoginContent { class: class }))
}

#[derive(Props, PartialEq)]
struct LoginContentProps {
    #[props(into)]
    class: String,
}

fn LoginContent(cx: Scope<LoginContentProps>) -> Element {
    cx.render(rsx!(
    div {
        class: cx.props.class.as_str(),
        MenuItems {}
        UserForm {}
        Footer { "This is the login page." }
    }
      ))
}
