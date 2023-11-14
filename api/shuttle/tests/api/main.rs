mod health;
mod health_local;
mod helpers;
mod user;

pub mod prelude {
    pub use crate::helpers::{new_user, TestApp, HOST, LOCAL};
}
