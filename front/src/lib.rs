pub mod components;
pub mod error;
mod nav;
pub mod pages;

pub mod prelude {
    pub use crate::components::{
        Aspect, Card, DarkModeButton, Env, Environment, Footer, Header, HomeTile, InputButton,
        MenuItems, Parent, RandomUserButton, ScreenHeight, Theme, UserCard, UserContent, UserForm,
        HOST, LOCAL,
    };
    pub use crate::error::{ClientError, ClientResult};
    pub use crate::nav::users_endpoint;
    pub use crate::pages::{Home, Login, NotFound, Route};
}
