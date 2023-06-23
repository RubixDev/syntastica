use std::convert::Infallible;

use crate::ts_runtime::QueryError;
use palette::rgb::FromHexError;
use thiserror::Error;

/// The main result type.
///
/// Uses [`Error`](enum@Error) for the error variant.
pub type Result<T> = std::result::Result<T, Error>;

/// The main error type.
#[derive(Debug, Error)]
pub enum Error {
    /// A [`Theme`](crate::theme::Theme) contained a link to a non-existent key.
    ///
    /// Contains the name of the non-existent key.
    ///
    /// Can occur when calling [`Theme::resolve_links`](crate::theme::Theme::resolve_links).
    #[error("link to unknown key '{0}'")]
    InvalidLink(String),

    /// The requested language is not supported by the current language collection.
    ///
    /// Contains the language name which was requested.
    #[error("unsupported language with name '{0}'")]
    UnsupportedLanguage(String),

    /// A [`LanguageSet`](crate::language_set::LanguageSet) did not provide queries for a
    /// provided parser.
    ///
    /// Contains the name of the language which is missing queries.
    #[error("missing queries for language '{0}'")]
    MissingQueries(String),

    /// A [`Theme`](crate::theme::Theme) contains a color with an invalid hex literal.
    ///
    /// Contains a [`palette::rgb::FromHexError`].
    #[error(transparent)]
    InvalidHex(#[from] FromHexError),

    /// The provided queries were malformed or not applicable to the parser.
    ///
    /// Contains a [`QueryError`].
    #[error(transparent)]
    MalformedQueries(#[from] QueryError),

    /// Highlighting failed, usually because of tree-sitter version mismatches.
    ///
    /// Contains a [`syntastica_highlight::Error`].
    #[error(transparent)]
    Highlight(#[from] syntastica_highlight::Error),

    /// A custom error which may be returned by external crates if no other variant fits.
    ///
    /// Contains a string describing the error.
    #[error("{0}")]
    Custom(String),
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!("`Infallible` cannot be constructed")
    }
}
