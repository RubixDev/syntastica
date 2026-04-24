;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-xcompose/a51d6366f041dbefec4da39a7eb3168a9b1cbc0e/queries/highlights.scm
(keysym) @constant

(text) @string

"include" @keyword

[
  (modifier)
  "None"
] @function

[
  (octal)
  (hex)
] @number

[
  "%L"
  "%H"
  "%S"
] @string.special

[
  "!"
  "~"
] @operator

[
  ":"
  "<"
  ">"
  "\""
] @punctuation.delimiter

(comment) @comment
