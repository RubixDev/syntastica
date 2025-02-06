#![doc = include_str!("../README.md")]
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

use std::{fmt::Write as _, fs};

use lazy_regex::regex_replace_all;
use rsexpr::{OwnedSexpr, OwnedSexprs};

#[inline]
fn _process(strip_comment: &str, nvim_like: bool, src: &str, proc: fn(&mut OwnedSexprs)) -> String {
    process(
        src,
        if nvim_like { Some(proc) } else { None },
        strip_comment,
    )
    .unwrap_or_else(|err| panic!("invalid input queries: {err}"))
}

/// Pre-process highlight queries.
///
/// See [the crate docs](crate#usage) for information on the parameters.
///
/// # Panics
/// The function panics if the query string cannot be parsed by [`rsexpr`].
#[inline]
pub fn process_highlights(strip_comment: &str, nvim_like: bool, src: &str) -> String {
    _process(strip_comment, nvim_like, src, _process_highlights)
}

/// Pre-process injection queries.
///
/// See [the crate docs](crate#usage) for information on the parameters.
///
/// # Panics
/// The function panics if the query string cannot be parsed by [`rsexpr`].
#[inline]
pub fn process_injections(strip_comment: &str, nvim_like: bool, src: &str) -> String {
    _process(strip_comment, nvim_like, src, _process_injections)
}

/// Pre-process locals queries.
///
/// See [the crate docs](crate#usage) for information on the parameters.
///
/// # Panics
/// The function panics if the query string cannot be parsed by [`rsexpr`].
#[inline]
pub fn process_locals(strip_comment: &str, nvim_like: bool, src: &str) -> String {
    _process(strip_comment, nvim_like, src, _process_locals)
}

#[inline]
fn _process_with_inherits(
    strip_comment: &str,
    nvim_like: bool,
    lang_name: &str,
    base_dir: &str,
    proc: fn(&mut OwnedSexprs),
    filename: &str,
) -> String {
    process_with_inherits(
        base_dir,
        lang_name,
        filename,
        if nvim_like { Some(proc) } else { None },
        strip_comment,
    )
}

/// Pre-process highlight queries with support for `; inherits <lang>` comments.
///
/// See [the crate docs](crate#usage) for information on the parameters.
///
/// # Panics
/// The function panics if any query file cannot be parsed by [`rsexpr`].
#[inline]
pub fn process_highlights_with_inherits(
    strip_comment: &str,
    nvim_like: bool,
    lang_name: &str,
    base_dir: &str,
) -> String {
    _process_with_inherits(
        strip_comment,
        nvim_like,
        lang_name,
        base_dir,
        _process_highlights,
        "highlights.scm",
    )
}

/// Pre-process injection queries with support for `; inherits <lang>` comments.
///
/// See [the crate docs](crate#usage) for information on the parameters.
///
/// # Panics
/// The function panics if any query file cannot be parsed by [`rsexpr`].
#[inline]
pub fn process_injections_with_inherits(
    strip_comment: &str,
    nvim_like: bool,
    lang_name: &str,
    base_dir: &str,
) -> String {
    _process_with_inherits(
        strip_comment,
        nvim_like,
        lang_name,
        base_dir,
        _process_injections,
        "injections.scm",
    )
}

/// Pre-process locals queries with support for `; inherits <lang>` comments.
///
/// See [the crate docs](crate#usage) for information on the parameters.
///
/// # Panics
/// The function panics if any query file cannot be parsed by [`rsexpr`].
#[inline]
pub fn process_locals_with_inherits(
    strip_comment: &str,
    nvim_like: bool,
    lang_name: &str,
    base_dir: &str,
) -> String {
    _process_with_inherits(
        strip_comment,
        nvim_like,
        lang_name,
        base_dir,
        _process_locals,
        "locals.scm",
    )
}

