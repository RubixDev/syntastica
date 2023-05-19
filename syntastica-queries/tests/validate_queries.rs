syntastica_macros::queries_test!();

fn validate_query(lang: tree_sitter::Language, query: &str, kind: &str) {
    if let Err(err) = tree_sitter::Query::new(lang, query) {
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
