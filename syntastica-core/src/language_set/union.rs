use std::borrow::Cow;

use syntastica_highlight::HighlightConfiguration;
use tft::FileType;

use crate::Result;

use super::{LanguageSet, SupportedLanguage};

/// A combination of two arbitrary [`LanguageSet`]s into one.
///
/// [`Union`] implements [`LanguageSet`] for a pair of any two [`LanguageSet`]s. This allows
/// combining multiple sets. The `R` set acts as the fallback. If a language is requested, first
/// the left set is queried for a match, and only if it fails to provide one the right set will be
/// queried.
///
/// The accompanying language type is [`EitherLang`].
#[derive(Default)]
pub struct Union<L, R> {
    left: L,
    right: R,
}

/// Either one of two language types.
///
/// Used to represent a [`SupportedLanguage`] for the [`Union`] language set.
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum EitherLang<L, R> {
    /// A language of the left set.
    Left(L),

    /// A language of the right set.
    Right(R),
}

impl<L, R> Union<L, R> {
    /// Create a new [`Union`] set by combining `left` and `right`.
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }

    /// Get a reference to the left set.
    pub fn left(&self) -> &L {
        &self.left
    }

    /// Get a reference to the right set.
    pub fn right(&self) -> &R {
        &self.right
    }
}

impl<'s, L, R> LanguageSet<'s> for Union<L, R>
where
    L: LanguageSet<'s>,
    R: LanguageSet<'s>,
{
    type Language = EitherLang<L::Language, R::Language>;

    fn get_language(&self, language: Self::Language) -> Result<&HighlightConfiguration> {
        match language {
            EitherLang::Left(lang) => self.left.get_language(lang),
            EitherLang::Right(lang) => self.right.get_language(lang),
        }
    }
}

impl<L, R> From<L> for EitherLang<L, R> {
    fn from(value: L) -> Self {
        Self::Left(value)
    }
}

impl<'set, L, R, S, T> SupportedLanguage<'set, Union<S, T>> for EitherLang<L, R>
where
    L: SupportedLanguage<'set, S>,
    R: SupportedLanguage<'set, T>,
{
    fn name(&self) -> Cow<'_, str> {
        match self {
            EitherLang::Left(lang) => lang.name(),
            EitherLang::Right(lang) => lang.name(),
        }
    }

    fn for_name(name: impl AsRef<str>, set: &'set Union<S, T>) -> Result<Self> {
        L::for_name(name.as_ref(), &set.left)
            .map(Self::Left)
            .or_else(|_| R::for_name(name, &set.right).map(Self::Right))
    }

    fn for_file_type(file_type: FileType, set: &'set Union<S, T>) -> Option<Self> {
        L::for_file_type(file_type, &set.left)
            .map(Self::Left)
            .or_else(|| R::for_file_type(file_type, &set.right).map(Self::Right))
    }

    fn for_injection(name: impl AsRef<str>, set: &'set Union<S, T>) -> Option<Self> {
        L::for_injection(name.as_ref(), &set.left)
            .map(Self::Left)
            .or_else(|| R::for_injection(name, &set.right).map(Self::Right))
    }
}
