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
use lazy_regex::regex_captures;
use once_cell::sync::Lazy;
use reqwest::redirect::Policy;
use semver::{Version, VersionReq};
use toml::Table;
use url::{form_urlencoded, Url};

use crate::fetch_queries::fetch_query;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CratesIoLookup {
    Match(Option<String>),
    Mismatch,
    Unavailable,
}

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

    let rev =
        get_rev(&url, None).with_context(|| "unable to fetch latest revision of repository")?;

    let content_url = url_to_content_url(&url, &rev);
    let path_in_url = match &path {
        Some(path) => format!("/{path}"),
        None => String::new(),
    };

    println!("info: found revision '{rev}'");
    let external_c = content_url.as_ref().is_some_and(|url| {
        reqwest::blocking::get(format!("{url}{path_in_url}/src/scanner.c"))
            .is_ok_and(|response| response.status().is_success())
    });
    println!("info: found external C scanner: {external_c}");
    let external_cpp = content_url.as_ref().is_some_and(|url| {
        reqwest::blocking::get(format!("{url}{path_in_url}/src/scanner.cc"))
            .is_ok_and(|response| response.status().is_success())
    });
    println!("info: found external C++ scanner: {external_cpp}");

    let ffi_func = detect_ffi_func(&url, &rev, path.as_deref())?
        .unwrap_or_else(|| format!("tree_sitter_{name}"));

    let package = content_url
        .as_ref()
        .and_then(|url| try_get_package(url))
        .unwrap_or_else(|| format!("tree-sitter-{}", name.replace('_', "-")));
    println!("info: using package name '{package}'");

    let crates_io = match try_get_crates_io_version(&package, &url) {
        CratesIoLookup::Match(Some(version)) => format!("crates-io = \"{version}\""),
        _ => "# crates-io = \"\"".into(),
    };
    println!("info: found crates.io version: '{crates_io}'");

    let mut queries_injections = false;
    let mut queries_locals = false;
    fs::create_dir_all(crate::WORKSPACE_DIR.join(format!("queries/{name}")))?;
    fs::write(
        crate::WORKSPACE_DIR.join(format!("queries/{name}/highlights.scm")),
        "",
    )?;
    for kind in ["highlights", "injections", "locals"] {
        let queries = fetch_query(&name, kind)?;
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
file-types = []
[languages.parser]
git = {{ url = "{url}", rev = "{rev}"{path} }}
external-scanner = {{ c = {external_c}, cpp = {external_cpp} }}
ffi-func = "{ffi_func}"
rust-const = "LANGUAGE"
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

pub fn get_rev(url: &str, branch: Option<&str>) -> Result<String> {
    let mut cmd = Command::new("git");
    cmd.args(["ls-remote", url]);
    if let Some(branch) = branch {
        let branch = branch.trim();
        if !branch.is_empty() {
            if branch.starts_with("refs/") {
                cmd.arg(branch);
            } else {
                cmd.arg(format!("refs/heads/{branch}"));
            }
        }
    }
    let output = cmd.output()?.stdout;
    let output = String::from_utf8(output)?;
    let line = output
        .lines()
        .next()
        .ok_or_else(|| anyhow!("output is empty"))?;
    line.split_whitespace()
        .next()
        .map(str::to_string)
        .ok_or_else(|| anyhow!("unable to parse rev from output"))
}

static URL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"https:\/\/(github|gitlab)\.com\/([^\/]*)\/([^\/?#]*)").unwrap());

