use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    env, fs,
    path::{Path, PathBuf},
    sync::{mpsc, Arc, Mutex},
    thread,
};

use anyhow::{bail, Context, Result};
use lazy_regex::{regex_captures, regex_replace};
use serde::Deserialize;
use serde_json::Value as JsonValue;

use crate::{add_lang, fetch_queries, schema, WORKSPACE_DIR};

#[derive(Debug, Deserialize, Clone)]
struct GrammarMapping {
    grammar: String,
    #[serde(default)]
    grammar_repo: Option<String>,
    grammar_rev: String,
    #[serde(default)]
    grammar_branch: Option<String>,
    #[serde(default)]
    helix_file_types: Option<Vec<JsonValue>>,
    #[serde(default)]
    nvim_filetype: Option<String>,
    #[serde(default)]
    effective_filetype: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GithubContentEntry {
    name: String,
    #[serde(rename = "type")]
    kind: String,
}

pub fn run() -> Result<()> {
    let mut dry_run = false;
    let mut limit: Option<usize> = None;
    let mut jobs: Option<usize> = None;
    let mut update_existing = false;

    let mut args = env::args().skip(2);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--dry-run" => dry_run = true,
            "--update-existing" => update_existing = true,
            "--jobs" | "-j" => {
                let value = args.next().with_context(|| "missing value for --jobs")?;
                jobs = Some(value.parse::<usize>().context("invalid --jobs value")?);
            }
            "--limit" => {
                let value = args.next().with_context(|| "missing value for --limit")?;
                limit = Some(value.parse::<usize>().context("invalid --limit value")?);
            }
            "--help" | "-h" => {
                println!(
                    "{}",
                    r###"
Usage: cargo xtask sync-breeze [--dry-run] [--limit N]
       cargo xtask sync-breeze [--jobs N] [--update-existing]

Reads ../breeze-tree-sitter-parsers/grammars.json and adds missing languages to
syntastica-macros/languages.toml, but only when nvim-treesitter provides a
runtime query directory with a highlights.scm for the language.
"###
                    .trim()
                );
                return Ok(());
            }
            other => bail!("unknown arg '{other}' (try --help)"),
        }
    }

    let grammar_mappings_list = load_grammar_mappings()?;
    let grammar_mappings: HashMap<String, GrammarMapping> = grammar_mappings_list
        .iter()
        .cloned()
        .map(|m| (m.grammar.clone(), m))
        .collect();

    let nvim_query_dirs = fetch_nvim_runtime_query_dirs()?;

    let langs_toml_path = WORKSPACE_DIR.join("syntastica-macros/languages.toml");
    let langs_toml = fs::read_to_string(&langs_toml_path)?;
    let existing_cfg: schema::LanguageConfig =
        toml::from_str(&langs_toml).expect("invalid `languages.toml`");
    let existing_names = existing_cfg
        .languages
        .iter()
        .map(|l| l.name.clone())
        .collect::<HashSet<_>>();
    let mut reserved_names = existing_names.clone();

    let mut to_add = Vec::new();
    let mut to_update: HashMap<String, GrammarMapping> = HashMap::new();
    for mapping in grammar_mappings_list.into_iter() {
        let Some(repo) = mapping.grammar_repo.clone() else {
            continue;
        };
        if mapping.grammar_rev.trim().is_empty() {
            bail!(
                "sync-breeze: grammar_rev is required but missing for {}",
                mapping.grammar
            );
        }
        if should_skip_grammar(&mapping.grammar) {
            continue;
        }
        if let Some(limit) = limit {
            if to_add.len() >= limit {
                break;
            }
        }

        let target_name = canonical_syntastica_name(&mapping.grammar, &grammar_mappings);
        if reserved_names.contains(&target_name) {
            if update_existing {
                to_update.insert(target_name.clone(), mapping.clone());
            }
            println!(
                "sync-breeze: skipping {} (target name '{}' already exists)",
                mapping.grammar, target_name
            );
            continue;
        }
        reserved_names.insert(target_name.clone());

        let Some(nvim_name) = resolve_nvim_query_name(
            &mapping.grammar,
            &target_name,
            &nvim_query_dirs,
            &grammar_mappings,
        ) else {
            continue;
        };

        to_add.push((
            MappedGrammar {
                name: mapping.grammar,
                repo,
                path: None,
                rev: mapping.grammar_rev.clone(),
                branch: mapping.grammar_branch.clone(),
            },
            target_name,
            nvim_name,
        ));
    }

    if to_add.is_empty() {
        if !update_existing || to_update.is_empty() {
            println!("sync-breeze: nothing to do");
            return Ok(());
        }
    }

    if !to_add.is_empty() {
        println!("sync-breeze: adding {} languages", to_add.len());
    }

    let mut new_blocks: Vec<(String, String)> = Vec::new();
    if !to_add.is_empty() {
        let total_count = to_add.len();
        let job_count = jobs
            .unwrap_or_else(default_job_count)
            .max(1)
            .min(total_count);
        let queue = Arc::new(Mutex::new(VecDeque::from(to_add)));
        let (result_tx, result_rx) = mpsc::channel();
        let mappings = Arc::new(grammar_mappings.clone());

        for _ in 0..job_count {
            let queue = Arc::clone(&queue);
            let result_tx = result_tx.clone();
            let mappings = Arc::clone(&mappings);
            thread::spawn(move || loop {
                let item = {
                    let mut guard = queue.lock().expect("sync-breeze queue poisoned");
                    guard.pop_front()
                };
                let Some((grammar, target_name, nvim_name)) = item else {
                    break;
                };

                let result =
                    build_language_block(&grammar, &target_name, &nvim_name, &mappings, dry_run)
                        .map(|block| (target_name, block));

                if result_tx.send(result).is_err() {
                    break;
                }
            });
        }
        drop(result_tx);

        let mut errors: Vec<anyhow::Error> = Vec::new();
        for _ in 0..total_count {
            match result_rx.recv() {
                Ok(Ok(item)) => new_blocks.push(item),
                Ok(Err(err)) => errors.push(err),
                Err(err) => bail!("sync-breeze worker hung up: {err}"),
            }
        }

        if !errors.is_empty() {
            let first = errors.remove(0);
            bail!(
                "sync-breeze failed with {} error(s); first error: {first:?}",
                errors.len() + 1
            );
        }
    }

    if dry_run {
        println!(
            "sync-breeze: dry-run, not writing {}",
            langs_toml_path.display()
        );
        return Ok(());
    }

    let mut blocks = langs_toml
        .split("\n\n")
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    if update_existing && !to_update.is_empty() {
        println!("sync-breeze: updating {} languages", to_update.len());
        let mut update_jobs = Vec::new();
        for (idx, block) in blocks.iter().enumerate() {
            let Some(name) = extract_lang_name(block) else {
                continue;
            };
            let Some(mapping) = to_update.get(name) else {
                continue;
            };
            let Some(repo) = mapping.grammar_repo.as_ref() else {
                continue;
            };
            let path = extract_git_path(block);
            update_jobs.push((
                idx,
                name.to_string(),
                repo.clone(),
                path,
                block.clone(),
                mapping.grammar_rev.clone(),
                mapping.grammar_branch.clone(),
                mapping.grammar.clone(),
            ));
        }

        if !update_jobs.is_empty() {
            let total = update_jobs.len();
            let job_count = jobs.unwrap_or_else(default_job_count).max(1).min(total);
            let queue = Arc::new(Mutex::new(VecDeque::from(update_jobs)));
            let (result_tx, result_rx) = mpsc::channel();

            for _ in 0..job_count {
                let queue = Arc::clone(&queue);
                let result_tx = result_tx.clone();
                thread::spawn(move || loop {
                    let item = {
                        let mut guard = queue.lock().expect("sync-breeze update queue poisoned");
                        guard.pop_front()
                    };
                    let Some((idx, name, repo, path, block, rev, branch, query_name)) = item else {
                        break;
                    };
                    let result = update_existing_block(
                        &block,
                        &repo,
                        path.as_deref(),
                        &name,
                        &rev,
                        branch.as_deref(),
                    )
                    .and_then(|updated| {
                        refresh_highlights_query(&name, &query_name, dry_run)?;
                        Ok((idx, name, updated))
                    });
                    if result_tx.send(result).is_err() {
                        break;
                    }
                });
            }
            drop(result_tx);

            let mut updated = 0usize;
            for _ in 0..total {
                match result_rx.recv() {
                    Ok(Ok((idx, name, updated_block))) => {
                        updated += 1;
                        println!("sync-breeze: updating {name} ({updated}/{total})");
                        blocks[idx] = updated_block;
                    }
                    Ok(Err(err)) => return Err(err),
                    Err(err) => bail!("sync-breeze update worker hung up: {err}"),
                }
            }
        }
    }

    blocks.extend(new_blocks.into_iter().map(|(_, block)| block));
    blocks.sort_unstable_by_key(|block| extract_lang_name(block).unwrap_or_default().to_owned());
    fs::write(&langs_toml_path, blocks.join("\n\n"))?;

    Ok(())
}

