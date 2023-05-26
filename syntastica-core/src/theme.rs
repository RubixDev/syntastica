use std::{borrow::Borrow, collections::BTreeMap, ops::Index};

use crate::{
    style::{Color, Style},
    Error, Result,
};

#[derive(Clone, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Theme(BTreeMap<String, ThemeValue>);

#[derive(Clone, Hash, Debug)]
pub struct ResolvedTheme(BTreeMap<String, Style>);

#[derive(Clone, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ThemeValue {
    Simple(String),
    Extended {
        color: Option<String>,
        #[cfg_attr(feature = "serde", serde(default))]
        underline: bool,
        #[cfg_attr(feature = "serde", serde(default))]
        strikethrough: bool,
        #[cfg_attr(feature = "serde", serde(default))]
        italic: bool,
        #[cfg_attr(feature = "serde", serde(default))]
        bold: bool,
        link: Option<String>,
    },
}

impl Theme {
    pub fn new(highlights: BTreeMap<String, ThemeValue>) -> Self {
        Self(highlights)
    }

    pub fn into_inner(self) -> BTreeMap<String, ThemeValue> {
        self.0
    }

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
                                Style::new(Color::from_hex(color)?, false, false, false, false)
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
                                Color::from_hex(color.expect("links have been resolved"))?,
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
    pub fn new(highlights: BTreeMap<String, Style>) -> Self {
        Self(highlights)
    }

    pub fn into_inner(self) -> BTreeMap<String, Style> {
        self.0
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&Style>
    where
        String: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.get(key)
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
