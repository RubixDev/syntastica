use std::fmt::{self, Display, Formatter};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LanguageConfig {
    pub languages: Vec<Language>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Language {
    pub name: String,
    pub group: Group,
    pub file_extensions: Vec<String>,
    // TODO: injection regex
    pub parser: Parser,
    pub queries: Queries,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Group {
    Some,
    Most,
    All,
}

impl Group {
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
    pub rust_func: Option<String>,
    pub package: String,
    pub crates_io: Option<String>,
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
