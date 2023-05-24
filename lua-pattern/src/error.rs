use crate::Token;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("unfinished escape: finish `%` with a class or escape character or use `%%` to match `%` literally")]
    UnfinishedEscape,

    #[error("missing `]` to close set")]
    UnclosedSet,

    #[error("missing characters for `%b` pattern. Example: `%b()`")]
    MissingCharsForBalanced,

    #[error("missing `[` after `%f` in pattern. Example: `%f[%w]`")]
    MissingSetForFrontier,

    #[error("unexpected token in input: `{0}`")]
    UnexpectedToken(Token),

    #[error("reference to unknown capture with id `{0}`")]
    InvalidCaptureRef(u8),

    #[error("range is open ended: token after `-` was `{0}`")]
    OpenEndedRange(Token),
}
