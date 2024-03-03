#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    let theme = use_shared_state::<Theme>(cx);
    let class = Theme::get(&theme, "background");
    cx.render(rsx!(HomeContent { class: class }))
}

#[derive(Props, PartialEq)]
struct HomeContentProps {
    #[props(into)]
    class: String,
}

fn HomeContent(cx: Scope<HomeContentProps>) -> Element {
    cx.render(rsx!(
    div {
        class: cx.props.class.as_str(),
        MenuItems {}
        Parent { "Welcome to FamilyWise!" }
        Footer { Status { } }
    }
      ))
}
