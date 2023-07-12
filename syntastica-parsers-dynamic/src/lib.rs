// TODO: docs

use std::{
    borrow::Cow,
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::Result;
use syntastica_core::language_set::{HighlightConfiguration, LanguageSet};

use loader::{Config, Loader};

mod loader;

// TODO: `query_search_dirs`?
pub struct LanguageLoader {
    loader: Loader,
    highlight_configs: RefCell<HashMap<String, &'static HighlightConfiguration>>,
}

impl LanguageLoader {
    pub fn new(parser_search_dirs: Vec<PathBuf>) -> Result<Self> {
        let mut loader = Loader::new()?;
        loader.configure_highlights(syntastica_core::theme::THEME_KEYS);
        loader.find_all_languages(&Config {
            parser_directories: parser_search_dirs,
        })?;
        Ok(Self {
            loader,
            highlight_configs: RefCell::new(HashMap::new()),
        })
    }
}

impl LanguageSet for LanguageLoader {
    fn for_extension<'a>(&self, file_extension: &'a str) -> Option<Cow<'a, str>> {
        Some(
            self.loader
                .language_configuration_for_file_name(Path::new(file_extension))
                .ok()??
                .1
                .scope
                .clone()?
                .into(),
        )
    }

    fn for_injection<'a>(&self, name: &'a str) -> Option<Cow<'a, str>> {
        self.for_extension(name).or_else(|| {
            Some(
                self.loader
                    .language_configuration_for_injection_string(name)
                    .ok()??
                    .1
                    .scope
                    .clone()?
                    .into(),
            )
        })
    }

    fn get_language(&self, name: &str) -> Result<&HighlightConfiguration, syntastica_core::Error> {
        if let Some(config) = self.highlight_configs.borrow().get(name) {
            return Ok(*config);
        }

        let (lang, lang_config) = self
            .loader
            .language_configuration_for_scope(name)
            .map_err(|err| syntastica_core::Error::Custom(err.to_string()))?
            .ok_or_else(|| syntastica_core::Error::UnsupportedLanguage(name.to_owned()))?;

        let config_ref = lang_config
            .highlight_config(lang)
            .map_err(|err| syntastica_core::Error::Custom(err.to_string()))?
            .ok_or_else(|| syntastica_core::Error::UnsupportedLanguage(name.to_owned()))?;
        self.highlight_configs
            .borrow_mut()
            .insert(name.to_owned(), config_ref);

        Ok(config_ref)
    }
}

impl Drop for LanguageLoader {
    fn drop(&mut self) {
        for (_, config_ref) in self.highlight_configs.borrow_mut().drain() {
            // SAFETY: references were created using `Box::leak` and are only given out with the
            // lifetime of `&self`
            drop(unsafe { Box::from_raw(config_ref as *const _ as *mut HighlightConfiguration) });
        }
    }
}
