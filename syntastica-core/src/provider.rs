//! Defines the [`LanguageProvider`] trait and some related types.
//!
//! Also re-exports [`syntastica_highlight::HighlightConfiguration`] and [`tree_sitter::Language`].

use std::{borrow::Cow, collections::HashMap, ops::Deref};

pub use syntastica_highlight::HighlightConfiguration;

pub use crate::ts_runtime::Language;

use crate::theme::THEME_KEYS;

/// The type returned by [`LanguageProvider::get_parsers`].
///
/// It is a map from language names to instances of [`Language`].
pub type Parsers = HashMap<String, Language>;

/// The type returned by [`LanguageProvider::get_queries`].
///
/// It is a map from language names to an array of queries with three elements. These three
/// elements are the `highlights` queries, the `injections` queries, and the `locals` queries, in
/// that order.
pub type Queries<'a> = HashMap<String, [Cow<'a, str>; 3]>;

/// The type returned by [`LanguageProvider::get_languages`].
///
/// It is a map from language names to instances of [`HighlightConfiguration`]. Every
/// [`HighlightConfiguration`] internally contains a [`Language`] and the three types of queries,
/// and as such this type is a combination of both [`Parsers`] and [`Queries`].
pub type Languages = HashMap<String, HighlightConfiguration>;

/// A wrapper around [`Languages`] which guarantees the contained [`HighlightConfiguration`]s to be
/// configured with all [`THEME_KEYS`].
///
/// This type can hold that guarantee, because the only way to instantiate is, is through the
/// [`ConfiguredLanguages::new`] function, which directly configures the given [`Languages`].
pub struct ConfiguredLanguages(Languages);

impl ConfiguredLanguages {
    /// Configure the provided [`Languages`] using all [`THEME_KEYS`], returning an instance of
    /// [`ConfiguredLanguages`].
    pub fn new(mut unconfigured: Languages) -> Self {
        for lang in unconfigured.values_mut() {
            lang.configure(THEME_KEYS);
        }
        Self(unconfigured)
    }
}

impl Deref for ConfiguredLanguages {
    type Target = Languages;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Describes a type that is able to provide tree-sitter parsers and queries.
///
/// All three syntastica parser collections provide a type called `LanguageProviderImpl` which
/// implements this trait. See
/// [the project overview](https://rubixdev.github.io/syntastica/syntastica/#parser-collections)
/// for more information on that.
///
/// A [`LanguageProvider`] has two different uses:
///
/// 1. Providing tree-sitter parsers and queries (see
///    [`get_parsers`](LanguageProvider::get_parsers),
///    [`get_queries`](LanguageProvider::get_queries), and
///    [`get_languages`](LanguageProvider::get_languages))
/// 2. Resolving a language name based on a file extension or injection name (see
///    [`for_extension`](LanguageProvider::for_extension) and
///    [`for_injection`](LanguageProvider::for_injection))
pub trait LanguageProvider {
    /// Provide parsers for all supported (or selected) languages this provider has to offer.
    ///
    /// For every entry in the returned map, the map returned by
    /// [`get_queries`](LanguageProvider::get_queries) must contain an entry with the same key.
    ///
    /// See [`Parsers`] for information on the return type.
    fn get_parsers(&self) -> Result<Parsers, crate::Error>;

    /// Provide queries for all supported (or selected) languages this provider has to offer.
    ///
    /// For every entry in the map returned by [`get_parsers`](LanguageProvider::get_parsers),
    /// the map returned here must contain an entry with the same key.
    ///
    /// See [`Queries`] for information on the return type.
    fn get_queries(&self) -> Result<Queries<'_>, crate::Error>;

    /// Find a language based on the given file extension.
    ///
    /// If the provider supports the given file extension, then the name of the language that file
    /// extension belongs to should be returned. This name must be a key in the map returned by
    /// [`get_languages`](LanguageProvider::get_languages). If the file extension is _not_
    /// supported, `None` should be returned.
    fn for_extension<'a>(&self, file_extension: &'a str) -> Option<Cow<'a, str>>;

    /// Find a language for an injection.
    ///
    /// The passed `name` could be any string that somehow identifies a language. This very much
    /// depends on the source of the injection. For example, in fenced code blocks in markdown, the
    /// text after the opening `` ``` `` is passed to this function.
    ///
    /// Note that this function does not get called, if the `name` is a valid key in the map
    /// returned by [`get_languages`](LanguageProvider::get_languages).
    ///
    /// If the provider supports the requested language, then the name of the language that should
    /// be used for the injection should be returned. This name must be a key in the map returned
    /// by [`get_languages`](LanguageProvider::get_languages). If no matching language is found,
    /// `None` should be returned.
    ///
    /// The default implementation forwards to [`for_extension`](LanguageProvider::for_extension).
    fn for_injection<'a>(&self, name: &'a str) -> Option<Cow<'a, str>> {
        self.for_extension(name)
    }

    /// Provide all supported (or selected) languages this provider has to offer.
    ///
    /// See [`Languages`] for information on the return type.
    ///
    /// The default implementation calls [`get_parsers`](LanguageProvider::get_parsers) and
    /// [`get_queries`](LanguageProvider::get_queries), and merges the results.
    // TODO: it might be slightly faster to remove the reliance on two hashmaps by removing `get_parsers` and `get_queries`
    fn get_languages(&self) -> Result<Languages, crate::Error> {
        let parsers = self.get_parsers()?;
        let queries = self.get_queries()?;
        debug_assert!(parsers.len() <= queries.len());

        let mut map = HashMap::with_capacity(parsers.len());
        for (name, parser) in parsers {
            let queries = queries
                .get(&name)
                .ok_or_else(|| crate::Error::MissingQueries(name.clone()))?;
            map.insert(
                name,
                HighlightConfiguration::new(parser, &queries[0], &queries[1], &queries[2])?,
            );
        }

        Ok(map)
    }
}
