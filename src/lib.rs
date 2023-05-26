#![doc = include_str!("../README.md")]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(rust_2018_idioms)]
// #![deny(missing_docs)]
// TODO crate documentation

mod processor;
pub mod renderer;

pub use processor::Processor;
pub use renderer::render;
pub use syntastica_core::*;

use providers::LanguageProvider;
use renderer::Renderer;
use theme::ResolvedTheme;

/// Source code with style information attached, as returned by the [`Processor`].
/// The type is a vector of lines of the source code, and each line is a vector of 2-tuples.
/// The first element of each 2-tuple is a slice of the source text, and the second
/// element is an optional theme key for that region of text. The theme key is guaranteed to be
/// present in [`THEME_KEYS`]. The [`render`] function takes this type, a [`Renderer`], and a
/// [`ResolvedTheme`] as input in order to render the text for end users.
pub type Highlights<'src> = Vec<Vec<(&'src str, Option<&'static str>)>>;

/// Convenience function for [processing](Processor) and [rendering](render) code once. When
/// planning to render the same input multiple times, use [`process_once`](Processor::process_once)
/// instead. When planning to process multiple different inputs, with the same
/// [`LanguageProvider`], use a [`Processor`] and call [`process`](Processor::process) for each
/// input.
pub fn highlight<T, E>(
    code: &str,
    language_name: &str,
    language_provider: &impl LanguageProvider,
    renderer: &mut impl Renderer,
    theme: T,
) -> Result<String>
where
    T: TryInto<ResolvedTheme, Error = E>,
    crate::Error: From<E>,
{
    Ok(render(
        &Processor::process_once(code, language_name, language_provider)?,
        renderer,
        theme.try_into()?,
    ))
}
