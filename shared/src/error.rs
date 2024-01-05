//! Error handling module for the client.
//!
//! Creates a custom Error type `SharedError`, and an alias
//! for Result using the `SharedError`, called `SharedResult`.

/// The SharedError enum takes failure types from different libraries and converts them to a variant
/// of SharedError using the framework from the thiserror library.
#[derive(thiserror::Error, Debug)]
pub enum SharedError {
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
    // /// Error returned by the serde_json library.
    // #[error("Conversion to JSON failed.")]
    // JsonError(#[from] serde_json::Error),
    /// Database access error.
    #[error("Database access error: {value:?}.")]
    DatabaseError {
        /// String representation of the error message.
        value: String,
    },
    #[error("Unknown error occurred.")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Unknown error occurred.")]
    UnknownError,
}

/// Alias for the Result type using the local Error type.
pub type SharedResult<T> = Result<T, SharedError>;
