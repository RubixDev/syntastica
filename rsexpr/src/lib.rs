//! # rsexpr
//!
//! Small and simple S-expression parsing and manipulation with support for
//! square-bracketed groups and strings. Used by
//! [syntastica](https://crates.io/crates/syntastica) for processing tree-sitter
//! queries.
//!
//! Have a look at [`Sexpr`], [`OwnedSexpr`], [`from_slice`], and [`from_slice_multi`] for more
//! information.
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

mod display;
mod error;
mod lex;
mod parser;

use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
};

pub use error::*;
use lex::Token;
use parser::Parser;

/// Parse a [`Sexpr`] from bytes. This fails if there is more than one S-expression in the
/// input. To allow an arbitrary amount of S-expressions, have a look at [`from_slice_multi`].
///
/// ## Example
/// ```
/// let sexpr = rsexpr::from_slice(b"((\"foo bar\")(baz [1 2 3]))").unwrap();
/// println!("{sexpr:#}");
/// if let rsexpr::Sexpr::List(list) = sexpr {
///     assert_eq!(list.len(), 2);
/// }
/// ```
///
/// ## Errors
/// If the parsing failed, a list of [`Error`]s is returned.
/// Additionally, the function will fail if the input does not contain exactly _one_ S-expression
/// (see [`Error::EmptyInput`] and [`Error::ExtraSexprs`]).
pub fn from_slice(input: &(impl AsRef<[u8]> + ?Sized)) -> Result<Sexpr<'_>> {
    let (mut sexprs, mut errors) = Parser::parse(lex::lex(input.as_ref()));
    if sexprs.len() > 1 {
        errors.push(Error::ExtraSexprs);
    }
    match errors.is_empty() {
        true => match sexprs.is_empty() {
            true => Err(vec![Error::EmptyInput]),
            false => Ok(sexprs.swap_remove(0)),
        },
        false => Err(errors),
    }
}

/// Parse multiple [`Sexpr`]s from bytes. To only parse a single one, have a look at
/// [`from_slice`].
///
/// ## Example
/// ```
/// let sexprs = rsexpr::from_slice_multi(b"(\"foo bar\") (baz [1 2 3])").unwrap();
/// for sexpr in &sexprs {
///     println!("{sexpr:#}\n");
/// }
/// assert_eq!(sexprs.len(), 2);
/// ```
///
/// ## Errors
/// If the parsing failed, a list of [`Error`]s is returned.
pub fn from_slice_multi(input: &(impl AsRef<[u8]> + ?Sized)) -> Result<Vec<Sexpr<'_>>> {
    let (sexprs, errors) = Parser::parse(lex::lex(input.as_ref()));
    match errors.is_empty() {
        true => Ok(sexprs),
        false => Err(errors),
    }
}

