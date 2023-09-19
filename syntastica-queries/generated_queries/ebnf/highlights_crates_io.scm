[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

"=" @keyword.operator

[
  "|"
  "*"
  "-"
] @operator

[
  ";"
  ","
] @punctuation.delimiter

(
  (identifier) @constant
  (#match? @constant "^[A-Z][[A-Z][0-9]_]+$")
)

(
  (identifier) @symbol
  (#match? @symbol "^[a-z]")
)

(
  (identifier) @type
  (#match? @type "^[A-Z]")
)

(comment) @comment

(integer) @number

(special_sequence) @string.special.grammar

(terminal) @string.grammar
