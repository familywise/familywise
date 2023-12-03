use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

// #[derive(Props, PartialEq)]
// pub struct MenuItemProps<'a> {
//     href: &'a str,
//     title: &'a str,
// }
//
// pub fn MenuItem<'a>(cx: Scope<'a, MenuItemProps<'a>>) -> Element {
//     cx.render(rsx!(
//     a {
//         // class: "bg-slate-300 p-2 m-3 rounded border-2 border-slate-100 shadow-lg shadow-teal-200 text-zinc-800",
//         class: "m-3",
//         href: cx.props.href,
//         cx.props.title
//     }
//                   ))
// }

pub fn MenuItems(cx: Scope) -> Element {
    log::trace!("Main body drawing.");
    cx.render(rsx!(
            menu {
                // class: "flex-row justify-center bg-red-300 p-3",
                HomeTile {}
                Link { class: "m-3", to: Route::Home {}, "Home" }
                Link { class: "m-3", to: Route::Login {}, "Login" }
                DarkModeButton {
                    on_click: move |event| {
                        log::info!("{:?}", &event);
                        let theme = use_shared_state::<Theme>(cx);
                        let mut msg = "".to_string();
                        match theme {
                            Some(value) => {
                                let t = value.read().clone();
                                *value.write() = t.next();
                            }
                            None => msg.push_str("No theme found."),
                        }
                    },
                }
    }))
}
