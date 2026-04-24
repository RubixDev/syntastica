use std::{env, fs, path::PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use lazy_regex::regex_captures;
use once_cell::sync::Lazy;
use serde::Deserialize;

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
            let res = reqwest::blocking::get(&url).with_context(|| "query request failed")?;

            let query = match res.status().is_success() {
                true => res.text()?,
                false => {
                    // If github and 404, try flipping master <-> main branch
                    if host == "github" && res.status() == 404 {
                        let fallback_branch = if branch == "master" { "main" } else { "master" };
                        let fallback_url = format!("https://raw.githubusercontent.com/{user}/{repo}/{fallback_branch}/{path}");
                        println!("trying fallback: {fallback_url}");
                        let fallback_res = reqwest::blocking::get(&fallback_url)
                            .with_context(|| "fallback query request failed")?;
                        if fallback_res.status().is_success() {
                            fallback_res.text()?
                        } else {
                            bail!(
                                "query request returned non-success status code: {} (also tried fallback: {})",
                                res.status(),
                                fallback_res.status()
                            )
                        }
                    } else {
                        bail!(
                            "query request returned non-success status code: {}",
                            res.status()
                        )
                    }
                }
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

fn forked_from_url(url: &str, content: &str) -> String {
    format!(";; Forked from {url}\n{content}")
}

pub fn fetch_query(name: &str, kind: &str) -> Result<Option<String>> {
    if kind == "highlights" {
        if let Some(query) = read_mapped_highlights(name)? {
            return Ok(Some(query));
        }
    } else if kind == "injections" || kind == "locals" {
        if let Some(query) = read_mapped_sibling_local(name, kind)? {
            return Ok(Some(query));
        }
    }

    if let Some(query) = read_local_query(name, kind)? {
        return Ok(Some(forked_from(
            name,
            kind,
            &format_query(&query).context("failed to parse downloaded queries")?,
        )));
    }

    // Try both old and new URL patterns
    let urls = [
        format!("https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/main/runtime/queries/{name}/{kind}.scm"),
        format!("https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/master/queries/{name}/{kind}.scm"),
    ];

    for url in urls {
        if let Ok(res) = reqwest::blocking::get(&url) {
            if res.status().is_success() {
                if let Ok(query) = res.text() {
                    return Ok(Some(forked_from(
                        name,
                        kind,
                        &format_query(&query).context("failed to parse downloaded queries")?,
                    )));
                }
            }
        }
    }

    Ok(None)
}

fn format_query(query: &str) -> Result<String> {
    Ok(format!(
        "{:#}",
        rsexpr::from_slice_multi(query).map_err(|errs| anyhow!(errs
            .into_iter()
            .map(|err| err.to_string())
            .collect::<Vec<_>>()
            .join(", ")))?
    ))
}

fn read_local_query(name: &str, kind: &str) -> Result<Option<String>> {
    let Some(root) = local_nvim_treesitter_root() else {
        return Ok(None);
    };

    let runtime_path = root
        .join("runtime/queries")
        .join(name)
        .join(format!("{kind}.scm"));
    if runtime_path.exists() {
        return Ok(Some(fs::read_to_string(runtime_path)?));
    }

    let legacy_path = root.join("queries").join(name).join(format!("{kind}.scm"));
    if legacy_path.exists() {
        return Ok(Some(fs::read_to_string(legacy_path)?));
    }

    Ok(None)
}

fn local_nvim_treesitter_root() -> Option<PathBuf> {
    if let Ok(path) = env::var("NVIM_TREESITTER_PATH").or_else(|_| env::var("NVIM_TREESITTER_REPO"))
    {
        let root = PathBuf::from(path);
        if root.exists() {
            return Some(root);
        }
    }
    let home = env::var("HOME").ok()?;
    let root = PathBuf::from(home).join("github/nvim-treesitter/nvim-treesitter");
    if root.exists() {
        Some(root)
    } else {
        None
    }
}

#[derive(Debug, Deserialize, Clone)]
struct GrammarMapping {
    grammar: String,
    #[serde(default)]
    grammar_repo: Option<String>,
    grammar_rev: String,
    #[serde(default)]
    nvim_filetype: Option<String>,
    #[serde(default)]
    effective_filetype: Option<String>,
    #[serde(default)]
    highlights_scm_path: Option<String>,
    #[serde(default)]
    highlights_scm_repo: Option<String>,
    #[serde(default)]
    highlights_scm_ref: Option<String>,
}

fn read_mapped_highlights(name: &str) -> Result<Option<String>> {
    let Some(mapping) = find_grammar_mapping(name)? else {
        return Ok(None);
    };
    let Some(path) = mapping
        .highlights_scm_path
        .as_deref()
        .map(str::trim)
        .filter(|p| !p.is_empty())
    else {
        return Ok(None);
    };

    if let Some(text) = read_mapped_file_local(&mapping, path)? {
        return Ok(Some(text));
    }

    if let Some(text) = read_mapped_highlights_remote(&mapping, path)? {
        return Ok(Some(text));
    }

    Ok(None)
}

fn read_mapped_sibling_local(name: &str, kind: &str) -> Result<Option<String>> {
    let Some(mapping) = find_grammar_mapping(name)? else {
        return Ok(None);
    };
    let Some(path) = mapping
        .highlights_scm_path
        .as_deref()
        .map(str::trim)
        .filter(|p| !p.is_empty())
    else {
        return Ok(None);
    };

    let sibling_path = sibling_query_path(path, kind);
    read_mapped_file_local(&mapping, &sibling_path)
}

fn sibling_query_path(highlights_path: &str, kind: &str) -> String {
    let mut path = PathBuf::from(highlights_path);
    path.set_file_name(format!("{kind}.scm"));
    path.to_string_lossy().into_owned()
}

fn read_mapped_file_local(mapping: &GrammarMapping, path: &str) -> Result<Option<String>> {
    let Some(root) = local_grammar_root() else {
        return Ok(None);
    };

    let rel_path = path.trim_start_matches('/');
    let repo_root = root.join(&mapping.grammar);
    let mut candidates = Vec::new();
    candidates.push(repo_root.join(rel_path));
    if !mapping.grammar_rev.trim().is_empty() {
        candidates.push(repo_root.join(&mapping.grammar_rev).join(rel_path));
    }

    for candidate in candidates {
        if candidate.is_file() {
            let content = fs::read_to_string(&candidate)
                .with_context(|| format!("failed reading {}", candidate.display()))?;
            let formatted = format_query(&content).context("failed to parse downloaded queries")?;
            let url = mapped_highlights_url(mapping, rel_path)
                .unwrap_or_else(|| format!("file://{}", candidate.to_string_lossy().into_owned()));
            return Ok(Some(forked_from_url(&url, &formatted)));
        }
    }

    Ok(None)
}

fn read_mapped_highlights_remote(mapping: &GrammarMapping, path: &str) -> Result<Option<String>> {
    let Some(url) = mapped_highlights_url(mapping, path.trim_start_matches('/')) else {
        return Ok(None);
    };

    let res = match reqwest::blocking::get(&url) {
        Ok(res) => res,
        Err(_) => return Ok(None),
    };
    if !res.status().is_success() {
        return Ok(None);
    }
    let query = res.text()?;
    let formatted = format_query(&query).context("failed to parse downloaded queries")?;
    Ok(Some(forked_from_url(&url, &formatted)))
}

fn mapped_highlights_url(mapping: &GrammarMapping, rel_path: &str) -> Option<String> {
    let (repo, rev) = match mapping.highlights_scm_repo.as_ref() {
        Some(repo) => {
            let Some(rev) = mapping
                .highlights_scm_ref
                .as_deref()
                .map(str::trim)
                .filter(|r| !r.is_empty())
            else {
                return None;
            };
            (repo, rev)
        }
        None => (mapping.grammar_repo.as_ref()?, mapping.grammar_rev.as_str()),
    };
    let content_url = crate::add_lang::url_to_content_url(repo, rev)?;
    Some(format!(
        "{}/{}",
        content_url.trim_end_matches('/'),
        rel_path.trim_start_matches('/')
    ))
}

fn find_grammar_mapping(name: &str) -> Result<Option<&'static GrammarMapping>> {
    let mappings = grammar_mappings()?;
    let name = name.trim();
    if name.is_empty() {
        return Ok(None);
    }

    let mut candidates = vec![name.to_string()];
    if let Some(stripped) = name.strip_suffix("_only") {
        candidates.push(stripped.to_string());
    }

    for candidate in candidates {
        let candidate_sanitized = sanitize_lang_name(&candidate);
        if let Some(mapping) = mappings.iter().find(|m| {
            m.grammar == candidate
                || sanitize_lang_name(&m.grammar) == candidate_sanitized
                || m.effective_filetype.as_deref() == Some(candidate.as_str())
                || m.nvim_filetype.as_deref() == Some(candidate.as_str())
        }) {
            return Ok(Some(mapping));
        }
    }

    Ok(None)
}

fn grammar_mappings_path() -> PathBuf {
    crate::WORKSPACE_DIR.join("grammars-mapping-filetypes.json")
}

fn grammar_mappings() -> Result<&'static [GrammarMapping]> {
    static MAPPINGS: Lazy<Result<Vec<GrammarMapping>>> = Lazy::new(|| {
        let path = grammar_mappings_path();
        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read grammar mappings at {}", path.display()))?;
        let mappings: Vec<GrammarMapping> =
            serde_json::from_str(&content).context("invalid grammar mappings json")?;
        Ok(mappings)
    });
    match MAPPINGS.as_ref() {
        Ok(mappings) => Ok(mappings.as_slice()),
        Err(err) => Err(anyhow!("unable to load grammar mappings: {err:?}")),
    }
}

fn local_grammar_root() -> Option<PathBuf> {
    if let Ok(path) = env::var("SYNTASTICA_PARSERS_CLONE_DIR") {
        let root = PathBuf::from(path);
        if root.exists() {
            return Some(root);
        }
    }
    None
}

fn sanitize_lang_name(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    if out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    out
}
