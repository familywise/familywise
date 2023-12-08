#![allow(non_snake_case)]
use dioxus::prelude::*;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn add(items: &Vec<String>) -> String {
        let mut res = "".to_string();
        let mut i = 0;
        for item in items {
            res.push_str(item);
            i += 1;
            if i < res.len() {
                res.push_str(" ");
            }
        }
        res
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }

    pub fn get(theme: &Option<&UseSharedState<Theme>>, aspect: &str) -> String {
        log::trace!("Getting theme.");
        let mut class = "".to_string();
        match theme {
            Some(value) => {
                let theme = *value.read();
                match aspect {
                    "background" => class.push_str(&theme.background()),
                    "button" => class.push_str(&theme.button()),
                    "center" => class.push_str(&theme.center()),
                    "column" => class.push_str(&theme.column()),
                    "input" => class.push_str(&theme.input()),
                    "m3" => class.push_str(&theme.input()),
                    _ => {}
                }
                log::trace!("Theme set.");
            }
            None => {
                log::trace!("No theme found.");
            }
        }
        class
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Light => write!(f, "💡"),
            Self::Dark => write!(f, "🌙"),
        }
    }
}

pub trait Aspect {
    fn background(&self) -> String;
    fn button(&self) -> String;
    fn center(&self) -> String;
    fn column(&self) -> String;
    fn input(&self) -> String;
    fn m3(&self) -> String;
    fn p3(&self) -> String;
}

impl Aspect for Theme {
    fn background(&self) -> String {
        match self {
            Self::Light => {
                "min-h-screen max-w-full bg-gradient-to-r from-slate-400 to-slate-500 text-zinc-800 divide-y divide-zinc-500".to_string()
            }
            Self::Dark => {
                "min-h-screen max-w-full bg-gradient-to-r from-zinc-700 via-zinc-900 to-zinc-800 text-slate-100 divide-y divide-zinc-500".to_string()
            }
        }
    }

    fn button(&self) -> String {
        match self {
            Self::Light => {
                "my-1 px-2 bg-gradient-to-r from-blue-300 via-blue-200 to-blue-300 rounded-full shadow shadow-zinc-700"
                    .to_string()
            }
            Self::Dark => {
                "my-1 px-2 bg-gradient-to-r from-slate-300 via-slate-100 to-slate-300 rounded-full shadow shadow-slate-300 text-zinc-900"
                    .to_string()
            }
        }
    }

    fn column(&self) -> String {
        "flex flex-col px-2 m-3 space-y-4".to_string()
    }

    fn center(&self) -> String {
        "flex flex-row justify-center".to_string()
    }

    fn input(&self) -> String {
        match self {
            Self::Light => "mx-3 px-2 bg-slate-300".to_string(),
            Self::Dark => "mx-3 px-2 bg-zinc-600".to_string(),
        }
    }

    fn m3(&self) -> String {
        "m-3".to_string()
    }

    fn p3(&self) -> String {
        "p-3".to_string()
    }
}
