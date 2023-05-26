use std::collections::HashMap;

use syntastica::{renderer::TerminalRenderer, theme::Theme, Highlights, Processor};
use syntastica_parsers_git::LanguageProviderImpl;
use syntastica_themes as themes;

fn main() {
    let examples: HashMap<String, String> =
        toml::from_str(include_str!("./example_programs.toml")).unwrap();

    let highlights = Processor::process_once(
        &examples["rust"],
        "rust",
        &LanguageProviderImpl::with_languages(&["rust", "regex"]),
    )
    .unwrap();

    run_examples(&highlights);
}

fn run_examples(h: &Highlights) {
    example(h, themes::one::dark(), "one::dark");
    example(h, themes::one::darker(), "one::darker");
    example(h, themes::one::cool(), "one::cool");
    example(h, themes::one::deep(), "one::deep");
    example(h, themes::one::warm(), "one::warm");
    example(h, themes::one::warmer(), "one::warmer");
    example(h, themes::one::light(), "one::light");
    example(h, themes::gruvbox::dark(), "gruvbox::dark");
    example(h, themes::gruvbox::light(), "gruvbox::light");
}

fn example(highlights: &Highlights, theme: Theme, name: &str) {
    let theme = theme.resolve_links().unwrap();
    let bg_color = theme.get("bg0").copied().map(|style| style.color());
    println!(
        "\n\x1b[1m{name}:\x1b[0m\n{0}\n{1}{0}",
        "-".repeat(50),
        syntastica::render(highlights, &mut TerminalRenderer::new(bg_color), theme),
    );
}
