use std::{
    env,
    path::{Path, PathBuf},
    process,
};

use anyhow::Result;
use once_cell::sync::Lazy;

mod codegen;
mod set_version;

pub static WORKSPACE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
});

fn main() {
    if let Err(err) = try_main() {
        eprintln!("error running task: {err}");
        process::exit(1);
    }
}

fn try_main() -> Result<()> {
    match env::args().nth(1).unwrap_or(String::new()).as_str() {
        "--help" | "-h" | "" => println!(
            "{}",
            r###"
Usage: Run with `cargo xtask <task>`, eg. `cargo xtask codegen`.

    Tasks:
        codegen:                Run all codegen subtasks
        codegen queries:        Generate the `lib.rs` file for syntastica-queries
        codegen parsers-dep:    Generate parts of the `Cargo.toml` for syntastica-parsers
        codegen parsers-gitdep: Generate parts of the `Cargo.toml` for syntastica-parsers-gitdep
        codegen parser-lists:   Generate the parser lists in all three syntastica-parsers READMEs
        codegen themes:         Generate the themes for syntastica-themes
        set-version <version>:  Set the version of all syntastica crates
            "###
            .trim(),
        ),
        "codegen" => codegen::run()?,
        "set-version" => set_version::run()?,
        task => eprintln!(
            "unknown task '{task}', run `cargo xtask --help` to see a list of available tasks"
        ),
    }

    Ok(())
}
