use std::fs;

use anyhow::Result;

const HEADER: &str = r##"
// DISCLAIMER: All code below this line is generated with `cargo xtask codegen js-list`
// in the syntastica workspace. Do not edit this code manually!
/**
 * A list of all supported languages.
 *
 * @see The {@link Language} type.
 */
export const LANGUAGES = [
"##;

pub fn write() -> Result<()> {
    let ts_path = crate::WORKSPACE_DIR.join("syntastica-js/src/index.ts");
    let mut ts = fs::read_to_string(&ts_path)?;

    if let Some((preserve, _)) = ts.split_once(HEADER) {
        ts.truncate(preserve.len());
    }
    ts += HEADER;

    for lang in &crate::LANGUAGE_CONFIG.languages {
        ts += "    '";
        ts += &lang.name;
        ts += "',\n";
    }

    ts += &r#"
] as const

/**
 * A list of all valid themes.
 *
 * @see The {@link Theme} type.
 */
export const THEMES = [
"#[1..];
    for theme in super::theme_list::find_all_themes()? {
        ts += "    '";
        ts += &theme;
        ts += "',\n";
    }
    ts += "] as const\n";

    fs::write(&ts_path, ts)?;

    Ok(())
}
