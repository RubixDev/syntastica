use std::fs;

use anyhow::Result;
use lazy_regex::regex_replace;

use crate::add_lang;

pub fn run() -> Result<()> {
    let langs_toml_path = crate::WORKSPACE_DIR.join("syntastica-macros/languages.toml");
    let langs_toml = fs::read_to_string(&langs_toml_path)?;
    let langs = langs_toml
        .split("\n\n")
        .zip(&crate::LANGUAGE_CONFIG.languages)
        .map(|(toml, lang)| {
            println!("\x1b[1;34m>>> updating {}\x1b[0m", lang.name);

            // fetch new versions
            let rev = add_lang::get_rev(&lang.parser.git.url)?;
            println!("rev: {} -> {rev}", &lang.parser.git.rev);
            let crates_io = add_lang::try_get_crates_io_version(&lang.parser.package);
            println!("crates.io: {:?} -> {crates_io:?}", &lang.parser.crates_io);
            // TODO: check compat for gitdep?
            let content_url = add_lang::url_to_content_url(&lang.parser.git.url, &rev);
            let path_in_url = match &lang.parser.git.path {
                Some(path) => format!("/{path}"),
                None => String::new(),
            };
            let external_c = content_url.as_ref().map_or(false, |url| {
                reqwest::blocking::get(format!("{url}{path_in_url}/src/scanner.c"))
                    .map_or(false, |response| response.status().is_success())
            });
            println!("external C: {external_c}");
            let external_cpp = content_url.as_ref().map_or(false, |url| {
                reqwest::blocking::get(format!("{url}{path_in_url}/src/scanner.cc"))
                    .map_or(false, |response| response.status().is_success())
            });
            println!("external C++: {external_cpp}");

            // update in toml string
            let toml = regex_replace!(
                r#"^(\s*git\s*=\s*\{\s*url\s*=\s*"[^"]*"\s*,\s*rev\s*=\s*")[^"]*("\s*(?:,\s*path\s*=\s*"[^"]*"\s*)?\}\s*)$"#m,
                toml,
                |_, start, end| format!("{start}{rev}{end}"),
            );
            let toml = if let Some(crates_io) = crates_io {
                regex_replace!(
                    r#"^(?:\s*#\s*)?(\s*crates-io\s*=\s*")[^"]*("\s*)$"#m,
                    &toml,
                    |_, start, end| format!("{start}{crates_io}{end}"),
                )
            } else {
                toml
            };
            let toml = regex_replace!(
                r#"^(\s*external-scanner\s*=\s*\{\s*c\s*=\s*)(?:true|false)(\s*,\s*cpp\s*=\s*)(?:true|false)(\s*\}\s*)$"#m,
                &toml,
                |_, start, middle, end| format!("{start}{external_c}{middle}{external_cpp}{end}"),
            );

            Ok(toml.into_owned())
        })
        .collect::<Result<Vec<_>>>()?
        .join("\n\n");
    fs::write(&langs_toml_path, langs)?;

    Ok(())
}