pub fn url_to_content_url(url: &str, rev: &str) -> Option<String> {
    match URL_REGEX.captures(url) {
        Ok(Some(groups)) => match &groups[1] {
            "github" => Some(format!(
                "https://raw.githubusercontent.com/{}/{}/{rev}",
                &groups[2],
                groups[3].strip_suffix(".git").unwrap_or(&groups[3]),
            )),
            "gitlab" => Some(format!(
                "https://gitlab.com/{}/{}/-/raw/{rev}",
                &groups[2],
                groups[3].strip_suffix(".git").unwrap_or(&groups[3]),
            )),
            _ => unreachable!("the regex only allows above options"),
        },
        _ => None,
    }
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

pub fn detect_ffi_func(url: &str, rev: &str, path: Option<&str>) -> Result<Option<String>> {
    let content_url = url_to_content_url(url, rev);
    let Some(content_url) = content_url else {
        return Ok(None);
    };

    let path_in_url = match path {
        Some(path) => format!("/{path}"),
        None => String::new(),
    };
    let base_url = format!("{content_url}{path_in_url}");

    let parser_url = format!("{base_url}/src/parser.c");
    let candidates = [
        "grammar.json",
        "src/grammar.json",
        "grammar.js",
        "src/grammar.js",
    ]
    .map(|candidate| format!("{base_url}/{candidate}"));

    let (parser_text, candidate_texts) =
        std::thread::scope(|scope| -> Result<(Option<String>, Vec<Option<String>>)> {
            let parser_handle = scope.spawn(|| fetch_raw(&parser_url));
            let candidate_handles = candidates.iter().map(|url| scope.spawn(|| fetch_raw(url)));

            let parser_text = parser_handle
                .join()
                .map_err(|_| anyhow!("ffi parser fetch thread panicked"))??;

            let mut candidate_texts = Vec::with_capacity(candidates.len());
            for handle in candidate_handles {
                let text = handle
                    .join()
                    .map_err(|_| anyhow!("ffi candidate fetch thread panicked"))??;
                candidate_texts.push(text);
            }
            Ok((parser_text, candidate_texts))
        })?;

    if let Some(text) = parser_text {
        if let Some(name) = detect_ffi_from_parser_c(&text) {
            return Ok(Some(name));
        }
    }

    for text in candidate_texts {
        if let Some(text) = text {
            if let Some(name) = detect_grammar_name(&text) {
                let normalized = normalize_grammar_name(&name);
                return Ok(Some(format!("tree_sitter_{normalized}")));
            }
        }
    }

    Ok(None)
}

fn fetch_raw(url: &str) -> Result<Option<String>> {
    let response = match reqwest::blocking::get(url) {
        Ok(response) => response,
        Err(_) => return Ok(None),
    };
    if !response.status().is_success() {
        return Ok(None);
    }
    Ok(Some(response.text()?))
}

fn detect_ffi_from_parser_c(text: &str) -> Option<String> {
    let captures = regex_captures!(
        r"(?:TS_PUBLIC\s+)?(?:extern\s+)?const\s+TSLanguage\s*\*\s*([A-Za-z0-9_]+)\s*\(",
        text
    )?;
    Some(captures.1.to_string())
}

fn detect_grammar_name(text: &str) -> Option<String> {
    if let Some(captures) = regex_captures!(r#""name"\s*:\s*"([^"]+)""#, text) {
        return Some(captures.1.to_string());
    }
    if let Some(captures) = regex_captures!(r#"name\s*:\s*['"]([^'"]+)['"]"#, text) {
        return Some(captures.1.to_string());
    }
    None
}

fn normalize_grammar_name(name: &str) -> String {
    name.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

static CRATES_IO_CLIENT: Lazy<SyncClient> = Lazy::new(|| {
    SyncClient::new(
        "syntastica xtask (github.com/RubixDev/syntastica)",
        Duration::from_millis(1200),
    )
    .unwrap()
});

static REPO_HTTP_CLIENT: Lazy<reqwest::blocking::Client> = Lazy::new(|| {
    reqwest::blocking::Client::builder()
        .user_agent("syntastica xtask (github.com/RubixDev/syntastica)")
        .redirect(Policy::limited(5))
        .timeout(Duration::from_secs(5))
        .build()
        .expect("failed to build http client")
});

static TREE_SITTER_VERSION: Lazy<Version> = Lazy::new(|| {
    Version::parse(
        toml::from_str::<Table>(
            &fs::read_to_string(crate::WORKSPACE_DIR.join("Cargo.toml")).unwrap(),
        )
        .unwrap()["workspace"]["dependencies"]
            .get("tree-sitter")
            .map(|ts_dep| match ts_dep.as_str() {
                Some(str) => str,
                None => ts_dep["version"].as_str().unwrap(),
            })
            .unwrap(),
    )
    .unwrap()
});

pub fn try_get_crates_io_version(package: &str, parser_url: &str) -> CratesIoLookup {
    let info = match CRATES_IO_CLIENT.get_crate(package) {
        Ok(info) => info,
        Err(_) => return CratesIoLookup::Unavailable,
    };
    let repo = match info.crate_data.repository.as_deref() {
        Some(repo) => repo,
        None => return CratesIoLookup::Mismatch,
    };
    if !repo_matches(parser_url, repo) {
        return CratesIoLookup::Mismatch;
    }
    let version = info.versions.first().map(|v| v.num.clone());
    match version {
        Some(version) => {
            if is_compatible_tree_sitter(package, &version) {
                CratesIoLookup::Match(Some(version))
            } else {
                CratesIoLookup::Match(None)
            }
        }
        None => CratesIoLookup::Match(None),
    }
}

fn repo_matches(parser_url: &str, crate_repo_url: &str) -> bool {
    let left = normalize_repo_id(parser_url);
    let right = normalize_repo_id(crate_repo_url);
    if left.is_some() && left == right {
        return true;
    }

    let left = resolve_repo_id(parser_url).or(left);
    let right = resolve_repo_id(crate_repo_url).or(right);
    match (left, right) {
        (Some(left), Some(right)) => left == right,
        _ => false,
    }
}

fn normalize_repo_id(url: &str) -> Option<String> {
    let mut raw = url.trim();
    if raw.is_empty() {
        return None;
    }
    if let Some(rest) = raw.strip_prefix("git+") {
        raw = rest;
    }

    let parsed = if raw.starts_with("git@") && raw.contains(':') {
        let mut ssh = raw.replacen("git@", "ssh://git@", 1);
        ssh = ssh.replacen(':', "/", 1);
        Url::parse(&ssh).ok()
    } else {
        Url::parse(raw).ok()
    };

    let parsed = parsed?;
    let mut host = parsed.host_str()?.to_lowercase();
    if let Some(stripped) = host.strip_prefix("www.") {
        host = stripped.to_string();
    }

    let mut segments: Vec<&str> = parsed
        .path()
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    if segments.is_empty() {
        return None;
    }
    if host == "api.github.com" {
        if segments.first() == Some(&"repos") {
            segments.remove(0);
        }
        host = "github.com".to_string();
    }

    if let Some(pos) = segments
        .iter()
        .position(|segment| matches!(*segment, "tree" | "blob" | "-"))
    {
        segments.truncate(pos);
    }

    if let Some(last) = segments.last_mut() {
        if let Some(stripped) = last.strip_suffix(".git") {
            *last = stripped;
        }
    }
    if segments.is_empty() {
        return None;
    }

    let max_segments = if host == "github.com" || host.ends_with(".github.com") {
        2
    } else {
        segments.len()
    };
    let segments = &segments[..segments.len().min(max_segments)];
    Some(format!("{host}/{}", segments.join("/").to_lowercase()))
}

fn resolve_repo_id(url: &str) -> Option<String> {
    let https = to_https_url(url)?;
    resolve_repo_id_via_redirect(&https).or_else(|| resolve_repo_id_via_api(&https))
}

fn resolve_repo_id_via_redirect(url: &str) -> Option<String> {
    let response = REPO_HTTP_CLIENT
        .head(url)
        .send()
        .or_else(|_| REPO_HTTP_CLIENT.get(url).send())
        .ok()?;
    normalize_repo_id(response.url().as_str())
}

fn resolve_repo_id_via_api(url: &str) -> Option<String> {
    let parsed = Url::parse(url).ok()?;
    let host = parsed.host_str()?.to_lowercase();
    if host.ends_with("github.com") {
        resolve_github_repo_id(&parsed)
    } else if host.ends_with("gitlab.com") {
        resolve_gitlab_repo_id(&parsed)
    } else {
        None
    }
}

#[derive(serde::Deserialize)]
struct GithubRepoInfo {
    html_url: String,
}

fn resolve_github_repo_id(url: &Url) -> Option<String> {
    let mut segments: Vec<&str> = url
        .path()
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    if segments.first() == Some(&"repos") {
        segments.remove(0);
    }
    if segments.len() < 2 {
        return None;
    }
    let owner = segments[0];
    let repo = segments[1].trim_end_matches(".git");
    let api_url = format!("https://api.github.com/repos/{owner}/{repo}");
    let response = REPO_HTTP_CLIENT.get(api_url).send().ok()?;
    if !response.status().is_success() {
        return None;
    }
    let info: GithubRepoInfo = response.json().ok()?;
    normalize_repo_id(&info.html_url)
}

#[derive(serde::Deserialize)]
struct GitlabProjectInfo {
    web_url: String,
}

fn resolve_gitlab_repo_id(url: &Url) -> Option<String> {
    let segments: Vec<&str> = url
        .path()
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    if segments.len() < 2 {
        return None;
    }
    let path = segments.join("/");
    let encoded: String = form_urlencoded::byte_serialize(path.as_bytes()).collect();
    let api_url = format!("https://{}/api/v4/projects/{encoded}", url.host_str()?);
    let response = REPO_HTTP_CLIENT.get(api_url).send().ok()?;
    if !response.status().is_success() {
        return None;
    }
    let info: GitlabProjectInfo = response.json().ok()?;
    normalize_repo_id(&info.web_url)
}

fn to_https_url(url: &str) -> Option<String> {
    let raw = url.trim();
    if raw.is_empty() {
        return None;
    }
    if let Some(rest) = raw.strip_prefix("git+") {
        return Some(rest.to_string());
    }
    if raw.starts_with("http://") || raw.starts_with("https://") {
        return Some(raw.to_string());
    }
    if raw.starts_with("git@") && raw.contains(':') {
        let mut ssh = raw.replacen("git@", "ssh://git@", 1);
        ssh = ssh.replacen(':', "/", 1);
        let parsed = Url::parse(&ssh).ok()?;
        let host = parsed.host_str()?;
        let path = parsed
            .path()
            .trim_start_matches('/')
            .trim_end_matches(".git");
        if path.is_empty() {
            return None;
        }
        return Some(format!("https://{host}/{path}"));
    }
    if raw.starts_with("ssh://") {
        let parsed = Url::parse(raw).ok()?;
        let host = parsed.host_str()?;
        let path = parsed
            .path()
            .trim_start_matches('/')
            .trim_end_matches(".git");
        if path.is_empty() {
            return None;
        }
        return Some(format!("https://{host}/{path}"));
    }
    None
}

fn is_compatible_tree_sitter(package: &str, version: &str) -> bool {
    match CRATES_IO_CLIENT.crate_dependencies(package, version) {
        Ok(deps) => deps.into_iter().any(|dep| {
            dep.crate_id == "tree-sitter-language"
                || (dep.crate_id == "tree-sitter"
                    && VersionReq::parse(&dep.req)
                        .is_ok_and(|req| req.matches(&TREE_SITTER_VERSION)))
        }),
        Err(_) => false,
    }
}
