use thiserror::Error;

use crate::ParenKind;

pub type Result<T> = std::result::Result<T, Vec<Error>>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing closing parenthesis `{0}`")]
    MissingClosingParen(ParenKind),

    #[error("extra closing parenthesis `{0}`")]
    ExtraClosingParen(ParenKind),

    #[error("input contains no S-expression")]
    EmptyInput,

    #[error("input contains more than one S-expression")]
    ExtraSexprs,
}
