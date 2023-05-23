use std::collections::HashMap;

use syntastica::{
    providers::{ConfiguredLanguages, ParserProvider},
    renderer::TerminalRenderer,
    Highlighter,
};
use syntastica_parsers_git::ParserProviderImpl;
use syntastica_themes as themes;

fn main() {
    let examples: HashMap<String, String> =
        toml::from_str(include_str!("./example_programs.toml")).unwrap();
    let code = &examples["rust"];
    let mut hl = Highlighter::new();

    example(code, &mut hl, themes::one::dark(), "one::dark");
    example(code, &mut hl, themes::one::darker(), "one::darker");
    example(code, &mut hl, themes::one::cool(), "one::cool");
    example(code, &mut hl, themes::one::deep(), "one::deep");
    example(code, &mut hl, themes::one::warm(), "one::warm");
    example(code, &mut hl, themes::one::warmer(), "one::warmer");
    example(code, &mut hl, themes::one::light(), "one::light");
    example(code, &mut hl, themes::gruvbox::dark(), "gruvbox::dark");
    example(code, &mut hl, themes::gruvbox::light(), "gruvbox::light");
}

fn example(
    code: &str,
    highlighter: &mut Highlighter,
    theme: syntastica::config::Config,
    name: &str,
) {
    let provider = ParserProviderImpl::with_languages(&["rust"]);
    println!(
        "\n\x1b[1m{name}:\x1b[0m\n{0}\n{1}{0}",
        "-".repeat(50),
        syntastica::render(
            &syntastica::process(
                code.trim(),
                "rust",
                &ConfiguredLanguages::try_configure(&provider, theme).unwrap(),
                |lang_name| provider.for_injection(lang_name),
                highlighter
            )
            .unwrap(),
            &mut TerminalRenderer
        )
    );
}
