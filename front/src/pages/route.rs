#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/home")]
    #[redirect("/:..segments", |segments: Vec<String>| Route::Home {})]
    Home {},
    #[route("/login")]
    Login {},
}
