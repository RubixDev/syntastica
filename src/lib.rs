#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

pub mod config;
mod error;
mod language_provider;
pub mod renderer;

use std::{num::ParseIntError, result};

use config::ResolvedConfig;
pub use error::*;
use language_provider::LanguageProvider;
use renderer::Renderer;
use thiserror::Error;
use tree_sitter_highlight::{Highlight, HighlightEvent, Highlighter};

pub type Highlights<'a> = Vec<Vec<(&'a str, Option<Style>)>>;

#[cfg(feature = "parsers-some")]
pub fn highlight(
    code: &str,
    file_extension: &str,
    renderer: &mut impl Renderer,
    config: config::Config,
) -> Result<String> {
    use language_provider::DefaultLanguageProvider;

    Ok(render(
        &process(
            code,
            file_extension,
            DefaultLanguageProvider,
            config.resolve_links()?,
        )?,
        renderer,
    ))
}

pub fn process<'a>(
    code: &'a str,
    file_extension: &str,
    mut language_provider: impl LanguageProvider,
    config: ResolvedConfig,
) -> Result<Highlights<'a>> {
    let config = config.into_inner();
    let mut highlight_keys = Vec::with_capacity(config.len());
    let mut highlight_styles = Vec::with_capacity(config.len());
    for (key, style) in config {
        highlight_keys.push(key);
        highlight_styles.push(style);
    }

    // TODO: do not call `prepare` in here to allow reuse from outside
    let mut configs = language_provider.prepare()?;
    for config in configs.values_mut() {
        config.configure(&highlight_keys);
    }
    let highlight_config = language_provider
        .by_extension(file_extension)
        .and_then(|key| configs.get(key.as_ref()))
        .ok_or_else(|| Error::UnsupportedFileExt(file_extension.to_owned()))?;

    let mut highlighter = Highlighter::new();

    let mut out = vec![vec![]];
    let mut style_stack = vec![];
    for event in highlighter.highlight(highlight_config, code.as_bytes(), None, |lang_name| {
        configs.get(lang_name).or_else(|| {
            language_provider
                .by_extension(lang_name)
                .and_then(|name| configs.get(name.as_ref()))
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
                        .map(|style_idx| highlight_styles[*style_idx]);
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

#[derive(Clone, Copy, Hash, Debug)]
pub struct Style {
    color: Color,
    underline: bool,
    strikethrough: bool,
    italic: bool,
    bold: bool,
}

impl Style {
    pub fn new(
        color: Color,
        underline: bool,
        strikethrough: bool,
        italic: bool,
        bold: bool,
    ) -> Self {
        Self {
            color,
            underline,
            strikethrough,
            italic,
            bold,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn underline(&self) -> bool {
        self.underline
    }

    pub fn strikethrough(&self) -> bool {
        self.strikethrough
    }

    pub fn italic(&self) -> bool {
        self.italic
    }

    pub fn bold(&self) -> bool {
        self.bold
    }
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum ParseHexError {
    #[error("length of stripped hex color must be '6' but was '{0}'")]
    InvalidLength(usize),
    #[error("encountered invalid hexadecimal digit: '{0}'")]
    InvalidDigit(#[from] ParseIntError),
}

#[derive(Clone, Copy, Hash, Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    /// Constructs a new color from the red, green, and blue components.
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    /// Tries to parse a string as a hexadecimal color value.
    /// Valid values include `cdfa20` and `#282C34`.
    pub fn from_hex(hex: impl AsRef<str>) -> result::Result<Self, ParseHexError> {
        let hex = hex.as_ref().strip_prefix('#').unwrap_or(hex.as_ref());
        if hex.len() != 6 {
            Err(ParseHexError::InvalidLength(hex.len()))?;
        }
        let red = u8::from_str_radix(&hex[0..2], 16)?;
        let green = u8::from_str_radix(&hex[2..4], 16)?;
        let blue = u8::from_str_radix(&hex[4..6], 16)?;
        Ok(Self { red, green, blue })
    }

    /// Returns the color as a 3-tuple of the red, green, and blue components,
    /// each as an integer between 0 and 255.
    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }

    /// Returns the red component of this color as an integer between 0 and 255.
    pub fn red(&self) -> u8 {
        self.red
    }

    /// Returns the green component of this color as an integer between 0 and 255.
    pub fn green(&self) -> u8 {
        self.green
    }

    /// Returns the blue component of this color as an integer between 0 and 255.
    pub fn blue(&self) -> u8 {
        self.blue
    }

    /// Returns a lowercase hexadecimal representation of this color without a leading `#`.
    pub fn as_hex(&self) -> String {
        format!(
            "{r:02x}{g:02x}{b:02x}",
            r = self.red,
            g = self.green,
            b = self.blue
        )
    }
}
