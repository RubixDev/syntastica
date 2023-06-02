use std::{
    collections::{BTreeMap, HashMap},
    env, fs,
};

use anyhow::Result;
use syntastica_core::theme::ThemeValue;

use crate::schema::Group;

mod js_lang_list;
mod parser_lists;
mod parsers_dep;
mod parsers_gitdep;
mod queries;
mod theme_gruvbox;
mod theme_one;

const TOML_AUTOGEN_HEADER: &str = "
###########################################
### All following code is autogenerated ###
### by running `cargo xtask codegen` in ###
### the syntastica workspace. #############
###########################################
";
const TOML_FEATURES_HEAD: &str = r##"
[features]
#! ### Features
default = []

#! At least one of
#! <span class="stab portability"><code>some</code></span>,
#! <span class="stab portability"><code>most</code></span>, and
#! <span class="stab portability"><code>all</code></span>
#! must be enabled.

## Include parsers for the most widely known supported languages.
"##;
const TOML_FEATURES_MOST: &str = r##"
## Implies <span class="stab portability"><code>some</code></span>.
## Include parsers for most common languages.
"##;
const TOML_FEATURES_ALL: &str = r##"
## Implies <span class="stab portability"><code>most</code></span>.
## Include parsers for all supported languages.
"##;
const TOML_FEATURES_DOCS: &str = r##"
## Meant to be enabled when building docs
docs = ["dep:document-features", "dep:rustc_version"]
"##;

#[derive(Clone, Debug)]
enum RawThemeValue {
    Link(String),
    Styles(HashMap<String, String>),
    Ignore,
}

fn is_arg(test: &str) -> bool {
    env::args().nth(2).map_or(true, |arg| arg == test)
}

pub fn run() -> Result<()> {
    if is_arg("queries") {
        let mut queries_lib_rs = r###"
#![doc = include_str!("../README.md")]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![cfg_attr(rustfmt, rustfmt_skip)]
"###
        .trim_start()
        .to_owned();

        let queries_dir = crate::WORKSPACE_DIR.join("syntastica-queries/generated_queries");
        let _ = fs::remove_dir_all(&queries_dir);
        fs::create_dir_all(&queries_dir)?;
        fs::write(
            queries_dir.join("README.md"),
            include_str!("./codegen/generated_queries_readme.md"),
        )?;

        for (
            ref name,
            [highlights, injections, locals, highlights_crates_io, injections_crates_io, locals_crates_io],
        ) in queries::make_queries()?
        {
            let lang_dir = queries_dir.join(name);
            fs::create_dir(&lang_dir)?;

            fs::write(lang_dir.join("highlights.scm"), highlights)?;
            fs::write(lang_dir.join("injections.scm"), injections)?;
            fs::write(lang_dir.join("locals.scm"), locals)?;
            fs::write(
                lang_dir.join("highlights_crates_io.scm"),
                highlights_crates_io,
            )?;
            fs::write(
                lang_dir.join("injections_crates_io.scm"),
                injections_crates_io,
            )?;
            fs::write(lang_dir.join("locals_crates_io.scm"), locals_crates_io)?;

            queries_lib_rs += &format!(
                r###"
pub const {lang}_HIGHLIGHTS: &str = include_str!("../generated_queries/{name}/highlights.scm");
pub const {lang}_INJECTIONS: &str = include_str!("../generated_queries/{name}/injections.scm");
pub const {lang}_LOCALS: &str = include_str!("../generated_queries/{name}/locals.scm");
pub const {lang}_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/{name}/highlights_crates_io.scm");
pub const {lang}_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/{name}/injections_crates_io.scm");
pub const {lang}_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/{name}/locals_crates_io.scm");
"###,
                lang = name.to_uppercase()
            )
        }
        fs::write(
            crate::WORKSPACE_DIR.join("syntastica-queries/src/lib.rs"),
            queries_lib_rs,
        )?;
    }

    if is_arg("parsers-dep") {
        parsers_dep::write()?;
    }

    if is_arg("parsers-gitdep") {
        parsers_gitdep::write()?;
    }

    if is_arg("parser-lists") {
        parser_lists::write()?;
    }

    if is_arg("js-lang-list") {
        js_lang_list::write()?;
    }

    if is_arg("themes") {
        fs::write(
            crate::WORKSPACE_DIR.join("syntastica-themes/src/gruvbox.rs"),
            theme_gruvbox::make_theme()?,
        )?;
        fs::write(
            crate::WORKSPACE_DIR.join("syntastica-themes/src/one.rs"),
            theme_one::make_theme()?,
        )?;
    }

    Ok(())
}

