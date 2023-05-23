#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

pub mod config;
mod error;
pub mod providers;
pub mod renderer;
pub mod style;

use config::ResolvedConfig;
pub use error::*;
pub use tree_sitter_highlight::Highlighter;

use providers::{ConfiguredLanguages, LanguageProvider, ParserProvider};
use renderer::Renderer;
use style::Style;
use thiserror::Error;
use tree_sitter_highlight::{Highlight, HighlightEvent};

pub type Highlights<'src> = Vec<Vec<(&'src str, Option<Style>)>>;

pub fn highlight<C, E>(
    code: &str,
    // TODO: use "language_name"
    file_extension: &str,
    language_provider: &impl LanguageProvider,
    renderer: &mut impl Renderer,
    config: C,
) -> Result<String>
where
    C: TryInto<ResolvedConfig, Error = E>,
    crate::Error: From<E>,
{
    Ok(render(
        &process_once(code, file_extension, language_provider, config)?,
        renderer,
    ))
}

pub fn process_once<'src, C, E>(
    code: &'src str,
    // TODO: use "language_name"
    file_extension: &str,
    language_provider: &impl LanguageProvider,
    config: C,
) -> Result<Highlights<'src>>
where
    C: TryInto<ResolvedConfig, Error = E>,
    crate::Error: From<E>,
{
    process(
        code,
        file_extension,
        &ConfiguredLanguages::try_configure(language_provider, config)?,
        language_provider,
        &mut Highlighter::new(),
    )
}

pub fn process<'src>(
    code: &'src str,
    // TODO: use "language_name"
    file_extension: &str,
    languages: &ConfiguredLanguages,
    provider: &impl ParserProvider,
    highlighter: &mut Highlighter,
) -> Result<Highlights<'src>> {
    let highlight_config = provider
        .for_extension(file_extension)
        .and_then(|key| languages.get(key.as_ref()))
        .ok_or_else(|| Error::UnsupportedFileExt(file_extension.to_owned()))?;

    let mut out = vec![vec![]];
    let mut style_stack = vec![];
    for event in highlighter.highlight(highlight_config, code.as_bytes(), None, |lang_name| {
        languages.get(lang_name).or_else(|| {
            provider
                .for_injection(lang_name)
                .and_then(|name| languages.get(name.as_ref()))
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
                    let style = style_stack
                        .last()
                        .map(|style_idx| languages.highlight_styles()[*style_idx]);
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
