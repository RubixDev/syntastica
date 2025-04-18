use std::{env, fs};

use anyhow::Result;

use crate::schema::Group;

mod js_lists;
mod js_pkgs;
mod parser_lists;
mod parsers_dep;
mod parsers_git;
mod parsers_gitdep;
mod queries;
mod theme_list;

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

#! Every supported language has a feature with the same name as the respective public function.
#! Additionally the three feature groups
#! <span class="stab portability"><code>some</code></span>,
#! <span class="stab portability"><code>most</code></span>, and
#! <span class="stab portability"><code>all</code></span>
#! are available.

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

    if is_arg("parsers-git") {
        parsers_git::write()?;
    }

    if is_arg("parser-lists") {
        parser_lists::write()?;
    }

    if is_arg("js-list") {
        js_lists::write()?;
    }

    if is_arg("js-pkgs") {
        js_pkgs::write()?;
    }

    if is_arg("theme-list") {
        theme_list::write()?;
    }

    Ok(())
}

#[derive(PartialEq, Eq)]
enum ParserCollection {
    Git,
    GitDep,
    Dep,
}

fn parsers_toml_feature(group: Group) -> String {
    let mut feature_str = format!("{group} = [\n");
    if let Some(group) = group.next_smaller() {
        feature_str += &format!("    \"{group}\",\n");
    }
    for lang in crate::LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(|lang| lang.group == group)
    {
        feature_str += "    \"";
        feature_str += &lang.name;
        feature_str += "\",\n";
    }
    feature_str + "]\n"
}

fn parsers_toml_lang_features(collection: ParserCollection) -> String {
    let mut out = String::new();
    for lang in &crate::LANGUAGE_CONFIG.languages {
        out += &lang.name;
        out += " = [";
        if collection != ParserCollection::Git
            && lang.parser.supports(collection == ParserCollection::GitDep)
        {
            out += "\"dep:";
            out += &lang.parser.package;
            out += "\"";
        }
        out += "]\n";
    }
    out
}

fn parsers_toml_deps(toml: &mut String, git: bool) {
    let mut added_packages = vec![];
    for lang in crate::LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(|lang| lang.parser.supports(git))
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
version = "={version}"
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
