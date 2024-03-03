#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;
use shared::prelude::*;
use std::fmt;

pub enum ButtonType {
    Primary,
    Secondary,
}

impl fmt::Display for ButtonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ButtonType::Primary => write!(f, "bg-blue-700 hover:bg-blue-800 active:bg-blue-900"),
            ButtonType::Secondary => write!(f, "bg-rose-700 hover:bg-rose-800 active:bg-rose-900"),
        }
    }
}

#[component]
pub fn Button<'a>(
    cx: Scope<'a>,
    button_type: ButtonType,
    onclick: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
) -> Element {
    cx.render(rsx!(button {
        class: "text-slate-200 inline-flex items-center border-0 py-1 px-3 focus:outline-none rounded mt-4 md:mt-0 {button_type.to_string()}",
        onclick: move |event| onclick.call(event),
        children
    }))
}

#[derive(Props)]
struct InputButtonProps<'a> {
    #[props(into)]
    #[props(default="p-3 flex flex-row justify-center".to_string())]
    button: String,
    on_click: EventHandler<'a, MouseEvent>,
}

fn InputButtonInner<'a>(cx: Scope<'a, InputButtonProps<'a>>) -> Element<'a> {
    log::trace!("Input button drawing.");
    cx.render(rsx!(button {
        class: cx.props.button.as_str(),
        onclick: move |event| cx.props.on_click.call(event),
        "Submit"
    }))
}

#[derive(Props, PartialEq)]
pub struct InputButtonParams {
    #[props(into)]
    username: String,
    #[props(into)]
    password: String,
}

pub fn InputButton(cx: Scope<InputButtonParams>) -> Element {
    let user_input = move |user: User| {
        cx.spawn({
            async move {
                let client = reqwest::Client::new();
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    "application/json".parse().unwrap(),
                );
                let response = client
                    .post(&guests_endpoint())
                    .headers(headers)
                    .json(&user)
                    .send()
                    .await
                    .expect("after send")
                    .json::<User>()
                    .await;
                match response {
                    Ok(user) => {
                        log::info!("User created: {:#?}", &user);
                    }
                    Err(e) => {
                        log::info!("Error: {}", e.to_string());
                    }
                }
            }
        })
    };
    let theme = use_shared_state::<Theme>(cx);
    let button = Theme::get(&theme, "button");
    let get_user = User::new(cx.props.username.as_str(), cx.props.password.as_str());
    log::info!("Working user values: {:#?}", &get_user);

    cx.render(rsx!(InputButtonInner {
        button: button,
        on_click: move |evt| {
            log::trace!("{:#?}", &evt);
            log::info!("Username: {}", get_user.username_ref());
            log::info!("Password: {}", get_user.password_hash_ref());
            user_input(get_user.clone());
            // match input_user(cx) {
            //     Ok(user_result) => log::info!("User created {:#?}", user_result),
            //     Err(e) => log::info!("Error: {}", e.to_string()),
            // }
        },
    }))
}

pub fn input_user(cx: Scope<InputButtonParams>) -> ClientResult<User> {
    let mut user = User::new(cx.props.username.as_str(), cx.props.password.as_str());
    let future = use_future(cx, (&user,), |user| async move {
        log::info!("Passing user to client: {:#?}", &user);
        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        let res = client
            .post(&format!("http://127.0.0.1:8000/api/users"))
            .headers(headers)
            .json(&user)
            .send()
            .await
            .expect("after send")
            .json::<User>()
            .await
            .expect("after deserializing to json");
        log::info!("Response is {:#?}", &res);
        res
    });

    match future.value() {
        Some(user_result) => {
            user.set_id(user_result.id());
            log::info!("Passing user to gui: {:#?}", &user);
            Ok(user)
        }
        None => Err(ClientError::UnknownError),
    }
}

#[derive(Props)]
pub struct RandomUserButtonProps<'a> {
    #[props(into)]
    #[props(default="p-3 flex flex-row justify-center".to_string())]
    button: String,
    on_click: EventHandler<'a, MouseEvent>,
    msg: &'a str,
}

pub fn RandomUserButton<'a>(cx: Scope<'a, RandomUserButtonProps<'a>>) -> Element {
    cx.render(rsx!(
    button {
        class: cx.props.button.as_str(),
        onclick: move |event| cx.props.on_click.call(event),
        cx.props.msg
    }
              ))
}
