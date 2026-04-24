;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-svelte/ae5199db47757f785e43a14b332118a5474de1a2/queries/highlights.scm
; inherits: html
(raw_text) @none

[
  "as"
  "key"
  "html"
  "snippet"
  "render"
] @keyword

"const" @type.qualifier

[
  "if"
  "else"
  "then"
] @keyword.conditional

"each" @keyword.repeat

[
  "await"
  "then"
] @keyword.coroutine

"catch" @keyword.exception

"debug" @keyword.debug

[
  "{"
  "}"
] @punctuation.bracket

[
  "#"
  ":"
  "/"
  "@"
] @tag.delimiter
