#![allow(clippy::needless_raw_string_hashes)]

use std::{
    env,
    path::{Path, PathBuf},
    process,
};

use anyhow::Result;
use once_cell::sync::Lazy;

mod add_lang;
mod build_js_langs;
mod codegen;
mod fetch_queries;
mod set_version;
mod theme_svgs;
mod update_langs;
mod update_vite_example;

mod schema {
    include!("../../syntastica-macros/src/schema.rs");
}

pub static WORKSPACE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
});
pub static LANGUAGE_CONFIG: Lazy<schema::LanguageConfig> = Lazy::new(|| {
    toml::from_str(include_str!("../../syntastica-macros/languages.toml"))
        .expect("invalid `languages.toml`")
});

fn main() {
    if let Err(err) = try_main() {
        eprintln!("error running task:\n{err:?}");
        process::exit(1);
    }
}

fn try_main() -> Result<()> {
    match env::args().nth(1).unwrap_or_default().as_str() {
        "--help" | "-h" | "" => println!(
            "{}",
            r###"
Usage: Run with `cargo xtask <task>`, eg. `cargo xtask codegen`.

    Tasks:
        codegen:                              Run all codegen subtasks
        codegen queries:                      Generate the `lib.rs` file for syntastica-queries
        codegen parsers-dep:                  Generate parts of the `Cargo.toml` for syntastica-parsers
        codegen parsers-gitdep:               Generate parts of the `Cargo.toml` for syntastica-parsers-gitdep
        codegen parsers-git:                  Generate parts of the `Cargo.toml` for syntastica-parsers-git
        codegen parser-lists:                 Generate the parser lists in all three syntastica-parsers READMEs
        codegen js-list:                      Generate the theme list in the JavaScript bindings
        codegen js-pkgs:                      Generate the JS packages for all supported languages
        codegen theme-list:                   Generate the `THEMES` list and `from_str` function for syntastica-themes
        set-version <version>:                Set the version of all syntastica crates
        add-lang <group> <name> <url> [path]: Add boilerplate code for a new language called <name> with sources at <url>/[path] in the feature group <group>
        update-langs                          Search for new versions of languages
        fetch-queries                         Fetch latest upstream versions of forked queries
        theme-svgs                            Create SVGs for all themes using Typst and the `custom_renderer` example
        build-js-langs                        Build all JS language packages in the generated `syntastica-js/langs` directory
        update-vite-example                   Update the vite example project to include all languages
            "###
            .trim(),
        ),
        "codegen" => codegen::run()?,
        "set-version" => set_version::run()?,
        "add-lang" => add_lang::run()?,
        "update-langs" => update_langs::run()?,
        "fetch-queries" => fetch_queries::run()?,
        "theme-svgs" => theme_svgs::run()?,
        "build-js-langs" => build_js_langs::run()?,
        "update-vite-example" => update_vite_example::run()?,
        task => eprintln!(
            "unknown task '{task}', run `cargo xtask --help` to see a list of available tasks"
        ),
    }

    Ok(())
}
