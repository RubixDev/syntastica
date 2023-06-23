#![doc = include_str!("../README.md")]
//!
#![cfg_attr(
    feature = "docs",
    cfg_attr(doc, doc = ::document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#))
)]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

use std::{borrow::Cow, cell::UnsafeCell};

use syntastica_core::{
    language_set::{HighlightConfiguration, LanguageSet},
    Error, Result,
};

#[cfg(not(feature = "some"))]
compile_error!("current feature set includes no parsers");

syntastica_macros::parsers_ffi!();

// TODO: share the following code between the parser collection crates

/// An implementation of [`LanguageSet`]
/// including all languages in the enabled feature set.
///
/// Languages are loaded the first time they are requested and will then be reused for
/// later accesses. To pre-load a list of languages, use
/// [`preload`](LanguageSetImpl::preload) or [`preload_all`](LanguageSetImpl::preload_all).
pub struct LanguageSetImpl(UnsafeCell<[Option<HighlightConfiguration>; LANG_COUNT]>);

impl LanguageSet for LanguageSetImpl {
    fn get_language(&self, name: &str) -> Result<&HighlightConfiguration> {
        if let Some(idx) = IDX_MAP.get(&name) {
            // SAFETY: We only ever give out shared references to list entries, and only
            // after they have been initialized. As such it is safe to mutate an entry
            // which is still `None` and then give out a shared reference.
            let list = unsafe { self.0.get().as_ref() }.unwrap();
            match list[*idx].as_ref() {
                Some(config) => Ok(config),
                None => {
                    // SAFETY: see above
                    let list = unsafe { self.0.get().as_mut() }.unwrap();
                    let conf = __get_language(*idx)?;
                    list[*idx] = Some(conf);
                    Ok(list[*idx].as_ref().unwrap())
                }
            }
        } else {
            Err(Error::UnsupportedLanguage(name.to_owned()))
        }
    }

    fn for_extension<'a>(&self, file_extension: &'a str) -> Option<Cow<'a, str>> {
        EXTENSION_MAP
            .get(&file_extension)
            .map(|name| (*name).into())
    }

    // TODO: injection regex
    // fn for_injection<'a>(&self, name: &'a str) -> ::std::option::Option<::std::borrow::Cow<'a, str>> {
    //     ::std::option::Option::None
    // }
}

const INIT: Option<HighlightConfiguration> = None;
impl LanguageSetImpl {
    /// Create a new [`LanguageSetImpl`] with no pre-loaded languages.
    pub fn new() -> Self {
        Self(UnsafeCell::new([INIT; LANG_COUNT]))
    }

    /// Pre-load the given list of languages.
    ///
    /// To pre-load all supported languages, use [`preload_all`](LanguageSetImpl::preload_all).
    ///
    /// # Errors
    /// If the `languages` list contains a name of a language that is not included in this set, an
    /// [`Error::UnsupportedLanguage`] error is returned and no further languages are loaded.
    pub fn preload(&mut self, languages: &[&str]) -> Result<()> {
        for lang in languages {
            match IDX_MAP.get(lang) {
                Some(idx) => {
                    let entry = &mut self.0.get_mut()[*idx];
                    if entry.is_none() {
                        *entry = Some(__get_language(*idx)?);
                    }
                }
                None => return Err(Error::UnsupportedLanguage(lang.to_string())),
            }
        }
        Ok(())
    }

    /// Pre-load all languages in this set.
    ///
    /// To pre-load a specific set of languages, use [`preload`](LanguageSetImpl::preload).
    pub fn preload_all(&mut self) {
        self.preload(LANGUAGES)
            .expect("constant `LANGUAGES` list should only contain valid names")
    }
}

impl Default for LanguageSetImpl {
    fn default() -> Self {
        Self::new()
    }
}
