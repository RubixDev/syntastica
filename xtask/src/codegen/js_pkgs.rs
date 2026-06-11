use std::fs;

use anyhow::Result;
use heck::ToPascalCase as _;
use toml::Table;

pub fn write() -> Result<()> {
    let langs_dir = crate::WORKSPACE_DIR.join("syntastica-js/langs");
    let _ = fs::remove_dir_all(&langs_dir);
    fs::create_dir_all(&langs_dir)?;
    fs::write(
        langs_dir.join("README.md"),
        include_str!("./js_langs_readme.md"),
    )?;

    for lang in &crate::LANGUAGE_CONFIG.languages {
        if !lang.wasm {
            continue;
        }
        let lang_dir = langs_dir.join(&lang.name);
        let src_dir = lang_dir.join("src");
        fs::create_dir_all(&src_dir)?;
        fs::write(lang_dir.join("LICENSE"), include_str!("../../../LICENSE"))?;

        let lang_name = &lang.name;
        let lang_name_pascal = &lang.name.to_pascal_case();

        fs::write(
            lang_dir.join("README.md"),
            format!(
                r###"# `syntastica-js-{lang_name}`

{lang_name_pascal} language support for
[`syntastica-js`](https://www.npmjs.com/package/@syntastica/core).
"###
            ),
        )?;

        fs::write(
            lang_dir.join("Cargo.toml"),
            format!(
                r###"[package]
name = "syntastica-js-{lang_name}"
version.workspace = true
authors.workspace = true
documentation = "https://rubixdev.github.io/syntastica/syntastica_js/"
edition.workspace = true
license.workspace = true
repository.workspace = true
description = "{lang_name_pascal} language support for syntastica-js"

[lib]
crate-type = ["cdylib"]

[dependencies]
syntastica-macros = {{ workspace = true, features = ["js"] }}
syntastica-queries.workspace = true

tree-sitter.workspace = true

[build-dependencies]
syntastica-macros = {{ workspace = true, features = ["js"] }}

cc.workspace = true
tree-sitter-generate.workspace = true
"###
            ),
        )?;

        fs::write(
            lang_dir.join("build.rs"),
            format!(
                r###"const EMSCRIPTEN_FLAGS: &[&str] = &[
    "-sTOTAL_MEMORY=33554432",
    "-sSIDE_MODULE=2",
    "-sWASM=1",
    "-sNODEJS_CATCH_EXIT=0",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {{
    if std::env::var("TARGET").is_ok_and(|s| s == "wasm32-unknown-emscripten") {{
        for flag in EMSCRIPTEN_FLAGS {{
            println!("cargo::rustc-link-arg={{flag}}");
        }}

        syntastica_macros::js_lang_build!("{lang_name}");
    }}
    Ok(())
}}

include!("../../../syntastica-parsers-git/build_helper.rs");
"###
            ),
        )?;

        let syntastica_version = toml::from_str::<Table>(
            &fs::read_to_string(crate::WORKSPACE_DIR.join("Cargo.toml")).unwrap(),
        )
        .unwrap()["workspace"]["package"]["version"]
            .as_str()
            .unwrap()
            .to_string();
        fs::write(
            lang_dir.join("package.json"),
            format!(
                r###"{{
  "$schema": "https://raw.githubusercontent.com/SchemaStore/schemastore/master/src/schemas/json/package.json",
  "name": "@syntastica/lang-{lang_name}",
  "version": "{syntastica_version}",
  "description": "{lang_name_pascal} language support for syntastica-js",
  "keywords": ["tree-sitter", "highlight", "parsing", "syntax"],
  "homepage": "https://github.com/RubixDev/syntastica/tree/main/syntastica-js#readme",
  "bugs": "https://github.com/RubixDev/syntastica/issues",
  "license": "MPL-2.0",
  "author": "RubixDev",
  "funding": {{
    "url": "https://ko-fi.com/rubixdev",
    "type": "ko-fi"
  }},
  "files": ["{lang_name}.wasm"],
  "repository": {{
    "type": "git",
    "url": "git+https://github.com/RubixDev/syntastica.git",
    "directory": "syntastica-js/langs/{lang_name}"
  }},
  "publishConfig": {{
    "access": "public"
  }},
  "scripts": {{
    "build": "env CFLAGS=-fPIC cargo build --profile release-wasm --target wasm32-unknown-emscripten",
    "postbuild": "cp \"${{CARGO_TARGET_DIR:-../../../target}}/wasm32-unknown-emscripten/release-wasm/syntastica_js_{lang_name}.wasm\" {lang_name}.wasm"
  }}
}}
"###
            ),
        )?;

        fs::write(
            src_dir.join("lib.rs"),
            format!(
                r###"#![no_std]
#![warn(rust_2018_idioms)]

use tree_sitter::Language;

syntastica_macros::js_lang_info!();
syntastica_macros::js_lang_lib!("{lang_name}");
"###
            ),
        )?;
    }

    Ok(())
}
