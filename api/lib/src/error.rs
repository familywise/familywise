//! Error handling module for the library.
//!
//! Creates a custom Error type `WiseError`, and an alias
//! for Result using the `WiseError`, called `WiseResult`.

/// The WiseError enum takes failure types from different libraries and converts them to a variant
/// of WiseError using the framework from the thiserror library.
#[derive(thiserror::Error, Debug)]
pub enum WiseError {
    /// Error returned from local builder pattern.
    #[error("Value not provided for {value:?}.")]
    UserBuildError {
        /// Vector of fields passed into UserBuilder that triggered the error.
        value: Vec<String>,
    },
    /// Produced by calling build on a builder with incomplete fields.
    #[error("Required values not provided.")]
    BuildError,
    /// Error returned by the std::io module.
    #[error("Input/output error from std.")]
    Io(#[from] std::io::Error),
    /// Error returned by the std::env module.
    #[error("Could not read environmental variables from .env.")]
    EnvError(#[from] std::env::VarError),
    /// Local error returned by the authorize module.
    #[error("Authorization failed.")]
    AuthError,
    /// Error returned by the serde_json library.
    #[error("Conversion to JSON failed.")]
    JsonError(#[from] serde_json::Error),
    /// Database access error.
    #[error("Database access error: {value:?}.")]
    DatabaseError {
        /// String representation of the error message.
        value: String,
    },
}

/// Alias for the Result type using the local Error type.
pub type WiseResult<T> = Result<T, WiseError>;
