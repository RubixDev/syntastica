# `syntastica-parsers-dynamic`

A parser “collection” for `syntastica` which loads languages at runtime.

See
[the project overview](https://rubixdev.github.io/syntastica/syntastica/#parser-collections)
for more information on all parser collections.

This parser “collection” doesn't actually include any parsers. Instead, it
provides a [`LanguageLoader`] that implements [`LanguageSet`] by loading
languages at runtime from a given set of directories, very similar to how
[`tree-sitter-loader`](https://crates.io/crates/tree-sitter-loader) does, which
is used by the tree-sitter CLI.
