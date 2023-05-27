//! Defines the [`Style`] and [`Color`] types used by
//! [`ResolvedTheme`](crate::theme::ResolvedTheme)s.

use std::{num::ParseIntError, result};

use thiserror::Error;

/// Defines how to style a region of text.
///
/// Besides a [`Color`], the four following booleans can be set:
///
/// - `underline`
/// - `strikethrough`
/// - `italic`
/// - `bold`
///
/// # Instantiation
///
/// A [`Style`] can mainly be created in three ways:
///
/// - Using [`Style::new`] to specify the color and all four boolean flags explicitly.
/// - Using [`Style::color_only`] to specify a color's red, green, and blue values and keep all
///   booleans disabled.
/// - Using the [`From<Color>`](#impl-From<Color>-for-Style) implementation to create a [`Style`]
///   with the given [`Color`] and all booleans disabled.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct Style {
    color: Color,
    underline: bool,
    strikethrough: bool,
    italic: bool,
    bold: bool,
}

impl Style {
    /// Create a new [`Style`].
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

    /// Create a new [`Style`] with only color information attached.
    ///
    /// The function does not accept a [`Color`] instance, but instead takes the red, green, and
    /// blue values separately to create a new color. To create a color-only [`Style`] from an
    /// existing [`Color`] instance, use the [`From<Color>`](#impl-From<Color>-for-Style)
    /// implementation.
    pub fn color_only(red: u8, green: u8, blue: u8) -> Self {
        Self::new(Color::new(red, green, blue), false, false, false, false)
    }

    /// Get this [`Style`]'s [`Color`].
    pub fn color(&self) -> Color {
        self.color
    }

    /// Whether this [`Style`] is underlined.
    pub fn underline(&self) -> bool {
        self.underline
    }

    /// Whether this [`Style`] is strikethrough.
    pub fn strikethrough(&self) -> bool {
        self.strikethrough
    }

    /// Whether this [`Style`] is italic.
    pub fn italic(&self) -> bool {
        self.italic
    }

    /// Whether this [`Style`] is bold.
    pub fn bold(&self) -> bool {
        self.bold
    }
}

impl From<Color> for Style {
    fn from(color: Color) -> Self {
        Self::new(color, false, false, false, false)
    }
}

/// An error that can occur when parsing a [`Color`] from a hexadecimal string literal.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum ParseHexError {
    /// The length of the input (excluding a possibly leading `#`) did not equal 6.
    ///
    /// Contains the length the input was found to have.
    #[error("length of stripped hex color must be '6' but was '{0}'")]
    InvalidLength(usize),

    /// The input contained an invalid hexadecimal digit.
    ///
    /// Contains a [`ParseIntError`].
    #[error("encountered invalid hexadecimal digit: '{0}'")]
    InvalidDigit(#[from] ParseIntError),
}

/// A non-transparent color with red, green, and blue values between 0 and 255.
///
/// # Instantiation
///
/// A [`Color`] can be created in two different ways:
///
/// - Using [`Color::new`] to specify the red, green, and blue values explicitly.
/// - Using [`Color::from_hex`] to parse a color from a hexadecimal string.
// TODO: look for existing color crate
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
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
    ///
    /// The input string may include a leading `#`, and the length without the `#` must be 6.
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
