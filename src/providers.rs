use std::{borrow::Cow, collections::HashMap, debug_assert};

pub use tree_sitter::Language;
use tree_sitter_highlight::HighlightConfiguration;

pub trait ParserProvider {
    fn get_parsers(&mut self) -> Result<HashMap<String, Language>, crate::Error>;

    fn by_extension(&self, file_extension: &str) -> Option<Cow<'_, str>>;

    fn by_injection_name(&self, name: &str) -> Option<Cow<'_, str>>;
}

pub trait LanguageProvider: ParserProvider {
    fn get_queries(&mut self) -> Result<HashMap<String, [Cow<'_, str>; 3]>, crate::Error>;

    fn prepare(&mut self) -> Result<HashMap<String, HighlightConfiguration>, crate::Error> {
        let parsers = self.get_parsers()?;
        let queries = self.get_queries()?;
        debug_assert!(parsers.len() == queries.len());

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

#[cfg(feature = "queries")]
impl<T> LanguageProvider for T
where
    T: ParserProvider,
{
    fn get_queries(&mut self) -> Result<HashMap<String, [Cow<'_, str>; 3]>, crate::Error> {
        syntastica_macros::queries_provider!()
    }
}
