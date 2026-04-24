;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/sproto/highlights.scm
;; Licensed under the Apache License 2.0
(comment) @comment @spell

[
  "."
  ":"
] @punctuation.delimiter

"*" @operator

[
  "request"
  "response"
] @keyword

(type_definition
  name: (identifier) @type
)

(nested_type_definition
  name: (identifier) @type
)

(type_specifier) @type

[
  "integer"
  "boolean"
  "string"
  "binary"
  "double"
] @type.builtin

(protocol_definition
  name: (identifier) @function
)

(field_definition
  name: (identifier) @property
)

(map_specifier
  key: (identifier) @property
)

(integer) @number

[
  "("
  ")"
  "{"
  "}"
] @punctuation.bracket
