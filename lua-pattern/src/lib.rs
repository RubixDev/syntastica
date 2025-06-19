#![doc = include_str!("../README.md")]
//! ## Usage
//! - Lua patterns can be parsed to a tree with [`parse`].
//! - Parsed patterns can be converted to regex strings with [`try_to_regex`].
//!
//! For example:
//! ```
//! use lua_pattern::{Class, PatternObject};
//!
//! let tree = lua_pattern::parse("%l").unwrap();
//! assert_eq!(tree, [PatternObject::Class(Class::Lowercase)]);
//! #[cfg(feature = "to-regex")]
//! assert_eq!(
//!     lua_pattern::try_to_regex(&tree, false, false).unwrap(),
//!     "[a-z]"
//! );
//! ```
#![cfg_attr(
    feature = "docs",
    cfg_attr(doc, doc = ::document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#))
)]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

mod error;
mod lexer;
mod parser;

#[cfg(feature = "to-regex")]
mod to_regex;

use std::fmt::{self, Display, Formatter};

pub use error::*;
use lexer::Lexer;
use parser::Parser;

#[cfg(feature = "to-regex")]
pub use to_regex::*;

///////////////
// Functions //
///////////////

/// Parse the given input string as a Lua pattern.
///
/// # Returns
/// This function returns a vector of [`PatternObject`]s if parsing was successful, or an [`Error`]
/// if the pattern could not be parsed.
///
/// # Errors
/// To see the possible errors, have a look at [`Error`].
pub fn parse(pattern: impl AsRef<str>) -> Result<Pattern> {
    Parser::parse(Lexer::lex(pattern.as_ref())?)
}

///////////
// Types //
///////////

/// A list of [`PatternObject`]s, representing an entire Lua pattern.
pub type Pattern = Vec<PatternObject>;

/// A single object of a Lua pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternObject {
    /// Match any character (`.`).
    Any,
    /// Match the start of the string (`^`).
    Start,
    /// Match the end of the string (`$`).
    End,

    /// A sequence of characters to match literally (eg. `Hello, World!`).
    String(String),
    /// A [`PatternObject`] followed by a [`Quantifier`] (eg. `a?`, `.*`).
    Quantifier(Quantifier, Box<PatternObject>),
    /// An escaped character to match literally (eg. `%%`).
    Escaped(char),
    /// A [character class](Class) (eg. `%w`, `%L`).
    Class(Class),
    /// A reference to a previous capture group (eg. `%1`).
    CaptureRef(u8),
    /// A balanced pattern (eg. `%bxy`). Matches all characters starting at `x` until the
    /// corresponding `y`.
    Balanced(char, char),
    /// A frontier pattern (eg. `%f[a-z]`). Matches if the following character matches the set and
    /// the previous character does not match the set. The `bool` indicated whether the set is
    /// inverted.
    Frontier(bool, Vec<SetPatternObject>),

    /// A capture group with a numeric ID and the contained [`Pattern`] (eg. `(a)`).
    Capture(u8, Pattern),
    /// A set of [`SetPatternObject`]s (eg. `[a-z_%u]`, `[^a-z_%u]`), the `bool` specifies whether
    /// the set is inverted. If the set is _not_ inverted, it matches if any of the contained
    /// entries matches. Otherwise, it matches if none of the contained entries match.
    Set(bool, Vec<SetPatternObject>),
}

/// An entry of a [set](PatternObject::Set).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetPatternObject {
    /// A character to match literally (eg. `a`).
    Char(char),
    /// An escaped character to match literally (eg. `%%`, `%]`).
    Escaped(char),
    /// A range of characters (eg. `a-z`). Matches if any character in the range matches.
    Range(char, char),
    /// A [character class](Class) (eg. `%w`, `%L`).
    Class(Class),
}

