use std::{
    env,
    process::{Command, Stdio},
};

use anyhow::{bail, Result};

pub fn run() -> Result<()> {
    let cargo = env::var("CARGO")?;
    // first run: everything plus precomp-git
    if !Command::new(&cargo)
        .args([
            "run",
            "--package=xtask",
            "--features=precomp-git",
            "codegen",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?
        .success()
    {
        bail!("child process failed");
    }
    // second run: just queries for precomp-crates-io
    // (must be separate or else the two parser collections would mess up linking)
    if !Command::new(&cargo)
        .args([
            "run",
            "--package=xtask",
            "--features=precomp-crates-io",
            "codegen",
            "queries",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?
        .success()
    {
        bail!("child process failed");
    }

    Ok(())
}
