use thiserror::Error;

// To avoid having to make certain dependencies a public dependency,
// we use a boxed error type that can be converted from any error type.
type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {status}: {body}")]
    Http { status: u16, body: String },

    #[error("API error {function}: {message}")]
    Api { function: String, message: String },

    #[error("ResourceSpace returned false during login: invalid credentials")]
    InvalidCredentials,

    #[error("ResourceSpace returned: invalid signature")]
    InvalidSignature,

    #[error("ResourceSpace returned false for `{function}` with params: {params}")]
    OperationFailed {
        function: String,
        params: serde_json::Value,
    },

    #[error("Failed to deserialize response for `{function}`: {source}")]
    Deserialize {
        function: String,
        #[source]
        source: BoxError,
    },

    #[error("Transport error: {0}")]
    Transport(BoxError),

    #[error("IO error: {0}")]
    Io(#[source] BoxError),

    #[error("Client error: {0}")]
    Client(#[source] BoxError),

    #[error("URL error: {0}")]
    Url(#[source] BoxError),

    #[error("Validation error: {0}")]
    Validation(#[source] BoxError),
}

/// Wraps a transport failure, dropping every query parameter except `function`.
pub(crate) fn transport(error: reqwest::Error) -> Error {
    let redacted = error.url().map(|url| {
        let function = url
            .query_pairs()
            .find(|(key, _)| key == "function")
            .map(|(_, value)| value.into_owned());

        let mut redacted = url.clone();
        redacted.set_query(None);
        if let Some(function) = function {
            redacted
                .query_pairs_mut()
                .append_pair("function", &function);
        }
        redacted
    });

    match redacted {
        Some(url) => Error::Transport(error.with_url(url).into()),
        None => Error::Transport(error.into()),
    }
}
