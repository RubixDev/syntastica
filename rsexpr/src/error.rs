use thiserror::Error;

use crate::ParenKind;

/// The default result type. The error variant is a vector of [`Error`](crate::Error)s
pub type Result<T> = std::result::Result<T, Vec<Error>>;

/// The crate error type.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    /// The input is missing a closing parenthesis
    #[error("missing closing parenthesis `{0}`")]
    MissingClosingParen(ParenKind),

    /// The input has an unexpected extra closing parenthesis
    #[error("extra closing parenthesis `{0}`")]
    ExtraClosingParen(ParenKind),

    /// The input contains no S-expression
    #[error("input contains no S-expression")]
    EmptyInput,

    /// The input contains more than one S-expression
    #[error("input contains more than one S-expression")]
    ExtraSexprs,
}
