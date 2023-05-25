//! # rsexpr
//!
//! Small and simple S-expression parsing and manipulation with support for
//! square-bracketed groups and strings. Used by
//! [syntastica](https://crates.io/crates/syntastica) for processing tree-sitter
//! queries.
//!
//! Have a look at [`Sexpr`], [`OwnedSexpr`], [`from_slice`], and [`from_slice_multi`] for more
//! information.
#![cfg_attr(
    feature = "docs",
    cfg_attr(doc, doc = ::document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#))
)]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

mod display;
mod error;
mod lex;
mod parser;

use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
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
pub fn from_slice_multi(input: &(impl AsRef<[u8]> + ?Sized)) -> Result<Sexprs<'_>> {
    let (sexprs, errors) = Parser::parse(lex::lex(input.as_ref()));
    match errors.is_empty() {
        true => Ok(sexprs),
        false => Err(errors),
    }
}

/// A thin wrapper around `Vec<Sexpr>` with its own [`Display`] implementation.
/// See [`Sexpr`] for more information.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Sexprs<'src>(Vec<Sexpr<'src>>);

/// A thin wrapper around `Vec<OwnedSexpr>` with its own [`Display`] implementation.
/// See [`OwnedSexpr`] for more information.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct OwnedSexprs(Vec<OwnedSexpr>);

