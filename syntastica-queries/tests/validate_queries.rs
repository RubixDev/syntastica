use std::collections::HashMap;

use once_cell::sync::Lazy;
use syntastica::providers::ParserProvider;
use tree_sitter::{Language, Query};

syntastica_macros::queries_test!();

static PARSERS: Lazy<HashMap<String, Language>> = Lazy::new(|| {
    syntastica_parsers_git::ParserProviderGit
        .get_parsers()
        .unwrap()
});

fn validate_query(lang: Language, query: &str, kind: &str) {
    match Query::new(lang, query) {
        Ok(query) => {
            for predicate in
                (0..query.pattern_count()).flat_map(|idx| query.general_predicates(idx))
            {
                let predicate_name = predicate.operator.as_ref();
                if ![
                    "set!",
                    "any-of?",
                    "contains?",
                    "lua-match?",
                    "not-lua-match?",
                    // TODO: validate whether the following predicates are actually supported
                    "offset!",
                    "has-ancestor?",
                    "not-has-ancestor?",
                    "has-parent?",
                    "not-has-parent?",
                ]
                .contains(&predicate_name)
                {
                    panic!("{kind} queries use unsupported predicate '{predicate_name}'");
                }
            }
        }
        Err(err) => {
            eprintln!("invalid {kind} queries: {err}");

            eprintln!("\n{}", "-".repeat(50));
            let context = query
                .lines()
                .enumerate()
                .skip(err.row.saturating_sub(10))
                .take(21);
            for (row, line) in context {
                match row == err.row {
                    true => eprintln!("--> {line}"),
                    false => eprintln!("    {line}"),
                }
            }
            eprintln!("{}\n", "-".repeat(50));

            panic!("invalid {kind} queries: {err}");
        }
    }
}
