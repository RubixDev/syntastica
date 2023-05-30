#![doc = include_str!("../README.md")]
#![cfg_attr(
    feature = "docs",
    cfg_attr(doc, doc = ::document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#))
)]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(rust_2018_idioms)]
// #![deny(missing_docs)]

use syntastica_core::theme::Theme;

pub mod gruvbox;
pub mod one;

/// Try to get a theme given its path as a string.
///
/// # Example
///
/// ```
/// assert_eq!(
///     syntastica_themes::from_str("one::dark"),
///     Some(syntastica_themes::one::dark()),
/// );
/// ```
pub fn from_str(theme_name: impl AsRef<str>) -> Option<Theme> {
    // TODO: all themes should return a `ResolvedTheme`
    let raw = match theme_name.as_ref() {
        "one::dark" => one::dark(),
        "one::darker" => one::darker(),
        "one::cool" => one::cool(),
        "one::deep" => one::deep(),
        "one::warm" => one::warm(),
        "one::warmer" => one::warmer(),
        "one::light" => one::light(),
        "gruvbox::dark" => gruvbox::dark(),
        "gruvbox::light" => gruvbox::light(),
        _ => return None,
    };
    Some(raw)
}
