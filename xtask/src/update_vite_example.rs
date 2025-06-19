use std::process::Command;

use anyhow::{bail, Result};

pub fn run() -> Result<()> {
    let langs = crate::LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(|lang| lang.wasm)
        .map(|lang| &lang.name)
        .collect::<Vec<_>>();
    let demo_dir = crate::WORKSPACE_DIR.join("examples/wasm/vite");
    let status = Command::new("npm")
        .current_dir(&demo_dir)
        .arg("install")
        .args(langs.iter().map(|lang| format!("@syntastica/lang-{lang}")))
        .status()?;
    if !status.success() {
        bail!("npm exited with non-zero exit code: {status}");
    }

    Ok(())
}
