//! Defines the [`LanguageSet`] trait and some related types.
//!
//! Also re-exports [`syntastica_highlight::HighlightConfiguration`], [`tree_sitter::Language`],
//! and [`tft::FileType`].

use std::{borrow::Cow, path::Path};

pub use syntastica_highlight::HighlightConfiguration;
pub use tft::FileType;

pub use crate::ts_runtime::Language;

mod union;

pub use union::*;

/// A language included in a [`LanguageSet`].
///
/// Instances can be obtained with [`for_name`](SupportedLanguage::for_name),
/// [`for_file_type`](SupportedLanguage::for_file_type), and
/// [`for_injection`](SupportedLanguage::for_injection).
pub trait SupportedLanguage<'set, S>: Sized {
    /// Get the name for this language.
    ///
    /// Passing the output of this function to [`for_name`](SupportedLanguage::for_name) must
    /// always result in an [`Ok`] value.
    // TODO: should this function really be part of the trait?
    fn name(&self) -> Cow<'_, str>;

    /// Get the language with the given name.
    ///
    /// If no language for that name exists, implementations should return an
    /// [`UnsupportedLanguage`](crate::Error::UnsupportedLanguage) error.
    ///
    /// `syntastica` itself does not provide a list of valid language names, but the
    /// [official parser collections](https://rubixdev.github.io/syntastica/syntastica/#parser-collections)
    /// do. However, every string that may be returned by [`name`](SupportedLanguage::name) must
    /// result in an [`Ok`] value.
    fn for_name(name: impl AsRef<str>, set: &'set S) -> Result<Self, crate::Error>;

    /// Find a language based on the given [`FileType`].
    ///
    /// Implementations should return the language that supports the given file type, if there is
    /// any.
    fn for_file_type(file_type: FileType, set: &'set S) -> Option<Self>;

    /// Find a language for an injection.
    ///
    /// The passed `name` could be any string that somehow identifies a language. This very much
    /// depends on the source of the injection. For example, in fenced code blocks in markdown, the
    /// text after the opening `` ``` `` is passed to this function.
    ///
    /// Note that this function does not get called, if [`for_name`](SupportedLanguage::for_name)
    /// was able to return a language for `name`.
    ///
    /// The default implementation tries to detect a [`FileType`] using `name` as both a filename
    /// and a file extension, and passes that to
    /// [`for_file_type`](SupportedLanguage::for_file_type).
    fn for_injection(name: impl AsRef<str>, set: &'set S) -> Option<Self> {
        let name = name.as_ref();
        // try to detect a file type, once using the name as a full path and once using it as the
        // extension, and if one is found, pass it to `self.for_file_type`
        tft::try_detect(Path::new(name), "")
            .or_else(|| tft::try_detect(Path::new(&format!("file.{name}")), ""))
            .and_then(|ft| Self::for_file_type(ft, set))
    }
}

/// Describes a type that is able to provide tree-sitter parsers and queries.
///
/// All official syntastica parser collections provide a type called `LanguageSetImpl` which
/// implements this trait. See
/// [the project overview](https://rubixdev.github.io/syntastica/syntastica/#parser-collections)
/// for more information on that.
pub trait LanguageSet<'s>: Sized {
    /// A type identifying a language that is included in this set.
    ///
    /// The given type should usually be an enum of all included languages.
    type Language: SupportedLanguage<'s, Self>;

    /// Get the [`HighlightConfiguration`] for a given [language](LanguageSet::Language).
    ///
    /// **The returned [`HighlightConfiguration`] _must_ be
    /// [configured](HighlightConfiguration::configure) with
    /// [`THEME_KEYS`](crate::theme::THEME_KEYS).**
    ///
    /// The function is given an instance of [`Self::Language`] and as such should not need to
    /// return an [`Error::UnsupportedLanguage`](crate::Error::UnsupportedLanguage) error.
    fn get_language(
        &self,
        language: Self::Language,
    ) -> Result<&HighlightConfiguration, crate::Error>;
}
