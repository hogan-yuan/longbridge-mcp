use rmcp::model::ErrorData as McpError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("longbridge: {0}")]
    Longbridge(Box<longbridge::Error>),
    #[error("serialize: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),
    #[error("database: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("jwt: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("session not found: {0}")]
    SessionNotFound(String),
    #[error("not authenticated")]
    NotAuthenticated,
    #[error("oauth: {0}")]
    OAuth(String),
    #[error("{0}")]
    Other(String),
}

impl From<longbridge::Error> for Error {
    fn from(err: longbridge::Error) -> Self {
        Self::Longbridge(Box::new(err))
    }
}

impl Error {
    /// Shorthand for use with `.map_err(Error::longbridge)`.
    pub fn longbridge(err: longbridge::Error) -> Self {
        Self::Longbridge(Box::new(err))
    }
}

impl From<Error> for McpError {
    fn from(err: Error) -> Self {
        McpError::internal_error(err.to_string(), None)
    }
}
