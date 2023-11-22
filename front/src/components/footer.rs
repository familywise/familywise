use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    log::info!("Footer being drawn on blue background.");
    cx.render(rsx!(
        footer {
            class: "bg-blue-300 shadow-md",
            // class: "w-full h-16 p-2 box-border gap-6 flex flex-row justify-center items-center",
            div {
                class: "container mx-auto flex flex-wrap p-0 flex-col md:flex-row justify-between items-center bg-inherit",
                p {
                    class: "prose prose-gray prose-lg bg-inherit",
                    "This is a footer."
                }
            }
        }
    ))
}
