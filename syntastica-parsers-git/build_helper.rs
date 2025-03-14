use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

#[allow(unused)]
fn git(repo_dir: &Path) -> Command {
    let mut cmd = env::var_os("SYNTASTICA_PARSERS_GIT_PATH")
        .map_or_else(|| Command::new("git"), Command::new);
    cmd.current_dir(repo_dir)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());
    cmd
}

#[allow(unused, clippy::too_many_arguments)]
fn compile_parser(
    name: &str,
    url: &str,
    rev: &str,
    external_c: bool,
    external_cpp: bool,
    path: Option<&str>,
    wasm: bool,
    wasm_unknown: bool,
    generate: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let target = env::var("TARGET")?;

    // some parsers are not supported for wasm targets
    if env::var("CARGO_CFG_TARGET_FAMILY")?
        .split(',')
        .any(|family| family == "wasm")
        && !wasm
    {
        return Ok(());
    }

    // some external scanners are not supported on the `wasm32-unknown-unknown` target
    if target == "wasm32-unknown-unknown" && !wasm_unknown {
        return Ok(());
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let clone_dir = env::var("SYNTASTICA_PARSERS_CLONE_DIR")
        .map_or_else(|_| out_dir.join("clone"), PathBuf::from);
    let c_lib_name = format!("parser_{name}{}_{rev}", path.unwrap_or("")).replace('/', "_");
    let c_lib_filename = format!("lib{c_lib_name}.a");
    let cpp_lib_name = format!("scanner_{name}{}_{rev}", path.unwrap_or("")).replace('/', "_");
    let cpp_lib_filename = format!("lib{cpp_lib_name}.a");

    if let Ok(dir) = env::var("SYNTASTICA_PARSERS_CACHE_DIR") {
        if Path::new(&dir)
            .join(&target)
            .join(&c_lib_filename)
            .is_file()
            && (!external_cpp
                || Path::new(&dir)
                    .join(&target)
                    .join(&cpp_lib_filename)
                    .is_file())
        {
            println!("cargo::rustc-link-lib=static={c_lib_name}");
            if external_cpp {
                println!("cargo::rustc-link-lib=static={cpp_lib_name}");
                println!("cargo::rustc-link-lib=stdc++");
            }
            println!(
                "cargo::rustc-link-search=native={}",
                Path::new(&dir).join(&target).canonicalize()?.display()
            );
            return Ok(());
        }
    }

    // clone repo into `parsers/{name}/{rev}`, if it does not already exist
    let repo_dir = clone_dir.join(name).join(rev);
    if !repo_dir.exists() {
        println!("cloning repository for {name}");
        fs::create_dir_all(&repo_dir)?;
        git(&repo_dir).arg("init").status()?;
        git(&repo_dir)
            .args(["remote", "add", "origin", url])
            .status()?;
        git(&repo_dir)
            .args(["fetch", "--depth=1", "origin", rev])
            .status()?;
        git(&repo_dir).args(["checkout", "FETCH_HEAD"]).status()?;
    }

    let mut src_dir = repo_dir;
    if let Some(path) = path {
        src_dir = src_dir.join(path);
    }

    if generate && !src_dir.join("src/parser.c").is_file() {
        println!("generating parser for {name}");
        tree_sitter_generate::generate_parser_in_directory(
            &src_dir,
            None,
            None,
            tree_sitter_generate::ABI_VERSION_MAX,
            None,
            env::var("SYNTASTICA_PARSERS_JS_RUNTIME").ok().as_deref(),
        )?;
    }

    println!("building parser for {name}");
    src_dir = src_dir.join("src");
    let mut c_config = cc::Build::new();
    c_config.include(&src_dir);
    c_config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs")
        .flag_if_supported("-w");
    let parser_path = src_dir.join("parser.c");
    c_config.file(&parser_path);

    if external_c {
        let scanner_path = src_dir.join("scanner.c");
        c_config.file(&scanner_path);
        println!("cargo::rerun-if-changed={}", scanner_path.display());
    }

    if target == "wasm32-unknown-unknown" {
        c_config.include(
            // this is set by the `wasm32-unknown-unknown-openbsd-libc` crate
            std::env::var_os("DEP_WASM32_UNKNOWN_UNKNOWN_OPENBSD_LIBC_INCLUDE")
                .expect("failed to find wasm libc"),
        );
    }

    println!("cargo::rerun-if-changed={}", parser_path.display());
    c_config.compile(&c_lib_name);
    println!("finished building parser for {name}");

    if let Ok(dir) = env::var("SYNTASTICA_PARSERS_CACHE_DIR") {
        let cache_dir = Path::new(&dir).join(&target);
        fs::create_dir_all(&cache_dir)?;
        fs::copy(
            out_dir.join(&c_lib_filename),
            cache_dir.join(&c_lib_filename),
        )?;
    }

    if external_cpp {
        println!("building cpp scanner for {name}");
        let mut cpp_config = cc::Build::new();
        cpp_config.cpp(true);
        cpp_config.include(&src_dir);
        cpp_config
            .flag_if_supported("-Wno-unused-parameter")
            .flag_if_supported("-Wno-unused-but-set-variable")
            .flag_if_supported("-w");
        let scanner_path = src_dir.join("scanner.cc");
        cpp_config.file(&scanner_path);
        println!("cargo::rerun-if-changed={}", scanner_path.display());
        cpp_config.compile(&cpp_lib_name);
        println!("finished building cpp scanner for {name}");

        if let Ok(dir) = env::var("SYNTASTICA_PARSERS_CACHE_DIR") {
            fs::copy(
                out_dir.join(&cpp_lib_filename),
                Path::new(&dir).join(&target).join(&cpp_lib_filename),
            )?;
        }
    }

    Ok(())
}
