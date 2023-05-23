use thiserror::Error;
use tree_sitter::QueryError;

use crate::style::ParseHexError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("link to unknown key '{0}'")]
    InvalidLink(String),

    #[error("unsuppoerted file extension '{0}'")]
    UnsupportedFileExt(String),

    #[error("missing queries for language '{0}'")]
    MissingQueries(String),

    #[error(transparent)]
    InvalidHex(#[from] ParseHexError),

    #[error(transparent)]
    MalformedQueries(#[from] QueryError),

    #[error(transparent)]
    Highlight(#[from] tree_sitter_highlight::Error),
}
