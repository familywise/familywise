use dioxus::prelude::*;

pub fn Card(cx: Scope) -> Element {
    log::info!("Drawing card.");
    cx.render(rsx!(
        body {
            class: "flex flex-row justify-center prose prose-gray prose-xl bg-red-300 p-3",
            div {
            "This is the card body."
            }
        }
    ))
}
