#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct UserFormProps {
    #[props(into)]
    #[props(default="p-3 flex flex-row justify-center".to_string())]
    center: String,
    #[props(into)]
    #[props(default="bg-slate-400".to_string())]
    input: String,
}

pub fn UserForm(cx: Scope) -> Element {
    let theme = use_shared_state::<Theme>(cx);
    let center = Theme::get(&theme, "center");
    let input = Theme::get(&theme, "input");
    cx.render(rsx!(UserFormInner {
        center: center,
        input: input
    }))
}

fn UserFormInner(cx: Scope<UserFormProps>) -> Element {
    let name = use_state(cx, || "".to_string());
    let pass = use_state(cx, || "".to_string());
    cx.render(rsx!(
    body {
        div {
            class: cx.props.center.as_str(),
            div {
                class: "px-3 py-1",
                p { "Enter username:"
                    input {
                    class: cx.props.input.as_str(),
                    value: "{name}",
                    oninput: move |evt| name.set(evt.value.clone()),
                    }
                }
                p { "Enter password:"
                    input {
                    class: cx.props.input.as_str(),
                    value: "{pass}",
                    oninput: move |evt| pass.set(evt.value.clone()),
                    }
                }
                InputButton { username: name.get(), password: pass.get() }
            }
        }
    }))
}
