# syntastica-queries

Collection of tree-sitter queries for
[syntastica](https://crates.io/crates/syntastica)

This crate defines constants for three types of tree-sitter queries for lots of
parsers. It is intended to be used via
[syntastica-parsers](https://crates.io/crates/syntastica-parsers),
[syntastica-parsers-git](https://crates.io/crates/syntastica-parsers-git), or
[syntastica-parsers-gitdep](https://github.com/RubixDev/syntastica/tree/main/syntastica-parsers-gitdep).

The three types of queries are:

1. `highlights`: defining the highlight captures for nodes
2. `injections`: defining where other languages are injected for highlighting
3. `locals`: keeping track of scopes, variables, parameters, etc. to have
   occurrences of those be highlighted the same everywhere

The constants are defined as `<language_name>_<kind>[_CRATES_IO]` where `<kind>`
is one of `HIGHLIGHTS`, `INJECTIONS`, or `LOCALS`. The `INJECTIONS` and `LOCALS`
may be empty for some languages. The constants with the `_CRATES_IO` suffix aim
to be compatible with the latest version of the parser that was published on
[crates.io](https://crates.io). These are used by
[syntastica-parsers](https://crates.io/crates/syntastica-parsers), whereas the
normal queries are used by
[syntastica-parsers-git](https://crates.io/crates/syntastica-parsers-git) and
[syntastica-parsers-gitdep](https://github.com/RubixDev/syntastica/tree/main/syntastica-parsers-gitdep).

The crate source is automatically generated with `cargo xtask codegen` inside
the syntastica workspace.
