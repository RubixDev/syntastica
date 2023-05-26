#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

mod processor;
pub mod renderer;

pub use processor::Processor;
pub use renderer::render;
pub use syntastica_core::*;

use providers::LanguageProvider;
use renderer::Renderer;
use theme::ResolvedTheme;

pub type Highlights<'src> = Vec<Vec<(&'src str, Option<&'static str>)>>;

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
