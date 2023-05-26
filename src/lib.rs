#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

pub mod renderer;

use std::borrow::Cow;

pub use syntastica_core::*;
pub use syntastica_highlight::Highlighter;

use providers::{ConfiguredLanguages, LanguageProvider};
use renderer::Renderer;
use style::Style;
use syntastica_highlight::{Highlight, HighlightEvent};
use theme::ResolvedTheme;

pub type Highlights<'src> = Vec<Vec<(&'src str, Option<&'static str>)>>;

pub fn highlight<T, E>(
    code: &str,
    language_name: &str,
    language_provider: &impl LanguageProvider,
    renderer: &mut impl Renderer,
    theme: T,
) -> Result<String>
where
    T: TryInto<ResolvedTheme, Error = E>,
    crate::Error: From<E>,
{
    Ok(render(
        &process_once(code, language_name, language_provider)?,
        renderer,
        theme.try_into()?,
    ))
}

pub fn process_once<'src>(
    code: &'src str,
    language_name: &str,
    language_provider: &impl LanguageProvider,
) -> Result<Highlights<'src>> {
    process(
        code,
        language_name,
        &ConfiguredLanguages::try_configure(language_provider)?,
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

pub fn render(
    highlights: &Highlights<'_>,
    renderer: &mut impl Renderer,
    theme: ResolvedTheme,
) -> String {
    let mut out = renderer.head().into_owned();
    for line in highlights {
        for (text, style) in line {
            let unstyled = renderer.unstyled(text);
            match style.and_then(|key| find_style(key, &theme)) {
                Some(style) => out += &renderer.styled(&unstyled, style),
                None => out += &unstyled,
            }
        }
        out += &renderer.newline();
    }
    out + &renderer.tail()
}

/// Try to find the best possible style supported by the them given a theme key. For example, if
/// `key` is `keyword.operator` but `theme` only has a style defined for `keyword`, then the style
/// for `keyword` is used.
fn find_style(mut key: &'static str, theme: &ResolvedTheme) -> Option<Style> {
    // if the theme contains the entire key, use that
    if let Some(style) = theme.get(key) {
        return Some(*style);
    }

    // otherwise continue to strip the right-most part of the key
    while let Some((rest, _)) = key.rsplit_once('.') {
        // until the theme contains the key
        if let Some(style) = theme.get(rest) {
            return Some(*style);
        }
        key = rest;
    }

    // or when the theme doesn't have any matching style, try to use the `text` style as a fallback
    theme.get("text").copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_finding() {
        let theme = theme! {
            "keyword": "#000000",
            "keyword.return": "#ff0000",
        }
        .resolve_links()
        .unwrap();

        assert_eq!(
            find_style("keyword.return", &theme),
            Some(Style::color_only(255, 0, 0)),
        );
        assert_eq!(
            find_style("keyword.operator", &theme),
            Some(Style::color_only(0, 0, 0)),
        );
        assert_eq!(
            find_style("keyword", &theme),
            Some(Style::color_only(0, 0, 0)),
        );
        assert_eq!(find_style("other", &theme), None);
    }

    #[test]
    fn style_fallback() {
        let theme = theme! {
            "text": "#000000",
        }
        .resolve_links()
        .unwrap();

        assert_eq!(
            find_style("other", &theme),
            Some(Style::color_only(0, 0, 0))
        );
    }
}