fn process(
    src: &str,
    processor: Option<fn(&mut OwnedSexprs)>,
    strip_comment: &str,
) -> Result<String, String> {
    let mut new_queries = rsexpr::from_slice_multi(src)
        .map_err(|errs| {
            errs.iter()
                .map(rsexpr::Error::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        })?
        .into_iter()
        .map(OwnedSexpr::from)
        .collect();
    new_queries = group_root_level_captures(new_queries);
    strip(&mut new_queries, strip_comment.as_bytes());
    remove_comments(&mut new_queries);
    if let Some(func) = processor {
        func(&mut new_queries);
    }
    new_queries = ungroup_root_level_captures(new_queries);
    let new_queries = format!("{new_queries}");

    Ok(new_queries)
}

fn process_with_inherits(
    base_dir: &str,
    lang_name: &str,
    filename: &str,
    processor: Option<fn(&mut OwnedSexprs)>,
    strip_comment: &str,
) -> String {
    let queries = read_queries(base_dir, lang_name, filename);
    process(&queries, processor, strip_comment).unwrap_or_else(|err| {
        panic!("invalid queries in file '{base_dir}/{lang_name}/{filename}': {err}")
    })
}

fn read_queries(base_dir: &str, lang_name: &str, filename: &str) -> String {
    let path = format!("{base_dir}/{lang_name}/{filename}");
    let queries = match fs::read_to_string(&path) {
        Ok(queries) => queries,
        Err(err) => {
            eprintln!("warning: failed to read '{path}': {err}");
            String::new()
        }
    };
    regex_replace_all!(
        r";+\s*inherits\s*:?\s*([a-z_,()-]+)\s*",
        &queries,
        |_, langs: &str| {
            langs.split(',').fold(String::new(), |mut out, lang| {
                _ = write!(out, "\n{}\n", read_queries(base_dir, lang.trim(), filename));
                out
            })
        }
    )
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
            // remove empty groups
            OwnedSexpr::List(list) if list.is_empty() => {}
            OwnedSexpr::List(list)
                if list
                    .first()
                    .map_or(false, |sexpr| !matches!(sexpr, OwnedSexpr::Atom(_)))
                    && list
                        .iter()
                        .filter(|sexpr| matches!(sexpr, OwnedSexpr::List(_) | OwnedSexpr::Group(_)))
                        .count()
                        <= 1 - list
                            .first()
                            .map_or(false, |sexpr| matches!(sexpr, OwnedSexpr::String(_)))
                            as usize =>
            {
                new_queries.extend(list);
            }
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

fn strip(queries: &mut OwnedSexprs, skip_comment: &[u8]) {
    let mut delete_next = false;
    queries.retain(|query| {
        let delete_this = delete_next;
        delete_next = matches!(query, OwnedSexpr::Comment(comment) if comment == skip_comment);
        !delete_this
    });

    for query in queries {
        if let OwnedSexpr::List(children) | OwnedSexpr::Group(children) = query {
            strip(children, skip_comment);
        }
    }
}

fn _process_locals(queries: &mut OwnedSexprs) {
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
                if let Some("@definition") = std::str::from_utf8(other)
                    .ok()
                    .and_then(|str| str.split('.').next())
                {
                    *atom = b"@local.definition".to_vec()
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

fn _process_injections(queries: &mut OwnedSexprs) {
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
                b"combined" => {
                    *tree = OwnedSexpr::List(
                        vec![
                            OwnedSexpr::Atom(b"#set!".to_vec()),
                            OwnedSexpr::Atom(b"injection.combined".to_vec()),
                        ]
                        .into(),
                    )
                }
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

fn _process_highlights(queries: &mut OwnedSexprs) {
    queries.reverse();
    for query in queries {
        replace_predicates(query);
    }
}

fn replace_predicates(tree: &mut OwnedSexpr) {
    if let OwnedSexpr::List(list) | OwnedSexpr::Group(list) = tree {
        match list.first() {
            Some(OwnedSexpr::Atom(atom)) if atom.first() == Some(&b'#') => {
                let match_predicate = OwnedSexpr::Atom(match atom.starts_with(b"#not-") {
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
