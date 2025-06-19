#![doc = include_str!("../README.md")]
#![cfg_attr(
    feature = "docs",
    cfg_attr(doc, doc = ::document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#))
)]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

use std::{
    borrow::Cow,
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
    path::PathBuf,
};

use anyhow::Result;
use syntastica_core::language_set::{
    FileType, HighlightConfiguration, LanguageSet, SupportedLanguage,
};

use loader::{Config, Loader};

mod loader;

/// A successfully loaded language.
///
/// Instances can only be obtained through the functions of the [`SupportedLanguage`] trait, and
/// they are tied to a [`LanguageLoader`] through a lifetime. The loader is responsible for actually
/// locating and initializing the languages.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Lang<'loader>(Box<str>, PhantomData<&'loader ()>);

impl Debug for Lang<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Lang").field(&self.0).finish()
    }
}

impl<'loader> SupportedLanguage<'loader, LanguageLoader> for Lang<'loader> {
    fn name(&self) -> Cow<'_, str> {
        self.0.as_ref().into()
    }

    fn for_name(
        name: impl AsRef<str>,
        set: &'loader LanguageLoader,
    ) -> syntastica_core::Result<Self> {
        let name = name.as_ref();
        if set.highlight_configs.borrow().contains_key(name) {
            return Ok(Self(name.into(), PhantomData));
        }

        let (lang, lang_config) = set
            .loader
            .language_configuration_for_name(name)
            .map_err(|err| syntastica_core::Error::Custom(err.to_string()))?
            .ok_or_else(|| syntastica_core::Error::UnsupportedLanguage(name.to_string()))?;

        let config_ref = lang_config
            // TODO: allow custom query paths
            .highlight_config(lang, None)?
            .ok_or_else(|| syntastica_core::Error::UnsupportedLanguage(name.to_string()))?;

        let name = Box::<str>::from(name);
        set.highlight_configs
            .borrow_mut()
            .insert(name.clone(), config_ref);
        Ok(Self(name, PhantomData))
    }

    fn for_file_type(_file_type: FileType, _set: &'loader LanguageLoader) -> Option<Self> {
        // TODO: detection by filetype when tft allows getting a file name/extension from a
        // FileType instance
        None
    }

    fn for_injection(name: impl AsRef<str>, set: &'loader LanguageLoader) -> Option<Self> {
        let (lang, lang_config) = set
            .loader
            .language_configuration_for_injection_string(name.as_ref())
            .ok()??;
        let name = lang_config.language_name.as_str();

        if set.highlight_configs.borrow().contains_key(name) {
            return Some(Self(name.into(), PhantomData));
        }

        let config_ref = lang_config
            // TODO: allow custom query paths
            .highlight_config(lang, None)
            .ok()??;

        let name = Box::<str>::from(name);
        set.highlight_configs
            .borrow_mut()
            .insert(name.clone(), config_ref);
        Some(Self(name, PhantomData))
    }
}

/// A [`LanguageSet`] implementation that loads languages dynamically at runtime.
pub struct LanguageLoader {
    loader: Loader,
    highlight_configs: RefCell<HashMap<Box<str>, &'static HighlightConfiguration>>,
}

impl LanguageLoader {
    /// Create a new [`LanguageLoader`] which searches for parsers in `parser_search_dirs`.
    ///
    /// The directories are scanned once during creation.
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

impl<'loader> LanguageSet<'loader> for LanguageLoader {
    type Language = Lang<'loader>;

    fn get_language(
        &self,
        language: Self::Language,
    ) -> syntastica_core::Result<&HighlightConfiguration> {
        Ok(self
            .highlight_configs
            .borrow()
            .get(&language.0)
            .expect("Lang instances should initialize highlight configs"))
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
