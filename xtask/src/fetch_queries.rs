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
    Ok(())
}
