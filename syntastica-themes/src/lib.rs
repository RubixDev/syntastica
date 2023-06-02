#![doc = include_str!("../README.md")]
#![cfg_attr(
    feature = "docs",
    cfg_attr(doc, doc = ::document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#))
)]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(rust_2018_idioms)]
// #![deny(missing_docs)]

use syntastica_core::theme::ResolvedTheme;

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
// TODO: is it somehow possible to auto generate this?
pub fn from_str(theme_name: impl AsRef<str>) -> Option<ResolvedTheme> {
    match theme_name.as_ref() {
        "one::dark" => Some(one::dark()),
        "one::darker" => Some(one::darker()),
        "one::cool" => Some(one::cool()),
        "one::deep" => Some(one::deep()),
        "one::warm" => Some(one::warm()),
        "one::warmer" => Some(one::warmer()),
        "one::light" => Some(one::light()),
        "gruvbox::dark" => Some(gruvbox::dark()),
        "gruvbox::light" => Some(gruvbox::light()),
        _ => None,
    }
}
