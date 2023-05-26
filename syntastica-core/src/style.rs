use std::{num::ParseIntError, result};

use thiserror::Error;

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
