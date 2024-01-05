#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;
use shared::prelude::*;

#[derive(Props)]
pub struct UserFormProps<'a> {
    #[props(into)]
    #[props(default="p-3 flex flex-row justify-center".to_string())]
    button: String,
    #[props(into)]
    #[props(default="p-3 flex flex-row justify-center".to_string())]
    center: String,
    #[props(into)]
    #[props(default="flex flex-col justify-center".to_string())]
    column: String,
    #[props(into)]
    #[props(default="bg-slate-400".to_string())]
    input: String,
    name: &'a UseState<String>,
    pass: &'a UseState<String>,
    see_pass: &'a UseState<bool>,
}

#[inline_props]
fn UserFormOptions(
    cx: Scope,
    name: UseState<String>,
    pass: UseState<String>,
    visible: UseState<bool>,
) -> Element {
    let get_user = use_state(cx, || ());
    {
        let name = name.clone();
        let pass = pass.clone();
        use_effect(cx, get_user, |_| async move {
            let user = get_random_user().await;
            name.set(user.username());
            pass.set(user.password_hash());
        })
    }

    let get_username = use_state(cx, || ());
    {
        let name = name.clone();
        use_effect(cx, get_username, |_| async move {
            match get_random_username().await {
                Ok(val) => {
                    name.set(val);
                }
                Err(e) => {
                    log::info!("{}", e.to_string());
                }
            }
        })
    }

    let get_password = use_state(cx, || ());
    {
        let pass = pass.clone();
        use_effect(cx, get_password, |_| async move {
            let password = get_random_password().await;
            pass.set(password);
        })
    }
    cx.render(rsx!(
    div {
        class: "self-center",
        input {
            class: "mx-3",
            r#type: "checkbox",
            id: "see_pass",
            oninput: move |_| {
                visible.set(!visible.get());
            }
        }
        label {
            r#for: "see_pass",
            "Show Password",
        }
    }
    button {
        class: "rounded-full border-2 border-red-300 shrink px-2",
        onclick: move |_| {
            get_username.set(());
        },
        "Name"
    }
    button {
        class: "rounded-full border-2 border-red-300 shrink px-2",
        onclick: move |_| {
            get_password.set(());
        },
        "Pass"
    }
    button {
        class: "rounded-full border-2 border-red-300 shrink px-2",
        onclick: move |_| {
            get_user.set(());
        },
        "Both"
    }

              ))
}

pub fn UserForm(cx: Scope) -> Element {
    log::info!("Drawing user form.");
    let theme = use_shared_state::<Theme>(cx);
    let button = Theme::get(&theme, "button");
    let center = Theme::get(&theme, "center");
    let column = Theme::get(&theme, "column");
    let input = Theme::get(&theme, "input");
    let pad = Theme::get(&theme, "p3");
    let center = Theme::add(&vec![center, pad]);
    let name = use_state(cx, || "".to_string());
    let pass = use_state(cx, || "".to_string());
    let see_opts = use_state(cx, || false);
    let see_pass = use_state(cx, || false);

    cx.render(rsx!(
        div {
            class: "flex flex-row justify-center",
            UserFormInner {
            button: button,
            center: center,
            column: column,
            input: input,
            name: name,
            pass: pass,
            see_pass: see_pass,
            }
            div {
                class: "flex flex-col justify-center",
                div {
                    class: "self-center",
                    input {
                        class: "mx-3",
                        r#type: "checkbox",
                        id: "see_opts",
                        oninput: move |_| {
                            see_opts.set(!see_opts.get());
                        }
                    }
                    label {
                        r#for: "see_opts",
                        "Show Options",
                    }
                }
                div {
                    hidden: if *see_opts.get() {
                        "false"
                    } else {
                        "true"
                    },
                    UserFormOptions { name: name.clone(), pass: pass.clone(), visible: see_pass.clone(), }
                }
            }
        }

    ))
}

fn UserFormInner<'a>(cx: Scope<'a, UserFormProps<'a>>) -> Element {
    let draft_name = use_state(cx, || "".to_string());
    {
        let draft_name = draft_name.clone();
        use_effect(cx, &cx.props.name.get().clone(), |name| async move {
            let test = draft_name.get();
            if name != *test {
                draft_name.set(name);
            }
        })
    }

    let draft_pass = use_state(cx, || "".to_string());
    {
        let draft_pass = draft_pass.clone();
        use_effect(cx, &cx.props.pass.get().clone(), |pass| async move {
            let test = draft_pass.get();
            if pass != *test {
                draft_pass.set(pass);
            }
        })
    }

    cx.render(rsx!(
    div {
        class: "flex flex-col justify-center p-3",
        h1 {
            class: cx.props.center.as_str(),
            "Create New User"
        }
        form {
            class: "flex flex-col justify-center",
            div {
                class: "flex flex-row justify-center my-5",
                label {
                    class: "items-center",
                    r#for: "username",
                    "Enter username:"
                }
                input {
                    class: cx.props.input.as_str(),
                    id: "username",
                    value: draft_name.get().as_str(),
                    oninput: move |evt| cx.props.name.set(evt.value.clone()),
                }
            }
            div {
                class: "flex flex-row justify-center",
                label {
                    class: "items-center",
                    r#for: "password",
                    "Enter password:"
                }
                input {
                    class: cx.props.input.as_str(),
                    id: "password",
                    r#type: if *cx.props.see_pass.get() {
                        "text"
                    } else {
                        "password"
                    },
                    value: draft_pass.get().as_str(),
                    oninput: move |evt| cx.props.pass.set(evt.value.clone()),
                }
            }
        }
        div {
            class: "flex flex-row justify-center p-2 space-x-6",
            InputButton { username: cx.props.name.get(), password: cx.props.pass.get() }
        }

    }
    ))
}

async fn get_random_user() -> User {
    let path = format!("{}/random", users_endpoint());
    log::info!("Getting random user from {}", &path);
    reqwest::get(&path)
        .await
        .unwrap()
        .json::<User>()
        .await
        .unwrap()
}

async fn get_random_username() -> ClientResult<String> {
    let path = format!("{}/random/username", users_endpoint());
    log::info!("Getting random username from {}", &path);
    match reqwest::get(&path).await?.json::<String>().await {
        Ok(response) => Ok(response),
        Err(e) => {
            log::info!("{}", e.to_string());
            Err(ClientError::UnknownError)
        }
    }
}

async fn get_random_password() -> String {
    let path = format!("{}/random/password", users_endpoint());
    log::info!("Getting random password from {}", &path);
    reqwest::get(&path)
        .await
        .unwrap()
        .json::<String>()
        .await
        .unwrap()
}
