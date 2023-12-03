#![allow(non_snake_case)]
use dioxus::prelude::*;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
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
                    "center" => class.push_str(&theme.center()),
                    "input" => class.push_str(&theme.input()),
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
    fn center(&self) -> String;
    fn input(&self) -> String;
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

    fn center(&self) -> String {
        match self {
            Self::Light => "p-3 flex flex-row justify-center".to_string(),
            Self::Dark => "p-3 flex flex-row justify-center".to_string(),
        }
    }

    fn input(&self) -> String {
        match self {
            Self::Light => "mx-3 px-2 bg-slate-300".to_string(),
            Self::Dark => "mx-3 px-2 bg-zinc-600".to_string(),
        }
    }
}