fn grammar_mappings_path() -> Result<PathBuf> {
    Ok(WORKSPACE_DIR.join("grammars-mapping-filetypes.json"))
}

fn load_grammar_mappings() -> Result<Vec<GrammarMapping>> {
    let path = grammar_mappings_path()?;
    let content = fs::read_to_string(&path)
        .with_context(|| format!("failed to read grammar mappings at {}", path.display()))?;
    let mappings: Vec<GrammarMapping> =
        serde_json::from_str(&content).context("invalid grammar mappings json")?;
    Ok(mappings)
}

fn get_file_types(
    grammar_name: &str,
    mappings: &HashMap<String, GrammarMapping>,
) -> Result<Option<Vec<String>>> {
    let Some(mapping) = mappings.get(grammar_name) else {
        return Ok(None);
    };

    // `file-types` in syntastica is a list of `palate::FileType` strings.
    //
    // effective_filetype is canonical. helix_file_types are aliases which contribute to the union
    // (resolved via extension detection) but must not influence query resolution or create
    // separate query dirs.
    let effective = mapping
        .effective_filetype
        .as_deref()
        .map(str::trim)
        .filter(|ft| !ft.is_empty())
        .map(|ft| {
            ft.parse::<palate::FileType>().map_err(|err| {
                anyhow::anyhow!(
                    "sync-breeze: invalid effective_filetype {:?} for grammar {:?}: {err}",
                    ft,
                    grammar_name
                )
            })
        })
        .transpose()?
        .map(|ft| ft.to_string());

    let mut extra = BTreeSet::<String>::new();
    if let Some(file_types) = &mapping.helix_file_types {
        for value in file_types.iter().filter_map(|v| v.as_str()) {
            let ext = value.trim().trim_start_matches('.');
            if ext.is_empty() {
                continue;
            }
            if let Some(ft) = palate::try_detect(Path::new(&format!("file.{ext}")), "") {
                let ft = ft.to_string();
                if effective.as_ref().is_some_and(|e| e == &ft) {
                    continue;
                }
                extra.insert(ft);
            }
        }
    }

    let mut out = Vec::<String>::new();
    if let Some(effective) = effective {
        out.push(format!("\"{effective}\""));
    }
    out.extend(extra.into_iter().map(|ft| format!("\"{ft}\"")));

    Ok((!out.is_empty()).then_some(out))
}

