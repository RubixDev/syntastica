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
//! The crate source is automatically generated with `cargo xtask codegen` inside the
//! syntastica workspace.
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![cfg_attr(rustfmt, rustfmt_skip)]

pub const ASM_HIGHLIGHTS: &str = include_str!("../generated_queries/asm/highlights.scm");
pub const ASM_INJECTIONS: &str = include_str!("../generated_queries/asm/injections.scm");
pub const ASM_LOCALS: &str = include_str!("../generated_queries/asm/locals.scm");

pub const BASH_HIGHLIGHTS: &str = include_str!("../generated_queries/bash/highlights.scm");
pub const BASH_INJECTIONS: &str = include_str!("../generated_queries/bash/injections.scm");
pub const BASH_LOCALS: &str = include_str!("../generated_queries/bash/locals.scm");

pub const C_HIGHLIGHTS: &str = include_str!("../generated_queries/c/highlights.scm");
pub const C_INJECTIONS: &str = include_str!("../generated_queries/c/injections.scm");
pub const C_LOCALS: &str = include_str!("../generated_queries/c/locals.scm");

pub const CPP_HIGHLIGHTS: &str = include_str!("../generated_queries/cpp/highlights.scm");
pub const CPP_INJECTIONS: &str = include_str!("../generated_queries/cpp/injections.scm");
pub const CPP_LOCALS: &str = include_str!("../generated_queries/cpp/locals.scm");

pub const CSS_HIGHLIGHTS: &str = include_str!("../generated_queries/css/highlights.scm");
pub const CSS_INJECTIONS: &str = include_str!("../generated_queries/css/injections.scm");
pub const CSS_LOCALS: &str = include_str!("../generated_queries/css/locals.scm");

pub const GO_HIGHLIGHTS: &str = include_str!("../generated_queries/go/highlights.scm");
pub const GO_INJECTIONS: &str = include_str!("../generated_queries/go/injections.scm");
pub const GO_LOCALS: &str = include_str!("../generated_queries/go/locals.scm");

pub const HTML_HIGHLIGHTS: &str = include_str!("../generated_queries/html/highlights.scm");
pub const HTML_INJECTIONS: &str = include_str!("../generated_queries/html/injections.scm");
pub const HTML_LOCALS: &str = include_str!("../generated_queries/html/locals.scm");

pub const JAVA_HIGHLIGHTS: &str = include_str!("../generated_queries/java/highlights.scm");
pub const JAVA_INJECTIONS: &str = include_str!("../generated_queries/java/injections.scm");
pub const JAVA_LOCALS: &str = include_str!("../generated_queries/java/locals.scm");

pub const JAVASCRIPT_HIGHLIGHTS: &str = include_str!("../generated_queries/javascript/highlights.scm");
pub const JAVASCRIPT_INJECTIONS: &str = include_str!("../generated_queries/javascript/injections.scm");
pub const JAVASCRIPT_LOCALS: &str = include_str!("../generated_queries/javascript/locals.scm");

pub const JSON_HIGHLIGHTS: &str = include_str!("../generated_queries/json/highlights.scm");
pub const JSON_INJECTIONS: &str = include_str!("../generated_queries/json/injections.scm");
pub const JSON_LOCALS: &str = include_str!("../generated_queries/json/locals.scm");

pub const PYTHON_HIGHLIGHTS: &str = include_str!("../generated_queries/python/highlights.scm");
pub const PYTHON_INJECTIONS: &str = include_str!("../generated_queries/python/injections.scm");
pub const PYTHON_LOCALS: &str = include_str!("../generated_queries/python/locals.scm");

pub const REGEX_HIGHLIGHTS: &str = include_str!("../generated_queries/regex/highlights.scm");
pub const REGEX_INJECTIONS: &str = include_str!("../generated_queries/regex/injections.scm");
pub const REGEX_LOCALS: &str = include_str!("../generated_queries/regex/locals.scm");

pub const RUST_HIGHLIGHTS: &str = include_str!("../generated_queries/rust/highlights.scm");
pub const RUST_INJECTIONS: &str = include_str!("../generated_queries/rust/injections.scm");
pub const RUST_LOCALS: &str = include_str!("../generated_queries/rust/locals.scm");

pub const TSX_HIGHLIGHTS: &str = include_str!("../generated_queries/tsx/highlights.scm");
pub const TSX_INJECTIONS: &str = include_str!("../generated_queries/tsx/injections.scm");
pub const TSX_LOCALS: &str = include_str!("../generated_queries/tsx/locals.scm");

pub const TYPESCRIPT_HIGHLIGHTS: &str = include_str!("../generated_queries/typescript/highlights.scm");
pub const TYPESCRIPT_INJECTIONS: &str = include_str!("../generated_queries/typescript/injections.scm");
pub const TYPESCRIPT_LOCALS: &str = include_str!("../generated_queries/typescript/locals.scm");
