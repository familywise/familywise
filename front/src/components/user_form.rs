#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;
use shared::prelude::*;

#[derive(Props, PartialEq)]
pub struct UserFormProps {
    #[props(into)]
    #[props(default="p-3 flex flex-row justify-center".to_string())]
    center: String,
    #[props(into)]
    #[props(default="flex flex-col justify-center".to_string())]
    column: String,
    #[props(into)]
    #[props(default="bg-slate-400".to_string())]
    input: String,
}

impl UserFormProps {
    pub fn random_user(cx: Scope<Self>) -> ClientResult<&User> {
        let future = use_future(cx, (), |_| async move {
            reqwest::get(HOST).await?.json::<User>().await
        });
        match future.value() {
            Some(Ok(user)) => Ok(user),
            Some(Err(e)) => Err(ClientError::DatabaseError {
                value: e.to_string(),
            }),
            None => Err(ClientError::UnknownError),
        }
    }
}

pub fn UserForm(cx: Scope) -> Element {
    let theme = use_shared_state::<Theme>(cx);
    let center = Theme::get(&theme, "center");
    let column = Theme::get(&theme, "column");
    let input = Theme::get(&theme, "input");
    let pad = Theme::get(&theme, "p3");
    let center = Theme::add(&vec![center, pad]);
    let margin = Theme::get(&theme, "m3");
    let container = Theme::add(&vec![column.clone(), margin]);
    cx.render(rsx!(UserFormInner {
        center: center,
        column: column,
        input: input,
    }))
}

fn UserFormInner(cx: Scope<UserFormProps>) -> Element {
    let get_user = UserFormProps::random_user(cx);
    let mut name = "".to_string();
    let mut pass = "".to_string();
    match get_user {
        Ok(user) => {
            name.push_str(user.username_ref());
            pass.push_str(user.password_hash_ref());
        }
        Err(_) => {
            log::info!("Error fetching random user.");
        }
    }

    let name = use_state(cx, || name);
    let pass = use_state(cx, || pass);
    cx.render(rsx!(
    body {
        div {
            class: "flex flex-col justify-center m-3",
            h1 {
                class: cx.props.center.as_str(),
                "Create New User"
            }
            div {
                class: cx.props.center.as_str(),
            div {
                class: cx.props.column.as_str(),
                p { "Enter Username:" }
                p { "Enter Password:" }
            }
            div {
                class: cx.props.column.as_str(),
                input {
                    class: cx.props.input.as_str(),
                    value: "{name}",
                    oninput: move |evt| name.set(evt.value.clone()),
                }
                input {
                    class: cx.props.input.as_str(),
                    value: "{pass}",
                    oninput: move |evt| pass.set(evt.value.clone()),
                }
            }
            }
            div {
                class: cx.props.center.as_str(),
                InputButton { username: name.get(), password: pass.get() }
            }
        }
    }))
}
