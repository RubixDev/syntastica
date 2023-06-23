//! Defines items related to theming the output.

use std::{borrow::Borrow, collections::BTreeMap, ops::Index, str::FromStr};

use crate::{
    style::{Color, Style},
    Error, Result,
};

/// All theme keys that are recognized by syntastica.
///
/// A [`Theme`] or [`ResolvedTheme`] should define styles for any subset of these.
///
/// <details>
/// <summary>View the full list</summary>
///
/// ```ignore
#[doc = include_str!("./theme_keys.rs")]
/// ```
///
/// </details>
///
/// The list is based on the list from
/// [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter/blob/master/CONTRIBUTING.md).
pub const THEME_KEYS: &[&str] = &include!("./theme_keys.rs");

/// A raw theme which may contain links to other items inside.
///
/// Internally, this type stores a map from [`String`]s to [`ThemeValue`]s. This map can be
/// retrieved using [`Theme::into_inner`]. The map keys do _not_ all have to be in [`THEME_KEYS`];
/// other custom keys can be used, for example to define a set of colors and reuse them with links
/// everywhere else.
///
/// When using the <span class="stab portability"><code>serde</code></span> feature, this type
/// implements serde's `Serialize` and `Deserialize` traits.
///
/// # Instantiation
///
/// The easiest way to create a [`Theme`] is with the [`theme!`](crate::theme!) macro.
/// Alternatively, a [`Theme`] may be created from a [`BTreeMap<String, ThemeValue>`] using
/// [`Theme::new`].
#[derive(Clone, Hash, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Theme(BTreeMap<String, ThemeValue>);

/// A [`Theme`] where all internal links have been resolved.
///
/// Instead of [`ThemeValue`]s, a [`ResolvedTheme`] has [`Style`]s as values. These cannot link to
/// other entries of the theme but completely define a style on their own.
///
/// A [`ResolvedTheme`] can be created from a [`Theme`] with [`Theme::resolve_links`] or the
/// [`TryFrom<Theme>`](#impl-TryFrom<Theme>-for-ResolvedTheme) implementation.
///
/// To get the style for a key in this theme, the preferred method is using
/// [`ResolvedTheme::find_style`], which will return the best match it finds. See the method's docs
/// for more information. Alternatively, [`ResolvedTheme::get`] can be used to only look for an
/// exact match.
#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub struct ResolvedTheme(BTreeMap<String, Style>);

/// A value of a [`Theme`] containing style information and/or a link to another key in the
/// [`Theme`].
///
/// When using the <span class="stab portability"><code>serde</code></span> feature, this type
/// implements serde's `Serialize` and `Deserialize` traits using the untagged enum representation.
#[derive(Clone, Hash, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ThemeValue {
    /// May either be a hexadecimal color literal, or a `$` followed by the name of another
    /// theme key.
    ///
    /// In the latter case, this value links to the [`ThemeValue`] which the [`Theme`] specifies
    /// for the provided theme key.
    Simple(String),
    /// A color or link with additional style information.
    Extended {
        /// The color to use for this style, specified as a hexadecimal string.
        ///
        /// Either this or [`link`](ThemeValue::Extended::link) has to be set, or calls to
        /// [`Theme::resolve_links`] will fail.
        color: Option<String>,

        /// Whether the text should be underlined. (default is `false`)
        #[cfg_attr(feature = "serde", serde(default))]
        underline: bool,

        /// Whether the text should be strikethrough. (default is `false`)
        #[cfg_attr(feature = "serde", serde(default))]
        strikethrough: bool,

        /// Whether the text should be italic. (default is `false`)
        #[cfg_attr(feature = "serde", serde(default))]
        italic: bool,

        /// Whether the text should be bold. (default is `false`)
        #[cfg_attr(feature = "serde", serde(default))]
        bold: bool,

        /// A link to the theme entry with the given key.
        ///
        /// Either this or [`color`](ThemeValue::Extended::color) has to be set, or calls to
        /// [`Theme::resolve_links`] will fail.
        link: Option<String>,
    },
}

impl Theme {
    /// Create a new [`Theme`] from a map of [theme keys](THEME_KEYS) to [`ThemeValue`]s.
    pub fn new(highlights: BTreeMap<String, ThemeValue>) -> Self {
        Self(highlights)
    }

    /// Consume `self` and return the contained theme map.
    ///
    /// May be used to merge multiple [`Theme`]s together.
    pub fn into_inner(self) -> BTreeMap<String, ThemeValue> {
        self.0
    }

