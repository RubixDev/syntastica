// TODO: docs

use std::{borrow::Cow, cell::RefCell, collections::HashMap, path::PathBuf};

use anyhow::Result;
use syntastica_core::language_set::{
    FileType, HighlightConfiguration, LanguageSet, SupportedLanguage,
};

use loader::{Config, Loader};

mod loader;

// TODO: this workaround is not perfect in any way
pub enum Lang {
    Scope(Box<str>),
    Injection(Box<str>),
}

impl SupportedLanguage for Lang {
    fn name(&self) -> Cow<'_, str> {
        match self {
            Self::Scope(scope) => Cow::Borrowed(scope),
            Self::Injection(name) => Cow::Owned(format!("###{name}")),
        }
    }

    fn for_name(name: impl AsRef<str>) -> syntastica_core::Result<Self> {
        match name.as_ref().strip_prefix("###") {
            Some(injection_name) => Ok(Self::Injection(injection_name.into())),
            None => Ok(Self::Scope(name.as_ref().into())),
        }
    }

    fn for_file_type(_file_type: FileType) -> Option<Self> {
        None
    }

    fn for_injection(name: impl AsRef<str>) -> Option<Self> {
        Some(Self::Injection(name.as_ref().into()))
    }
}

// TODO: `query_search_dirs`?
pub struct LanguageLoader {
    loader: Loader,
    highlight_configs: RefCell<HashMap<Box<str>, &'static HighlightConfiguration>>,
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
    type Language = Lang;

    fn get_language(
        &self,
        language: Self::Language,
    ) -> syntastica_core::Result<&HighlightConfiguration> {
        let name = match language {
            Lang::Scope(scope) => scope,
            Lang::Injection(name) => self
                .loader
                .language_configuration_for_injection_string(&name)
                .map_err(|_| syntastica_core::Error::UnsupportedLanguage(name.to_string()))?
                .ok_or_else(|| syntastica_core::Error::UnsupportedLanguage(name.to_string()))?
                .1
                .scope
                .clone()
                .ok_or_else(|| syntastica_core::Error::UnsupportedLanguage(name.to_string()))?
                .into_boxed_str(),
        };

        if let Some(config) = self.highlight_configs.borrow().get(name.as_ref()) {
            return Ok(*config);
        }

        let (lang, lang_config) = self
            .loader
            .language_configuration_for_scope(name.as_ref())
            .map_err(|err| syntastica_core::Error::Custom(err.to_string()))?
            .ok_or_else(|| syntastica_core::Error::UnsupportedLanguage(name.to_string()))?;

        let config_ref = lang_config
            .highlight_config(lang)
            .map_err(|err| syntastica_core::Error::Custom(err.to_string()))?
            .ok_or_else(|| syntastica_core::Error::UnsupportedLanguage(name.to_string()))?;
        self.highlight_configs.borrow_mut().insert(name, config_ref);

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
