#[cfg(feature = "parsers-all")]
use std::collections::BTreeMap;

#[cfg(feature = "parsers-all")]
use syntastica::renderer::TerminalRenderer;

#[cfg(not(feature = "parsers-all"))]
fn main() {
    compile_error!("this example requires the `parsers-all` feature to be enabled");
}

#[cfg(feature = "parsers-all")]
fn main() {
    let examples: BTreeMap<String, String> =
        toml::from_str(include_str!("./example_programs.toml")).unwrap();
    for (file_ext, code) in &examples {
        println!("\n\x1b[1m{file_ext}:\x1b[0m\n{}", "-".repeat(50));
        example(code, file_ext);
        println!("{}", "-".repeat(50))
    }
}

#[cfg(feature = "parsers-all")]
fn example(code: &str, file_extension: &str) {
    println!(
        "{}",
        syntastica::highlight(
            code.trim(),
            file_extension,
            &mut TerminalRenderer,
            syntastica_themes::one::onedark()
        )
        .unwrap()
    );
}
