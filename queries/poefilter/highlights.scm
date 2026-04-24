;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-poe-filter/205a7d576984feb38a9fc2d8cfe729617f9e0548/queries/highlights.scm
[
  "Show"
  "Hide"
  "Minimal"
] @module

[
  "Import"
  "Optional"
] @keyword

(condition
  (name) @keyword
)

(action
  (name) @keyword
)

(continue) @keyword

(operator) @operator

(string) @string

(file) @string.special

[
  (quality)
  (rarity)
  (influence)
  (colour)
  (shape)
] @constant.builtin

(sockets) @variable.builtin

(number) @number

(boolean) @boolean

[
  (disable)
  "Temp"
] @constant

(comment) @comment

"\"" @punctuation.delimiter
