use std::borrow::Cow;

use syntastica_core::{
    providers::{ConfiguredLanguages, LanguageProvider},
    Error, Result,
};
use syntastica_highlight::{Highlight, HighlightEvent, Highlighter};

use crate::Highlights;

pub struct Processor<'callback> {
    languages: ConfiguredLanguages,
    highlighter: Highlighter,
    injection_callback: InjectionCallback<'callback>,
}

type InjectionCallback<'a> = Box<dyn Fn(&str) -> Option<Cow<'_, str>> + 'a>;

impl<'callback> Processor<'callback> {
    pub fn new(
        languages: ConfiguredLanguages,
        injection_callback: impl Fn(&str) -> Option<Cow<'_, str>> + 'callback,
    ) -> Self {
        Self {
            languages,
            highlighter: Highlighter::new(),
            injection_callback: Box::new(injection_callback),
        }
    }

    pub fn try_new(language_provider: &'callback impl LanguageProvider) -> Result<Self> {
        Ok(Self {
            languages: ConfiguredLanguages::try_configure(language_provider)?,
            highlighter: Highlighter::new(),
            injection_callback: Box::new(|lang_name| language_provider.for_injection(lang_name)),
        })
    }

    pub fn with_injection_callback(
        &mut self,
        injection_callback: impl Fn(&str) -> Option<Cow<'_, str>> + 'callback,
    ) -> &mut Self {
        self.injection_callback = Box::new(injection_callback);
        self
    }

    pub fn process_once<'src>(
        code: &'src str,
        language_name: &str,
        language_provider: &'callback impl LanguageProvider,
    ) -> Result<Highlights<'src>> {
        Self::try_new(language_provider)?.process(code, language_name)
    }

    pub fn process<'src>(
        &mut self,
        code: &'src str,
        language_name: &str,
    ) -> Result<Highlights<'src>> {
        let highlight_config = self
            .languages
            .get(language_name)
            .ok_or_else(|| Error::UnsupportedLanguage(language_name.to_owned()))?;

        let mut out = vec![vec![]];
        let mut style_stack = vec![];
        for event in
            self.highlighter
                .highlight(highlight_config, code.as_bytes(), None, |lang_name| {
                    // if `lang_name` matches a language/parser name in `languages`, use that language
                    self.languages
                        .get(lang_name)
                        // else if `injection_callback` returns a name, try getting a language for that name
                        .or_else(|| {
                            (self.injection_callback)(lang_name)
                                .and_then(|name| self.languages.get(name.as_ref()))
                        })
                        // else, `lang_name` might be a mimetype like `text/css`, so try both again with the
                        // text after the last `/`
                        .or_else(|| {
                            lang_name.rsplit_once('/').and_then(|(_, name)| {
                                self.languages.get(name).or_else(|| {
                                    (self.injection_callback)(name)
                                        .and_then(|name| self.languages.get(name.as_ref()))
                                })
                            })
                        })
                })?
        {
            match event? {
                HighlightEvent::HighlightStart(Highlight(highlight)) => style_stack.push(highlight),
                HighlightEvent::HighlightEnd => {
                    style_stack.pop();
                }
                HighlightEvent::Source { start, end } => {
                    let ends_with_newline = code[start..end].ends_with('\n');
                    let mut lines = code[start..end].lines().peekable();
                    while let Some(line) = lines.next() {
                        let style = style_stack.last().and_then(|idx| {
                            let key = crate::THEME_KEYS[*idx];
                            match key {
                                "none" => None,
                                _ => Some(key),
                            }
                        });
                        out.last_mut()
                            .expect(
                                "`out` is initialized with one element and never shrinks in size",
                            )
                            .push((line, style));

                        if lines.peek().is_some() || ends_with_newline {
                            out.push(vec![]);
                        }
                    }
                }
            }
        }

        Ok(out)
    }
}
