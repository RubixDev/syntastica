use std::fs;

use anyhow::Result;

pub fn write() -> Result<()> {
    let toml_path = crate::WORKSPACE_DIR.join("syntastica-parsers/Cargo.toml");
    let mut toml = fs::read_to_string(&toml_path)?;

    if let Some((preserve, _)) = toml.split_once(super::TOML_AUTOGEN_HEADER) {
        toml.truncate(preserve.len());
    }
    toml += super::TOML_AUTOGEN_HEADER;

    toml += super::TOML_FEATURES_HEAD;
    syntastica_macros::parsers_dep_toml_feature_some!();
    toml += super::TOML_FEATURES_MOST;
    syntastica_macros::parsers_dep_toml_feature_most!();
    toml += super::TOML_FEATURES_ALL;
    syntastica_macros::parsers_dep_toml_feature_all!();

    toml += super::TOML_FEATURES_DOCS;

    syntastica_macros::parsers_dep_toml_deps!();

    fs::write(&toml_path, toml)?;

    Ok(())
}
