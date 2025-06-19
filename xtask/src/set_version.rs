use std::{env, fs};

use anyhow::{Context, Result};
use fancy_regex::{Captures, Regex};
use once_cell::sync::Lazy;

static MAIN_VERSION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(\[workspace\][\s\S]*?package\.version[ \t]*=[ \t]*")(.*?)(")"#).unwrap()
});
static DEP_VERSION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)^([ \t]*([a-z_-]+)[ \t]*=[ \t]*\{[ \t]*version[ \t]*=[ \t]*")(.*?)(")"#)
        .unwrap()
});

pub fn run() -> Result<()> {
    let new_version = env::args()
        .nth(2)
        .with_context(|| "missing version for `set-version` task")?;

    for path in glob::glob(&format!("{}/**/Cargo.toml", crate::WORKSPACE_DIR.display()))? {
        let path = path?;
        println!();
        let cargo_toml = fs::read_to_string(&path)?;

        let cargo_toml = MAIN_VERSION_REGEX
            .replace(&cargo_toml, |captures: &Captures| {
                println!("{} -> {new_version} (workspace)", &captures[2]);
                format!("{}{new_version}{}", &captures[1], &captures[3])
            })
            .into_owned();

        let cargo_toml = DEP_VERSION_REGEX
            .replace_all(&cargo_toml, |captures: &Captures| {
                if !captures[2].starts_with("syntastica") {
                    println!("skipping dependency {}", &captures[2]);
                    return format!("{}{}{}", &captures[1], &captures[3], &captures[4]);
                }
                println!("dep: {} -> {new_version} ({})", &captures[3], &captures[2]);
                format!("{}{new_version}{}", &captures[1], &captures[4])
            })
            .into_owned();

        fs::write(&path, cargo_toml)?;
    }

    Ok(())
}
