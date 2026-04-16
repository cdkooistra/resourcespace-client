use thiserror::Error;

// TODO: what errors can RS return?

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum RsError {
    #[error("API error {status}: {message}")]
    Api { status: u16, message: String },

    #[error("Operation returned false")]
    OperationFailed,

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Other error: {0}")]
    Other(String),

    #[error("Validation error: {0}")]
    Validation(String),
}
