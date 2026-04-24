use std::{
    env,
    fmt::Write as _,
    fs,
    io::{self, Write as IoWrite},
    thread,
};

use anyhow::{Context, Result};
use lazy_regex::regex_replace;

use crate::add_lang;

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
                    r#"
Usage: cargo xtask update-langs [--jobs N]
"#
                    .trim()
                );
                return Ok(());
            }
            other => return Err(anyhow::anyhow!("unknown arg '{other}' (try --help)")),
        }
    }

    let langs_toml_path = crate::WORKSPACE_DIR.join("syntastica-macros/languages.toml");
    let langs_toml = fs::read_to_string(&langs_toml_path)?;
    let blocks: Vec<String> = langs_toml.split("\n\n").map(ToString::to_string).collect();
    let langs = crate::LANGUAGE_CONFIG.languages.clone();
    let total = blocks.len().min(langs.len());
    let job_count = jobs.unwrap_or(1).max(1).min(total.max(1));

    let updated_blocks = if job_count == 1 {
        let mut out = Vec::with_capacity(total);
        for (toml, lang) in blocks.iter().take(total).zip(langs.iter()) {
            let result = update_lang_block(toml, lang)?;
            print!("{}", result.log);
            io::stdout().flush().ok();
            out.push(result.toml);
        }
        out
    } else {
        let (work_tx, work_rx) = flume::unbounded::<usize>();
        for idx in 0..total {
            work_tx
                .send(idx)
                .context("update-langs work channel closed")?;
        }
        drop(work_tx);

        let (event_tx, event_rx) = flume::unbounded::<UpdateEvent>();
        let mut out = vec![String::new(); total];
        let mut received = 0usize;

        thread::scope(|scope| -> Result<()> {
            let mut handles = Vec::with_capacity(job_count);
            for _ in 0..job_count {
                let work_rx = work_rx.clone();
                let event_tx = event_tx.clone();
                let blocks = &blocks;
                let langs = &langs;
                handles.push(scope.spawn(move || {
                    for idx in work_rx.iter() {
                        let name = langs[idx].name.clone();
                        if event_tx.send(UpdateEvent::Started { name }).is_err() {
                            break;
                        }
                        let result = update_lang_block(&blocks[idx], &langs[idx]);
                        if event_tx
                            .send(UpdateEvent::Finished { idx, result })
                            .is_err()
                        {
                            break;
                        }
                    }
                }));
            }
            drop(event_tx);

            for event in event_rx.iter() {
                match event {
                    UpdateEvent::Started { name } => {
                        println!("starting {name}");
                    }
                    UpdateEvent::Finished { idx, result } => match result {
                        Ok(updated) => {
                            print!("{}", updated.log);
                            io::stdout().flush().ok();
                            out[idx] = updated.toml;
                            received += 1;
                        }
                        Err(err) => return Err(err),
                    },
                }
            }

            for handle in handles {
                if handle.join().is_err() {
                    return Err(anyhow::anyhow!("update-langs worker thread panicked"));
                }
            }
            Ok(())
        })?;

        if received != total {
            return Err(anyhow::anyhow!(
                "update-langs workers ended early: received {received}/{total} results"
            ));
        }

        out
    };

    fs::write(&langs_toml_path, updated_blocks.join("\n\n"))?;

    Ok(())
}

struct UpdateLangResult {
    toml: String,
    log: String,
}

enum UpdateEvent {
    Started {
        name: String,
    },
    Finished {
        idx: usize,
        result: Result<UpdateLangResult>,
    },
}

