use std::{borrow::Cow, collections::HashMap};

use tree_sitter_highlight::HighlightConfiguration;

pub trait LanguageProvider {
    fn prepare(&mut self) -> Result<HashMap<String, HighlightConfiguration>, crate::Error>;

    fn by_extension(&self, file_extension: &str) -> Option<Cow<'_, str>>;
}

#[cfg(feature = "parsers-some")]
pub struct DefaultLanguageProvider;

#[cfg(feature = "parsers-some")]
impl LanguageProvider for DefaultLanguageProvider {
    fn prepare(&mut self) -> Result<HashMap<String, HighlightConfiguration>, crate::Error> {
        syntastica_macros::lang_provider_prepare!()
    }

    fn by_extension(&self, file_extension: &str) -> Option<Cow<'_, str>> {
        syntastica_macros::lang_provider_extensions!()
    }
}
