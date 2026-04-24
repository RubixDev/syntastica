;; Forked from https://raw.githubusercontent.com/mleonidas/tree-sitter-authzed/83e5c26a8687eb4688fe91d690c735cc3d21ad81/queries/highlights.scm
; Keywords
[
  (caveat_literal)
  (definition_literal)
] @keyword

"import" @keyword.import

"as" @keyword

; Boolean literals
[
  (true)
  (false)
] @boolean

; Constants
(nil) @constant.builtin

; Comments
[
  (hash_literal)
  (comment)
] @comment

; Operators
[
  (plus_literal)
  (minus_literal)
  (amp_literal)
  (pipe_literal)
] @operator

(stabby) @operator

; Binary operators
(binary_expression
  operator: [
    "=="
    "!="
    "<"
    "<="
    ">"
    ">="
    "in"
    "&&"
    "||"
  ] @operator
)

; Top-level identifiers (definition/caveat names)
(definition
  name: (identifier) @function
)

(caveat
  name: (identifier) @function
)

; Block structure with function.builtin
(block
  (relation
    (relation_literal) @function.builtin
    relation_name: (identifier) @constant
  )
)

(block
  (permission
    (permission_literal) @variable.builtin
    param_name: (identifier) @type
  )
)

; Relations
(rel_expression
  (identifier) @type
)

(relation
  (rel_expression
    (hash_literal)
    .
    (identifier) @constant
  )
)

; Permissions
(perm_expression
  (identifier) @property
)

; Function method calls
(call_expression
  function: (selector_expression
    operand: (identifier) @constant
    field: (field_identifier) @function.method
  )
)

(perm_expression
  (stabby) @operator
  .
  (identifier) @function
)

; String literals
[
  (raw_string_literal)
  (interpreted_string_literal)
] @string

; Import statements
(import_statement
  path: (_) @string
  alias: (identifier) @variable
)

; Parameters and types
(parameter_declaration
  name: (identifier) @parameter
  type: (_) @type
)

(generic_type
  base_type: (identifier) @type
)

; Built-in types
[
  "any"
  "int"
  "uint"
  "bool"
  "string"
  "double"
  "bytes"
  "duration"
  "timestamp"
] @type.builtin

; Wildcards
(wildcard_literal) @operator

(wildcard_type) @type.builtin

; Numbers
[
  (int_literal)
  (float_literal)
  (imaginary_literal)
] @number
