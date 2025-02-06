use syntastica::theme::THEME_KEYS;
use tree_sitter::{Language, Query};

pub fn validate_query(lang: &Language, query: &str, kind: &str) {
    match Query::new(lang, query) {
        Ok(query) => {
            for predicate in
                (0..query.pattern_count()).flat_map(|idx| query.general_predicates(idx))
            {
                let predicate_name = predicate.operator.as_ref();
                if ![
                    "set!",
                    "any-of?",
                    "not-any-of?",
                    "contains?",
                    "not-contains?",
                    "lua-match?",
                    "not-lua-match?",
                    // TODO: check whether the following predicates are actually supported
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
            for name in query.capture_names() {
                if !name.starts_with('_')
                    && !THEME_KEYS.contains(name)
                    && ![
                        "injection.content",
                        "injection.language",
                        "content",
                        "language",
                        "combined",
                        "local.scope",
                        "local.reference",
                        "local.definition",
                        "scope",
                        "reference",
                        "definition",
                    ]
                    .contains(name)
                    && !name.starts_with("local.definition.")
                    && !name.starts_with("definition.")
                {
                    panic!("{kind} queries use unsupported capture name '{name}'");
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
                    true => eprintln!(
                        "--> {}\x1b[1;31m{}\x1b[0m{}",
                        &line[..err.column],
                        &line[err.column..(err.column + 10).min(line.len())],
                        &line[(err.column + 10).min(line.len())..]
                    ),
                    false => eprintln!("    {line}"),
                }
            }
            eprintln!("{}\n", "-".repeat(50));

            panic!("invalid {kind} queries: {err}");
        }
    }
}
