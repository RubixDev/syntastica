use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    process::Command,
    time::Duration,
};

use anyhow::{anyhow, Context, Result};
use crates_io_api::SyncClient;
use fancy_regex::Regex;
use once_cell::sync::Lazy;
use serde_json::{Map, Value};

static URL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"https:\/\/(github|gitlab)\.com\/([^\/]*)\/([^\/?#]*)").unwrap());

pub fn run() -> Result<()> {
    let group = env::args()
        .nth(2)
        .with_context(|| "missing group for `add-lang` task")?;
    let name = env::args()
        .nth(3)
        .with_context(|| "missing name for `add-lang` task")?;
    let url = env::args()
        .nth(4)
        .with_context(|| "missing url for `add-lang` task")?;
    let path = env::args().nth(5);

    let rev = get_rev(&url).with_context(|| "unable to fetch latest revision of repository")?;

    let content_url = match URL_REGEX.captures(&url) {
        Ok(Some(groups)) => match &groups[1] {
            "github" => Some(format!(
                "https://raw.githubusercontent.com/{}/{}/{rev}",
                &groups[2], &groups[3],
            )),
            "gitlab" => Some(format!(
                "https://gitlab.com/{}/{}/-/raw/{rev}",
                &groups[2], &groups[3],
            )),
            _ => unreachable!("the regex only allows above options"),
        },
        _ => None,
    };
    let path_in_url = match &path {
        Some(path) => format!("/{path}"),
        None => String::new(),
    };

    println!("info: found revision '{rev}'");
    let file_extensions = content_url
        .as_ref()
        .and_then(|url| try_get_extensions(url))
        .unwrap_or_default();
    let external_c = content_url.as_ref().map_or(false, |url| {
        reqwest::blocking::get(format!("{url}{path_in_url}/src/scanner.c"))
            .map_or(false, |response| response.status().is_success())
    });
    println!("info: found external C scanner: {external_c}");
    let external_cpp = content_url.as_ref().map_or(false, |url| {
        reqwest::blocking::get(format!("{url}{path_in_url}/src/scanner.cc"))
            .map_or(false, |response| response.status().is_success())
    });
    println!("info: found external C++ scanner: {external_cpp}");

    let package = content_url
        .as_ref()
        .and_then(|url| try_get_package(url))
        .unwrap_or_else(|| format!("tree-sitter-{}", name.replace('_', "-")));
    println!("info: using package name '{package}'");

    let crates_io = match try_get_crates_io_version(&package) {
        Some(version) => format!("crates-io = \"{version}\""),
        None => "# crates-io = \"\"".into(),
    };
    println!("info: found crates.io version: '{crates_io}'");

    let mut queries_injections = false;
    let mut queries_locals = false;
    fs::create_dir_all(crate::WORKSPACE_DIR.join(format!("queries/{name}")))?;
    fs::write(
        crate::WORKSPACE_DIR.join(format!("queries/{name}/highlights.scm")),
        "",
    )?;
    const BASE_URL: &str =
        "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/HEAD/queries";
    for kind in ["highlights", "injections", "locals"] {
        let queries = reqwest::blocking::get(format!("{BASE_URL}/{name}/{kind}.scm"))
            .ok()
            .and_then(|res| match res.status().is_success() {
                true => res.text().ok(),
                false => None,
            })
            .map(|s| forked_from(&name, kind, &s));
        if let Some(text) = queries {
            fs::write(
                crate::WORKSPACE_DIR.join(format!("queries/{name}/{kind}.scm")),
                text,
            )?;
            if kind == "injections" {
                queries_injections = true;
            }
            if kind == "locals" {
                queries_locals = true;
            }
        }
    }

    let langs_toml_path = crate::WORKSPACE_DIR.join("syntastica-macros/languages.toml");
    let langs_toml = fs::read_to_string(&langs_toml_path)?;

    let mut langs = langs_toml
        .split("\n\n")
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    langs.push(format!(
        r###"[[languages]]
name = "{name}"
group = "{group}"
file-extensions = {file_extensions:?}
[languages.parser]
git = {{ url = "{url}", rev = "{rev}"{path} }}
external-scanner = {{ c = {external_c}, cpp = {external_cpp} }}
ffi-func = "tree_sitter_{name}"
rust-func = "language"
package = "{package}"
{crates_io}
[languages.queries]
nvim-like = true
injections = {queries_injections}
locals = {queries_locals}"###,
        path = match &path {
            Some(path) => format!(", path = \"{path}\""),
            None => String::new(),
        }
    ));
    langs.sort_unstable_by_key(|lang| {
        lang.split_once("name = \"")
            .unwrap()
            .1
            .split_once('"')
            .unwrap()
            .0
            .to_owned()
    });
    fs::write(&langs_toml_path, langs.join("\n\n"))?;

    let mut queries_lib = OpenOptions::new()
        .append(true)
        .open(crate::WORKSPACE_DIR.join("syntastica-queries/src/lib.rs"))?;
    write!(
        queries_lib,
        r###"
pub const {name}_HIGHLIGHTS: &str = "";
pub const {name}_INJECTIONS: &str = "";
pub const {name}_LOCALS: &str = "";
pub const {name}_HIGHLIGHTS_CRATES_IO: &str = "";
pub const {name}_INJECTIONS_CRATES_IO: &str = "";
pub const {name}_LOCALS_CRATES_IO: &str = "";
"###,
        name = name.to_uppercase()
    )?;

    let mut example_programs_toml = OpenOptions::new()
        .append(true)
        .open(crate::WORKSPACE_DIR.join("examples/example_programs.toml"))?;
    writeln!(example_programs_toml, "\n{name} = '''\n'''")?;

    Ok(())
}

