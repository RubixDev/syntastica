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

pub fn from_slice_multi(input: &(impl AsRef<[u8]> + ?Sized)) -> Result<Vec<Sexpr<'_>>> {
    let (sexprs, errors) = Parser::parse(lex::lex(input.as_ref()));
    match errors.is_empty() {
        true => Ok(sexprs),
        false => Err(errors),
    }
}

/// A single node of the tree
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

/// An owned version of [`Sexpr`]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParenKind {
    Round,
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
