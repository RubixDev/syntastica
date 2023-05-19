macro_rules! lang_tests {
    ($($lang:ident),* $(,)?) => {
        $(
            #[test]
            fn $lang() {
                let lang = syntastica_parsers::$lang();
                const QUERIES: (&str, &str, &str) = syntastica_queries::$lang!();
                validate_query(lang, QUERIES.0, "highlights");
                validate_query(lang, QUERIES.1, "injections");
                validate_query(lang, QUERIES.2, "locals");
            }
        )*
    };
}

lang_tests! {
    c,
    cpp,
    css,
    go,
    html,
    java,
    javascript,
    json,
    python,
    rust,
    tsx,
    typescript,
    asm,
    regex,
}

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
