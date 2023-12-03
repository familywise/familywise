use dioxus::prelude::*;

pub const HOST: &str = "https://familywise.shuttleapp.rs";
pub const LOCAL: &str = "http://127.0.0.1:8000";

#[derive(Copy, Clone, PartialEq)]
pub enum Env {
    Local,
    Production,
}

impl std::fmt::Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = "".to_owned();
        match self {
            Self::Local => out.push_str("http://127.0.0.1:8000"),
            Self::Production => out.push_str("https://familywise.shuttleapp.rs"),
        }
        write!(f, "{}", out)
    }
}

pub fn Environment(cx: Scope) -> Element {
    log::info!("Getting environment.");
    let env;
    match use_shared_state::<Env>(cx) {
        Some(value) => env = *value.read(),
        None => {
            let local = "http://127.0.0.1:8000";
            let future = use_future(cx, (), |_| async move {
                reqwest::get(format!("{}/health", local)).await
            });
            env = match future.value() {
                Some(_) => Env::Local,
                None => Env::Production,
            };
            use_shared_state_provider(cx, || env)
        }
    }

    match env {
        Env::Local => log::info!("Local environment detected."),
        Env::Production => log::info!("Production environment selected."),
    }

    cx.render(rsx!(
        body {
            class: "flex flex-row justify-center prose prose-gray prose-xl bg-red-300 p-3",
            div {
                format!("{}", env)
            }
        }
    ))
}
