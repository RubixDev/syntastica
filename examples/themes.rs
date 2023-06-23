use std::collections::HashMap;

use syntastica::{renderer::TerminalRenderer, Highlights, Processor};
use syntastica_core::theme::ResolvedTheme;
use syntastica_parsers_git::LanguageSetImpl;

fn main() {
    let examples: HashMap<String, String> =
        toml::from_str(include_str!("./example_programs.toml")).unwrap();

    let highlights =
        Processor::process_once(&examples["rust"], "rust", &LanguageSetImpl::new()).unwrap();

    for theme in syntastica_themes::THEMES {
        example(
            &highlights,
            syntastica_themes::from_str(theme).unwrap(),
            theme,
        );
    }
}

fn example(highlights: &Highlights, theme: ResolvedTheme, name: &str) {
    let bg_color = theme.get("bg0").copied().map(|style| style.color());
    println!(
        "\n\x1b[1m{name}:\x1b[0m\n{0}\n{1}{0}",
        "-".repeat(50),
        syntastica::render(highlights, &mut TerminalRenderer::new(bg_color), theme),
    );
}
