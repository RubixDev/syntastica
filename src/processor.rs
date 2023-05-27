use std::borrow::Cow;

use syntastica_core::{
    providers::{ConfiguredLanguages, LanguageProvider},
    Error, Result,
};
use syntastica_highlight::{Highlight, HighlightEvent, Highlighter};

use crate::Highlights;

/// A type for easy reuse of resources when highlighting multiple inputs.
///
/// When planning to process multiple different inputs, potentially in multiple different
/// languages, create and store an instance of this type, so that the resusable resources will be
/// reused.
///
/// Additionally, the [`Processor::process_once`] function provides a quick way to process an input
/// once without keeping the reusable resources.
///
/// # Instatiation
///
/// A [`Processor`] can be created in two ways.
///
/// 1. Using [`Processor::new`] you provide an instance of [`ConfiguredLanguages`]. The retuned
///    [`Processor`] will not support language injections.
/// 2. Using [`Processor::new_with_callback`] you provide an instance of [`ConfiguredLanguages`]
///    as well as an _injection callback_ for language injections.
/// 3. Using [`Processor::try_from_provider`] you can just provide a [`LanguageProvider`] and the
///    languages and injection callback will be retrieved from it.
///
/// # Examples
///
/// ## Example: process once
///
/// This example uses the [`Processor::process_once`] function to process one input without keeping
/// a [`Processor`] instance.
///
/// ```
/// use syntastica::{Processor, style::Style};
/// use syntastica_parsers::LanguageProviderImpl;
///
/// let highlights = Processor::process_once(
///     "fn",   // the code to process
///     "rust", // the code's language
///     // any valid `LanguageProvider` supporting the required language
///     &LanguageProviderImpl::with_languages(&["rust"]),
/// ).unwrap();
///
/// assert_eq!(highlights, vec![vec![("fn", Some("keyword.function"))]]);
/// ```
///
/// ## Example: instantiation with `Processor::new`
///
/// This example uses the [`Processor::new`] function to create a [`Processor`] instance which can
/// then be used to process multiple different inputs.
///
/// ```
/// use syntastica::{Processor, style::Style};
///
/// // get an instance of `ConfiguredLanguages` somewhere
/// // let languages = ...;
/// # use syntastica::providers::{LanguageProvider, ConfiguredLanguages};
/// # let languages = ConfiguredLanguages::new(syntastica_parsers::LanguageProviderImpl::with_languages(&["rust", "python"]).get_languages().unwrap());
///
/// // create a `Processor`
/// let mut processor = Processor::new(languages);
///
/// // process some input
/// let highlights = processor.process("fn", "rust").unwrap();
/// assert_eq!(highlights, vec![vec![("fn", Some("keyword.function"))]]);
///
/// // process more input
/// let highlights = processor.process("def", "python").unwrap();
/// assert_eq!(highlights, vec![vec![("def", Some("keyword.function"))]]);
/// ```
///
/// ## Example: instantiation with `Processor::new_with_callback`
///
/// This example uses the [`Processor::new_with_callback`] function to create a [`Processor`]
/// instance which can then be used to process multiple different inputs. In addition to the
/// previous example, language injections can be handled by a custom callback. Here this callback
/// always returns `None`, but keeps track of the encountered injection names.
///
/// ```
/// use syntastica::{Processor, style::Style};
///
/// // get an instance of `ConfiguredLanguages` somewhere
/// // let languages = ...;
/// # use syntastica::providers::{LanguageProvider, ConfiguredLanguages};
/// # let languages = ConfiguredLanguages::new(syntastica_parsers::LanguageProviderImpl::with_languages(&["rust", "python"]).get_languages().unwrap());
///
/// // create a `Processor` with an injection callback
/// let mut encountered_injections = vec![];
/// let mut processor = Processor::new_with_callback(
///     languages,
///     |name| {
///         encountered_injections.push(name.to_owned());
///         None
///     },
/// );
///
/// // process some input
/// let highlights = processor.process("fn", "rust").unwrap();
/// assert_eq!(highlights, vec![vec![("fn", Some("keyword.function"))]]);
///
/// // `encountered_injections` is still empty here
///
/// // process more input
/// let highlights = processor.process("# comment", "python").unwrap();
/// assert_eq!(highlights, vec![vec![("# comment", Some("comment"))]]);
///
/// // the processor contains a mutable reference to `encountered_injections`, so it has to be
/// // dropped before that variable can be read again
/// drop(processor);
///
/// // an injection for the `comment` language was encountered in the python input
/// assert_eq!(encountered_injections, vec!["comment".to_owned()]);
/// ```
///
/// ## Example: instantiation with `Processor::try_from_provider`
///
/// This example uses the [`Processor::try_from_provider`] function to create a [`Processor`]
/// instance which can then be used to process multiple different inputs. Unlike the previous
/// example, only a [`LanguageProvider`] must be provided, which then provides the languages and
/// handles injections.
///
/// ```
/// use syntastica::{Processor, style::Style};
/// use syntastica_parsers_git::LanguageProviderImpl;
///
/// // get a `LanguageProvider`
/// let language_provider = LanguageProviderImpl::with_languages(&["rust", "python", "regex"]);
///
/// // create a `Processor` using that `LanguageProvider`
/// let mut processor = Processor::try_from_provider(&language_provider).unwrap();
///
/// // process some input
/// let highlights = processor.process("# comment", "python").unwrap();
/// assert_eq!(highlights, vec![vec![("# comment", Some("comment"))]]);
///
/// // process input with injections
/// let highlights = processor.process(r#"Regex::new(r".")"#, "rust").unwrap();
/// assert_eq!(highlights, vec![vec![
///     ("Regex", Some("type")),
///     ("::", Some("punctuation.delimiter")),
///     ("new", Some("function.call")),
///     ("(", Some("punctuation.bracket")),
///     ("r", Some("string")),
///     ("\"", Some("string")),
///     (".", Some("punctuation.special")), // this is the injected regex language
///     ("\"", Some("string")),
///     (")", Some("punctuation.bracket")),
/// ]]);
/// ```
pub struct Processor<'callback> {
    languages: ConfiguredLanguages,
    highlighter: Highlighter,
    injection_callback: InjectionCallback<'callback>,
}

