#![doc = include_str!("../README.md")]
#![cfg_attr(
    feature = "docs",
    cfg_attr(doc, doc = ::document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#))
)]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

mod processor;
pub mod renderer;

pub use processor::Processor;
pub use renderer::render;
pub use syntastica_core::*;

use language_set::LanguageSet;
use renderer::Renderer;
use theme::ResolvedTheme;

/// Source code with style information attached, as returned by the [`Processor`].
///
/// The type is a vector of lines of the source code, and each line is a vector of 2-tuples.
/// The first element of each 2-tuple is a slice of the source text, and the second
/// element is an optional theme key for that region of text. The theme key is guaranteed to be
/// present in [`THEME_KEYS`](theme::THEME_KEYS). The [`render`] function takes this type,
/// a [`Renderer`], and a [`ResolvedTheme`] as arguments in order to render the text for end users.
pub type Highlights<'src> = Vec<Vec<(&'src str, Option<&'static str>)>>;

/// Convenience function for [processing](Processor) and directly [rendering](render) code once.
///
/// **Only use this function if you do not plan to highlight multiple inputs!**
/// When planning to render the same input multiple times, use [`Processor::process_once`] instead.
/// When planning to process multiple different inputs with the same [`LanguageSet`], use a
/// [`Processor`] and call [`Processor::process`] for each input.
///
/// # Errors
///
/// The function may error in the following situations:
///
/// - [`Processor::process_once`] returns an error.
/// - The `TryInto<ResolvedTheme>` implementation on `T` errors. If `T` is [`Theme`](theme::Theme),
///   look at [`Theme::resolve_links`](theme::Theme::resolve_links) for when this might happen.
pub fn highlight<T, E>(
    code: impl AsRef<str>,
    language_name: &str,
    language_set: &impl LanguageSet,
    renderer: &mut impl Renderer,
    theme: T,
) -> Result<String>
where
    T: TryInto<ResolvedTheme, Error = E>,
    crate::Error: From<E>,
{
    Ok(render(
        &Processor::process_once(code.as_ref(), language_name, language_set)?,
        renderer,
        theme.try_into()?,
    ))
}