    /// Try to resolve all links in this [`Theme`] and return a [`ResolvedTheme`].
    ///
    /// # Errors
    ///
    /// The function may return the following errors:
    ///
    /// - [`Error::InvalidHex`] if a color string was an invalid hexadecimal literal.
    /// - [`Error::InvalidLink`] if a link points to a non-existent key.
    pub fn resolve_links(mut self) -> Result<ResolvedTheme> {
        self.resolve_links_impl()?;
        Ok(ResolvedTheme::new(
            self.0
                .into_iter()
                .map(|(key, value)| {
                    Ok((
                        key,
                        match value {
                            ThemeValue::Simple(color) => {
                                Style::new(Color::from_str(&color)?, false, false, false, false)
                            }
                            ThemeValue::Extended {
                                color,
                                underline,
                                strikethrough,
                                italic,
                                bold,
                                link: _,
                            } => Style::new(
                                // TODO: maybe rework to not rely on unwrapping
                                Color::from_str(&color.expect("links have been resolved"))?,
                                underline,
                                strikethrough,
                                italic,
                                bold,
                            ),
                        },
                    ))
                })
                .collect::<Result<_>>()?,
        ))
    }

    fn resolve_links_impl(&mut self) -> Result<()> {
        let mut must_reresolve = false;
        let mut replacements = vec![];
        for (key, value) in self.0.iter() {
            let link_key = match value {
                ThemeValue::Simple(str) if str.starts_with('$') => &str[1..],
                ThemeValue::Extended {
                    link: Some(str), ..
                } => str,
                _ => continue,
            };
            let resolved = value.resolve_link(
                self.0
                    .get(link_key)
                    .ok_or_else(|| Error::InvalidLink(link_key.to_owned()))?,
            );
            if matches!(&resolved, ThemeValue::Simple(str) if str.starts_with('$'))
                || matches!(&resolved, ThemeValue::Extended { link: Some(_), .. })
            {
                must_reresolve = true;
            }
            replacements.push((key.clone(), resolved));
        }
        for (key, replacement) in replacements {
            *self.0.get_mut(&key).expect("key validity checked above") = replacement;
        }
        if must_reresolve {
            self.resolve_links_impl()?;
        }
        Ok(())
    }
}

impl From<BTreeMap<String, ThemeValue>> for Theme {
    fn from(highlights: BTreeMap<String, ThemeValue>) -> Self {
        Self::new(highlights)
    }
}

impl ResolvedTheme {
    /// Create a new [`ResolvedTheme`] from a map of [theme keys](THEME_KEYS) to [`Style`]s.
    pub fn new(highlights: BTreeMap<String, Style>) -> Self {
        Self(highlights)
    }

    /// Consume `self` and return the contained theme map.
    ///
    /// May be used to merge multiple [`ResolvedTheme`]s together.
    pub fn into_inner(self) -> BTreeMap<String, Style> {
        self.0
    }

    /// Returns a reference to the style corresponding to the key.
    pub fn get<Q>(&self, key: &Q) -> Option<&Style>
    where
        String: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.get(key)
    }

    /// Try to find the best possible style supported by the them given a theme key.
    ///
    /// For example, if `key` is `keyword.operator` but this theme only has a style defined for
    /// `keyword`, then the style for `keyword` is returned. Additionally, if no style is found,
    /// the method tries to use the keys `text` and `text.literal` as fallbacks.
    pub fn find_style(&self, mut key: &str) -> Option<Style> {
        // if the theme contains the entire key, use that
        if let Some(style) = self.get(key) {
            return Some(*style);
        }

        // otherwise continue to strip the right-most part of the key
        while let Some((rest, _)) = key.rsplit_once('.') {
            // until the theme contains the key
            if let Some(style) = self.get(rest) {
                return Some(*style);
            }
            key = rest;
        }

        // or when the theme doesn't have any matching style, try to use the `text` style as a fallback
        self.get("text")
            .or_else(|| self.get("text.literal"))
            .copied()
    }
}

impl<Q> Index<&Q> for ResolvedTheme
where
    String: Borrow<Q>,
    Q: Ord + ?Sized,
{
    type Output = Style;

    fn index(&self, key: &Q) -> &Self::Output {
        self.get(key).expect("no entry found for key")
    }
}

impl From<BTreeMap<String, Style>> for ResolvedTheme {
    fn from(highlights: BTreeMap<String, Style>) -> Self {
        Self::new(highlights)
    }
}

impl TryFrom<Theme> for ResolvedTheme {
    type Error = Error;

    /// Try to create a [`ResolvedTheme`] from a [`Theme`] by calling [`Theme::resolve_links`].
    fn try_from(value: Theme) -> Result<Self> {
        value.resolve_links()
    }
}

