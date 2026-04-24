;; Forked from https://raw.githubusercontent.com/tree-sitter/tree-sitter-json/001c28d7a29832b06b0e831ec77845553c89b56d/queries/highlights.scm
(pair
  key: (_) @string.special.key
)

(string) @string

(number) @number

[
  (null)
  (true)
  (false)
] @constant.builtin

(escape_sequence) @escape

(comment) @comment
