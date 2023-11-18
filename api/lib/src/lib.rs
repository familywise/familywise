pub mod configuration;
pub mod error;
pub mod get_data;
pub mod health;
pub mod interface;
pub mod state;
pub mod utils;

pub mod prelude {
    pub use crate::configuration::DatabaseSettings;
    pub use crate::error::{WiseError, WiseResult};
    pub use crate::health::check;
    pub use crate::interface::user::{FamilyResult, FamilyUser};
    pub use crate::state::{AppState, API_VERSION};
    pub use crate::utils::{name_part, prune_name, prune_str, RandomUser};
}