impl ThemeValue {
    fn resolve_link(&self, target: &Self) -> Self {
        match (self, target) {
            (ThemeValue::Simple(_), _) => target.clone(),
            (
                ThemeValue::Extended {
                    color: Some(color),
                    underline,
                    strikethrough,
                    italic,
                    bold,
                    link: _,
                },
                ThemeValue::Simple(_),
            )
            | (
                ThemeValue::Extended {
                    color: None,
                    underline,
                    strikethrough,
                    italic,
                    bold,
                    link: _,
                },
                ThemeValue::Simple(color),
            ) => Self::Extended {
                color: Some(color.clone()),
                underline: *underline,
                strikethrough: *strikethrough,
                italic: *italic,
                bold: *bold,
                link: None,
            },
            (
                ThemeValue::Extended {
                    color: color @ Some(_),
                    underline,
                    strikethrough,
                    italic,
                    bold,
                    link: _,
                },
                ThemeValue::Extended {
                    color: _,
                    underline: other_underline,
                    strikethrough: other_strikethrough,
                    italic: other_italic,
                    bold: other_bold,
                    link,
                },
            )
            | (
                ThemeValue::Extended {
                    color: None,
                    underline,
                    strikethrough,
                    italic,
                    bold,
                    link: _,
                },
                ThemeValue::Extended {
                    color,
                    underline: other_underline,
                    strikethrough: other_strikethrough,
                    italic: other_italic,
                    bold: other_bold,
                    link,
                },
            ) => Self::Extended {
                color: color.clone(),
                underline: *underline || *other_underline,
                strikethrough: *strikethrough || *other_strikethrough,
                italic: *italic || *other_italic,
                bold: *bold || *other_bold,
                link: link.clone(),
            },
        }
    }
}

/// Convenience macro for constructing new [`Theme`]s.
///
/// Currently, the macro is very strict about the input's structure. See the [example](#example)
/// below to learn more. Also note that for [extended values](ThemeValue::Extended), either
/// [`color`](ThemeValue::Extended::color) or [`link`](ThemeValue::Extended::link) must be set, or
/// any call to [`resolve_links`](Theme::resolve_links) will fail.
///
/// See the documentation for [`Theme`] and [`ResolvedTheme`] for more information on themes.
///
/// # Example
///
/// ```
/// use syntastica_core::{theme, theme::{Theme, ThemeValue}};
/// use std::collections::BTreeMap;
///
/// let theme = theme! {
///     // specify colors using hex literals
///     "purple": "#c678dd",
///     "blue": "#61afef",
///     "green": "#98c379",
///
///     // link to other keys using a `$` sign
///     "keyword": "$purple",
///     "function": "$blue",
///
///     // specify more styling options in curly braces
///     // (note that currently this order required by the macro)
///     "string": {
///         color: None, // either `None` or `"#<color>"`
///         underline: false,
///         strikethrough: false,
///         italic: true,
///         bold: false,
///         link: "green", // either `None` or `"<key>"`
///     },
/// };
///
/// assert_eq!(theme, Theme::new(BTreeMap::from([
///     ("purple".to_owned(), ThemeValue::Simple("#c678dd".to_owned())),
///     ("blue".to_owned(), ThemeValue::Simple("#61afef".to_owned())),
///     ("green".to_owned(), ThemeValue::Simple("#98c379".to_owned())),
///     ("keyword".to_owned(), ThemeValue::Simple("$purple".to_owned())),
///     ("function".to_owned(), ThemeValue::Simple("$blue".to_owned())),
///     ("string".to_owned(), ThemeValue::Extended {
///         color: None,
///         underline: false,
///         strikethrough: false,
///         italic: true,
///         bold: false,
///         link: Some("green".to_owned()),
///     }),
/// ])));
/// ```
#[macro_export(local_inner_macros)]
macro_rules! theme {
    ($($tt:tt)*) => {
        theme_impl!($($tt)*)
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! theme_impl {
    () => {};
    ($($key:literal : $value:tt),* $(,)?) => {{
        let mut theme = ::std::collections::BTreeMap::new();
        $(
            theme.insert($key.to_owned(), theme_impl!(@value $value));
        )*
        $crate::theme::Theme::new(theme)
    }};
    (@value $str:literal) => {
        $crate::theme::ThemeValue::Simple($str.to_owned())
    };
    (@value {
        color: $color:tt,
        underline: $underline:expr,
        strikethrough: $strikethrough:expr,
        italic: $italic:expr,
        bold: $bold:expr,
        link: $link:tt $(,)?
    }) => {
        $crate::theme::ThemeValue::Extended {
            color: theme_impl!(@option $color),
            underline: $underline,
            strikethrough: $strikethrough,
            italic: $italic,
            bold: $bold,
            link: theme_impl!(@option $link),
        }
    };
    (@option None) => { None };
    (@option $str:literal) => { Some($str.to_owned()) };
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
            theme.find_style("keyword.return"),
            Some(Style::color_only(255, 0, 0)),
        );
        assert_eq!(
            theme.find_style("keyword.operator"),
            Some(Style::color_only(0, 0, 0)),
        );
        assert_eq!(
            theme.find_style("keyword"),
            Some(Style::color_only(0, 0, 0)),
        );
        assert_eq!(theme.find_style("other"), None);
    }

    #[test]
    fn style_fallback() {
        let theme = theme! {
            "text": "#000000",
        }
        .resolve_links()
        .unwrap();

        assert_eq!(theme.find_style("other"), Some(Style::color_only(0, 0, 0)));
    }
}
