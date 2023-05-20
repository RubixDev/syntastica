use std::fs;

use anyhow::Result;

mod queries;

pub fn run() -> Result<()> {
    let mut queries_lib_rs = r###"
//! This crate defines constants for three types of tree-sitter queries for lots of parsers.
//! It is intended to be used via [syntastica](https://crates.io/crates/syntastica).
//!
//! The three types of queries are:
//!
//! 1. `highlights`: defining the highlight captures for nodes
//! 2. `injections`: defining where other languages are injected for highlighting
//! 3. `locals`: keeping track of scopes, variables, parameters, etc. to have occurrences of those
//!    be highlighted the same everywhere
//!
//! The constants are defined as `<language_name>_<kind>` where `<kind>` is one of `HIGHLIGHTS`,
//! `INJECTIONS`, or `LOCALS`. The `INJECTIONS` and `LOCALS` may be empty for some languages.
//!
//! The source `lib.rs` file is automatically generated with `cargo xtask codegen` inside the
//! syntastica workspace.
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![cfg_attr(rustfmt, rustfmt_skip)]
"###
    .trim_start()
    .to_owned();
    for (name, [highlights, injections, locals]) in queries::make_queries()? {
        queries_lib_rs += &format!(
            r###"
pub const {lang}_HIGHLIGHTS: &str = {highlights:?};
pub const {lang}_INJECTIONS: &str = {injections:?};
pub const {lang}_LOCALS: &str = {locals:?};
"###,
            lang = name.to_uppercase()
        )
    }
    fs::write(
        crate::WORKSPACE_DIR.join("syntastica-queries/src/lib.rs"),
        queries_lib_rs,
    )?;

    Ok(())
}
