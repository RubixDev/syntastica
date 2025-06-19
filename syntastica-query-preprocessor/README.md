# `syntastica-query-preprocessor`

The tree-sitter query pre-processor used by
[`syntastica`](https://crates.io/crates/syntastica).

See
[the project overview](https://rubixdev.github.io/syntastica/syntastica/#general-side-products)
for more information.

## Overview

`syntastica-query-preprocessor` is a pre-processor for tree-sitter queries which
allows usage of `; inherits <lang>` comments, conditional skipping of nodes with
comments, usage of additional predicates like `lua-match?`, `contains?` and
`any-of?`, Neovim's old injections syntax, and order reversing for priority
flipping. The crate can be used to use queries designed for Neovim with the
official [tree-sitter rust bindings](https://crates.io/crates/tree-sitter) with
minimal manual changes.

## Usage

This crate exposes six public functions:

- [`process_highlights`]
- [`process_injections`]
- [`process_locals`]
- [`process_highlights_with_inherits`]
- [`process_injections_with_inherits`]
- [`process_locals_with_inherits`]

The first three are used for simple pre-processing of a single query source file
which is passed in as a `&str`.

The last three assume a directory structure of
`queries/<lang_name>/<highlights|injections|locals>.scm` (where `queries` is the
directory pointed to by the `base_dir` parameter), and read the queries from the
file system. This allows usage of `; inherits <lang_one>[,<lang_two>,...]`
comments inside the query files to include the contents of another language's
query file at the location of the comment. The `lang_name` parameter specifies
the directory name in which the entry query file is located.

All six functions take the two arguments `strip_comment` and `nvim_like`. The
`strip_comment` argument specifies a comment string that will indicate nodes to
be excluded from the output. This must include the leading `;`, so passing an
empty string has the effect of not excluding anything.

Setting `nvim_like` to `true` mainly does the following:

`#[not-]lua-match?`, `#[not-]any-of?` and `#[not-]contains?` predicates are
replaced with a `#match?` or `#not-match?` predicate expressing the same
constraint. Note that regular expressions cannot represent all Lua patterns, so
the conversion from `lua-match` to `match` may fail and panic.

Additional behavior differs for the kind of queries:

- ~~For highlight queries, the order of the queries is reversed. This is needed
  because Neovim prioritizes queries lower down in the file over earlier ones,
  whereas the official
  [tree-sitter Rust bindings](https://crates.io/crates/tree-sitter) do the
  opposite.~~ This is no longer the case, as `tree-sitter-highlight` and as such
  `syntastica-highlight` now also prioritize later queries over previous ones.
- For injection queries, Neovim used to use `@<lang_name>`, `@content`,
  `@language`, and `@combined` captures. These will be replaced with the new
  (and official) syntax using `#set! injection.language "<lang_name>"`,
  `@injection.content`, `@injection.language`, and `#set! injection.combined`
  respectively.
- For locals queries, Neovim used to use `@scope`, `@reference`, and various
  `@definition.<kind>` captures. These will be replaced by the new and official
  `@local.scope`, `@local.reference`, and `@local.definition` captures. Neovim
  now also uses captures in the form `@local.definition.<kind>`. These will also
  be trimmed to `@local.definition`.
