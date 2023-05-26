use std::collections::HashMap;

use syntastica::{renderer::TerminalRenderer, Processor};
use syntastica_parsers_git::LanguageProviderImpl;
use syntastica_themes as themes;

fn main() {
    let examples: HashMap<String, String> =
        toml::from_str(include_str!("./example_programs.toml")).unwrap();
    let code = &examples["rust"];
    let language_provider = LanguageProviderImpl::with_languages(&["rust", "regex"]);
    let mut processor = Processor::try_new(&language_provider).unwrap();
    run_examples(code, &mut processor);
}

fn run_examples(c: &str, p: &mut Processor) {
    example(c, p, themes::one::dark(), "one::dark");
    example(c, p, themes::one::darker(), "one::darker");
    example(c, p, themes::one::cool(), "one::cool");
    example(c, p, themes::one::deep(), "one::deep");
    example(c, p, themes::one::warm(), "one::warm");
    example(c, p, themes::one::warmer(), "one::warmer");
    example(c, p, themes::one::light(), "one::light");
    example(c, p, themes::gruvbox::dark(), "gruvbox::dark");
    example(c, p, themes::gruvbox::light(), "gruvbox::light");
}

fn example(code: &str, processor: &mut Processor, theme: syntastica::theme::Theme, name: &str) {
    let theme = theme.resolve_links().unwrap();
    let bg_color = theme.get("bg0").copied().map(|style| style.color());
    println!(
        "\n\x1b[1m{name}:\x1b[0m\n{0}\n{1}{0}",
        "-".repeat(50),
        syntastica::render(
            &processor.process(code, "rust").unwrap(),
            &mut TerminalRenderer::new(bg_color),
            theme,
        )
    );
}
