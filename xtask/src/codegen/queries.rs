use std::{collections::BTreeMap, fs};

use anyhow::Result;
use fancy_regex::Regex;
use once_cell::sync::Lazy;
use rsexpr::{OwnedSexpr, OwnedSexprs};
use syntastica_core::language_set::LanguageSet;
use syntastica_parsers_git::LanguageSetImpl;
use tree_sitter::{Language, Query};

static QUERIES_DIR: Lazy<String> =
    Lazy::new(|| format!("{}/queries", crate::WORKSPACE_DIR.display()));
static INHERITS_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r";+\s*inherits\s*:?\s*([a-z_,()-]+)\s*").unwrap());

pub fn make_queries() -> Result<BTreeMap<String, [String; 6]>> {
    let set = LanguageSetImpl::new();
    let mut map = BTreeMap::new();

    for lang in &crate::LANGUAGE_CONFIG.languages {
        let ts_lang = set.get_language(&lang.name)?.language;

        let query_file =
            |enabled: bool, filename: &str, func: fn(&mut OwnedSexprs), crates_io: bool| match (
                lang.queries.nvim_like,
                enabled,
            ) {
                (true, true) => validate(ts_lang, &lang.name, filename, Some(func), crates_io),
                (false, true) => validate(ts_lang, &lang.name, filename, None, crates_io),
                (_, false) => String::new(),
            };

        let highlights = query_file(true, "highlights.scm", process_highlights, false);
        let injections = query_file(
            lang.queries.injections,
            "injections.scm",
            process_injections,
            false,
        );
        let locals = query_file(lang.queries.locals, "locals.scm", process_locals, false);

        let highlights_crates_io = query_file(true, "highlights.scm", process_highlights, true);
        let injections_crates_io = query_file(
            lang.queries.injections,
            "injections.scm",
            process_injections,
            true,
        );
        let locals_crates_io = query_file(lang.queries.locals, "locals.scm", process_locals, true);

        map.insert(
            lang.name.clone(),
            [
                highlights,
                injections,
                locals,
                highlights_crates_io,
                injections_crates_io,
                locals_crates_io,
            ],
        );
    }

    Ok(map)
}

fn validate(
    lang: Language,
    lang_name: &str,
    filename: &str,
    processor: Option<fn(&mut OwnedSexprs)>,
    crates_io: bool,
) -> String {
    // read input
    let path = format!("{}/{lang_name}/{filename}", *QUERIES_DIR);
    let queries = read_queries(lang_name, filename);

    // validate input
    if let Err(err) = Query::new(lang, &queries) {
        eprintln!("warning: invalid input queries in file '{path}': {err}");
    }

    // run processor
    let mut new_queries = rsexpr::from_slice_multi(&queries)
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
        .collect();
    new_queries = group_root_level_captures(new_queries);
    if crates_io {
        strip_for_crates_io(&mut new_queries);
    }
    remove_comments(&mut new_queries);
    if let Some(func) = processor {
        func(&mut new_queries);
    }
    new_queries = ungroup_root_level_captures(new_queries);
    let new_queries = format!("{new_queries:#}");

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

fn group_root_level_captures(queries: OwnedSexprs) -> OwnedSexprs {
    let mut new_queries = OwnedSexprs::from(Vec::with_capacity(queries.len()));
    let mut iter = queries.into_iter().peekable();

    while let Some(sexpr) = iter.next() {
        // groups start with `List`, `Group`, `String`, or `Comment` nodes
        if let OwnedSexpr::List(_) | OwnedSexpr::Group(_) | OwnedSexpr::String(_) = sexpr {
            let mut group = OwnedSexprs::from(vec![sexpr]);
            // and include all following `Atom` nodes
            while let Some(OwnedSexpr::Atom(_)) = iter.peek() {
                group.push(iter.next().unwrap());
            }
            new_queries.push(match group.len() {
                // if the group only consists of one item, there is no need to wrap it
                1 => group.swap_remove(0),
                _ => OwnedSexpr::List(group),
            });
        } else {
            new_queries.push(sexpr);
        }
    }

    new_queries
}

fn ungroup_root_level_captures(queries: OwnedSexprs) -> OwnedSexprs {
    let mut new_queries = OwnedSexprs::from(Vec::with_capacity(queries.len()));

    for query in queries {
        match query {
            OwnedSexpr::List(list)
                if list
                    .first()
                    .map_or(false, |sexpr| !matches!(sexpr, OwnedSexpr::Atom(_)))
                    && list
                        .iter()
                        .filter(|sexpr| matches!(sexpr, OwnedSexpr::List(_) | OwnedSexpr::Group(_)))
                        .count()
                        <= 1 =>
            {
                new_queries.extend(list);
            }
            // remove empty groups
            OwnedSexpr::List(list) if list.is_empty() => {}
            _ => new_queries.push(query),
        }
    }

    new_queries
}

// TODO: preserve "Forked from" comments
fn remove_comments(queries: &mut OwnedSexprs) {
    queries.retain(|sexpr| !matches!(sexpr, OwnedSexpr::Comment(_)));
    for query in queries {
        if let OwnedSexpr::List(children) | OwnedSexpr::Group(children) = query {
            remove_comments(children);
        }
    }
}

fn strip_for_crates_io(queries: &mut OwnedSexprs) {
    let mut delete_next = false;
    queries.retain(|query| {
        let delete_this = delete_next;
        delete_next =
            matches!(query, OwnedSexpr::Comment(comment) if comment == b"; crates.io skip");
        !delete_this
    });

    for query in queries {
        if let OwnedSexpr::List(children) | OwnedSexpr::Group(children) = query {
            strip_for_crates_io(children);
        }
    }
}

fn process_locals(queries: &mut OwnedSexprs) {
    for query in queries {
        replace_locals_captures(query);
        replace_predicates(query);
    }
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
        OwnedSexpr::Comment(_) => {}
        OwnedSexpr::List(list) | OwnedSexpr::Group(list) => {
            for subtree in list {
                replace_locals_captures(subtree);
            }
        }
    }
}

fn process_injections(queries: &mut OwnedSexprs) {
    for query in queries {
        replace_injection_captures(query, 0);
        replace_predicates(query);
    }
}

fn replace_injection_captures(
    tree: &mut OwnedSexpr,
    mut predicate_count: usize,
) -> (bool, Option<OwnedSexpr>) {
    let mut is_predicate = false;
    let mut additional_sexp = None;
    match tree {
        OwnedSexpr::String(_) => {}
        OwnedSexpr::Comment(_) => {}
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

fn process_highlights(queries: &mut OwnedSexprs) {
    queries.reverse();
    for query in queries {
        replace_predicates(query);
    }
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
