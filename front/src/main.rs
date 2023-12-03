#![allow(non_snake_case)]
// Import the Dioxus prelude to gain access to the `rsx!` macro and the `Scope` and `Element` types.
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use front::prelude::*;
use prefers_color_scheme::prefers_dark;
use tracing::info;
use wasm_logger;

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    // Launch the web application using the App component as the root.
    dioxus_web::launch(App);
    info!("🚀 Client started successfully");
    log::info!("Browser logging initiated in Dioxus.");
}

fn App(cx: Scope) -> Element {
    let theme = use_shared_state::<Theme>(cx);
    if theme.is_none() {
        let mut preference = Theme::Light;
        if prefers_dark() {
            preference = Theme::Dark;
        }
        use_shared_state_provider(cx, || preference);
    }
    let window_ref = web_sys::window();
    match window_ref {
        Some(window) => {
            let ht = window.inner_height().unwrap().as_f64().unwrap();
            use_shared_state_provider(cx, || ScreenHeight::new(ht));
        }
        None => {
            use_shared_state_provider(cx, || ScreenHeight::new(50.0));
        }
    }
    cx.render(rsx! {
        Router::<Route> {}
    })
}