/// A single node of the tree. The [`Atom`](Sexpr::Atom) and [`String`](Sexpr::String) variants
/// reference the input slice. For an owned version have a look at [`OwnedSexpr`].
///
/// ## Display
/// [`Sexpr`] implements the [`Display`] trait for serializing to strings. By default, the output
/// will try to minimize the amount of spaces used and the resulting output will be on one line.
/// Enabling the formatter's `alternate` flag using `#`, causes the output to be human-friendly /
/// pretty-printed. Setting the `precision` with `.` additionally allows to specify the number of
/// spaces used for indentation (2 by default).
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
/// assert_eq!(format!("{sexpr:#.4}"), "[
///     a
///     b
///     c
/// ]");
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Sexpr<'src> {
    /// A list of [`Sexpr`]s surrounded by parentheses `(`, `)`
    List(Sexprs<'src>),
    /// A list of [`Sexpr`]s surrounded by brackets `[`, `]`
    Group(Sexprs<'src>),
    /// A sequence of bytes surrounded by quotes `"`
    String(Cow<'src, [u8]>),
    /// A sequence of bytes not including whitespace, parens, and quotes
    Atom(&'src [u8]),
    /// A line comment, including the leading `;`
    #[cfg(feature = "comments")]
    Comment(&'src [u8]),
}

/// An owned version of [`Sexpr`]. You can convert to and from [`Sexpr`] using the [`From`] trait.
///
/// ## Display
/// [`OwnedSexpr`] implements the [`Display`] trait for serializing to strings. By default, the output
/// will try to minimize the amount of spaces used and the resulting output will be on one line.
/// Enabling the formatter's `alternate` flag using `#`, causes the output to be human-friendly /
/// pretty-printed. Setting the `precision` with `.` additionally allows to specify the number of
/// spaces used for indentation (2 by default).
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
/// assert_eq!(format!("{sexpr:#.4}"), "[
///     a
///     b
///     c
/// ]");
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum OwnedSexpr {
    /// A list of [`OwnedSexpr`]s surrounded by parentheses `(`, `)`
    List(OwnedSexprs),
    /// A list of [`OwnedSexpr`]s surrounded by brackets `[`, `]`
    Group(OwnedSexprs),
    /// A sequence of bytes surrounded by quotes `"`
    String(Vec<u8>),
    /// A sequence of bytes not including whitespace, parens, and quotes
    Atom(Vec<u8>),
    /// A line comment, including the leading `;`
    #[cfg(feature = "comments")]
    Comment(Vec<u8>),
}

///////////////////////////
// Trait implementations //
///////////////////////////

impl<'src> Deref for Sexprs<'src> {
    type Target = Vec<Sexpr<'src>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for OwnedSexprs {
    type Target = Vec<OwnedSexpr>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sexprs<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DerefMut for OwnedSexprs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'src> FromIterator<Sexpr<'src>> for Sexprs<'src> {
    fn from_iter<T: IntoIterator<Item = Sexpr<'src>>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl FromIterator<OwnedSexpr> for OwnedSexprs {
    fn from_iter<T: IntoIterator<Item = OwnedSexpr>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl<'src> IntoIterator for Sexprs<'src> {
    type Item = Sexpr<'src>;
    type IntoIter = std::vec::IntoIter<Sexpr<'src>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl IntoIterator for OwnedSexprs {
    type Item = OwnedSexpr;
    type IntoIter = std::vec::IntoIter<OwnedSexpr>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, 'src> IntoIterator for &'a Sexprs<'src> {
    type Item = &'a Sexpr<'src>;
    type IntoIter = std::slice::Iter<'a, Sexpr<'src>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a OwnedSexprs {
    type Item = &'a OwnedSexpr;
    type IntoIter = std::slice::Iter<'a, OwnedSexpr>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, 'src> IntoIterator for &'a mut Sexprs<'src> {
    type Item = &'a mut Sexpr<'src>;
    type IntoIter = std::slice::IterMut<'a, Sexpr<'src>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<'a> IntoIterator for &'a mut OwnedSexprs {
    type Item = &'a mut OwnedSexpr;
    type IntoIter = std::slice::IterMut<'a, OwnedSexpr>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<'a> From<&'a OwnedSexprs> for Sexprs<'a> {
    fn from(value: &'a OwnedSexprs) -> Self {
        value.iter().map(Sexpr::from).collect()
    }
}

impl From<Sexprs<'_>> for OwnedSexprs {
    fn from(value: Sexprs<'_>) -> Self {
        value.into_iter().map(OwnedSexpr::from).collect()
    }
}

impl<'src> From<Vec<Sexpr<'src>>> for Sexprs<'src> {
    fn from(value: Vec<Sexpr<'src>>) -> Self {
        Self(value)
    }
}

impl From<Vec<OwnedSexpr>> for OwnedSexprs {
    fn from(value: Vec<OwnedSexpr>) -> Self {
        Self(value)
    }
}

impl Default for Sexprs<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for OwnedSexprs {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> From<&'a OwnedSexpr> for Sexpr<'a> {
    fn from(value: &'a OwnedSexpr) -> Self {
        match value {
            OwnedSexpr::List(list) => Self::List(list.into()),
            OwnedSexpr::Group(group) => Self::Group(group.iter().map(Sexpr::from).collect()),
            OwnedSexpr::String(string) => Self::String(Cow::Borrowed(string)),
            OwnedSexpr::Atom(atom) => Self::Atom(atom),
            #[cfg(feature = "comments")]
            OwnedSexpr::Comment(comment) => Self::Comment(comment),
        }
    }
}

impl From<Sexpr<'_>> for OwnedSexpr {
    fn from(value: Sexpr<'_>) -> Self {
        match value {
            Sexpr::List(list) => Self::List(list.into()),
            Sexpr::Group(group) => Self::Group(group.into()),
            Sexpr::String(string) => Self::String(string.into_owned()),
            Sexpr::Atom(atom) => Self::Atom(atom.to_vec()),
            #[cfg(feature = "comments")]
            Sexpr::Comment(comment) => Self::Comment(comment.to_vec()),
        }
    }
}

////////////////////////////
// Method implementations //
////////////////////////////

impl<'src> Sexprs<'src> {
    /// Create a new, empty list of [`Sexpr`]s
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl OwnedSexprs {
    /// Create a new, empty list of [`OwnedSexpr`]s
    pub fn new() -> Self {
        Self(vec![])
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
        Sexprs<'src>,
        "list"
    );
    impl_unwrap!(
        Sexpr,
        unwrap_group,
        unwrap_group_ref,
        Group,
        Sexprs<'src>,
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
        OwnedSexprs,
        "list"
    );
    impl_unwrap!(
        OwnedSexpr,
        unwrap_group,
        unwrap_group_ref,
        Group,
        OwnedSexprs,
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
