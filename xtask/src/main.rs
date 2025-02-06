#![allow(clippy::needless_raw_string_hashes)]

use std::{
    env,
    path::{Path, PathBuf},
    process,
};

use anyhow::Result;
use once_cell::sync::Lazy;

mod add_lang;
mod codegen;
mod fetch_queries;
mod set_version;
mod theme_svgs;
mod update_langs;

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
        eprintln!("error running task: {err}");
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
        codegen js-lists:                     Generate the language and theme lists in the JavaScript bindings
        codegen themes:                       Generate some themes for syntastica-themes
        codegen theme-list:                   Generate the `THEMES` list and `from_str` function for syntastica-themes
        set-version <version>:                Set the version of all syntastica crates
        add-lang <group> <name> <url> [path]: Add boilerplate code for a new language called <name> with sources at <url>/[path] in the feature group <group>
        update-langs                          Search for new versions of languages
        fetch-queries                         Fetch latest upstream versions of forked queries
        theme-svgs                            Create SVGs for all themes using Typst and the `custom_renderer` example
            "###
            .trim(),
        ),
        "codegen" => codegen::run()?,
        "set-version" => set_version::run()?,
        "add-lang" => add_lang::run()?,
        "update-langs" => update_langs::run()?,
        "fetch-queries" => fetch_queries::run()?,
        "theme-svgs" => theme_svgs::run()?,
        task => eprintln!(
            "unknown task '{task}', run `cargo xtask --help` to see a list of available tasks"
        ),
    }

    Ok(())
}
