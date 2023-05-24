use std::{collections::BTreeMap, fs};

use anyhow::Result;
use fancy_regex::Regex;
use once_cell::sync::Lazy;
use rsexpr::{OwnedSexpr, OwnedSexprs};
use syntastica::providers::ParserProvider;
use tree_sitter::{Language, Query};

static QUERIES_DIR: Lazy<String> =
    Lazy::new(|| format!("{}/queries", crate::WORKSPACE_DIR.display()));
static INHERITS_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r";+\s*inherits\s*:?\s*([a-z_,()-]+)\s*").unwrap());

pub fn make_queries() -> Result<BTreeMap<&'static str, [String; 3]>> {
    syntastica_macros::queries!()
}

fn validate(
    lang: Language,
    lang_name: &str,
    filename: &str,
    processor: impl Fn(Vec<OwnedSexpr>) -> Vec<OwnedSexpr>,
) -> String {
    // read input
    let path = format!("{}/{lang_name}/{filename}", *QUERIES_DIR);
    let queries = read_queries(lang_name, filename);

    // validate input
    if let Err(err) = Query::new(lang, &queries) {
        eprintln!("warning: invalid input queries in file '{path}': {err}");
    }

    // run processor
    let mut new_queries = processor(group_root_level_captures(
        rsexpr::from_slice_multi(&queries)
            .unwrap_or_else(|errs| {
                panic!(
                    "invalid queries in file '{path}': {}",
                    errs.iter()
                        .map(rsexpr::Error::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
            .into_iter()
            .map(OwnedSexpr::from)
            .collect(),
    ))
    .into_iter()
    .map(|tree| format!("{tree:#}"))
    .collect::<Vec<_>>()
    .join("\n\n");
    new_queries.push('\n');

    // validate output
    if let Err(err) = Query::new(lang, &new_queries) {
        eprintln!(
            "warning: processing queries in file '{path}' resulted in invalid queries: {err}"
        );
    }

    new_queries
}

fn read_queries(lang_name: &str, filename: &str) -> String {
    let path = format!("{}/{lang_name}/{filename}", *QUERIES_DIR);
    let queries = match fs::read_to_string(&path) {
        Ok(queries) => queries,
        Err(err) => {
            eprintln!("warning: failed to read '{path}': {err}");
            String::new()
        }
    };
    INHERITS_REGEX
        .replace_all(&queries, |captures: &fancy_regex::Captures| {
            captures[1]
                .split(',')
                .map(|lang| format!("\n{}\n", read_queries(lang.trim(), filename)))
                .collect::<String>()
        })
        .into_owned()
}

fn group_root_level_captures(queries: Vec<OwnedSexpr>) -> Vec<OwnedSexpr> {
    let mut new_queries = Vec::with_capacity(queries.len());
    let mut iter = queries.into_iter().peekable();

    // groups start with `List`, `Group`, or `String` nodes
    while let Some(sexp @ (OwnedSexpr::List(_) | OwnedSexpr::Group(_) | OwnedSexpr::String(_))) =
        iter.next()
    {
        let mut group = OwnedSexprs::from(vec![sexp]);
        // and include all following `Atom` nodes
        while let Some(OwnedSexpr::Atom(_)) = iter.peek() {
            group.push(iter.next().unwrap());
        }
        new_queries.push(match group.len() {
            // if the group only consists of one item, there is no need to wrap it
            1 => group.swap_remove(0),
            _ => OwnedSexpr::List(group),
        });
    }

    new_queries
}

fn process_locals(mut queries: Vec<OwnedSexpr>) -> Vec<OwnedSexpr> {
    for query in &mut queries {
        replace_locals_captures(query);
        replace_predicates(query);
    }
    queries
}

fn replace_locals_captures(tree: &mut OwnedSexpr) {
    match tree {
        OwnedSexpr::Atom(atom) => match atom.as_slice() {
            b"@scope" => *atom = b"@local.scope".to_vec(),
            b"@reference" => *atom = b"@local.reference".to_vec(),
            other => {
                match std::str::from_utf8(other)
                    .ok()
                    .and_then(|str| str.split('.').next())
                {
                    Some("@definition") => *atom = b"@local.definition".to_vec(),
                    Some(_) | None => {}
                }
            }
        },
        OwnedSexpr::String(_) => {}
        OwnedSexpr::List(list) | OwnedSexpr::Group(list) => {
            for subtree in list {
                replace_locals_captures(subtree);
            }
        }
    }
}

fn process_injections(mut queries: Vec<OwnedSexpr>) -> Vec<OwnedSexpr> {
    for query in &mut queries {
        replace_injection_captures(query, 0);
        replace_predicates(query);
    }
    queries
}

fn replace_injection_captures(
    tree: &mut OwnedSexpr,
    mut predicate_count: usize,
) -> (bool, Option<OwnedSexpr>) {
    let mut is_predicate = false;
    let mut additional_sexp = None;
    match tree {
        OwnedSexpr::String(_) => {}
        OwnedSexpr::Atom(atom) => match atom.as_slice() {
            [b'@', capture @ ..] if !capture.starts_with(b"_") => match capture {
                b"injection.content" | b"injection.language" => {}
                b"content" => *atom = b"@injection.content".to_vec(),
                b"language" => *atom = b"@injection.language".to_vec(),
                lang_name => {
                    if predicate_count == 0 {
                        additional_sexp = Some(OwnedSexpr::List(
                            vec![
                                OwnedSexpr::Atom(b"#set!".to_vec()),
                                OwnedSexpr::Atom(b"injection.language".to_vec()),
                                OwnedSexpr::String(lang_name.to_owned()),
                            ]
                            .into(),
                        ));
                    }
                    *atom = b"@injection.content".to_vec();
                }
            },
            [b'#', ..] => is_predicate = true,
            _ => {}
        },
        OwnedSexpr::List(subtrees) | OwnedSexpr::Group(subtrees) => {
            let mut insertions = vec![];
            for (index, subtree) in subtrees.iter_mut().enumerate() {
                let (is_predicate, additional_sexp) =
                    replace_injection_captures(subtree, predicate_count);
                if is_predicate {
                    predicate_count += 1;
                }
                if let Some(additional_sexp) = additional_sexp {
                    insertions.push((index + 1 + insertions.len(), additional_sexp));
                }
            }
            for (index, sexp) in insertions {
                subtrees.insert(index, sexp);
            }
        }
    }
    (is_predicate, additional_sexp)
}

fn process_highlights(mut queries: Vec<OwnedSexpr>) -> Vec<OwnedSexpr> {
    queries.reverse();

    for query in &mut queries {
        replace_predicates(query);
    }

    queries
}

fn replace_predicates(tree: &mut OwnedSexpr) {
    if let OwnedSexpr::List(list) | OwnedSexpr::Group(list) = tree {
        match list.first() {
            Some(OwnedSexpr::Atom(atom)) if atom.first() == Some(&b'#') => {
                let match_predicate = OwnedSexpr::Atom(match &atom[..4] == b"#not" {
                    false => b"#match?".to_vec(),
                    true => b"#not-match?".to_vec(),
                });
                match atom.as_slice() {
                    b"#lua-match?" | b"#not-lua-match?" => {
                        list[0] = match_predicate;
                        list[2] = OwnedSexpr::String(
                            lua_to_regex(std::str::from_utf8(list[2].unwrap_string_ref()).unwrap())
                                .into_bytes(),
                        );
                        list.truncate(3);
                    }
                    b"#any-of?" | b"#not-any-of?" => {
                        list[0] = match_predicate;
                        list[2] = OwnedSexpr::String(
                            format!(
                                    "^({})$",
                                    list[2..]
                                        .iter()
                                        .map(|arg| std::str::from_utf8(arg.unwrap_string_ref())
                                            .unwrap())
                                        .collect::<Vec<_>>()
                                        .join("|")
                                )
                            .into_bytes(),
                        );
                        list.truncate(3);
                    }
                    b"#contains?" | b"#not-contains?" => list[0] = match_predicate,
                    _ => {}
                }
            }
            _ => {
                for subtree in list {
                    replace_predicates(subtree);
                }
            }
        }
    }
}

fn lua_to_regex(pattern: &str) -> String {
    lua_pattern::try_to_regex(
        &lua_pattern::parse(pattern)
            .unwrap_or_else(|err| panic!("Lua pattern `{pattern}` could not be parsed: {err}")),
        false,
        false,
    )
    .unwrap_or_else(|err| {
        panic!("Lua pattern `{pattern}` could not be converted into a regex: {err}")
    })
}
