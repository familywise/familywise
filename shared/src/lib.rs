mod error;
mod models;

pub mod prelude {
    pub use crate::error::{SharedError, SharedResult};
    pub use crate::models::user::User;
}
