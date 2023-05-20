#[cfg(feature = "parsers-some")]
use std::collections::HashMap;

#[cfg(feature = "parsers-some")]
use syntastica::renderer::TerminalRenderer;

#[cfg(not(feature = "parsers-some"))]
fn main() {
    compile_error!("this example requires the `parsers-some` feature to be enabled");
}

#[cfg(feature = "parsers-some")]
#[rustfmt::skip]
fn main() {
    let examples: HashMap<String, String> =
        toml::from_str(include_str!("./example_programs.toml")).unwrap();
    let code = &examples["rs"];

    example(code, syntastica_themes::one::onedark(), "onedark");
    example(code, syntastica_themes::one::onedark_darker(), "onedark_darker");
    example(code, syntastica_themes::one::onedark_cool(), "onedark_cool");
    example(code, syntastica_themes::one::onedark_deep(), "onedark_deep");
    example(code, syntastica_themes::one::onedark_warm(), "onedark_warm");
    example(code, syntastica_themes::one::onedark_warmer(), "onedark_warmer");
    example(code, syntastica_themes::one::onelight(), "onelight");
    example(code, syntastica_themes::gruvbox::dark(), "gruvbox dark");
    example(code, syntastica_themes::gruvbox::light(), "gruvbox light");
}

#[cfg(feature = "parsers-some")]
fn example(code: &str, theme: syntastica::config::Config, name: &str) {
    println!(
        "\n\x1b[1m{name}:\x1b[0m\n{0}\n{1}{0}",
        "-".repeat(50),
        syntastica::highlight(code.trim(), "rs", &mut TerminalRenderer, theme).unwrap()
    );
}
