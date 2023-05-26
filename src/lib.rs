#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

pub mod renderer;

use std::borrow::Cow;

pub use syntastica_core::*;
pub use syntastica_highlight::Highlighter;

use config::ResolvedConfig;
use providers::{ConfiguredLanguages, LanguageProvider};
use renderer::Renderer;
use style::Style;
use syntastica_highlight::{Highlight, HighlightEvent};

pub type Highlights<'src> = Vec<Vec<(&'src str, Option<Style>)>>;

pub fn highlight<C, E>(
    code: &str,
    language_name: &str,
    language_provider: &impl LanguageProvider,
    renderer: &mut impl Renderer,
    config: C,
) -> Result<String>
where
    C: TryInto<ResolvedConfig, Error = E>,
    crate::Error: From<E>,
{
    Ok(render(
        &process_once(code, language_name, language_provider, config)?,
        renderer,
    ))
}

pub fn process_once<'src, C, E>(
    code: &'src str,
    language_name: &str,
    language_provider: &impl LanguageProvider,
    config: C,
) -> Result<Highlights<'src>>
where
    C: TryInto<ResolvedConfig, Error = E>,
    crate::Error: From<E>,
{
    process(
        code,
        language_name,
        &ConfiguredLanguages::try_configure(language_provider, config)?,
        |lang_name| language_provider.for_injection(lang_name),
        &mut Highlighter::new(),
    )
}

pub fn process<'src>(
    code: &'src str,
    language_name: &str,
    languages: &ConfiguredLanguages,
    injection_callback: impl Fn(&str) -> Option<Cow<'_, str>>,
    highlighter: &mut Highlighter,
) -> Result<Highlights<'src>> {
    let highlight_config = languages
        .get(language_name)
        .ok_or_else(|| Error::UnsupportedLanguage(language_name.to_owned()))?;

    let mut out = vec![vec![]];
    let mut style_stack = vec![];
    for event in highlighter.highlight(highlight_config, code.as_bytes(), None, |lang_name| {
        // if `lang_name` matches a language/parser name in `languages`, use that language
        languages
            .get(lang_name)
            // else if `injection_callback` returns a name, try getting a language for that name
            .or_else(|| injection_callback(lang_name).and_then(|name| languages.get(name.as_ref())))
            // else, `lang_name` might be a mimetype like `text/css`, so try both again with the
            // text after the last `/`
            .or_else(|| {
                lang_name.rsplit_once('/').and_then(|(_, name)| {
                    languages.get(name).or_else(|| {
                        injection_callback(name).and_then(|name| languages.get(name.as_ref()))
                    })
                })
            })
    })? {
        match event? {
            HighlightEvent::HighlightStart(Highlight(highlight)) => style_stack.push(highlight),
            HighlightEvent::HighlightEnd => {
                style_stack.pop();
            }
            HighlightEvent::Source { start, end } => {
                let ends_with_newline = code[start..end].ends_with('\n');
                let mut lines = code[start..end].lines().peekable();
                while let Some(line) = lines.next() {
                    let style = find_style(&style_stack, languages);
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

fn find_style(stack: &[usize], langs: &ConfiguredLanguages) -> Option<Style> {
    for index in stack.iter().rev() {
        let capture_name = langs.highlight_keys().get(*index).map(|name| name.as_str());

        // keep `@none` captures unstyled
        if capture_name == Some("none") {
            return None;
        }

        // try to get style for capture
        if let Some(style) = langs.highlight_styles().get(*index) {
            return Some(*style);
        }
    }

    langs.default_style()
}

pub fn render(highlights: &Highlights<'_>, renderer: &mut impl Renderer) -> String {
    let mut out = renderer.head().into_owned();
    for line in highlights {
        for (text, style) in line {
            let unstyled = renderer.unstyled(text);
            match style {
                Some(style) => out += &renderer.styled(&unstyled, *style),
                None => out += &unstyled,
            }
        }
        out += &renderer.newline();
    }
    out + &renderer.tail()
}
