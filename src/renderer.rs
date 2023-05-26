//! This module contains the [`Renderer`] trait and two implementors: [`TerminalRenderer`] and
//! [`HtmlRenderer`], as well as the [`render`] function, which is re-exported at the crate root.
//! See the individual documentation of these items for more information and examples.

use std::borrow::Cow;

use aho_corasick::AhoCorasick;
use syntastica_core::theme::ResolvedTheme;

use crate::{
    style::{Color, Style},
    Highlights,
};

/// A [`Renderer`] defines how to render [`Highlights`] to end users. The methods are invoked by
/// [`render`].
///
/// Syntastica comes with two implementors of this trait, namely
/// - [`TerminalRenderer`] for output to ANSI compatible terminals, and
/// - [`HtmlRenderer`] for output to websites.
///
/// If the purpose of a trait function is not clear, it may be worthwhile to look at the
/// implementation of [`TerminalRenderer`] and [`HtmlRenderer`].
pub trait Renderer {
    /// Is called at the start of rendering, before anything else. The returned string will be the
    /// start of the final output.
    ///
    /// The default implementation returns an empty string.
    fn head(&mut self) -> Cow<'static, str> {
        "".into()
    }

    /// Is called at the end of rendering, after the final call to [`Renderer::newline`].
    /// The returned string will be the end of the final output.
    ///
    /// The default implementation returns an empty string.
    fn tail(&mut self) -> Cow<'static, str> {
        "".into()
    }

    /// Is called at the end of every input line. The returned string will be used for line breaks
    /// in the output.
    ///
    /// The default implementation returns `"\n"`.
    fn newline(&mut self) -> Cow<'static, str> {
        "\n".into()
    }

    /// Is called for every region of the input (including styled ones). May be implemented to
    /// escape special characters.
    ///
    /// The default implementation returns `text` unchanged.
    fn unstyled<'a>(&mut self, text: &'a str) -> Cow<'a, str> {
        text.into()
    }

    /// Is called for every styled region of the input. The input was already passed to
    /// [`Renderer::unstyled`] before. Implementors should return a string representing the input
    /// `text` in the given [`Style`].
    fn styled<'a>(&mut self, text: &'a str, style: Style) -> Cow<'a, str>;
}

/// Render the given [`Highlights`] with a [`Renderer`] and [`ResolvedTheme`].
///
/// See the linked items for more information, and look at [`TerminalRenderer`] or
/// [`HtmlRenderer`] for examples.
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

////////////////////////////////////////

/// A [`Renderer`] implementor for HTML output. Use this type with the [`render`] function.
///
/// The resulting HTML is not wrapped in any parent tag and defines no font by itself. Each styled
/// text region uses a `span` tag and defines its styling inline using the `style` attribute.
///
/// ## Example
///
/// ```
/// use syntastica::{renderer::HtmlRenderer, theme};
///
/// // define a theme
/// let theme = theme! { "keyword": "#ff0000" }.resolve_links().unwrap();
///
/// // define highlights
/// let highlights = vec![vec![("<fn>", Some("keyword")), ("none", None)]];
///
/// // render to HTML
/// let output = syntastica::render(&highlights, &mut HtmlRenderer, theme);
///
/// assert_eq!(output, r#"<span style="color: rgb(255, 0, 0);">&lt;fn&gt;</span>none<br>"#);
/// ```
pub struct HtmlRenderer;

impl HtmlRenderer {
    /// Create a new [`HtmlRenderer`]. Because [`HtmlRenderer`] is a unit struct, this is the same
    /// as just writing `HtmlRenderer`, but this function is provided for clarification
    /// nonetheless.
    pub fn new() -> Self {
        Self
    }
}

impl Default for HtmlRenderer {
    fn default() -> Self {
        Self
    }
}

impl Renderer for HtmlRenderer {
    fn newline(&mut self) -> Cow<'static, str> {
        "<br>".into()
    }

    fn unstyled(&mut self, text: &str) -> Cow<'static, str> {
        AhoCorasick::new(["&", "<", ">", " ", "\n", "\t"])
            .unwrap()
            .replace_all(
                text,
                &[
                    "&amp;",
                    "&lt;",
                    "&gt;",
                    "&nbsp;<wbr>",
                    "<br>",
                    "&nbsp;&nbsp;&nbsp;&nbsp;<wbr>",
                ],
            )
            .into()
    }

    fn styled(&mut self, text: &str, style: Style) -> Cow<'static, str> {
        let (r, g, b) = style.color().rgb();
        let mut css = format!("color: rgb({r}, {g}, {b});");
        if style.underline() && style.strikethrough() {
            css += "text-decoration: underline line-through;"
        } else if style.underline() {
            css += "text-decoration: underline;"
        } else if style.strikethrough() {
            css += "text-decoration: line-through;"
        }
        if style.bold() {
            css += "font-weight: bold;"
        }
        if style.italic() {
            css += "font-style: italic;"
        }
        format!("<span style=\"{css}\">{text}</span>").into()
    }
}

/// A [`Renderer`] implementor for true-color ANSI terminal output. Use this type with the
/// [`render`] function.
///
/// A background color can optionally be set which will behind the output until the exact end of
/// each line.
///
/// ## Example
///
/// ```
/// use syntastica::{renderer::TerminalRenderer, theme};
///
/// // define a theme
/// let theme = theme! { "keyword": "#ff0000" }.resolve_links().unwrap();
///
/// // define highlights
/// let highlights = vec![vec![("fn", Some("keyword")), (" none", None)]];
///
/// // render to a string without a background color
/// let output = syntastica::render(&highlights, &mut TerminalRenderer::new(None), theme);
///
/// assert_eq!(output, "\x1b[38;2;255;0;0mfn\x1b[0m none\n");
/// ```
pub struct TerminalRenderer {
    background_color: Option<Color>,
}

impl TerminalRenderer {
    /// Create a new [`TerminalRenderer`] with an optional background color.
    pub fn new(background_color: Option<Color>) -> Self {
        Self { background_color }
    }
}

impl Renderer for TerminalRenderer {
    fn head(&mut self) -> Cow<'static, str> {
        match self.background_color {
            Some(color) => {
                let (r, g, b) = color.rgb();
                format!("\x1b[48;2;{r};{g};{b}m").into()
            }
            None => "".into(),
        }
    }

    fn tail(&mut self) -> Cow<'static, str> {
        match self.background_color {
            Some(_) => "\x1b[0m".into(),
            None => "".into(),
        }
    }

    fn newline(&mut self) -> Cow<'static, str> {
        match self.background_color {
            Some(color) => {
                let (r, g, b) = color.rgb();
                format!("\x1b[0m\n\x1b[48;2;{r};{g};{b}m").into()
            }
            None => "\n".into(),
        }
    }

    fn styled(&mut self, text: &str, style: Style) -> Cow<'static, str> {
        let (r, g, b) = style.color().rgb();
        let mut params = format!("38;2;{r};{g};{b};");
        if style.underline() {
            params += "4;"
        }
        if style.strikethrough() {
            params += "9;"
        }
        if style.italic() {
            params += "3;"
        }
        if style.bold() {
            params += "1;"
        }
        // trim last `;`
        params.truncate(params.len() - 1);
        let reset = match self.background_color.is_some() {
            true => "39;24;29;23;22",
            false => "0",
        };
        format!("\x1b[{params}m{text}\x1b[{reset}m").into()
    }
}

#[cfg(test)]
mod tests {
    use syntastica_core::theme;

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
