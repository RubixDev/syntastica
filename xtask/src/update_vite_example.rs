use std::process::Command;

use anyhow::Result;

pub fn run() -> Result<()> {
    let langs = crate::LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(|lang| lang.wasm)
        .map(|lang| &lang.name)
        .collect::<Vec<_>>();
    let demo_dir = crate::WORKSPACE_DIR.join("examples/wasm/vite");
    Command::new("npm")
        .current_dir(&demo_dir)
        .arg("install")
        .args(langs.iter().map(|lang| format!("@syntastica/lang-{lang}")))
        .status()?;

    Ok(())
}
