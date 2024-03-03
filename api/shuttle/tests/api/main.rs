mod health;
mod helpers;
mod integration;
mod user;

pub mod prelude {
    pub use crate::health::{book, health};
    pub use crate::helpers::{WildHost, HOST, LOCAL};
}
