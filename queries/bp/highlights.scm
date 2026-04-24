;; Forked from https://raw.githubusercontent.com/ambroisie/tree-sitter-bp/ee641d15390183d7535777947ce0f2f1fbcee69f/queries/highlights.scm
(comment) @comment

(operator) @operator

(integer_literal
  "-" @operator
)

[
  ","
  ":"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

(boolean_literal) @boolean

(integer_literal) @number

[
  (raw_string_literal)
  (interpreted_string_literal)
] @string

(escape_sequence) @string.escape

(identifier) @variable

(module
  type: (identifier) @function.call
)

(module
  (property
    field: (identifier) @variable.parameter
  )
)

[
  (unset)
  (default)
  (any)
] @variable.builtin

(condition
  name: (identifier) @function.builtin
)

(map_expression
  (property
    field: (identifier) @property
  )
)

(select_expression
  "select" @keyword.conditional
)
