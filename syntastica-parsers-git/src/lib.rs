#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

#[cfg(not(feature = "some"))]
compile_error!("current feature set includes no parsers");

#[cfg(feature = "some")]
syntastica_macros::parsers_ffi!();

#[cfg(feature = "some")]
impl ParserProviderImpl<'static> {
    pub fn all() -> Self {
        Self(None)
    }
}

#[cfg(feature = "some")]
impl<'src> ParserProviderImpl<'src> {
    pub fn with_languages(languages: &'src [&'src str]) -> Self {
        Self(Some(languages))
    }
}