fn fetch_nvim_runtime_query_dirs() -> Result<std::collections::BTreeSet<String>> {
    if let Some(dirs) = fetch_nvim_runtime_query_dirs_local() {
        return Ok(dirs);
    }

    let url = "https://api.github.com/repos/nvim-treesitter/nvim-treesitter/contents/runtime/queries?ref=main";
    let client = reqwest::blocking::Client::builder()
        .user_agent("syntastica xtask (sync-breeze)")
        .build()
        .context("failed to build http client")?;
    let res = client
        .get(url)
        .send()
        .context("failed to query github api")?;
    if !res.status().is_success() {
        bail!("github api returned {}", res.status());
    }
    let entries: Vec<GithubContentEntry> =
        res.json().context("failed to parse github api response")?;
    Ok(entries
        .into_iter()
        .filter(|e| e.kind == "dir")
        .map(|e| e.name)
        .collect())
}

fn fetch_nvim_runtime_query_dirs_local() -> Option<BTreeSet<String>> {
    let root = env::var("NVIM_TREESITTER_PATH")
        .ok()
        .or_else(|| env::var("NVIM_TREESITTER_REPO").ok())
        .map(PathBuf::from)
        .or_else(|| {
            let home = env::var("HOME").ok()?;
            Some(Path::new(&home).join("github/nvim-treesitter/nvim-treesitter"))
        })?;

    let queries_dir = root.join("runtime/queries");
    let entries = fs::read_dir(&queries_dir).ok()?;
    let mut dirs = BTreeSet::new();
    for entry in entries {
        let entry = entry.ok()?;
        let file_type = entry.file_type().ok()?;
        if !file_type.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        dirs.insert(name);
    }
    if dirs.is_empty() {
        None
    } else {
        Some(dirs)
    }
}

