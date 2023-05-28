#[cfg(feature = "some")]
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(not(all(feature = "docs", doc))) {
        syntastica_macros::parsers_git!();
    }

    // for documenting features when using nightly
    #[cfg(all(feature = "docs", doc))]
    {
        let channel = match rustc_version::version_meta().unwrap().channel {
            rustc_version::Channel::Dev => "CHANNEL_DEV",
            rustc_version::Channel::Nightly => "CHANNEL_NIGHTLY",
            rustc_version::Channel::Beta => "CHANNEL_BETA",
            rustc_version::Channel::Stable => "CHANNEL_STABLE",
        };
        println!("cargo:rustc-cfg={channel}");
    }
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}

#[cfg(feature = "some")]
fn git(repo_dir: &Path) -> Command {
    let mut cmd = Command::new("git");
    cmd.current_dir(repo_dir)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());
    cmd
}

#[cfg(feature = "some")]
fn compile_parser(
    name: &str,
    url: &str,
    rev: &str,
    external_c: bool,
    external_cpp: bool,
    path: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    // clone repo into `parsers/{name}`, if it does not already exists
    let repo_dir = PathBuf::from(format!("{}/{name}", env::var("OUT_DIR")?));
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
    src_dir = src_dir.join("src");

    println!("building parser for {name}");
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
        println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
    }

    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());
    c_config.compile(&format!("parser_{name}{}", path.unwrap_or("")).replace('/', "_"));
    println!("finished building parser for {name}");

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
        println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
        cpp_config.compile(&format!("scanner_{name}{}", path.unwrap_or("")).replace('/', "_"));
        println!("finished building cpp scanner for {name}");
    }

    Ok(())
}
