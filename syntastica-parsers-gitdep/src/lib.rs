#![doc = include_str!("../README.md")]
#![cfg_attr(
    feature = "docs",
    cfg_attr(doc, doc = ::document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#))
)]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

#[cfg(not(feature = "some"))]
compile_error!("current feature set includes no parsers");

syntastica_macros::parsers_gitdep!();

impl ParserProviderImpl<'static> {
    /// Create a new [`ParserProviderImpl`] with all languages in the enabled feature set.
    pub fn all() -> Self {
        Self(None)
    }
}

impl<'src> ParserProviderImpl<'src> {
    /// Create a new [`ParserProviderImpl`] with support for specific languages.
    pub fn with_languages(languages: &'src [&'src str]) -> Self {
        Self(Some(languages))
    }
}