fn canonical_syntastica_name(
    breeze_name: &str,
    mappings: &HashMap<String, GrammarMapping>,
) -> String {
    // First check the grammar mappings for effective_filetype
    if let Some(mapping) = mappings.get(breeze_name) {
        if let Some(effective) = &mapping.effective_filetype {
            if is_valid_lang_name(effective) {
                return effective.clone();
            }
        }
    }

    // Fallback to hardcoded mappings
    match sanitize_lang_name(breeze_name).as_str() {
        "csharp" => "c_sharp".to_owned(),
        other => other.to_owned(),
    }
}

fn is_valid_lang_name(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if first.is_ascii_digit() {
        return false;
    }
    if !(first.is_ascii_lowercase() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
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

fn resolve_nvim_query_name(
    breeze_name: &str,
    syntastica_name: &str,
    nvim_dirs: &std::collections::BTreeSet<String>,
    mappings: &HashMap<String, GrammarMapping>,
) -> Option<String> {
    // First check grammar mappings for nvim_filetype
    if let Some(mapping) = mappings.get(breeze_name) {
        if let Some(nvim_ft) = &mapping.nvim_filetype {
            if nvim_dirs.contains(nvim_ft) {
                return Some(nvim_ft.clone());
            }
        }
    }

    // Fallback to candidate names
    let candidates = [
        syntastica_name.to_owned(),
        breeze_name.to_owned(),
        breeze_name.replace('-', "_"),
    ];
    candidates.into_iter().find(|name| nvim_dirs.contains(name))
}

fn should_skip_grammar(name: &str) -> bool {
    matches!(name, "fsharp_signature")
}

fn default_job_count() -> usize {
    let detected = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    detected.min(8)
}

struct MappedGrammar {
    name: String,
    repo: String,
    path: Option<String>,
    rev: String,
    branch: Option<String>,
}

fn build_language_block(
    grammar: &MappedGrammar,
    target_name: &str,
    nvim_name: &str,
    mappings: &HashMap<String, GrammarMapping>,
    dry_run: bool,
) -> Result<String> {
    println!(">>> {target_name} (queries: {nvim_name})");

    let (external_c, external_cpp, generate) =
        detect_parser_metadata(&grammar.repo, &grammar.rev, grammar.path.as_deref())?;
    let ffi_func = add_lang::detect_ffi_func(&grammar.repo, &grammar.rev, grammar.path.as_deref())?
        .unwrap_or_else(|| format!("tree_sitter_{target_name}"));
    let wasm_unknown = !external_cpp;

    let (queries_injections, queries_locals) = write_queries(target_name, nvim_name, dry_run)?;
    let file_types = get_file_types(&grammar.name, mappings)?;

    Ok(format_language_block(
        target_name,
        &grammar.repo,
        &grammar.rev,
        grammar.branch.as_deref(),
        grammar.path.as_deref(),
        external_c,
        external_cpp,
        &ffi_func,
        wasm_unknown,
        generate,
        queries_injections,
        queries_locals,
        file_types,
    ))
}

fn detect_parser_metadata(url: &str, rev: &str, path: Option<&str>) -> Result<(bool, bool, bool)> {
    let content_url = add_lang::url_to_content_url(url, rev);
    let path_in_url = match path {
        Some(path) => format!("/{path}"),
        None => String::new(),
    };

    let external_c = content_url.as_ref().is_some_and(|base| {
        reqwest::blocking::get(format!("{base}{path_in_url}/src/scanner.c"))
            .is_ok_and(|response| response.status().is_success())
    });
    let external_cpp = content_url.as_ref().is_some_and(|base| {
        reqwest::blocking::get(format!("{base}{path_in_url}/src/scanner.cc"))
            .is_ok_and(|response| response.status().is_success())
    });

    let generate = content_url.as_ref().is_some_and(|base| {
        reqwest::blocking::get(format!("{base}{path_in_url}/src/parser.c"))
            .ok()
            .is_some_and(|response| response.status() == reqwest::StatusCode::NOT_FOUND)
    });

    Ok((external_c, external_cpp, generate))
}

fn write_queries(target_name: &str, nvim_name: &str, dry_run: bool) -> Result<(bool, bool)> {
    let mut queries_injections = false;
    let mut queries_locals = false;

    let out_dir = WORKSPACE_DIR.join(format!("queries/{target_name}"));
    if !dry_run {
        fs::create_dir_all(&out_dir)?;
    }

    let mut highlights = fetch_queries::fetch_query(nvim_name, "highlights")?
        .with_context(|| format!("missing highlights query for {nvim_name}"))?;
    if nvim_name == "sxhkdrc" {
        highlights = highlights
            .lines()
            .map(|line| {
                let trimmed = line.trim();
                if trimmed == r#"["\\n"] @punctuation.special"#
                    || trimmed == r#""\\n" @punctuation.special"#
                {
                    r#"["\n" "\\\n"] @punctuation.special"#.to_string()
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        if !highlights.ends_with('\n') {
            highlights.push('\n');
        }
    }
    highlights = postprocess_highlights_query(target_name, highlights);
    if !dry_run {
        fs::write(out_dir.join("highlights.scm"), highlights)?;
    }

    for kind in ["injections", "locals"] {
        let Some(text) = fetch_queries::fetch_query(nvim_name, kind)? else {
            continue;
        };
        if kind == "injections" {
            queries_injections = true;
        } else {
            queries_locals = true;
        }
        if !dry_run {
            fs::write(out_dir.join(format!("{kind}.scm")), text)?;
        }
    }

    Ok((queries_injections, queries_locals))
}

#[allow(clippy::too_many_arguments)]
fn format_language_block(
    name: &str,
    url: &str,
    rev: &str,
    branch: Option<&str>,
    path: Option<&str>,
    external_c: bool,
    external_cpp: bool,
    ffi_func: &str,
    wasm_unknown: bool,
    generate: bool,
    queries_injections: bool,
    queries_locals: bool,
    file_types: Option<Vec<String>>,
) -> String {
    let mut out = String::new();
    out.push_str("[[languages]]\n");
    out.push_str(&format!("name = \"{name}\"\n"));
    out.push_str("group = \"all\"\n");
    if let Some(types) = file_types {
        if types.is_empty() {
            out.push_str("file-types = []\n");
        } else {
            out.push_str(&format!("file-types = [{}]\n", types.join(", ")));
        }
    } else {
        out.push_str("file-types = []\n");
    }
    if !wasm_unknown {
        out.push_str("wasm-unknown = false\n");
    }
    out.push_str("[languages.parser]\n");
    let mut git_fields = format!("git = {{ url = \"{url}\", rev = \"{rev}\"");
    if let Some(branch) = branch {
        if !branch.trim().is_empty() {
            git_fields.push_str(&format!(", branch = \"{branch}\""));
        }
    }
    if let Some(path) = path {
        git_fields.push_str(&format!(", path = \"{path}\""));
    }
    git_fields.push_str(" }\n");
    out.push_str(&git_fields);
    out.push_str(&format!(
        "external-scanner = {{ c = {external_c}, cpp = {external_cpp} }}\n"
    ));
    out.push_str(&format!("ffi-func = \"{ffi_func}\"\n"));
    out.push_str(&format!(
        "package = \"tree-sitter-{}\"\n",
        name.replace('_', "-")
    ));
    if generate {
        out.push_str("generate = true\n");
    }
    out.push_str("[languages.queries]\n");
    out.push_str("nvim-like = true\n");
    out.push_str(&format!("injections = {queries_injections}\n"));
    out.push_str(&format!("locals = {queries_locals}\n"));
    out
}

fn extract_lang_name(block: &str) -> Option<&str> {
    let (_, rest) = block.split_once("name = \"")?;
    let (name, _) = rest.split_once('"')?;
    Some(name)
}

fn extract_git_path(block: &str) -> Option<String> {
    let captures = regex_captures!(r#"git\s*=\s*\{[^}]*\bpath\s*=\s*"([^"]+)""#, block)?;
    Some(captures.1.to_string())
}

fn update_existing_block(
    block: &str,
    repo_url: &str,
    path: Option<&str>,
    target_name: &str,
    rev: &str,
    branch: Option<&str>,
) -> Result<String> {
    let (external_c, external_cpp, generate) = detect_parser_metadata(repo_url, rev, path)?;
    let ffi_func = add_lang::detect_ffi_func(repo_url, rev, path)?
        .unwrap_or_else(|| format!("tree_sitter_{target_name}"));

    let mut block = block.to_string();
    block = regex_replace!(
        r#"^(\s*git\s*=\s*\{\s*url\s*=\s*")[^"]*("\s*,\s*rev\s*=\s*")[^"]*(".*\}\s*)(#.*)?$"#m,
        &block,
        |_, start, middle, end, comment| {
            format!("{start}{repo_url}{middle}{rev}{end}{comment}")
        },
    )
    .into_owned();
    if let Some(branch) = branch.map(str::trim).filter(|b| !b.is_empty()) {
        if block.contains("branch =") {
            block = regex_replace!(
                r#"^(\s*git\s*=\s*\{[^}]*\bbranch\s*=\s*")[^"]*(".*\}\s*)(#.*)?$"#m,
                &block,
                |_, start, end, comment| format!("{start}{branch}{end}{comment}"),
            )
            .into_owned();
        } else {
            block = regex_replace!(
                r#"^(\s*git\s*=\s*\{[^}]*)(\}\s*)(#.*)?$"#m,
                &block,
                |_, start, end, comment| format!("{start}, branch = \"{branch}\"{end}{comment}"),
            )
            .into_owned();
        }
    } else {
        block = regex_replace!(
            r#"^(\s*git\s*=\s*\{)\s*branch\s*=\s*"[^"]*"\s*,\s*"#m,
            &block,
            |_, start| format!("{start} ")
        )
        .into_owned();
        block = regex_replace!(r#",\s*branch\s*=\s*"[^"]*"\s*"#m, &block, |_| String::new())
            .into_owned();
    }
    block = regex_replace!(
        r#"^(\s*external-scanner\s*=\s*\{\s*c\s*=\s*)(?:true|false)(\s*,\s*cpp\s*=\s*)(?:true|false)(\s*\}\s*)(#.*)?$"#m,
        &block,
        |_, start, middle, end, comment| {
            format!("{start}{external_c}{middle}{external_cpp}{end}{comment}")
        },
    )
    .into_owned();
    block = regex_replace!(
        r#"^(\s*ffi-func\s*=\s*")[^"]*(")(\s*(?:#.*)?)$"#m,
        &block,
        |_, start, end_quote, tail| format!("{start}{ffi_func}{end_quote}{tail}"),
    )
    .into_owned();
    if block.contains("\ngenerate =") {
        block = regex_replace!(
            r#"^(\s*generate\s*=\s*)(?:true|false)(\s*(?:#.*)?)$"#m,
            &block,
            |_, start, tail| format!("{start}{generate}{tail}"),
        )
        .into_owned();
    }

    Ok(block)
}

fn refresh_highlights_query(target_name: &str, nvim_name: &str, dry_run: bool) -> Result<()> {
    let Some(mut text) = fetch_queries::fetch_query(nvim_name, "highlights")? else {
        bail!("missing highlights query for {target_name} (nvim: {nvim_name})");
    };
    if nvim_name == "sxhkdrc" {
        text = text
            .lines()
            .map(|line| {
                let trimmed = line.trim();
                if trimmed == r#"["\\n"] @punctuation.special"#
                    || trimmed == r#""\\n" @punctuation.special"#
                {
                    r#"["\n" "\\\n"] @punctuation.special"#.to_string()
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        if !text.ends_with('\n') {
            text.push('\n');
        }
    }
    text = postprocess_highlights_query(target_name, text);
    if dry_run {
        return Ok(());
    }
    let out_dir = WORKSPACE_DIR.join(format!("queries/{target_name}"));
    fs::create_dir_all(&out_dir)?;
    fs::write(out_dir.join("highlights.scm"), text)?;
    Ok(())
}

fn postprocess_highlights_query(target_name: &str, text: String) -> String {
    // Keep this intentionally minimal and targeted: sync-breeze updates highlights for existing
    // languages, which can wipe out small local tweaks. We apply known, stable fixups here.
    if target_name != "gleam" {
        return text;
    }

    const CRATES_IO_SKIP: &str = "; crates.io skip";
    const NON_CRATES_IO_SKIP: &str = "; non-crates.io skip";

    let input_has_bit_array = text
        .lines()
        .any(|l| l.trim_start().starts_with("(bit_array_segment_option)"));
    let input_has_bit_string = text
        .lines()
        .any(|l| l.trim_start().starts_with("(bit_string_segment_option)"));

    let mut out = Vec::<String>::new();
    fn ensure_skip(out: &mut Vec<String>, comment: &str) {
        let already = out
            .iter()
            .rev()
            .find(|l| !l.trim().is_empty())
            .is_some_and(|l| l.trim() == comment);
        if already {
            return;
        }
        if !out.last().is_some_and(|l| l.is_empty()) {
            out.push(String::new());
        }
        out.push(comment.to_string());
    }

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("(bit_array_segment_option) ")
            && trimmed.contains("@function.builtin")
        {
            ensure_skip(&mut out, CRATES_IO_SKIP);
            out.push("(bit_array_segment_option) @function.builtin".to_string());
            continue;
        }
        if trimmed.starts_with("(bit_string_segment_option) ")
            && trimmed.contains("@function.builtin")
        {
            ensure_skip(&mut out, NON_CRATES_IO_SKIP);
            out.push("(bit_string_segment_option) @function.builtin".to_string());
            continue;
        }
        out.push(line.to_string());
    }

    // If upstream removes one of these nodes, re-add it so codegen can still generate the
    // crates.io / non-crates.io variants as expected.
    if input_has_bit_array && !input_has_bit_string {
        let needle = "(bit_array_segment_option) @function.builtin";
        if let Some(idx) = out.iter().position(|l| l.trim() == needle) {
            let insert_at = idx + 1;
            let mut extra = Vec::new();
            extra.push(String::new());
            extra.push(NON_CRATES_IO_SKIP.to_string());
            extra.push("(bit_string_segment_option) @function.builtin".to_string());
            out.splice(insert_at..insert_at, extra);
        } else {
            out.push(String::new());
            out.push(NON_CRATES_IO_SKIP.to_string());
            out.push("(bit_string_segment_option) @function.builtin".to_string());
        }
    }

    let mut out = out.join("\n");
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}
