use std::collections::BTreeMap;

use syntastica::{providers::ConfiguredLanguages, renderer::TerminalRenderer, Highlighter};
use syntastica_parsers_git::ParserProviderGit;

fn main() {
    let languages =
        ConfiguredLanguages::try_configure(&ParserProviderGit, syntastica_themes::one::dark())
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
                &ParserProviderGit,
                highlighter
            )
            .unwrap(),
            &mut TerminalRenderer
        )
    );
}
