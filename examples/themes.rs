use std::collections::HashMap;

use syntastica::{
    providers::{ConfiguredLanguages, LanguageProvider},
    renderer::TerminalRenderer,
    Highlighter,
};
use syntastica_parsers_git::LanguageProviderImpl;
use syntastica_themes as themes;

fn main() {
    let examples: HashMap<String, String> =
        toml::from_str(include_str!("./example_programs.toml")).unwrap();
    let code = &examples["rust"];
    let mut highlighter = Highlighter::new();
    let languages = ConfiguredLanguages::try_configure(&LanguageProviderImpl::with_languages(&[
        "rust", "regex",
    ]))
    .unwrap();
    run_examples(code, &mut highlighter, &languages);
}

fn run_examples(c: &str, h: &mut Highlighter, l: &ConfiguredLanguages) {
    example(c, h, l, themes::one::dark(), "one::dark");
    example(c, h, l, themes::one::darker(), "one::darker");
    example(c, h, l, themes::one::cool(), "one::cool");
    example(c, h, l, themes::one::deep(), "one::deep");
    example(c, h, l, themes::one::warm(), "one::warm");
    example(c, h, l, themes::one::warmer(), "one::warmer");
    example(c, h, l, themes::one::light(), "one::light");
    example(c, h, l, themes::gruvbox::dark(), "gruvbox::dark");
    example(c, h, l, themes::gruvbox::light(), "gruvbox::light");
}

fn example(
    code: &str,
    highlighter: &mut Highlighter,
    languages: &ConfiguredLanguages,
    theme: syntastica::theme::Theme,
    name: &str,
) {
    let theme = theme.resolve_links().unwrap();
    let bg_color = theme.get("bg0").copied().map(|style| style.color());
    let provider = LanguageProviderImpl::with_languages(&["rust"]);
    println!(
        "\n\x1b[1m{name}:\x1b[0m\n{0}\n{1}{0}",
        "-".repeat(50),
        syntastica::render(
            &syntastica::process(
                code.trim(),
                "rust",
                languages,
                |lang_name| provider.for_injection(lang_name),
                highlighter
            )
            .unwrap(),
            &mut TerminalRenderer::new(bg_color),
            theme,
        )
    );
}