fn get_rev(url: &str) -> Result<String> {
    Ok(String::from_utf8(
        Command::new("git")
            .args(["ls-remote", url])
            .output()?
            .stdout,
    )?
    .lines()
    .next()
    .ok_or_else(|| anyhow!("output is empty"))?
    .replace("HEAD", "")
    .trim()
    .to_owned())
}

fn try_get_extensions(content_url: &str) -> Option<Vec<String>> {
    let json_str = reqwest::blocking::get(format!("{content_url}/package.json"))
        .ok()?
        .text()
        .ok()?;
    let json = serde_json::from_str::<Map<String, Value>>(&json_str).ok()?;
    let langs = json.get("tree-sitter")?.as_array()?;
    let mut file_extensions = vec![];
    if langs.len() > 1 {
        eprintln!("warning: 'package.json' contains information for multiple languages, all file extensions will be combined");
    }
    for lang in langs {
        let mut exts = lang
            .get("file-types")?
            .as_array()?
            .iter()
            .map(|val| val.as_str().map(ToOwned::to_owned))
            .collect::<Option<Vec<_>>>()?;
        eprintln!("{}: {exts:?}", lang.get("scope")?);
        file_extensions.append(&mut exts);
    }

    Some(file_extensions)
}

fn try_get_package(content_url: &str) -> Option<String> {
    let toml_str = reqwest::blocking::get(format!("{content_url}/Cargo.toml"))
        .ok()?
        .text()
        .ok()?;
    let toml = toml::from_str::<toml::map::Map<String, toml::Value>>(&toml_str).ok()?;
    Some(
        toml.get("package")?
            .as_table()?
            .get("name")?
            .as_str()?
            .to_owned(),
    )
}

fn try_get_crates_io_version(package: &str) -> Option<String> {
    match SyncClient::new(
        "syntastica xtask (github.com/RubixDev/syntastica)",
        Duration::from_secs(2),
    )
    .unwrap()
    .get_crate(package)
    {
        Ok(info) => Some(info.versions.first()?.num.clone()),
        Err(_) => None,
    }
}

fn forked_from(name: &str, file: &str, content: &str) -> String {
    format!(";; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/{name}/{file}.scm
;; Licensed under the Apache License 2.0
{content}")
}
