mod error;
mod lexer;
mod parser;

#[cfg(feature = "to-regex")]
mod to_regex;

use std::fmt::{self, Display, Formatter};

pub use error::*;
use lexer::Lexer;
use parser::Parser;

///////////////
// Functions //
///////////////

pub fn parse(pattern: impl AsRef<str>) -> Result<Pattern> {
    Parser::parse(Lexer::lex(pattern.as_ref())?)
}

#[cfg(feature = "to-regex")]
pub use to_regex::try_to_regex;

///////////
// Types //
///////////

pub type Pattern = Vec<PatternObject>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternObject {
    Any,
    Start,
    End,

    String(String),
    Quantifier(Quantifier, Box<PatternObject>),
    Escaped(char),
    Class(Class),
    CaptureRef(u8),
    Balanced(char, char),
    Frontier(Box<PatternObject>),

    Capture(u8, Pattern),
    Set(Vec<SetPatternObject>),
    InverseSet(Vec<SetPatternObject>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetPatternObject {
    Char(char),
    Escaped(char),
    Range(char, char),
    Class(Class),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantifier {
    /// `*`
    ZeroOrMore,
    /// `+`
    OneOrMore,
    /// `-`
    ZeroOrMoreLazy,
    /// `?`
    ZeroOrOne,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    /// `%a`
    Letters,
    /// `%c`
    Controls,
    /// `%d`
    Digits,
    /// `%g`
    Printable,
    /// `%l`
    Lowercase,
    /// `%p`
    Punctuations,
    /// `%s`
    Spaces,
    /// `%u`
    Uppercase,
    /// `%w`
    Alphanumerics,
    /// `%x`
    Hexadecimals,
    /// `%z`
    ZeroByte,

    /// `%A`
    NotLetters,
    /// `%C`
    NotControls,
    /// `%D`
    NotDigits,
    /// `%G`
    NotPrintable,
    /// `%L`
    NotLowercase,
    /// `%P`
    NotPunctuations,
    /// `%S`
    NotSpaces,
    /// `%U`
    NotUppercase,
    /// `%W`
    NotAlphanumerics,
    /// `%X`
    NotHexadecimals,
    /// `%Z`
    NotZeroByte,
}

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
