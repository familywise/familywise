// use crate::prelude::*;
use dioxus::prelude::*;

#[derive(Props)]
pub struct CardBody<'a> {
    children: Element<'a>,
}

pub fn Card<'a>(cx: Scope<'a, CardBody<'a>>) -> Element {
    log::info!("Drawing card.");
    cx.render(rsx!(
        body {
            div {
                class: "bg-contain flex flex-row justify-center p-3",
            div {
                class: "prose prose-2xl text-slate-100",
                &cx.props.children
            }
            }
        }
    ))
}
