use std::{borrow::Cow, collections::HashMap, debug_assert, ops::Deref};

use tree_sitter_highlight::HighlightConfiguration;

pub use tree_sitter::Language;

use crate::{config::ResolvedConfig, Style};

pub type Parsers = HashMap<String, Language>;
pub type Queries<'a> = HashMap<String, [Cow<'a, str>; 3]>;
pub type Languages = HashMap<String, HighlightConfiguration>;

pub struct ConfiguredLanguages {
    // TODO: wrap in Cow
    langs: Languages,
    highlight_keys: Vec<String>,
    highlight_styles: Vec<Style>,
}

impl ConfiguredLanguages {
    pub fn configure(mut unconfigured: Languages, config: ResolvedConfig) -> Self {
        let config = config.into_inner();
        let mut highlight_keys = Vec::with_capacity(config.len());
        let mut highlight_styles = Vec::with_capacity(config.len());
        for (key, style) in config {
            highlight_keys.push(key);
            highlight_styles.push(style);
        }

        for lang in unconfigured.values_mut() {
            lang.configure(&highlight_keys);
        }

        Self {
            langs: unconfigured,
            highlight_keys,
            highlight_styles,
        }
    }

    pub fn highlight_keys(&self) -> &[String] {
        &self.highlight_keys
    }

    pub fn highlight_styles(&self) -> &[Style] {
        &self.highlight_styles
    }
}

impl Deref for ConfiguredLanguages {
    type Target = Languages;

    fn deref(&self) -> &Self::Target {
        &self.langs
    }
}

pub trait ParserProvider {
    fn get_parsers(&self) -> Result<Parsers, crate::Error>;

    fn for_extension(&self, file_extension: &str) -> Option<Cow<'_, str>>;

    fn for_injection(&self, name: &str) -> Option<Cow<'_, str>> {
        self.for_extension(name)
    }
}

pub trait LanguageProvider: ParserProvider {
    fn get_queries(&self) -> Result<Queries, crate::Error>;

    fn get_languages(&self) -> Result<Languages, crate::Error> {
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
    fn get_queries(&self) -> Result<Queries, crate::Error> {
        syntastica_macros::queries_provider!()
    }
}
