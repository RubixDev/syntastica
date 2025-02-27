use std::fmt::{self, Display, Formatter};

use serde::Deserialize;
use tft::FileType;

#[derive(Clone, Debug, Deserialize)]
pub struct LanguageConfig {
    pub languages: Vec<Language>,
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Language {
    pub name: String,
    pub group: Group,
    pub file_types: Vec<FileType>,
    #[serde(default = "default_true")]
    pub wasm: bool,
    #[serde(default = "default_true")]
    pub wasm_unknown: bool,
    // TODO: injection regex
    pub parser: Parser,
    pub queries: Queries,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum Group {
    Some,
    Most,
    All,
}

impl Group {
    #[allow(dead_code)] // is used in `xtask/src/codegen.rs`
    pub fn next_smaller(&self) -> Option<Self> {
        match self {
            Group::Some => None,
            Group::Most => Some(Group::Some),
            Group::All => Some(Group::Most),
        }
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Some => write!(f, "some"),
            Self::Most => write!(f, "most"),
            Self::All => write!(f, "all"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Parser {
    pub git: ParserGit,
    pub external_scanner: ParserExternal,
    pub ffi_func: String,
    pub rust_const: Option<String>,
    pub package: String,
    pub crates_io: Option<String>,
    #[serde(default)]
    pub generate: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct ParserGit {
    pub url: String,
    pub rev: String,
    pub path: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct ParserExternal {
    pub c: bool,
    pub cpp: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Queries {
    pub nvim_like: bool,
    pub injections: bool,
    pub locals: bool,
}
