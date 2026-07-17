//! The crate error type.

/// Errors raised while parsing a spec, resolving indicators, or answering a
/// query. Every variant carries an owned message so it crosses the FFI boundary
/// as a plain string.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A JSON or TOML document failed to parse into the expected type.
    #[error("parse: {0}")]
    Parse(String),
    /// A spec referenced an indicator the registry does not know.
    #[error("unknown indicator: {0}")]
    UnknownIndicator(String),
    /// A query named a symbol that is not in the universe (or is not ready).
    #[error("unknown symbol: {0}")]
    UnknownSymbol(String),
    /// A spec is structurally invalid (empty, duplicated, or bad field).
    #[error("bad spec: {0}")]
    BadSpec(String),
    /// Input market data was malformed.
    #[error("data: {0}")]
    Data(String),
}

/// The crate result alias.
pub type Result<T> = core::result::Result<T, Error>;

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Parse(e.to_string())
    }
}
