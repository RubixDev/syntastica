use std::process::Command;

use anyhow::Result;

pub fn run() -> Result<()> {
    let langs_dir = crate::WORKSPACE_DIR.join("syntastica-js/langs");
    for entry in langs_dir.read_dir()? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        Command::new("npm")
            .current_dir(entry.path())
            .args(["run", "build"])
            .status()?;
    }

    Ok(())
}
