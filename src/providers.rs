use std::{borrow::Cow, collections::HashMap, debug_assert, ops::Deref};

use syntastica_highlight::HighlightConfiguration;

pub use tree_sitter::Language;

use crate::{config::ResolvedConfig, Style};

pub type Parsers = HashMap<String, Language>;
pub type Queries<'a> = HashMap<String, [Cow<'a, str>; 3]>;
pub type Languages = HashMap<String, HighlightConfiguration>;

pub struct ConfiguredLanguages {
    langs: Languages,
    highlight_keys: Vec<String>,
    highlight_styles: Vec<Style>,
    default_index: Option<usize>,
}

impl ConfiguredLanguages {
    pub fn configure(mut unconfigured: Languages, config: ResolvedConfig) -> Self {
        let config = config.into_inner();
        let mut highlight_keys = Vec::with_capacity(config.len());
        let mut highlight_styles = Vec::with_capacity(config.len());
        let mut default_index = None;
        for (index, (key, style)) in config.into_iter().enumerate() {
            if key == "text" {
                default_index = Some(index);
            }
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
            default_index,
        }
    }

    pub fn try_configure<C, E>(provider: &impl LanguageProvider, config: C) -> crate::Result<Self>
    where
        C: TryInto<ResolvedConfig, Error = E>,
        crate::Error: From<E>,
    {
        Ok(Self::configure(
            provider.get_languages()?,
            config.try_into()?,
        ))
    }

    pub fn highlight_keys(&self) -> &[String] {
        &self.highlight_keys
    }

    pub fn highlight_styles(&self) -> &[Style] {
        &self.highlight_styles
    }

    pub fn default_style(&self) -> Option<Style> {
        self.default_index.map(|idx| self.highlight_styles[idx])
    }
}

impl Deref for ConfiguredLanguages {
    type Target = Languages;

    fn deref(&self) -> &Self::Target {
        &self.langs
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
