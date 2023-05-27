use std::{fs, path::Path};

use anyhow::Result;

const HEADER: &str = "
<!-- Everything under here is autogenerated by running `cargo xtask codegen` -->
<!-- DO NOT EDIT! -->

## List of included parsers
";

pub fn write() -> Result<()> {
    let readme_path = crate::WORKSPACE_DIR.join("syntastica-parsers-git/README.md");
    let mut readme = read(&readme_path)?;
    syntastica_macros::parser_list_git!();
    fs::write(&readme_path, readme)?;

    let readme_path = crate::WORKSPACE_DIR.join("syntastica-parsers-gitdep/README.md");
    let mut readme = read(&readme_path)?;
    syntastica_macros::parser_list_gitdep!();
    fs::write(&readme_path, readme)?;

    let readme_path = crate::WORKSPACE_DIR.join("syntastica-parsers/README.md");
    let mut readme = read(&readme_path)?;
    syntastica_macros::parser_list_dep!();
    fs::write(&readme_path, readme)?;

    Ok(())
}

fn read(path: &Path) -> Result<String> {
    let mut readme = fs::read_to_string(path)?;
    if let Some((preserve, _)) = readme.split_once(HEADER) {
        readme.truncate(preserve.len());
    }
    readme += HEADER;
    Ok(readme)
}