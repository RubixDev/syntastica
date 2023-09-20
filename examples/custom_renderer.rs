use std::{borrow::Cow, collections::HashMap, error::Error};

use syntastica::{renderer::Renderer, Processor};
use syntastica_core::style::Style;
use syntastica_parsers_git::{Lang, LanguageSetImpl};

fn main() -> Result<(), Box<dyn Error>> {
    let examples: HashMap<String, String> =
        toml::from_str(include_str!("./example_programs.toml"))?;

    let language_set = LanguageSetImpl::new();
    language_set.preload(&[Lang::Rust, Lang::Regex, Lang::C])?;
    let mut processor = Processor::new(&language_set);
    let theme = syntastica_themes::one::light();

    println!(
        "{}",
        syntastica::render(
            &processor.process(&examples["rust"], Lang::Rust)?,
            &mut TypstRenderer,
            &theme
        ),
    );
    println!(
        "{}",
        syntastica::render(
            &processor.process(&examples["c"], Lang::C)?,
            &mut TypstRenderer,
            &theme
        ),
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

    fn unstyled<'a>(&mut self, text: &'a str) -> Cow<'a, str> {
        format!("raw({text:?});").into()
    }

    fn styled<'a>(&mut self, text: &'a str, style: Style) -> Cow<'a, str> {
        let (r, g, b) = style.color().into_components();
        let mut options = format!("fill:rgb({r},{g},{b}),");
        let mut front = String::new();
        let mut paren_count = 0;
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

        // trim the trailing `;` that `unstyled()` adds after raw text
        let text = &text[..text.len().saturating_sub(1)];
        format!("{front}text({options}{text}){};", ")".repeat(paren_count)).into()
    }
}