/// A single node of the tree. The [`Atom`](Sexpr::Atom) and [`String`](Sexpr::String) variants
/// reference the input slice. For an owned version have a look at [`OwnedSexpr`].
///
/// ## Display
/// [`Sexpr`] implements the [`Display`] trait for serializing to strings. By default, the output
/// will try to minimize the amount of spaces used and the resulting output will be on one line.
/// Enabling the formatter's `alternate` flag using `#`, causes the output to be human-friendly /
/// pretty-printed.
///
/// For example:
///
/// ```
/// let sexpr = rsexpr::from_slice(b"[ a b c ]").unwrap();
/// assert_eq!(format!("{sexpr}"), "[a b c]");
/// assert_eq!(format!("{sexpr:#}"), "[
///   a
///   b
///   c
/// ]");
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Sexpr<'src> {
    /// A list of [`Sexpr`]s surrounded by parentheses `(`, `)`
    List(Vec<Sexpr<'src>>),
    /// A list of [`Sexpr`]s surrounded by brackets `[`, `]`
    Group(Vec<Sexpr<'src>>),
    /// A sequence of bytes surrounded by quotes `"`
    String(Cow<'src, [u8]>),
    /// A sequence of bytes not including whitespace, parens, and quotes
    Atom(&'src [u8]),
}

/// An owned version of [`Sexpr`]. You can convert to and from [`Sexpr`] using the [`From`] trait.
///
/// ## Display
/// [`OwnedSexpr`] implements the [`Display`] trait for serializing to strings. By default, the output
/// will try to minimize the amount of spaces used and the resulting output will be on one line.
/// Enabling the formatter's `alternate` flag using `#`, causes the output to be human-friendly /
/// pretty-printed.
///
/// For example:
///
/// ```
/// let sexpr = rsexpr::OwnedSexpr::from(rsexpr::from_slice(b"[ a b c ]").unwrap());
/// assert_eq!(format!("{sexpr}"), "[a b c]");
/// assert_eq!(format!("{sexpr:#}"), "[
///   a
///   b
///   c
/// ]");
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum OwnedSexpr {
    /// A list of [`OwnedSexpr`]s surrounded by parentheses `(`, `)`
    List(Vec<OwnedSexpr>),
    /// A list of [`OwnedSexpr`]s surrounded by brackets `[`, `]`
    Group(Vec<OwnedSexpr>),
    /// A sequence of bytes surrounded by quotes `"`
    String(Vec<u8>),
    /// A sequence of bytes not including whitespace, parens, and quotes
    Atom(Vec<u8>),
}

impl From<Sexpr<'_>> for OwnedSexpr {
    fn from(value: Sexpr<'_>) -> Self {
        match value {
            Sexpr::List(list) => Self::List(list.into_iter().map(OwnedSexpr::from).collect()),
            Sexpr::Group(group) => Self::Group(group.into_iter().map(OwnedSexpr::from).collect()),
            Sexpr::String(string) => Self::String(string.into_owned()),
            Sexpr::Atom(atom) => Self::Atom(atom.to_vec()),
        }
    }
}

impl<'a> From<&'a OwnedSexpr> for Sexpr<'a> {
    fn from(value: &'a OwnedSexpr) -> Self {
        match value {
            OwnedSexpr::List(list) => Self::List(list.iter().map(Sexpr::from).collect()),
            OwnedSexpr::Group(group) => Self::Group(group.iter().map(Sexpr::from).collect()),
            OwnedSexpr::String(string) => Self::String(Cow::Borrowed(string)),
            OwnedSexpr::Atom(atom) => Self::Atom(atom),
        }
    }
}

macro_rules! impl_unwrap {
    ($type:ident, $func_name:ident, $func_name_ref:ident, $variant:ident, $result:ty, $name:literal) => {
        #[doc = concat!("Returns the contained [`", stringify!($variant), "`](", stringify!($type), "::", stringify!($variant), ") value, consuming `self`.")]
        #[doc = ""]
        #[doc = "## Panics"]
        #[doc = concat!("Panics if `self` is not [`", stringify!($variant), "`](", stringify!($type), "::", stringify!($variant), ").")]
        pub fn $func_name(self) -> $result {
            match self {
                $type::$variant(val) => val,
                other => panic!(
                    concat!(
                        "called `",
                        stringify!($type),
                        "::",
                        stringify!($func_name),
                        "()` on a non-",
                        $name,
                        " value: {}"
                    ),
                    other
                ),
            }
        }

        #[doc = concat!("Returns the contained [`", stringify!($variant), "`](", stringify!($type), "::", stringify!($variant), ") value by reference.")]
        #[doc = ""]
        #[doc = "## Panics"]
        #[doc = concat!("Panics if `self` is not [`", stringify!($variant), "`](", stringify!($type), "::", stringify!($variant), ").")]
        pub fn $func_name_ref(&self) -> &$result {
            match self {
                $type::$variant(val) => val,
                other => panic!(
                    concat!(
                        "called `",
                        stringify!($type),
                        "::",
                        stringify!($func_name_ref),
                        "()` on a non-",
                        $name,
                        " value: {}"
                    ),
                    other
                ),
            }
        }
    };
}

impl<'src> Sexpr<'src> {
    impl_unwrap!(
        Sexpr,
        unwrap_list,
        unwrap_list_ref,
        List,
        Vec<Sexpr<'src>>,
        "list"
    );
    impl_unwrap!(
        Sexpr,
        unwrap_group,
        unwrap_group_ref,
        Group,
        Vec<Sexpr<'src>>,
        "group"
    );
    impl_unwrap!(
        Sexpr,
        unwrap_string,
        unwrap_string_ref,
        String,
        Cow<'src, [u8]>,
        "string"
    );
    impl_unwrap!(
        Sexpr,
        unwrap_atom,
        unwrap_atom_ref,
        Atom,
        &'src [u8],
        "atom"
    );
}

impl OwnedSexpr {
    impl_unwrap!(
        OwnedSexpr,
        unwrap_list,
        unwrap_list_ref,
        List,
        Vec<OwnedSexpr>,
        "list"
    );
    impl_unwrap!(
        OwnedSexpr,
        unwrap_group,
        unwrap_group_ref,
        Group,
        Vec<OwnedSexpr>,
        "group"
    );
    impl_unwrap!(
        OwnedSexpr,
        unwrap_string,
        unwrap_string_ref,
        String,
        Vec<u8>,
        "string"
    );
    impl_unwrap!(
        OwnedSexpr,
        unwrap_atom,
        unwrap_atom_ref,
        Atom,
        Vec<u8>,
        "atom"
    );
}

/// A kind of parentheses. Used in [`Error::MissingClosingParen`] and [`Error::ExtraClosingParen`]
/// to indicate the kind of parentheses that caused the error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParenKind {
    /// Round parentheses: `()`
    Round,
    /// Square brackets: `[]`
    Square,
}

impl From<&ParenKind> for Token<'_> {
    fn from(value: &ParenKind) -> Self {
        match value {
            ParenKind::Round => Self::RParen,
            ParenKind::Square => Self::RBrack,
        }
    }
}

impl Display for ParenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Round => write!(f, ")"),
            Self::Square => write!(f, "]"),
        }
    }
}
