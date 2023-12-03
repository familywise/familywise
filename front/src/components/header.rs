use crate::prelude::*;
use dioxus::prelude::*;

#[derive(Props)]
pub struct HeaderProps<'a> {
    children: Element<'a>,
}

pub fn Header<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element {
    cx.render(rsx!(
        header {
            class: "sticky top-0",
            &cx.props.children
        }
    ))
}
