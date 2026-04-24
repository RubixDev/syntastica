;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-ungrammar/debd26fed283d80456ebafa33a06957b0c52e451/queries/highlights.scm
(comment) @comment @spell

(definition) @keyword

(identifier) @variable

(label_name) @label

(token) @string

[
  "="
  "|"
] @operator

[
  "*"
  "?"
] @operator.repeat

[":"] @punctuation.delimiter

[
  "("
  ")"
] @punctuation.bracket

(ERROR) @error
