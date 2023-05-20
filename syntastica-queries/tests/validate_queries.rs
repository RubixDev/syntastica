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
    if let Err(err) = Query::new(lang, query) {
        eprintln!("invalid {kind} queries: {err}");

        eprintln!("\n{}", "-".repeat(50));
        let context = query
            .lines()
            .enumerate()
            .skip(err.row.saturating_sub(5))
            .take(11);
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
