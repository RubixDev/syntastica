use std::collections::HashMap;

use syntastica::renderer::TerminalRenderer;
use syntastica_parsers_git::ParserProviderGit;

fn main() {
    let examples: HashMap<String, String> =
        toml::from_str(include_str!("./example_programs.toml")).unwrap();
    let code = &examples["rs"];

    example(code, syntastica_themes::one::dark(), "one::dark");
    example(code, syntastica_themes::one::darker(), "one::darker");
    example(code, syntastica_themes::one::cool(), "one::cool");
    example(code, syntastica_themes::one::deep(), "one::deep");
    example(code, syntastica_themes::one::warm(), "one::warm");
    example(code, syntastica_themes::one::warmer(), "one::warmer");
    example(code, syntastica_themes::one::light(), "one::light");
    example(code, syntastica_themes::gruvbox::dark(), "gruvbox::dark");
    example(code, syntastica_themes::gruvbox::light(), "gruvbox::light");
}

fn example(code: &str, theme: syntastica::config::Config, name: &str) {
    println!(
        "\n\x1b[1m{name}:\x1b[0m\n{0}\n{1}{0}",
        "-".repeat(50),
        syntastica::highlight(
            code.trim(),
            "rs",
            ParserProviderGit,
            &mut TerminalRenderer,
            theme
        )
        .unwrap()
    );
}
