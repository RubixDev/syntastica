use std::{fs, path::Path};

use anyhow::Result;

use crate::schema::{Group, Language};

const HEADER: &str = "
<!-- Everything under here is autogenerated by running `cargo xtask codegen` -->
<!-- DO NOT EDIT! -->

## List of included parsers
";

pub fn write() -> Result<()> {
    let readme_path = crate::WORKSPACE_DIR.join("syntastica-parsers-git/README.md");
    let mut readme = read(&readme_path)?;
    readme += &parser_list(|_| true, git_url);
    fs::write(&readme_path, readme)?;

    let readme_path = crate::WORKSPACE_DIR.join("syntastica-parsers-gitdep/README.md");
    let mut readme = read(&readme_path)?;
    readme += &parser_list(|lang| lang.parser.rust_const.is_some(), git_url);
    fs::write(&readme_path, readme)?;

    let readme_path = crate::WORKSPACE_DIR.join("syntastica-parsers/README.md");
    let mut readme = read(&readme_path)?;
    readme += &parser_list(
        |lang| lang.parser.rust_const.is_some() && lang.parser.crates_io.is_some(),
        crates_io_url,
    );
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

fn git_url(lang: &Language) -> String {
    format!("{}/tree/{}", lang.parser.git.url, lang.parser.git.rev)
}

fn crates_io_url(lang: &Language) -> String {
    match &lang.parser.crates_io {
        Some(version) => format!("https://docs.rs/{}/{version}/", lang.parser.package),
        None => lang.parser.git.url.clone(),
    }
}

fn parser_list(filter: impl Fn(&Language) -> bool, url: impl Fn(&Language) -> String) -> String {
    let mut some_list = String::new();
    let mut most_list = String::new();
    let mut all_list = String::new();
    for lang in &crate::LANGUAGE_CONFIG.languages {
        let str = format!(
            "- [{}]({}){}\n",
            lang.name,
            url(lang),
            if filter(lang) {
                ""
            } else {
                " (not supported by this collection)"
            }
        );
        match lang.group {
            Group::Some => some_list += &str,
            Group::Most => most_list += &str,
            Group::All => all_list += &str,
        }
    }

    format!(
        r##"
<!-- dprint-ignore-start -->

<details>
<summary>List of parsers included in the <span class="stab portability"><code>some</code></span> feature</summary>

{some_list}
</details>

<details>
<summary>List of parsers additionally included in the <span class="stab portability"><code>most</code></span> feature</summary>

{most_list}
</details>

<details>
<summary>List of parsers additionally included in the <span class="stab portability"><code>all</code></span> feature</summary>

{all_list}
</details>

<!-- dprint-ignore-end -->
"##
    )
}