fn update_lang_block(toml: &str, lang: &crate::schema::Language) -> Result<UpdateLangResult> {
    let mut log = String::new();
    writeln!(log, "\x1b[1;34m>>> updating {}\x1b[0m", lang.name)?;

    // fetch new versions
    let package = lang.parser.package.clone();
    let parser_url = lang.parser.git.url.clone();
    let crates_io_handle =
        thread::spawn(move || add_lang::try_get_crates_io_version(&package, &parser_url));
    let rev = add_lang::get_rev(&lang.parser.git.url, lang.parser.git.branch.as_deref())?;
    writeln!(log, "rev: {} -> {rev}", &lang.parser.git.rev)?;
    let content_url = add_lang::url_to_content_url(&lang.parser.git.url, &rev);
    let path_in_url = match &lang.parser.git.path {
        Some(path) => format!("/{path}"),
        None => String::new(),
    };
    let (external_c, external_cpp, ffi_func) =
        thread::scope(|scope| -> Result<(bool, bool, Option<String>)> {
            let external_c = scope.spawn(|| -> Result<bool> {
                Ok(content_url.as_ref().is_some_and(|url| {
                    reqwest::blocking::get(format!("{url}{path_in_url}/src/scanner.c"))
                        .is_ok_and(|response| response.status().is_success())
                }))
            });
            let external_cpp = scope.spawn(|| -> Result<bool> {
                Ok(content_url.as_ref().is_some_and(|url| {
                    reqwest::blocking::get(format!("{url}{path_in_url}/src/scanner.cc"))
                        .is_ok_and(|response| response.status().is_success())
                }))
            });
            let ffi_func = scope.spawn(|| -> Result<Option<String>> {
                add_lang::detect_ffi_func(
                    &lang.parser.git.url,
                    &rev,
                    lang.parser.git.path.as_deref(),
                )
            });

            let external_c = external_c
                .join()
                .map_err(|_| anyhow::anyhow!("update-langs external-c worker panicked"))??;
            let external_cpp = external_cpp
                .join()
                .map_err(|_| anyhow::anyhow!("update-langs external-cpp worker panicked"))??;
            let ffi_func = ffi_func
                .join()
                .map_err(|_| anyhow::anyhow!("update-langs ffi worker panicked"))??;
            Ok((external_c, external_cpp, ffi_func))
        })?;

    let crates_io = crates_io_handle
        .join()
        .map_err(|_| anyhow::anyhow!("update-langs crates.io worker panicked"))?;
    match &crates_io {
        add_lang::CratesIoLookup::Match(version) => writeln!(
            log,
            "crates.io: {:?} -> {version:?}",
            &lang.parser.crates_io
        )?,
        add_lang::CratesIoLookup::Mismatch => writeln!(
            log,
            "crates.io: {:?} -> (repo mismatch; unchanged)",
            &lang.parser.crates_io
        )?,
        add_lang::CratesIoLookup::Unavailable => writeln!(
            log,
            "crates.io: {:?} -> (unavailable; unchanged)",
            &lang.parser.crates_io
        )?,
    };
    // TODO: check compat for gitdep?
    writeln!(log, "external C: {external_c}")?;
    writeln!(log, "external C++: {external_cpp}")?;

    // update in toml string
    let toml = regex_replace!(
        r#"^(\s*git\s*=\s*\{[^}]*\brev\s*=\s*")[^"]*(".*\}\s*)$"#m,
        toml,
        |_, start, end| format!("{start}{rev}{end}"),
    );
    let toml = match crates_io {
        add_lang::CratesIoLookup::Match(Some(crates_io)) => regex_replace!(
            r#"^(?:\s*#\s*)?(\s*crates-io\s*=\s*")[^"]*("\s*)$"#m,
            &toml,
            |_, start, end| format!("{start}{crates_io}{end}"),
        ),
        add_lang::CratesIoLookup::Match(None) => {
            if let Some(old_crates_io) = &lang.parser.crates_io {
                // no longer compatible, remove crates-io version
                regex_replace!(
                    r#"^(?:\s*#\s*)?(\s*crates-io\s*=\s*")[^"]*("\s*)$"#m,
                    &toml,
                    |_, start, end| format!("# {start}{old_crates_io}{end}"),
                )
            } else {
                toml
            }
        }
        add_lang::CratesIoLookup::Mismatch | add_lang::CratesIoLookup::Unavailable => toml,
    };
    let toml = regex_replace!(
        r#"^(\s*external-scanner\s*=\s*\{\s*c\s*=\s*)(?:true|false)(\s*,\s*cpp\s*=\s*)(?:true|false)(\s*\}\s*)$"#m,
        &toml,
        |_, start, middle, end| format!("{start}{external_c}{middle}{external_cpp}{end}"),
    );
    let toml = if let Some(ffi_func) = ffi_func {
        regex_replace!(
            r#"^(\s*ffi-func\s*=\s*")[^"]*("\s*)$"#m,
            &toml,
            |_, start, end| format!("{start}{ffi_func}{end}"),
        )
    } else {
        toml
    };

    Ok(UpdateLangResult {
        toml: toml.into_owned(),
        log,
    })
}
