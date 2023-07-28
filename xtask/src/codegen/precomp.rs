#![cfg_attr(
    all(not(feature = "precomp-git"), not(feature = "precomp-crates-io")),
    allow(unused_variables, unused_imports)
)]

use std::fs;

use anyhow::Result;
use syntastica_core::language_set::HighlightConfiguration;

pub fn write(lang: &str, queries: [&str; 6]) -> Result<()> {
    #[cfg(feature = "precomp-git")]
    {
        println!("pre-compiling HighlightConfiguration for {lang} (git)");
        let conf = HighlightConfiguration::new(
            syntastica_parsers_git::from_name(lang).unwrap(),
            queries[0],
            queries[1],
            queries[2],
        )?;
        let bin = conf.serialize()?;

        let dir = crate::WORKSPACE_DIR.join("syntastica-parsers-git/precomp");
        fs::create_dir_all(&dir)?;
        fs::write(dir.join(lang).with_extension("bin"), &bin)?;
        let dir = crate::WORKSPACE_DIR.join("syntastica-parsers-gitdep/precomp");
        fs::create_dir_all(&dir)?;
        fs::write(dir.join(lang).with_extension("bin"), &bin)?;
    }

    #[cfg(feature = "precomp-crates-io")]
    if let Some(ts_lang) = syntastica_parsers::from_name(lang) {
        println!("pre-compiling HighlightConfiguration for {lang} (crates.io)");
        let conf = HighlightConfiguration::new(ts_lang, queries[3], queries[4], queries[5])?;
        let bin = conf.serialize()?;

        let dir = crate::WORKSPACE_DIR.join("syntastica-parsers/precomp");
        fs::create_dir_all(&dir)?;
        fs::write(dir.join(lang).with_extension("bin"), bin)?;
    }

    Ok(())
}
