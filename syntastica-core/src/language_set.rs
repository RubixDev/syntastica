//! Defines the [`LanguageSet`] trait and some related types.
//!
//! Also re-exports [`syntastica_highlight::HighlightConfiguration`] and [`tree_sitter::Language`].

use std::borrow::Cow;

pub use syntastica_highlight::HighlightConfiguration;

pub use crate::ts_runtime::Language;

/// Describes a type that is able to provide tree-sitter parsers and queries.
///
/// All official syntastica parser collections provide a type called `LanguageSetImpl` which
/// implements this trait. See
/// [the project overview](https://rubixdev.github.io/syntastica/syntastica/#parser-collections)
/// for more information on that.
///
/// A [`LanguageSet`] has two different uses:
///
/// 1. Providing tree-sitter parsers and queries (see [`get_language`](LanguageSet::get_language))
/// 2. Resolving a language name based on a file extension or injection name (see
///    [`for_extension`](LanguageSet::for_extension) and
///    [`for_injection`](LanguageSet::for_injection))
pub trait LanguageSet {
    /// Find a language based on the given file extension.
    ///
    /// If the set includes a language for the given file extension, then the name of the language
    /// that file extension belongs to should be returned. This name must result in an `Ok` value
    /// when passed to [`get_language`](LanguageSet::get_language). If the file extension is _not_
    /// supported, `None` should be returned.
    fn for_extension<'a>(&self, file_extension: &'a str) -> Option<Cow<'a, str>>;

    /// Find a language for an injection.
    ///
    /// The passed `name` could be any string that somehow identifies a language. This very much
    /// depends on the source of the injection. For example, in fenced code blocks in markdown, the
    /// text after the opening `` ``` `` is passed to this function.
    ///
    /// Note that this function does not get called, if [`get_language`](LanguageSet::get_language)
    /// was able to return a language for `name`.
    ///
    /// If the set includes the requested language, then the name of the language that should
    /// be used for the injection should be returned. This name must result in an `Ok` value when
    /// passed to [`get_language`](LanguageSet::get_language). If no matching language is found,
    /// `None` should be returned.
    ///
    /// The default implementation forwards to [`for_extension`](LanguageSet::for_extension).
    fn for_injection<'a>(&self, name: &'a str) -> Option<Cow<'a, str>> {
        self.for_extension(name)
    }

    /// Get the language with the given name.
    ///
    /// **The returned [`HighlightConfiguration`] _must_ be configured with
    /// [`THEME_KEYS`](crate::theme::THEME_KEYS).**
    ///
    /// If the set does not include the requested language, implementations should return an
    /// [`UnsupportedLanguage`](crate::Error::UnsupportedLanguage) error.
    fn get_language(&self, name: &str) -> Result<&HighlightConfiguration, crate::Error>;
}
