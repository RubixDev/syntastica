use std::collections::BTreeMap;

use anyhow::Result;
use once_cell::sync::Lazy;

static QUERIES_DIR: Lazy<String> =
    Lazy::new(|| format!("{}/queries", crate::WORKSPACE_DIR.display()));

pub fn make_queries() -> Result<BTreeMap<String, [String; 6]>> {
    let mut map = BTreeMap::new();
    for lang in &crate::LANGUAGE_CONFIG.languages {
        const CRATES_IO_SKIP_COMMENT: &str = "; crates.io skip";
        const NON_CRATES_IO_SKIP_COMMENT: &str = "; non-crates.io skip";
        let query_file =
            |func: fn(&str, bool, &str, &str) -> String, enabled: bool, strip_comment: &str| {
                if enabled {
                    func(
                        strip_comment,
                        lang.queries.nvim_like,
                        &lang.name,
                        &QUERIES_DIR,
                    )
                } else {
                    String::new()
                }
            };

        let highlights = query_file(
            syntastica_query_preprocessor::process_highlights_with_inherits,
            true,
            NON_CRATES_IO_SKIP_COMMENT,
        );
        let injections = query_file(
            syntastica_query_preprocessor::process_injections_with_inherits,
            lang.queries.injections,
            NON_CRATES_IO_SKIP_COMMENT,
        );
        let locals = query_file(
            syntastica_query_preprocessor::process_locals_with_inherits,
            lang.queries.locals,
            NON_CRATES_IO_SKIP_COMMENT,
        );

        let highlights_crates_io = query_file(
            syntastica_query_preprocessor::process_highlights_with_inherits,
            true,
            CRATES_IO_SKIP_COMMENT,
        );
        let injections_crates_io = query_file(
            syntastica_query_preprocessor::process_injections_with_inherits,
            lang.queries.injections,
            CRATES_IO_SKIP_COMMENT,
        );
        let locals_crates_io = query_file(
            syntastica_query_preprocessor::process_locals_with_inherits,
            lang.queries.locals,
            CRATES_IO_SKIP_COMMENT,
        );

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
