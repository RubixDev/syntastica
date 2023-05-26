#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

mod processor;
pub mod renderer;

pub use processor::Processor;
pub use syntastica_core::*;

use providers::LanguageProvider;
use renderer::Renderer;
use style::Style;
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
        &Processor::process_once(code, language_name, language_provider)?,
        renderer,
        theme.try_into()?,
    ))
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
