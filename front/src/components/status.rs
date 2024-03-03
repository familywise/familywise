#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;
use polite::{FauxPas, Polite};

#[derive(Props)]
pub struct StatusUpdateProps<'a> {
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
    health: &'a UseState<bool>,
    book: &'a UseState<String>,
}

fn StatusUpdate<'a>(cx: Scope<'a, StatusUpdateProps<'a>>) -> Element {
    let get_health = use_state(cx, || ());
    {
        let health = cx.props.health.clone();
        use_effect(cx, get_health, |_| async move {
            if let Ok(val) = get_health_check().await {
                health.set(val);
            }
        })
    }

    let get_book = use_state(cx, || ());
    {
        let book = cx.props.book.clone();
        use_effect(cx, get_book, |_| async move {
            if let Ok(val) = get_book_check().await {
                book.set(val);
            }
        })
    }

    cx.render(rsx!(
    div {
        onload: move |_| {
            get_health.set(());
        },
        class: cx.props.center.as_str(),
        format!("{}", cx.props.health.get())
    }
    div {
        onload: move |_| {
            get_book.set(());
        },
        class: cx.props.center.as_str(),
        format!("{}", cx.props.book.get())
    }
              ))
}
pub fn Status(cx: Scope) -> Element {
    log::trace!("Status utility running.");
    let health = use_state(cx, || false);
    let book = use_state(cx, || "".to_string());

    cx.render(rsx!(
        div {
            class: "flex flex-row justify-center",
            // class: "w-full h-16 p-2 box-border gap-6 flex flex-row justify-center items-center",
            StatusUpdate{
                health: health,
                book: book,
            }
                }
    ))
}

async fn get_health_check() -> Polite<bool> {
    let path = format!("{}/check", LOCAL);
    log::info!("Checking health from {}", &path);
    match reqwest::get(&path).await?.json::<bool>().await {
        Ok(val) => Ok(val),
        Err(e) => {
            log::info!("{}", e.to_string());
            Err(FauxPas::Unknown)
        }
    }
}

async fn get_book_check() -> Polite<String> {
    let path = format!("{}/book", LOCAL);
    log::info!("Checking book from {}", &path);
    match reqwest::get(&path).await?.json::<String>().await {
        Ok(val) => Ok(val),
        Err(e) => {
            log::info!("{}", e.to_string());
            Err(FauxPas::Unknown)
        }
    }
}
