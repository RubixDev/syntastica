//! Defines the [`Style`] and [`Color`] types used by
//! [`ResolvedTheme`](crate::theme::ResolvedTheme)s.

use std::hash::{Hash, Hasher};

use palette::Srgb;

/// A non-transparent color with red, green, and blue values between 0 and 255.
///
/// The type is an alias to [`palette::Srgb<u8>`], so there are many ways of obtaining instances of
/// this type. For example, you could add a dependency an `palette` with the
/// <span class="stab portability"><code>named</code></span> feature enabled, to access a list of
/// predefined colors in the `palette::named` module.
pub type Color = Srgb<u8>;

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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

// palette's `Srgb` type does not implement `Hash`, but because we use `u8` as the generic type
// argument, we can implement it on `Style` manually
impl Hash for Style {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.color.red, state);
        Hash::hash(&self.color.green, state);
        Hash::hash(&self.color.blue, state);
        Hash::hash(&self.color.standard, state);

        Hash::hash(&self.underline, state);
        Hash::hash(&self.strikethrough, state);
        Hash::hash(&self.italic, state);
        Hash::hash(&self.bold, state);
    }
}
