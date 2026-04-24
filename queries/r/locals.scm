;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/r/locals.scm
;; Licensed under the Apache License 2.0
; locals.scm
(function_definition) @local.scope

(argument
  name: (identifier) @local.definition
)

(parameter
  name: (identifier) @local.definition
)

(binary_operator
  lhs: (identifier) @local.definition
  operator:
  "<-"
)

(binary_operator
  lhs: (identifier) @local.definition
  operator:
  "="
)

(binary_operator
  operator:
  "->"
  rhs: (identifier) @local.definition
)

(identifier) @local.reference
