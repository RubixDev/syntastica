use std::collections::BTreeMap;

use syntastica::{
    providers::ConfiguredLanguages, providers::LanguageProvider, renderer::TerminalRenderer,
};
use syntastica_parsers_git::ParserProviderGit;

fn main() {
    let languages = ConfiguredLanguages::configure(
        ParserProviderGit.get_languages().unwrap(),
        syntastica_themes::one::dark().resolve_links().unwrap(),
    );

    let examples: BTreeMap<String, String> =
        toml::from_str(include_str!("./example_programs.toml")).unwrap();

    for (file_ext, code) in &examples {
        println!("\n\x1b[1m{file_ext}:\x1b[0m\n{}", "-".repeat(50));
        example(&languages, code, file_ext);
        println!("{}", "-".repeat(50))
    }
}

fn example(languages: &ConfiguredLanguages, code: &str, file_extension: &str) {
    println!(
        "{}",
        syntastica::render(
            &syntastica::process(code.trim(), file_extension, languages, ParserProviderGit)
                .unwrap(),
            &mut TerminalRenderer
        )
    );
}
