use std::collections::BTreeMap;

use syntastica::renderer::TerminalRenderer;
use syntastica_parsers_git::ParserProviderGit;

fn main() {
    let examples: BTreeMap<String, String> =
        toml::from_str(include_str!("./example_programs.toml")).unwrap();
    for (file_ext, code) in &examples {
        println!("\n\x1b[1m{file_ext}:\x1b[0m\n{}", "-".repeat(50));
        example(code, file_ext);
        println!("{}", "-".repeat(50))
    }
}

fn example(code: &str, file_extension: &str) {
    println!(
        "{}",
        syntastica::highlight(
            code.trim(),
            file_extension,
            ParserProviderGit,
            &mut TerminalRenderer,
            syntastica_themes::one::dark()
        )
        .unwrap()
    );
}
