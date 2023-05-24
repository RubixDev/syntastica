use crate::Token;

/// The default result type. The error variant is [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// The crate's main error type.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    /// The input ended with a single `%` character.
    #[error("unfinished escape: finish `%` with a class or escape character or use `%%` to match `%` literally")]
    UnfinishedEscape,

    /// The closing bracket `]` for a [character set](crate::PatternObject::Set) is missing.
    #[error("missing `]` to close set")]
    UnclosedSet,

    /// A [balanced pattern `%b`](crate::PatternObject::Balanced) was present without two
    /// characters following it.
    #[error("missing characters for `%b` pattern. Example: `%b()`")]
    MissingCharsForBalanced,

    /// A [frontier pattern `%f`](crate::PatternObject::Frontier) was present without a
    /// [character set](crate::PatternObject::Set) following it.
    #[error("missing `[` after `%f` in pattern. Example: `%f[%w]`")]
    MissingSetForFrontier,

    /// The input contained an unexpected token, such as an extra closing bracket or a
    /// [quantifier](crate::PatternObject::Quantifier) without a leading
    /// [pattern](crate::PatternObject).
    #[error("unexpected token in input: `{0}`")]
    UnexpectedToken(Token),

    /// A [capture group backreference](crate::PatternObject::CaptureRef) referenced a group that
    /// does not exist.
    #[error("reference to unknown capture with id `{0}`")]
    InvalidCaptureRef(u8),

    /// A [character range](crate::SetPatternObject::Range) in a
    /// [character set](crate::PatternObject::Set) is missing its upper bound. The given token was
    /// found instead.
    #[error("range is open ended: token after `-` was `{0}`")]
    OpenEndedRange(Token),
}
