use std::collections::BTreeMap;

use syntastica::{
    providers::{ConfiguredLanguages, LanguageProvider},
    renderer::TerminalRenderer,
    Highlighter,
};
use syntastica_parsers_git::LanguageProviderImpl;

fn main() {
    let languages = ConfiguredLanguages::try_configure(
        &LanguageProviderImpl::all(),
        syntastica_themes::one::dark(),
    )
    .unwrap();
    let mut highlighter = Highlighter::new();

    let examples: BTreeMap<String, String> =
        toml::from_str(include_str!("./example_programs.toml")).unwrap();

    for (file_ext, code) in &examples {
        println!("\n\x1b[1m{file_ext}:\x1b[0m\n{}", "-".repeat(50));
        example(&languages, &mut highlighter, code, file_ext);
        println!("{}", "-".repeat(50))
    }
}

fn example(
    languages: &ConfiguredLanguages,
    highlighter: &mut Highlighter,
    code: &str,
    file_extension: &str,
) {
    println!(
        "{}",
        syntastica::render(
            &syntastica::process(
                code.trim(),
                file_extension,
                languages,
                |lang_name| LanguageProviderImpl::all().for_injection(lang_name),
                highlighter
            )
            .unwrap(),
            &mut TerminalRenderer::new(None),
        )
    );
}
