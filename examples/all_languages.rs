use std::{collections::BTreeMap, env};

use syntastica::{language_set::SupportedLanguage, renderer::TerminalRenderer, Processor};
use syntastica_parsers_git::{Lang, LanguageSetImpl};

fn main() {
    let filter = env::args().nth(1).unwrap_or_default();

    let language_set = LanguageSetImpl::new();
    let mut processor = Processor::new(&language_set);

    let examples: BTreeMap<String, String> =
        toml::from_str(include_str!("./example_programs.toml")).unwrap();

    for (lang, code) in &examples {
        if !lang.contains(&filter) {
            continue;
        }

        println!("\n\x1b[1m{lang}:\x1b[0m\n{}", "-".repeat(50));
        if let Err(err) = example(&mut processor, code, lang) {
            println!("ERROR: {err}");
        }
        println!("{}", "-".repeat(50))
    }
}

fn example(
    processor: &mut Processor<LanguageSetImpl>,
    code: &str,
    lang_name: &str,
) -> syntastica::Result<()> {
    println!(
        "{}",
        syntastica::render(
            &processor.process(code.trim(), Lang::for_name(lang_name, &())?)?,
            &mut TerminalRenderer::new(None),
            syntastica_themes::one::dark(),
        )
    );
    Ok(())
}
