use std::{borrow::Cow, collections::HashMap, debug_assert, ops::Deref};

use syntastica_highlight::HighlightConfiguration;

pub use tree_sitter::Language;

pub type Parsers = HashMap<String, Language>;
pub type Queries<'a> = HashMap<String, [Cow<'a, str>; 3]>;
pub type Languages = HashMap<String, HighlightConfiguration>;

pub struct ConfiguredLanguages(Languages);

impl ConfiguredLanguages {
    pub fn new(mut unconfigured: Languages) -> Self {
        for lang in unconfigured.values_mut() {
            lang.configure(crate::THEME_KEYS);
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

pub trait LanguageProvider {
    fn get_parsers(&self) -> Result<Parsers, crate::Error>;

    fn get_queries(&self) -> Result<Queries, crate::Error>;

    fn for_extension<'a>(&self, file_extension: &'a str) -> Option<Cow<'a, str>>;

    fn for_injection<'a>(&self, name: &'a str) -> Option<Cow<'a, str>> {
        self.for_extension(name)
    }

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
