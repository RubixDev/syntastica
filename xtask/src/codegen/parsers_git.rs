use std::fs;

use anyhow::Result;

use crate::schema::Group;

pub fn write() -> Result<()> {
    let toml_path = crate::WORKSPACE_DIR.join("syntastica-parsers-git/Cargo.toml");
    let mut toml = fs::read_to_string(&toml_path)?;

    if let Some((preserve, _)) = toml.split_once(super::TOML_AUTOGEN_HEADER) {
        toml.truncate(preserve.len());
    }
    toml += super::TOML_AUTOGEN_HEADER;

    toml += r###"
[features]
#! ## Features
default = ["runtime-c"]

#! Every supported language has a feature with the same name as the respective public function.
#! Additionally the three feature groups
#! <span class="stab portability"><code>some</code></span>,
#! <span class="stab portability"><code>most</code></span>, and
#! <span class="stab portability"><code>all</code></span>
#! are available.

## Include parsers for the most widely known supported languages.
"###;

    toml += &super::parsers_toml_feature(Group::Some);
    toml += super::TOML_FEATURES_MOST;
    toml += &super::parsers_toml_feature(Group::Most);
    toml += super::TOML_FEATURES_ALL;
    toml += &super::parsers_toml_feature(Group::All);

    toml += r###"
## Use the standard tree-sitter C runtime. See `syntastica`'s
## [WebAssembly support](https://rubixdev.github.io/syntastica/syntastica/#webassembly-support)
## for more information.
runtime-c = ["syntastica-core/runtime-c"]
## Use the pure Rust tree-sitter runtime. See `syntastica`'s
## [WebAssembly support](https://rubixdev.github.io/syntastica/syntastica/#webassembly-support)
## for more information.
runtime-c2rust = ["syntastica-core/runtime-c2rust"]
"###;

    toml += super::TOML_FEATURES_DOCS;

    toml += &super::parsers_toml_lang_features(super::ParserCollection::Git);

    fs::write(&toml_path, toml)?;

    Ok(())
}
