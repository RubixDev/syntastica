use std::{fs, path::PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use lazy_regex::regex_captures;
use once_cell::sync::Lazy;

pub fn run() -> Result<()> {
    static QUERIES_DIR: Lazy<PathBuf> = Lazy::new(|| crate::WORKSPACE_DIR.join("queries"));
    for lang in fs::read_dir(&*QUERIES_DIR)? {
        let lang = lang?;
        if lang.path().is_file() {
            continue;
        }
        let lang_name = lang.file_name().to_string_lossy().into_owned();

        for file in fs::read_dir(lang.path())? {
            let file = file?;

            let old_query = fs::read_to_string(file.path())?;
            let filename = file.file_name().to_string_lossy().into_owned();

            let Some((_, host, user, repo, branch, path)) = regex_captures!(
                r"^;; Forked from https://(github|gitlab)\.com/([^/]*)/([^/]*)/(?:-/)?(?:blob|tree)/([^/]*)/([^?#\n]*)",
                &old_query
            ) else {
                println!("\x1b[1;33mwarning:\x1b[22m {lang_name}/{filename} does not specify a fork source\x1b[0m");
                continue;
            };

            let url = match host {
                "github" => {
                    format!("https://raw.githubusercontent.com/{user}/{repo}/{branch}/{path}")
                }
                "gitlab" => format!("https://gitlab.com/{user}/{repo}/-/raw/{branch}/{path}"),
                _ => unreachable!("the regex only allows above options"),
            };
            println!("fetching new {lang_name}/{filename} from {url}");
            let res = reqwest::blocking::get(url).with_context(|| "query request failed")?;
            let query = match res.status().is_success() {
                true => res.text()?,
                false => bail!(
                    "query request returned non-success status code: {}",
                    res.status()
                ),
            };
            let query = format!(
                "{:#}",
                rsexpr::from_slice_multi(&query).map_err(|errs| anyhow!(errs
                    .into_iter()
                    .map(|err| err.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")))?
            );
            fs::write(file.path().with_file_name(format!("new.{filename}")), query)?;
        }
    }

    // look for missing queries
    let langs_toml_path = crate::WORKSPACE_DIR.join("syntastica-macros/languages.toml");
    let mut langs_toml = fs::read_to_string(&langs_toml_path)?;
    for lang in &crate::LANGUAGE_CONFIG.languages {
        let kinds = match (lang.queries.injections, lang.queries.locals) {
            (false, false) => &["injections", "locals"][..],
            (false, true) => &["injections"],
            (true, false) => &["locals"],
            (true, true) => &[],
        };

        for &kind in kinds {
            let queries = fetch_query(&lang.name, kind)?;
            if let Some(text) = queries {
                fs::write(
                    crate::WORKSPACE_DIR.join(format!("queries/{}/{kind}.scm", lang.name)),
                    text,
                )?;
                println!("found new {kind} queries for {}", lang.name);

                let (before, rest) = langs_toml
                    .split_once(&format!("\nname = \"{}\"", lang.name))
                    .expect("language should be lang config");
                if kind == "injections" {
                    langs_toml = format!(
                        "{before}\nname = \"{}\"{}",
                        lang.name,
                        rest.replacen("\ninjections = false", "\ninjections = true", 1),
                    );
                } else if kind == "locals" {
                    langs_toml = format!(
                        "{before}\nname = \"{}\"{}",
                        lang.name,
                        rest.replacen("\nlocals = false", "\nlocals = true", 1),
                    );
                }
            }
        }
    }
    fs::write(&langs_toml_path, langs_toml)?;

    Ok(())
}

fn forked_from(name: &str, file: &str, content: &str) -> String {
    format!(";; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/{name}/{file}.scm
;; Licensed under the Apache License 2.0
{content}")
}

pub fn fetch_query(name: &str, kind: &str) -> Result<Option<String>> {
    const BASE_URL: &str =
        "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/HEAD/queries";
    reqwest::blocking::get(format!("{BASE_URL}/{name}/{kind}.scm"))
        .ok()
        .and_then(|res| match res.status().is_success() {
            true => res.text().ok(),
            false => None,
        })
        .map(|query| {
            Ok::<_, anyhow::Error>(forked_from(
                name,
                kind,
                &format!(
                    "{:#}",
                    rsexpr::from_slice_multi(&query)
                        .map_err(|errs| anyhow!(errs
                            .into_iter()
                            .map(|err| err.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")))
                        .context("failed to parse downloaded queries")?
                ),
            ))
        })
        .transpose()
}
