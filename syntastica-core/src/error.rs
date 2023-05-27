use std::convert::Infallible;

use thiserror::Error;
use tree_sitter::QueryError;

use crate::style::ParseHexError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("link to unknown key '{0}'")]
    InvalidLink(String),

    #[error("unsuppoerted language with name '{0}'")]
    UnsupportedLanguage(String),

    #[error("missing queries for language '{0}'")]
    MissingQueries(String),

    #[error(transparent)]
    InvalidHex(#[from] ParseHexError),

    #[error(transparent)]
    MalformedQueries(#[from] QueryError),

    #[error(transparent)]
    Highlight(#[from] syntastica_highlight::Error),

    #[error("{0}")]
    Custom(String),
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!("`Infallible` cannot be constructed")
    }
}
