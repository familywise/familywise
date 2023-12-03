use dioxus::prelude::*;

#[derive(Props)]
pub struct FooterProps<'a> {
    children: Element<'a>,
}

pub fn Footer<'a>(cx: Scope<'a, FooterProps<'a>>) -> Element {
    log::info!("Footer drawing.");
    cx.render(rsx!(
        footer {
            class: "sticky bottom-0",
            // class: "w-full h-16 p-2 box-border gap-6 flex flex-row justify-center items-center",
                &cx.props.children
                }
    ))
}
