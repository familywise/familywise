pub mod components;
pub mod error;
pub mod pages;

pub mod prelude {
    pub use crate::components::{
        Aspect, Card, DarkModeButton, Env, Environment, Footer, Header, HomeTile, InputButton,
        MenuItems, Parent, ScreenHeight, Theme, UserCard, UserContent, UserForm, HOST, LOCAL,
    };
    pub use crate::error::{ClientError, ClientResult};
    pub use crate::pages::{Home, Login, NotFound, Route};
}