/// A quantifier, specifying the amount of times the leading [`PatternObject`] can occur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantifier {
    /// Allow zero or more occurrences, taking the longest matching sequence (`*`).
    ZeroOrMore,
    /// Allow one or more occurrences, taking the longest matching sequence (`+`).
    OneOrMore,
    /// Allow zero or more occurrences, taking the shortest matching sequence (`-`).
    ZeroOrMoreLazy,
    /// Allow zero or one occurrences (`?`).
    ZeroOrOne,
}

/// A character class, matching any character contained in the class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    /// Matches any letter; equivalent to `[a-zA-Z]` (`%a`).
    Letters,
    /// Matches any control character; equivalent to `[\0-\31]` (`%c`).
    Controls,
    /// Matches any digit; equivalent to `[0-9]` (`%d`).
    Digits,
    /// Matches any printable character except space; equivalent to `[\33-\126]` (`%g`).
    Printable,
    /// Matches any lowercase letter; equivalent to `[a-z]` (`%l`).
    Lowercase,
    /// Matches any punctuation character; equivalent to ``[!"#$%&'()*+,-./[\%]^_`{|}~]`` (`%p`).
    Punctuations,
    /// Matches any whitespace character; equivalent to `[ \t\n\v\f\r]` (`%s`).
    Spaces,
    /// Matches any uppercase letter; equivalent to `[A-Z]` (`%u`).
    Uppercase,
    /// Matches any alphanumeric character (digit or letter); equivalent to `[a-zA-Z0-9]` (`%w`).
    Alphanumerics,
    /// Matches any hexadecimal digit; equivalent to `[0-9a-fA-F]` (`%x`).
    Hexadecimals,
    /// Matches the NULL character / `0` byte; equivalent to `\0` (`%z`).
    ZeroByte,

    /// Matches any character, **except** all letters; equivalent to `[^a-zA-Z]` (`%A`).
    NotLetters,
    /// Matches any character, **except** all control characters; equivalent to `[^\0-\31]` (`%C`).
    NotControls,
    /// Matches any character, **except** all digits; equivalent to `[^0-9]` (`%D`).
    NotDigits,
    /// Matches any character, **except** all printable characters, but including space; equivalent
    /// to `[^\33-\126]` (`%G`).
    NotPrintable,
    /// Matches any character, **except** all lowercase letters; equivalent to `[^a-z]` (`%L`).
    NotLowercase,
    /// Matches any character, **except** all punctuation characters; equivalent to
    /// ``[^!"#$%&'()*+,-./[\%]^_`{|}~]`` (`%P`).
    NotPunctuations,
    /// Matches any character, **except** all whitespace characters; equivalent to `[^ \t\n\v\f\r]`
    /// (`%S`).
    NotSpaces,
    /// Matches any character, **except** all uppercase letters; equivalent to `[^A-Z]` (`%U`).
    NotUppercase,
    /// Matches any character, **except** all alphanumeric characters (digits and letters);
    /// equivalent to `[^a-zA-Z0-9]` (`%W`).
    NotAlphanumerics,
    /// Matches any character, **except** all hexadecimal digits; equivalent to `[^0-9a-fA-F]`
    /// (`%X`).
    NotHexadecimals,
    /// Matches the character, **except** the NULL character / `0` byte; equivalent to `[^\0]`
    /// (`%Z`).
    NotZeroByte,
}

/// A token as used by the internal lexer. Exposed to the public API for use in [`Error`]s.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    /// `^`
    Start,
    /// `$`
    End,
    /// `.`
    Any,
    /// `*`
    ZeroOrMore,
    /// `+`
    OneOrMore,
    /// `-`
    ZeroOrMoreLazy,
    /// `?`
    ZeroOrOne,
    /// `^` in a set
    Inverse,

    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `[`
    LBrack,
    /// `]`
    RBrack,

    /// A literal character
    Char(char),
    /// An escaped character
    Escaped(char),
    /// A character class
    Class(Class),
    /// A reference to a previous capture group
    CaptureRef(u8),
    /// `%b`
    Balanced(char, char),
    /// `%f`
    Frontier,

    /// End of file
    Eof,
}

