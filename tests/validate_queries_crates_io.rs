use std::collections::HashMap;

use once_cell::sync::Lazy;
use syntastica::provider::LanguageProvider;
use tree_sitter::Language;

mod _shared;
use _shared::*;

syntastica_macros::queries_test_crates_io!();

static PARSERS: Lazy<HashMap<String, Language>> = Lazy::new(|| {
    syntastica_parsers::LanguageProviderImpl::all()
        .get_parsers()
        .unwrap()
});
