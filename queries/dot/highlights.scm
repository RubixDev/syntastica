;; Forked from https://raw.githubusercontent.com/rydesun/tree-sitter-dot/80327abbba6f47530edeb0df9f11bd5d5c93c14d/queries/highlights.scm
[
  "strict"
  "graph"
  "digraph"
  "subgraph"
  "node"
  "edge"
] @keyword

(string_literal) @string

(number_literal) @number

[
  (edgeop)
  (operator)
] @operator

[
  ","
  ";"
] @punctuation.delimiter

[
  "{"
  "}"
  "["
  "]"
  "<"
  ">"
] @punctuation.bracket

(subgraph
  id: (id
    (identifier) @namespace
  )
)

(attribute
  name: (id
    (identifier) @type
  )
)

(attribute
  value: (id
    (identifier) @constant
  )
)

[
  (comment)
  (preproc)
] @comment

(ERROR) @error

(identifier) @variable