const INDENT: &str = "    ";
type Theme = BTreeMap<String, ThemeValue>;
fn to_theme_macro_call(map: &Theme) -> String {
    let mut out = "theme! {\n".to_owned();
    for (key, value) in map {
        let value = match value {
            ThemeValue::Simple(str) => format!("\"{str}\""),
            ThemeValue::Extended {
                color,
                underline,
                strikethrough,
                italic,
                bold,
                link,
            } => {
                let mut value = "{\n".to_owned();
                value += &format!(
                    "{}color: {},\n",
                    INDENT.repeat(3),
                    color
                        .as_ref()
                        .map(|s| format!("{s:?}"))
                        .unwrap_or("None".to_owned()),
                );
                value += &format!("{}underline: {},\n", INDENT.repeat(3), underline);
                value += &format!("{}strikethrough: {},\n", INDENT.repeat(3), strikethrough);
                value += &format!("{}italic: {},\n", INDENT.repeat(3), italic);
                value += &format!("{}bold: {},\n", INDENT.repeat(3), bold);
                value += &format!(
                    "{}link: {},\n",
                    INDENT.repeat(3),
                    link.as_ref()
                        .map(|s| format!("{s:?}"))
                        .unwrap_or("None".to_owned()),
                );
                value += &format!("{}}}", INDENT.repeat(2));
                value
            }
        };
        out += &format!("{}\"{key}\": {value},\n", INDENT.repeat(2));
    }
    out + INDENT + "}"
}

fn resolve_links(raw_theme: &mut BTreeMap<String, RawThemeValue>) {
    let mut links_left = true;
    while links_left {
        links_left = false;
        let raw_theme_copy = raw_theme.clone();
        for (key, value) in &mut *raw_theme {
            if !key.starts_with('@') {
                continue;
            }
            if let RawThemeValue::Link(link) = value {
                links_left = true;
                match raw_theme_copy.get(link) {
                    Some(new_value) => *value = new_value.clone(),
                    None => {
                        eprintln!("warning: ignoring key {key} because of invalid link to {link}");
                        *value = RawThemeValue::Ignore;
                    }
                }
            }
        }
    }
}

fn make_theme_file(
    name: &str,
    url: &str,
    palettes: BTreeMap<&&str, Theme>,
    theme: Theme,
) -> String {
    let mut out = format!("//! The {name} themes in this module were extracted from <{url}>");
    out += r###"
//!
//! The module source is automatically generated with `cargo xtask codegen` inside the
//! syntastica workspace.

use std::collections::BTreeMap;

use syntastica_core::{
    theme,
    theme::{ResolvedTheme, Theme, ThemeValue},
};
"###;

    for (variant, palette) in palettes {
        out += &format!(
            r###"
pub fn {variant}() -> ResolvedTheme {{
    let mut palette = {theme}
    .into_inner();
    palette.append(&mut theme());
    Theme::new(palette).resolve_links().unwrap()
}}
"###,
            theme = to_theme_macro_call(&palette),
        )
    }

    out += &format!(
        r###"
fn theme() -> BTreeMap<String, ThemeValue> {{
    {theme}
    .into_inner()
}}
"###,
        theme = to_theme_macro_call(&theme)
    );

    out
}

fn parsers_toml_feature(group: Group, crates_io: bool) -> String {
    let mut added_packages = vec![];
    let mut feature_str = format!("{group} = [\n");
    if let Some(group) = group.next_smaller() {
        feature_str += &format!("    \"{group}\",\n");
    }
    for lang in crate::LANGUAGE_CONFIG.languages.iter().filter(|lang| {
        lang.parser.rust_func.is_some()
            && lang.group == group
            && (!crates_io || lang.parser.crates_io.is_some())
    }) {
        let package = &lang.parser.package;
        if added_packages.contains(&package) {
            continue;
        }
        added_packages.push(package);
        feature_str += &format!("    \"dep:{package}\",\n");
    }
    feature_str + "]\n"
}

fn parsers_toml_deps(toml: &mut String, git: bool) {
    let mut added_packages = vec![];
    for lang in crate::LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(|lang| lang.parser.rust_func.is_some() && (git || lang.parser.crates_io.is_some()))
    {
        let package = &lang.parser.package;
        let url = &lang.parser.git.url;
        let rev = &lang.parser.git.rev;

        if added_packages.contains(&package) {
            continue;
        }
        added_packages.push(package);

        let dep_str = if git {
            format!(
                r##"
[dependencies.{package}]
optional = true
git = "{url}"
rev = "{rev}"
"##
            )
        } else {
            format!(
                r##"
[dependencies.{package}]
optional = true
version = "{version}"
"##,
                version = lang
                    .parser
                    .crates_io
                    .as_ref()
                    .expect("`None` is filtered above if `git` is `false`")
            )
        };
        *toml += &dep_str;
    }
}
