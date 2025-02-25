use syntastica_core::ts_runtime::Node;
use syntastica_core::{
    language_set::{LanguageSet, SupportedLanguage},
    theme::THEME_KEYS,
    Result,
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
/// # Instantiation
///
/// A [`Processor`] can be created by calling [`Processor::new`] with an implementation of
/// [`LanguageSet`].
///
/// # Examples
///
/// ## Example: process once
///
/// This example uses the [`Processor::process_once`] function to process one input without keeping
/// a [`Processor`] instance.
///
/// ```
/// use syntastica::{style::Style, Processor};
/// use syntastica_parsers::{Lang, LanguageSetImpl};
///
/// let highlights = Processor::process_once(
///     "fn",       // the code to process
///     Lang::Rust, // the code's language
///     // any valid `LanguageSet` supporting the required language
///     &LanguageSetImpl::new(),
/// )
/// .unwrap();
///
/// assert_eq!(highlights, vec![vec![("fn", Some("keyword.function"))]]);
/// ```
///
/// ## Example: instantiation with `Processor::new`
///
/// This example uses the [`Processor::new`] function to create a [`Processor`]
/// instance which can then be used to process multiple different inputs.
///
/// ```
/// use syntastica::{style::Style, Processor};
/// use syntastica_parsers_git::{Lang, LanguageSetImpl};
///
/// // get a `LanguageSet`
/// let language_set = LanguageSetImpl::new();
///
/// // create a `Processor` using that `LanguageSet`
/// let mut processor = Processor::new(&language_set);
///
/// // process some input
/// let highlights = processor.process("# comment", Lang::Python).unwrap();
/// assert_eq!(highlights, vec![vec![("# comment", Some("comment"))]]);
///
/// // process input with injections
/// let highlights = processor
///     .process(r#"Regex::new(r".")"#, Lang::Rust)
///     .unwrap();
/// assert_eq!(
///     highlights,
///     vec![vec![
///         ("Regex", Some("type")),
///         ("::", Some("punctuation.delimiter")),
///         ("new", Some("function.call")),
///         ("(", Some("punctuation.bracket")),
///         ("r\"", Some("string")),
///         // FIXME: this particular injection seems to not work atm
///         (".", Some("string.regexp")),
///         // (".", Some("variable.builtin")), // this is the injected regex language
///         ("\"", Some("string")),
///         (")", Some("punctuation.bracket")),
///     ]]
/// );
/// ```
pub struct Processor<'set, Set: LanguageSet<'set>> {
    set: &'set Set,
    highlighter: Highlighter,
}

impl<'set, Set: LanguageSet<'set>> Processor<'set, Set> {
    /// Create a new [`Processor`] given a [`LanguageSet`].
    ///
    /// See [the type documentation](Processor) for other means of instantiation and an example.
    pub fn new(set: &'set Set) -> Self {
        Self {
            set,
            highlighter: Highlighter::new(),
        }
    }

    /// Create a temporary [`Processor`] and run [`process`](Processor::process) once.
    ///
    /// **Only use this function if you do not plan to process multiple inputs!**
    ///
    /// See the documentation for [`process`](Processor::process) and
    /// [`new`](Processor::new) for more information on the parameters,
    /// return type, and possible errors.
    pub fn process_once<'src>(
        code: &'src str,
        language: Set::Language,
        set: &'set Set,
    ) -> Result<Highlights<'src>> {
        Self::new(set).process(code, language)
    }

    /// Process the given `code` using the language specified by `language_name`.
    ///
    /// # Returns
    ///
    /// On success, the function returns [`Highlights`] which can be used by
    /// [`render`](crate::render) for rendering to end users.
    ///
    /// # Errors
    ///
    /// The function may result in the following errors:
    ///
    /// - [`Error::UnsupportedLanguage`](crate::Error::UnsupportedLanguage) if the given
    ///   `language_name` is not supported by the [`LanguageSet`] which was passed during
    ///   instantiation of this [`Processor`].
    /// - [`Error::Highlight`](crate::Error::Highlight) if highlighting fails (mainly because of
    ///   tree-sitter version mismatches).
    pub fn process<'src>(
        &mut self,
        code: &'src str,
        language: Set::Language,
    ) -> Result<Highlights<'src>> {
        self.process_impl(code, language, None)
    }

    /// Process the given `code` using the language specified by `language_name` using an already
    /// parsed tree.
    ///
    /// Unlike [`process`](Processor::process), this does not parse the input text, but instead
    /// uses a parsed tree that is provided by the caller. This also means that **language
    /// injections must be handled by the caller**.
    ///
    /// This allows for incremental parsing, useful for e.g. text editors. See the
    /// [tree-sitter Rust documentation](https://github.com/tree-sitter/tree-sitter/tree/master/lib/binding_rust#editing)
    /// for more information.
    ///
    /// # Example
    /// ```
    /// use syntastica::{language_set::LanguageSet, renderer::TerminalRenderer, Processor};
    /// use syntastica_parsers::{Lang, LanguageSetImpl};
    /// use tree_sitter::{InputEdit, Parser, Point};
    ///
    /// // create a LanguageSet, Processor, Renderer, and ResolvedTheme
    /// let set = LanguageSetImpl::new();
    /// let mut processor = Processor::new(&set);
    /// let mut renderer = TerminalRenderer::new(None);
    /// let theme = syntastica_themes::one::dark();
    ///
    /// // create a tree-sitter parser
    /// let mut parser = Parser::new();
    /// // and set the desired language
    /// parser.set_language(&Lang::Rust.get())?;
    ///
    /// // parse, process, and render source code
    /// let code = "fn test() {}";
    /// let mut tree = parser.parse(code, None).unwrap();
    /// println!(
    ///     "{}",
    ///     syntastica::render(
    ///         &processor.process_tree(code, Lang::Rust, &tree.root_node())?,
    ///         &mut renderer,
    ///         &theme,
    ///     )
    /// );
    ///
    /// // edit the code and tree
    /// let new_code = "fn test(a: u32) {}";
    /// tree.edit(&InputEdit {
    ///     start_byte: 8,
    ///     old_end_byte: 8,
    ///     new_end_byte: 14,
    ///     start_position: Point::new(0, 8),
    ///     old_end_position: Point::new(0, 8),
    ///     new_end_position: Point::new(0, 14),
    /// });
    ///
    /// // re-parse, process, and render the code
    /// let new_tree = parser.parse(new_code, Some(&tree)).unwrap();
    /// println!(
    ///     "{}",
    ///     syntastica::render(
    ///         &processor.process_tree(new_code, Lang::Rust, &new_tree.root_node())?,
    ///         &mut renderer,
    ///         &theme,
    ///     )
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Returns
    ///
    /// On success, the function returns [`Highlights`] which can be used by
    /// [`render`](crate::render) for rendering to end users.
    ///
    /// # Errors
    ///
    /// The function may result in the following errors:
    ///
    /// - [`Error::UnsupportedLanguage`](crate::Error::UnsupportedLanguage) if the given
    ///   `language_name` is not supported by the [`LanguageSet`] which was passed during
    ///   instantiation of this [`Processor`].
    /// - [`Error::Highlight`](crate::Error::Highlight) if highlighting fails (mainly because of
    ///   tree-sitter version mismatches).
    pub fn process_tree<'src>(
        &mut self,
        code: &'src str,
        language: Set::Language,
        tree: &Node<'_>,
    ) -> Result<Highlights<'src>> {
        self.process_impl(code, language, Some(tree))
    }

    fn process_impl<'src>(
        &mut self,
        code: &'src str,
        language: Set::Language,
        tree: Option<&Node<'_>>,
    ) -> Result<Highlights<'src>> {
        let highlight_config = self.set.get_language(language)?;

        let injection_callback = |lang_name: &str| {
            let lang_name = lang_name.to_ascii_lowercase();
            // if `lang_name` is a supported language in the set, use that
            Set::Language::for_name(&lang_name, self.set)
                .ok()
                // else if `for_injection` returns a name, try getting a language for that name
                .or_else(|| Set::Language::for_injection(&lang_name, self.set))
                // else, `lang_name` might be a mimetype like `application/json`, so try both again
                // with the text after the last `/`
                .or_else(|| {
                    lang_name.rsplit_once('/').and_then(|(_, name)| {
                        Set::Language::for_name(name, self.set)
                            .ok()
                            .or_else(|| Set::Language::for_injection(name, self.set))
                    })
                })
                .and_then(|lang| self.set.get_language(lang).ok())
        };

        match tree {
            Some(tree) => process_highlight_iter(
                self.highlighter.highlight_existing_tree(
                    highlight_config,
                    code.as_bytes(),
                    None,
                    tree,
                )?,
                code,
            ),
            None => process_highlight_iter(
                self.highlighter.highlight(
                    highlight_config,
                    code.as_bytes(),
                    None,
                    injection_callback,
                )?,
                code,
            ),
        }
    }
}

fn process_highlight_iter(
    iter: impl Iterator<Item = std::result::Result<HighlightEvent, syntastica_highlight::Error>>,
    code: &str,
) -> Result<Highlights<'_>> {
    let mut out = vec![vec![]];
    let mut style_stack = vec![];

    for event in iter {
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
                        let key = THEME_KEYS[*idx];
                        match key {
                            "none" => None,
                            _ => Some(key),
                        }
                    });
                    out.last_mut()
                        .expect("`out` is initialized with one element and never shrinks in size")
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
