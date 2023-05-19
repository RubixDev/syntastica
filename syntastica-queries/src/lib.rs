#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

use std::{collections::HashMap, fs, path::Path};

use once_cell::sync::Lazy;
use regex::Regex;
use tree_sitter::{Language, Query, QueryPredicateArg};

syntastica_macros::queries!();

static QUERIES_DIR: Lazy<String> = Lazy::new(|| {
    format!(
        "{}/queries",
        Path::new(file!())
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .display()
    )
});
static INHERITS_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r";+\s*inherits\s*:?\s*([a-z_,()-]+)\s*").unwrap());

fn read_queries(lang_name: &str, filename: &str) -> String {
    let path = format!("{}/{lang_name}/{filename}", *QUERIES_DIR);
    let queries =
        fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read '{path}': {err}"));
    INHERITS_REGEX
        .replace_all(&queries, |captures: &regex::Captures| {
            captures[1]
                .split(',')
                .map(|lang| format!("\n{}\n", read_queries(lang.trim(), filename)))
                .collect::<String>()
        })
        .into_owned()
}

fn process_queries(lang: Language, lang_name: &str, filename: &str) -> String {
    let path = format!("{}/{lang_name}/{filename}", *QUERIES_DIR);
    let queries = read_queries(lang_name, filename);
    let query = Query::new(lang, &queries)
        .unwrap_or_else(|err| panic!("invalid queries in file '{path}': {err}"));

    let start_bytes: Vec<_> = (0..query.pattern_count())
        .map(|index| {
            (
                query.start_byte_for_pattern(index),
                query
                    .general_predicates(index)
                    .iter()
                    .filter_map(|predicate| match predicate.operator.as_ref() {
                        "lua-match?" => Some((
                            "#lua-match?",
                            (
                                "#match?",
                                vec![
                                    clone_predicate_arg(&predicate.args[0]),
                                    QueryPredicateArg::String(match &predicate.args[1] {
                                        QueryPredicateArg::String(str) => {
                                            lua_to_regex(str).into_boxed_str()
                                        }
                                        _ => panic!("second arg to #lua-match? must be string"),
                                    }),
                                ],
                            ),
                        )),
                        "not-lua-match?" => Some((
                            "#not-lua-match?",
                            (
                                "#not-match?",
                                vec![
                                    clone_predicate_arg(&predicate.args[0]),
                                    QueryPredicateArg::String(match &predicate.args[1] {
                                        QueryPredicateArg::String(str) => {
                                            lua_to_regex(str).into_boxed_str()
                                        }
                                        _ => panic!("second arg to #not-lua-match? must be string"),
                                    }),
                                ],
                            ),
                        )),
                        "any-of?" => Some((
                            "#any-of?",
                            (
                                "#match?",
                                vec![
                                    clone_predicate_arg(&predicate.args[0]),
                                    QueryPredicateArg::String(
                                        format!(
                                            "^({})$",
                                            predicate.args[1..]
                                                .iter()
                                                .map(|arg| match arg {
                                                    QueryPredicateArg::String(str) => str.as_ref(),
                                                    _ => panic!("args to #any-of? must be strings"),
                                                })
                                                .collect::<Vec<_>>()
                                                .join("|")
                                        )
                                        .into_boxed_str(),
                                    ),
                                ],
                            ),
                        )),
                        "contains?" => Some((
                            "#contains?",
                            (
                                "#match?",
                                vec![
                                    clone_predicate_arg(&predicate.args[0]),
                                    clone_predicate_arg(&predicate.args[1]),
                                ],
                            ),
                        )),
                        _ => None,
                    })
                    .collect::<HashMap<_, _>>(),
            )
        })
        .collect();
    let queries: String = start_bytes
        .iter()
        .enumerate()
        .map(|(index, (start, predicate_replacements))| {
            let mut q = match start_bytes.get(index + 1) {
                Some((end, _)) => &queries[*start..*end],
                None => &queries[*start..],
            }
            .to_string();
            for (predicate, replacement) in predicate_replacements {
                q = q.replace(
                    &format!(
                        "{}{}",
                        predicate,
                        q.split_once(predicate)
                            .expect("invalid replacement")
                            .1
                            .split_once(')')
                            .expect("invalid replacement")
                            .0
                    ),
                    &format!(
                        "{} {}",
                        replacement.0,
                        display_predicate_args(&query, &replacement.1)
                    ),
                );
            }
            q
        })
        .rev()
        .collect();
    queries
}

fn clone_predicate_arg(arg: &QueryPredicateArg) -> QueryPredicateArg {
    match arg {
        QueryPredicateArg::Capture(num) => QueryPredicateArg::Capture(*num),
        QueryPredicateArg::String(str) => QueryPredicateArg::String(str.clone()),
    }
}

fn display_predicate_args(query: &Query, args: &[QueryPredicateArg]) -> String {
    args.iter()
        .map(|arg| match arg {
            QueryPredicateArg::Capture(num) => {
                format!("@{}", query.capture_names()[*num as usize])
            }
            QueryPredicateArg::String(str) => format!("\"{str}\""),
        } + " ")
        .collect()
}

fn lua_to_regex(lua_pattern: &str) -> String {
    // TODO: correctly parse lua patterns (https://www.lua.org/pil/20.2.html and https://gitspartv.github.io/lua-patterns/)
    lua_pattern
        .replace('\\', r"\\")
        .replace("%.", r"\.")
        .replace("%%", r"%")
        .replace("%a", r"[a-zA-Z]")
        .replace("%A", r"[^a-zA-Z]")
        .replace("%c", r"[\0-\31]")
        .replace("%C", r"[^\0-\31]")
        .replace("%d", r"[0-9]")
        .replace("%D", r"[^0-9]")
        .replace("%g", r"[\33-\126]")
        .replace("%G", r"[^\33-\126]")
        .replace("%l", r"[a-z]")
        .replace("%L", r"[^a-z]")
        .replace("%p", r##"[!"#$%&'()*+,\-./:;<=>?@[\\\]^_`{|}~]"##)
        .replace("%P", r##"[^!"#$%&'()*+,\-./:;<=>?@[\\\]^_`{|}~]"##)
        .replace("%s", r"[ \t\n\v\f\r]")
        .replace("%S", r"[^ \t\n\v\f\r]")
        .replace("%u", r"[A-Z]")
        .replace("%U", r"[^A-Z]")
        .replace("%w", r"[a-zA-Z0-9]")
        .replace("%W", r"[^a-zA-Z0-9]")
        .replace("%x", r"[0-9a-fA-F]")
        .replace("%X", r"[^0-9a-fA-F]")
        .replace("%z", r"\0")
        .replace("%Z", r"[^\0]")
}