type InjectionCallback<'a> = Box<dyn FnMut(&str) -> Option<Cow<'_, str>> + 'a>;

impl<'callback> Processor<'callback> {
    /// Create a new [`Processor`] given [`ConfiguredLanguages`].
    ///
    /// See [the type documentation](Processor) for other means of instantiation and an example.
    ///
    /// The returned [`Processor`] will not support language injections, unless a callback is set
    /// with [`set_injection_callback`](Processor::set_injection_callback).
    /// Use [`new_with_callback`](Processor::new_with_callback) to directly specify a callback
    /// during creation.
    pub fn new(languages: ConfiguredLanguages) -> Self {
        Self::new_with_callback(languages, |_| None)
    }

    /// Create a new [`Processor`] given [`ConfiguredLanguages`] an _injection callback_.
    ///
    /// See [the type documentation](Processor) for other means of instantiation and an example.
    ///
    /// The injection callback will be called whenever another language may be used to
    /// process a part of the input (e.g. markdown fenced code blocks or JavaScript/CSS in HTML).
    /// It is called with the name of the language to inject, and it should return either
    /// `Some(<name>)` or `None`, based on whether the language is supported by the provided
    /// [`ConfiguredLanguages`]. When returning `Some(<name>)`, the `<name>` indicates the key to
    /// look up in the provided [`ConfiguredLanguages`].
    ///
    /// > Note: The callback will only be called if no language with the given name exists in the
    /// > provided [`ConfiguredLanguages`] directly.
    pub fn new_with_callback(
        languages: ConfiguredLanguages,
        injection_callback: impl FnMut(&str) -> Option<Cow<'_, str>> + 'callback,
    ) -> Self {
        Self {
            languages,
            highlighter: Highlighter::new(),
            injection_callback: Box::new(injection_callback),
        }
    }

    /// Create a new [`Processor`] given a [`LanguageProvider`].
    ///
    /// See [the type documentation](Processor) for other means of instantiation and an example.
    ///
    /// This function calls [`LanguageProvider::get_languages`] to get the supported languages, and
    /// uses [`LanguageProvider::for_injection`] for the injection callback. See
    /// [`new_with_callback`](Processor::new_with_callback) for more information.
    ///
    /// # Errors
    /// A call to this function will result in an [`Error`] if the call to
    /// [`LanguageProvider::get_languages`] fails.
    pub fn try_from_provider(language_provider: &'callback impl LanguageProvider) -> Result<Self> {
        Ok(Self {
            languages: ConfiguredLanguages::new(language_provider.get_languages()?),
            highlighter: Highlighter::new(),
            injection_callback: Box::new(|lang_name| language_provider.for_injection(lang_name)),
        })
    }

    /// Change the _injection callback_ of this [`Processor`].
    ///
    /// This overwrites the previous _injection callback_ to the new one for all following calls to
    /// [`process`](Processor::process). This should not be used to set a callback directly after a
    /// [`Processor`] was created with [`Processor::new`]. Instead, use
    /// [`new_with_callback`](Processor::new_with_callback) to directly create a [`Processor`] with
    /// the desired callback.
    pub fn set_injection_callback(
        &mut self,
        injection_callback: impl FnMut(&str) -> Option<Cow<'_, str>> + 'callback,
    ) {
        self.injection_callback = Box::new(injection_callback);
    }

    /// Create a temporary [`Processor`] and run [`process`](Processor::process) once.
    ///
    /// **Only use this function if you do not plan to process multiple inputs!**
    ///
    /// See the documentation for [`process`](Processor::process) and
    /// [`try_from_provider`](Processor::try_from_provider) for more information on the parameters
    /// and return type.
    pub fn process_once<'src>(
        code: &'src str,
        language_name: &str,
        language_provider: &'callback impl LanguageProvider,
    ) -> Result<Highlights<'src>> {
        Self::try_from_provider(language_provider)?.process(code, language_name)
    }

    /// Process the given `code` using the language specified by `language_name`.
    ///
    /// # Returns
    /// On success, the function returns [`Highlights`] which can be used by
    /// [`render`](crate::render) for rendering to end users.
    ///
    /// # Errors
    /// The function may return an [`Error::UnsupportedLanguage`] or [`Error::Highlight`]. The
    /// former is returned when the given `language_name` is not supported by the
    /// [`ConfiguredLanguages`] which were passed during instantiation of this [`Processor`]. The
    /// latter contains a [`syntastica_highlight::Error`], and is returned when highlighting fails
    /// (mainly because of tree-sitter version mismatches).
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
