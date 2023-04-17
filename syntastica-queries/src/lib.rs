use std::collections::HashMap;

use tree_sitter::{Language, Query, QueryPredicateArg};

macro_rules! langs {
    ($($feat:literal, $name:ident, process = $process:tt, injections = $injections:tt, locals = $locals:tt);* $(;)?) => {
        $(
            #[cfg(feature = $feat)]
            #[proc_macro]
            pub fn $name(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
                let _lang = syntastica_parsers::$name();
                let highlights = langs!(@process $process, _lang, $name, include_str!(concat!("../queries/", stringify!($name), "/highlights.scm")));
                let injections = langs!(@optional $injections, include_str!(concat!("../queries/", stringify!($name), "/injections.scm")));
                let locals = langs!(@optional $locals, include_str!(concat!("../queries/", stringify!($name), "/locals.scm")));
                quote::quote! { (#highlights, #injections, #locals) }.into()
            }
        )*
    };
    (@process true, $lang:ident, $name:ident, $val:expr) => { process_queries($lang, $val) };
    (@process false, $lang:ident, $name:ident, $val:expr) => { $val };
    (@optional true, $then:expr) => { $then };
    (@optional false, $then:expr) => { "" };
}

langs! {
    "some", rust, process = true, injections = true, locals = true;
    "some", python, process = true, injections = true, locals = true;
    "most", asm, process = true, injections = false, locals = false;
    "all", regex, process = false, injections = false, locals = false;
}

fn process_queries(lang: Language, source: &str) -> String {
    let query = Query::new(lang, source).unwrap_or_else(|err| panic!("invalid queries: {err}"));
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
                                            // TODO: correctly parse lua patterns (https://www.lua.org/pil/20.2.html and https://gitspartv.github.io/lua-patterns/)
                                            str.replace('\\', r"\\")
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
                                                .into_boxed_str()
                                        }
                                        _ => panic!("second arg to #lua-match? must be string"),
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
                Some((end, _)) => &source[*start..*end],
                None => &source[*start..],
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
