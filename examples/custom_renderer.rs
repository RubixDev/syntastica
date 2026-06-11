//! This example contains a custom renderer which outputs code that can be pasted into Typst
//! documents. It is used by the `theme-svgs` xtask to create SVGs of all themes with the Rust
//! example program.

use std::{borrow::Cow, collections::HashMap, env, error::Error};

use syntastica::renderer::Renderer;
use syntastica_core::style::Style;
use syntastica_parsers_git::{Lang, LanguageSetImpl};

fn main() -> Result<(), Box<dyn Error>> {
    let examples: HashMap<String, String> =
        toml::from_str(include_str!("./example_programs.toml"))?;
    let theme_name = env::args()
        .nth(1)
        .unwrap_or_else(|| "one::light".to_string());
    let theme = syntastica_themes::from_str(&theme_name)
        .ok_or_else(|| format!("unknown theme `{theme_name}`"))?;

    println!(
        "{}",
        syntastica::highlight(
            &examples["rust"],
            Lang::Rust,
            &LanguageSetImpl::new(),
            &mut TypstRenderer,
            theme
        )?,
    );

    Ok(())
}

/// A renderer for syntastica which outputs code for use in [Typst](https://typst.app/) documents.
pub struct TypstRenderer;
// for information on the trait methods, see the trait docs
impl Renderer for TypstRenderer {
    fn head(&mut self) -> Cow<'static, str> {
        "#{".into()
    }

    fn tail(&mut self) -> Cow<'static, str> {
        "}".into()
    }

    fn newline(&mut self) -> Cow<'static, str> {
        r"linebreak();".into()
    }

    fn escape<'a>(&mut self, text: &'a str) -> Cow<'a, str> {
        format!("raw({text:?})").into()
    }

    fn unstyled<'a>(&mut self, text: &'a str) -> Cow<'a, str> {
        format!("{text};").into()
    }

    fn styled<'a>(&mut self, text: &'a str, style: Style) -> Cow<'a, str> {
        let (r, g, b) = style.color().into_components();
        let mut options = format!("fill:rgb({r},{g},{b}),");
        let mut front = String::new();
        let mut paren_count = 0;
        if let Some(color) = style.bg() {
            let (r, g, b) = color.into_components();
            front += &format!("highlight(fill:rgb({r},{g},{b}),");
            paren_count += 1;
        }
        if style.underline() {
            front += "underline(";
            paren_count += 1;
        }
        if style.strikethrough() {
            front += "strike(";
            paren_count += 1;
        }
        if style.italic() {
            options += "style:\"italic\",";
        }
        if style.bold() {
            options += "weight:\"bold\",";
        }

        format!("{front}text({options}{text}){};", ")".repeat(paren_count)).into()
    }
}
