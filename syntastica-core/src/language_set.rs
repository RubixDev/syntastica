//! Defines the [`LanguageSet`] trait and some related types.
//!
//! Also re-exports [`syntastica_highlight::HighlightConfiguration`] and [`tree_sitter::Language`].

use std::{borrow::Cow, path::Path};

pub use syntastica_highlight::HighlightConfiguration;
pub use tft::FileType;

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
/// 2. Resolving a language name based on a file type or injection name (see
///    [`for_file_type`](LanguageSet::for_file_type) and
///    [`for_injection`](LanguageSet::for_injection))
pub trait LanguageSet {
    /// Find a language based on the given [`FileType`].
    ///
    /// If the set includes a language for the given file type, then the name of that language
    /// should be returned. This name must result in an `Ok` value when passed to
    /// [`get_language`](LanguageSet::get_language). If the file type is _not_ supported, [`None`]
    /// should be returned.
    fn for_file_type(&self, file_type: FileType) -> Option<Cow<'static, str>>;

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
    /// The default implementation tries to detect a [`FileType`] using `name` as both a filename
    /// and a file extension, and passes that to [`for_file_type`](LanguageSet::for_file_type).
    fn for_injection<'a>(&self, name: &'a str) -> Option<Cow<'a, str>> {
        // try to detect a file type, once using the name as a full path and once using it as the
        // extension, and if one is found, pass it to `self.for_file_type`
        tft::try_detect(Path::new(name), "")
            .or_else(|| tft::try_detect(Path::new(&format!("file.{name}")), ""))
            .and_then(|ft| self.for_file_type(ft))
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
