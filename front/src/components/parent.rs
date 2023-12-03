use crate::prelude::*;
use dioxus::prelude::*;

#[derive(Props)]
pub struct ParentProps<'a> {
    children: Element<'a>,
}

pub fn Parent<'a>(cx: Scope<'a, ParentProps<'a>>) -> Element {
    log::trace!("Parent container drawing.");
    let theme_ref = use_shared_state::<Theme>(cx);
    match theme_ref {
        Some(value) => {
            let theme = *value.read();
            let class = theme.background();
            cx.render(rsx!(
            Child { class: class, &cx.props.children }
            ))
        }
        None => cx.render(rsx!(
        div {
            class: "max-w-full h-10 bg-blue-300",
            &cx.props.children
        })),
    }
}

#[derive(Props)]
pub struct ChildProps<'a> {
    #[props(into)]
    class: String,
    children: Element<'a>,
}

pub fn Child<'a>(cx: Scope<'a, ChildProps<'a>>) -> Element {
    log::trace!("Child element drawing.");
    cx.render(rsx!(
    div {
        class: cx.props.class.as_str(),
        &cx.props.children
    }))
}