///////////////////////////
// Trait implementations //
///////////////////////////

impl TryFrom<Token> for Quantifier {
    type Error = ();

    fn try_from(value: Token) -> std::result::Result<Self, Self::Error> {
        match value {
            Token::ZeroOrMore => Ok(Self::ZeroOrMore),
            Token::OneOrMore => Ok(Self::OneOrMore),
            Token::ZeroOrMoreLazy => Ok(Self::ZeroOrMoreLazy),
            Token::ZeroOrOne => Ok(Self::ZeroOrOne),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for Class {
    type Error = ();

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'a' => Ok(Self::Letters),
            'c' => Ok(Self::Controls),
            'd' => Ok(Self::Digits),
            'g' => Ok(Self::Printable),
            'l' => Ok(Self::Lowercase),
            'p' => Ok(Self::Punctuations),
            's' => Ok(Self::Spaces),
            'u' => Ok(Self::Uppercase),
            'w' => Ok(Self::Alphanumerics),
            'x' => Ok(Self::Hexadecimals),
            'z' => Ok(Self::ZeroByte),

            'A' => Ok(Self::NotLetters),
            'C' => Ok(Self::NotControls),
            'D' => Ok(Self::NotDigits),
            'G' => Ok(Self::NotPrintable),
            'L' => Ok(Self::NotLowercase),
            'P' => Ok(Self::NotPunctuations),
            'S' => Ok(Self::NotSpaces),
            'U' => Ok(Self::NotUppercase),
            'W' => Ok(Self::NotAlphanumerics),
            'X' => Ok(Self::NotHexadecimals),
            'Z' => Ok(Self::NotZeroByte),

            _ => Err(()),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Start => write!(f, "^"),
            Token::End => write!(f, "$"),
            Token::Any => write!(f, "."),
            Token::ZeroOrMore => write!(f, "*"),
            Token::OneOrMore => write!(f, "+"),
            Token::ZeroOrMoreLazy => write!(f, "-"),
            Token::ZeroOrOne => write!(f, "?"),
            Token::Inverse => write!(f, "^"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrack => write!(f, "["),
            Token::RBrack => write!(f, "]"),
            Token::Char(char) => write!(f, "{char}"),
            Token::Escaped(char) => write!(f, "%{char}"),
            Token::Class(class) => write!(f, "{class}"),
            Token::CaptureRef(id) => write!(f, "%{id}"),
            Token::Balanced(open, close) => write!(f, "%b{open}{close}"),
            Token::Frontier => write!(f, "%f"),
            Token::Eof => write!(f, "end of file"),
        }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Class::Letters => write!(f, "%a"),
            Class::Controls => write!(f, "%c"),
            Class::Digits => write!(f, "%d"),
            Class::Printable => write!(f, "%g"),
            Class::Lowercase => write!(f, "%l"),
            Class::Punctuations => write!(f, "%p"),
            Class::Spaces => write!(f, "%s"),
            Class::Uppercase => write!(f, "%u"),
            Class::Alphanumerics => write!(f, "%w"),
            Class::Hexadecimals => write!(f, "%x"),
            Class::ZeroByte => write!(f, "%z"),
            Class::NotLetters => write!(f, "%A"),
            Class::NotControls => write!(f, "%C"),
            Class::NotDigits => write!(f, "%D"),
            Class::NotPrintable => write!(f, "%G"),
            Class::NotLowercase => write!(f, "%L"),
            Class::NotPunctuations => write!(f, "%P"),
            Class::NotSpaces => write!(f, "%S"),
            Class::NotUppercase => write!(f, "%U"),
            Class::NotAlphanumerics => write!(f, "%W"),
            Class::NotHexadecimals => write!(f, "%X"),
            Class::NotZeroByte => write!(f, "%Z"),
        }
    }
}
