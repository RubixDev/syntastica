use std::{
    collections::VecDeque,
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use anyhow::{bail, Context, Result};

fn git(repo_dir: &Path) -> Command {
    let mut cmd = env::var_os("SYNTASTICA_PARSERS_GIT_PATH")
        .map_or_else(|| Command::new("git"), Command::new);
    cmd.current_dir(repo_dir)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());
    cmd
}

fn status_ok(name: &str, status: std::process::ExitStatus, what: &str) -> Result<()> {
    if status.success() {
        Ok(())
    } else {
        bail!("{what} failed for {name}: {status}")
    }
}

fn run_generate(name: &str, gen_cwd: &Path, timeout: &str) -> Result<()> {
    // Match the breeze behavior: try `npx tree-sitter generate` first, then fall back
    // to the globally installed `tree-sitter`.
    let npx = Command::new("timeout")
        .args([timeout, "npx", "tree-sitter", "generate"])
        .current_dir(gen_cwd)
        .status();

    if let Ok(status) = npx {
        if status.success() {
            return Ok(());
        }
    }

    let status = Command::new("timeout")
        .args([timeout, "tree-sitter", "generate"])
        .current_dir(gen_cwd)
        .status()
        .with_context(|| format!("failed to run tree-sitter generate for {name}"))?;

    status_ok(name, status, "tree-sitter generate")
}

#[derive(Clone)]
struct LangSpec {
    name: String,
    url: String,
    rev: String,
    path: Option<String>,
}

fn fetch_and_prepare(lang: &LangSpec, clone_dir: &Path, gen_timeout: &str) -> Result<()> {
    let name = &lang.name;

    let repo_dir = clone_dir.join(name).join(&lang.rev);
    if repo_dir.exists() {
        let git_dir = repo_dir.join(".git");
        // If cache exists but looks corrupted, remove it.
        if !git_dir.is_dir()
            || fs::read_dir(&repo_dir)
                .map(|mut d| d.next().is_none())
                .unwrap_or(true)
        {
            fs::remove_dir_all(&repo_dir)
                .with_context(|| format!("failed to remove corrupted cache for {name}"))?;
        }
    }

    if !repo_dir.exists() {
        println!("==> fetching grammar sources: {name}");
        fs::create_dir_all(&repo_dir)?;
        status_ok(name, git(&repo_dir).arg("init").status()?, "git init")?;
        status_ok(
            name,
            git(&repo_dir)
                .args(["remote", "add", "origin", &lang.url])
                .status()?,
            "git remote add",
        )?;
        status_ok(
            name,
            git(&repo_dir)
                .args(["fetch", "--depth=1", "origin", &lang.rev])
                .status()?,
            "git fetch",
        )?;
        status_ok(
            name,
            git(&repo_dir).args(["checkout", "FETCH_HEAD"]).status()?,
            "git checkout",
        )?;
    }

    let gen_cwd = match &lang.path {
        Some(p) => repo_dir.join(p),
        None => repo_dir.clone(),
    };

    let src_dir = gen_cwd.join("src");
    let parser_c = src_dir.join("parser.c");
    if parser_c.is_file() {
        return Ok(());
    }

    let grammar_js = gen_cwd.join("grammar.js");
    if !grammar_js.is_file() {
        bail!(
            "missing src/parser.c for {name} and cannot generate because grammar.js is missing at {}",
            grammar_js.display()
        );
    }

    println!("==> generating parser.c: {name}");
    run_generate(name, &gen_cwd, gen_timeout)?;

    if !src_dir.is_dir() {
        bail!("tree-sitter generate did not create src/ for {name}");
    }
    if !parser_c.is_file() {
        bail!("tree-sitter generate did not produce src/parser.c for {name}");
    }

    Ok(())
}

pub fn run() -> Result<()> {
    let mut jobs: Option<usize> = None;

    let mut args = env::args().skip(2);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--jobs" | "-j" => {
                let value = args.next().with_context(|| "missing value for --jobs")?;
                jobs = Some(value.parse::<usize>().context("invalid --jobs value")?);
            }
            "--help" | "-h" => {
                println!(
                    "{}",
                    r###"
Usage: cargo xtask build-grammars [-j N]

Fetch all grammar repos into SYNTASTICA_PARSERS_CLONE_DIR and run `tree-sitter generate`
when src/parser.c is missing.
"###
                    .trim()
                );
                return Ok(());
            }
            other => bail!("unknown arg '{other}' (try --help)"),
        }
    }

    let clone_dir = env::var("SYNTASTICA_PARSERS_CLONE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| crate::WORKSPACE_DIR.join(".cache/syntastica/parsers/clone"));

    let gen_timeout = env::var("SYNTASTICA_GRAMMAR_GENERATE_TIMEOUT")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "10m".to_string());

    fs::create_dir_all(&clone_dir)
        .with_context(|| format!("failed to create clone dir {}", clone_dir.display()))?;

    let mut specs = Vec::new();
    for lang in crate::LANGUAGE_CONFIG.languages.iter() {
        specs.push(LangSpec {
            name: lang.name.clone(),
            url: lang.parser.git.url.clone(),
            rev: lang.parser.git.rev.clone(),
            path: lang.parser.git.path.clone(),
        });
    }
    specs.sort_by(|a, b| a.name.cmp(&b.name));

    let total = specs.len();
    let jobs = jobs.unwrap_or_else(|| {
        thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    });

    println!("Priming {total} grammars with {jobs} jobs...");

    let queue = Arc::new(Mutex::new(VecDeque::from(specs)));
    let (tx, rx) = mpsc::channel::<Result<String>>();

    for _ in 0..jobs.min(total.max(1)) {
        let queue = Arc::clone(&queue);
        let tx = tx.clone();
        let clone_dir = clone_dir.clone();
        let gen_timeout = gen_timeout.clone();
        thread::spawn(move || loop {
            let next = { queue.lock().ok().and_then(|mut q| q.pop_front()) };
            let Some(lang) = next else { break };

            let name = lang.name.clone();
            let res = fetch_and_prepare(&lang, &clone_dir, &gen_timeout).map(|_| name);
            let _ = tx.send(res);
        });
    }
    drop(tx);

    let mut done = 0usize;
    let mut failures = Vec::new();

    for res in rx {
        done += 1;
        match res {
            Ok(name) => {
                println!("  [{done}/{total}] {name}");
            }
            Err(err) => {
                println!("  [{done}/{total}] ERROR: {err:#}");
                failures.push(err);
            }
        }
    }

    if !failures.is_empty() {
        bail!("failed to prime {} grammars", failures.len());
    }

    Ok(())
}
